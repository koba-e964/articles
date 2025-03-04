# SECCON CTF 13 Final
ZK Lovers で参加した。国内 4 位だった。

## [crypto] RSA+
### 問題
好きな素数 $p, q$ をジャッジに送る。ジャッジは 512 bit の一様乱数 $x$ を生成し、$n=pq, g = \lfloor n/2 \rfloor, h = \lfloor n/3 \rfloor$ として $r := (x^g+x^h) \bmod n$ を返す。$x$ を特定せよ。
### 解法
p = ($1 \pmod 3$ である素数), q = 2 とすると $g = p, h = \lfloor 2p/3 \rfloor = 2(p-1)/3$ である。このとき $x^g \equiv x, (x^h)^3 \equiv 1 \pmod p$ であるため、 $x^h$ としてありえる 3 通りを全探索すれば解ける。

### コード
::: details presolve.py

presolve.py
```python
from Crypto.Util.number import isPrime


def order_prim(g: int, p: int, n: int) -> int:
    g = pow(g, (n - 1) // p, n)
    if g == 1:
        return 1
    return p


def order_log(g: int, n: int, prod_list: list[int]) -> int:
    ans = 1
    for p in prod_list:
        ans *= order_prim(g, p, n)
    return ans


def main() -> None:
    prod = 1
    prod_list = []

    for i in range(2, 380):
        if not isPrime(i):
            continue
        prod *= i
        prod_list.append(i)

    for i in range(1, 1000000):
        p = prod * i + 1
        if not isPrime(i):
            continue
        if p.bit_length() <= 520:
            continue
        if isPrime(p):
            prod_list.append(i)
            break
    g = 2
    while True:
        if all(order_prim(g, x, p) != 1 for x in prod_list):
            break
        g += 1

    print(f'bigp = ' + ' * '.join(map(str, prod_list)) + ' + 1')
    print(f'prod_list =', prod_list)
    print(f'assert bigp.bit_length() == {p.bit_length()}')
    print(f'assert isPrime(bigp)')
    print(f'gen = {g}')


if __name__ == '__main__':
    main()
```
:::

::: details solve.py
```python
import sys
from pwn import process, remote
from Crypto.Util.number import getPrime, isPrime

local = len(sys.argv) == 1
io = process(['python3', 'server.py']) if local else remote(sys.argv[1], int(sys.argv[2]))

bigp = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 29 * 31 * 37 * 41 * 43 * 47 * 53 * 59 * 61 * 67 * 71 * 73 * 79 * 83 * 89 * 97 * 101 * 103 * 107 * 109 * 113 * 127 * 131 * 137 * 139 * 149 * 151 * 157 * 163 * 167 * 173 * 179 * 181 * 191 * 193 * 197 * 199 * 211 * 223 * 227 * 229 * 233 * 239 * 241 * 251 * 257 * 263 * 269 * 271 * 277 * 281 * 283 * 293 * 307 * 311 * 313 * 317 * 331 * 337 * 347 * 349 * 353 * 359 * 367 * 373 * 379 * 2081 + 1
prod_list = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 2081]
assert bigp.bit_length() == 521
assert isPrime(bigp)
gen = 19

if __name__ == '__main__':
    p = bigp
    q = 2
    io.recvuntil(b'Your favorite prime p (hex) > ')
    io.sendline(hex(p)[2:].encode())
    io.recvuntil(b'Your favorite prime q (hex) > ')
    io.sendline(hex(q)[2:].encode())
    io.recvuntil(b'r = ')
    r = int(io.recvline().decode().strip())
    print(f'r = {r}, {r % p = }')
    for y in range(3):
        d = pow(gen, (p - 1) // 3 * y, p)
        x = (r - d) % p
        print(f'# {x.bit_length() = }')
        if x.bit_length() == 512:
            break
    io.recvuntil(b'Guess x > ')
    io.sendline(str(x).encode())
    print(io.recvline().decode())
    io.close()
```
:::
## [crypto] DLP+
sigma さんが 1 日目にやってくれたので私は 2 日目に少し改造して提出した。

::: details solve-sigma.py

```python
from sympy.ntheory.modular import crt
from pwn import *
import time

p = 2**2281 - 1
qlist = [2281,3**2,5**2,7,11,13,17,31,41,61,151,191,229,241,331,457,571,761,1217,1321,4561,32377,54721,61681,90289,131101,148961,160969,174763,185821,247381,524287,525313,1101811,1212847,160465489,420778751,3996146881,4562284561]
order_g = 2281*2

Bs = []
giants = []
xpows = []

# Baby-step Giant-step法
def bsgs_pre(x, q):
    B = int(q**0.4 * 0.5) + 1
    if q <= 3000:
        B = 1
    table = {}
    n = q//B + 1
    pw = pow(x, B, p)
    pw2 = 1
    for i in range(1, n+1):
        pw2 = pw2 * pw % p
        table[pw2] = i*B
    Bs.append(B)
    giants.append(table)
    xpows.append(x)
    print('done : ', q)

# x^res = y mod p, res < q は保証する
def bsgs(x, y, p, index):
    if y == 1:
        return 0
    q = qlist[index]

    B = Bs[index]
    rhs = y
    for b in range(B):
        if rhs in giants[index]:
            return (giants[index][rhs] - b + q) % q
        rhs = rhs * x % p
    return None

# Pohlig–Hellman法
def pohlig_hellman(x, y, r2281):
    ms = []
    rs = []
    for index, q in enumerate(qlist):
        xpow = xpows[index]
        ypow = pow(y, (p-1)//q, p)
        # 枝刈り
        # mod 2281 ではすでに値が指定されていて、これと一致しなければいけない
        if index == 0:
            if pow(xpow, r2281, p) != ypow:
                return None
            ms.append(q)
            rs.append(r2281)
        else:
            e = bsgs(xpow, ypow, p, index)
            # print('q,x: ', q,' ',x)
            if (e is None):
                return None
            ms.append(q)
            rs.append(e)
    print('ms: ', ms)
    print('rs: ', rs)
    e = crt(ms, rs)[0]
    return e

def solve(r, start: float):
    g = p // 2
    h = p // 3
    gpow = 1

    for ge in range(order_g):
        if ge%100 == 0:
            print(f'# ({time.time() - start:.2f}s) ge: ', ge)
        hpow = (r - gpow + p) % p
        x = pohlig_hellman(h, hpow, ge%2281)
        if x is not None:
            if x % order_g == ge and pow(h, x, p) == hpow:
                print('ge: ', ge)
                print('maybe: ', x)
                return x
        gpow = gpow * g % p

def recv_r(io):
    io.sendlineafter(b'Your favorite prime (hex) > ', hex(p).encode())
    io.recvuntil(b'r = ')
    r = io.recvline().decode().strip()
    return int(r)

def test():
    for q in qlist:
        assert((p-1) % q == 0)
    print('ok divisors')
    prod = 1
    for q in qlist:
        prod *= q
    print('length', prod.bit_length(), 'bits')

def test2():
    g = p // 2
    h = p // 3
    x = 31
    hpow = pow(h, x, p)
    print(pohlig_hellman(h, hpow))

start = time.time()

g = p // 2
h = p // 3


for q in qlist:
    bsgs_pre(pow(h, (p-1)//q,p), q)
print(f'# ({time.time()-start:.2f}s) bsgs_pre done')

print('Bs: ', Bs)
print('qs: ', qlist)

io = process(['python3', 'server.py']) if len(sys.argv) == 1 else remote(sys.argv[1], int(sys.argv[2]))
r = recv_r(io)
print('r: ', r)


# x = order_g * 1000 + 13
# x = randbelow(2**512)
# r = (pow(g, x, p) + pow(h, x, p)) % p

# test()
# test2()
# exit()

ans = solve(r, start)
io.sendlineafter(b'Guess x > ', str(ans).encode())
print(io.recvline().strip().decode())
```

:::

## [reversing] simple_reversing
### 問題
x64 linux 用の実行可能ファイルが与えられる。そこからフラグを見つけよ。
### 解法 (未完成)
strings などで中身を調べると mruby という文字列が見つかる。どうやら mruby VM 用のコードが埋め込まれているらしい。これを解析すれば良さそうだが、mruby であることに気付いたのが競技終了 30 分前ほどだったので時間が足りなかった。

## [KotH] HECCON
### 問題
準同型暗号で秘密裏に計算をやる役をこなせ。長さ $8192 = 2^{13}$ の実数のベクトルが与えられるので、round ごとに決められた計算をせよ。ジャッジ側で真の値との平均誤差 (MAE) を計算し、それの小さい方が勝ち。

Round 2: 長さ 8192 のベクトルを $4096 \times 2$ 行列とみなす。行列の行ごとに 2 個の  max を計算し、それを行の最初の要素に格納せよ。残りの要素はどうなっていても問題ない。
Round 4: 長さ 8192 のベクトルを $512 \times 16$ 行列とみなす。行列の行ごとに 16 個の max を計算し、それを行の最初の要素に格納せよ。残りの要素はどうなっていても問題ない。

### 解法 (Round 2)
一日目は私は用事があってあまり参加できていなかったので、チームメイトの人に実装してもらった。
方針としては、[ReLU 関数](https://ja.wikipedia.org/wiki/%E6%AD%A3%E8%A6%8F%E5%8C%96%E7%B7%9A%E5%BD%A2%E9%96%A2%E6%95%B0)を多項式で近似して、 $x[0] + \mathrm{ReLU}(x[1]-x[0])$ を計算するというもの。かなり良い順位が取れたらしい。

::: details solve-sugim48.py
```python
import pyhelayers
from pyhelayers import CTileTensor as CTT, CTile
import numpy as np

from sklearn.linear_model import HuberRegressor, LinearRegression
from sklearn.preprocessing import PolynomialFeatures
from sklearn.pipeline import make_pipeline
from scipy.optimize import minimize


def calc_coef(d):
    N = 2**12

    xs = []
    
    mu = np.random.uniform(-1, 1, (N, 1))
    sigma = np.random.uniform(0.5, 1.0, (N, 1))
    x = np.random.normal(mu, sigma, (N, 2))
    
    for i in range(N):
        xs.append(x[i][0] - x[i][1])

    xs = np.array(xs)
    ys = np.maximum(xs, 0)

    def l1_loss(coeffs, x, y):
        poly_vals = np.polyval(coeffs, x)
        return np.sum(np.abs(poly_vals - y))
    
    initial_guess = np.polyfit(xs, ys, d)
    result = minimize(l1_loss, initial_guess, args=(xs, ys), method='Powell')
    coef = result.x
    
    print(coef)

    m = 0
    for i in range(len(xs)):
        # a = coef_3[0] * (xs[i] ** 3) + coef_3[1] * xs[i] * xs[i] + coef_3[2] * xs[i] + coef_3[3]
        x = xs[i]
        a = 0
        for j in range(d + 1):
            a += coef[j] * (x ** (d - j))
        b = max(0.0, xs[i])
        m += abs(a - b)
    print(m / len(xs))
    
    return coef



def zero(x):
    xx = CTile(x)
    xx.multiply_scalar(0)
    return xx
    

def one(x):
    xx = CTile(x)
    xx.multiply_scalar(0)
    xx.add_scalar(1)
    return xx


def multiply_scalar(x, v):
    xx = CTile(x)
    print(v)
    xx.multiply_scalar(v)
    return xx

def multiply(x, y):
    xx = CTile(x)
    xx.multiply(y)
    return xx

def pow(x, n):
    xx = one(x)
    for i in range(n):
        xx.multiply(x)
    
    return xx

def add_scalar(x, v):
    xx = CTile(x)
    xx.add_scalar(v)
    return xx

def add(x, y):
    xx = CTile(x)
    xx.add(y)
    return xx

def sub(x, y):
    xx = CTile(x)
    xx.sub(y)
    return xx

def rotate(x, n):
    xx = CTile(x)
    xx.rotate(n)
    return xx


def relu(x, d):
    coef = calc_coef(d)
    
    res = zero(x)
    
    for i in range(d + 1):
        res.add_scalar(coef[i])
        if i != d:
            res.multiply(x)
    
    return res


def main():
    he_context = pyhelayers.SealCkksContext()
    he_context.load_from_file("./pubkey")
    encoder = pyhelayers.Encoder(he_context)
    buf = open("./enc", "rb").read()
    a = pyhelayers.load_ctile(he_context, buf)
    
    b = rotate(a, 1)
    b_a = sub(b, a)
    r = relu(b_a, 8)
    ans = add(a, r)
        
    ans_buf = ans.save_to_buffer()
    with open('./ans.enc', 'wb') as f:
        f.write(ans_buf)
    

main()
```
:::

### 解法 (Round 4)
Round 2 と似た問題だった。Round 2 では 2 要素の max だったのが、Round 4 では 16 要素の max になった。
単に 16 要素全部足そうとするとエラーになる (TODO: どのようなエラーなのか追記する) ので、
- ReLU の精度を落とす
- ステージごとに ReLU の近似をする範囲を変える
- 最終ステージだけ足して 2 で割る

をやった。

一日目のチームメイトの遺産のおかげで、割と早い時間帯に 1 位を取れた。
- Round 4 は 13:30-17:00 の 3.5 時間
- 1 位を取ったのは開始約 1 時間後

![](https://storage.googleapis.com/zenn-user-upload/5e03739b8c4c-20250304.png)

::: details solve-chal4.py
```python
import pyhelayers
from pyhelayers import CTileTensor as CTT, CTile
import numpy as np

from sklearn.linear_model import HuberRegressor, LinearRegression
from sklearn.preprocessing import PolynomialFeatures
from sklearn.pipeline import make_pipeline
from scipy.optimize import minimize

import sys

N = 2**9
M = 2**4


def calc_coef(d, lo, hi):
    N = 2**12

    xs = []
    
    mu = np.random.uniform(-1, 1, (N, 1))
    sigma = np.random.uniform(0.5, 1.0, (N, 1))
    x = [lo + (hi - lo) * i / N for i in range(N)]
    
    for i in range(N):
        xs.append(x[i])

    xs = np.array(xs)
    ys = np.maximum(xs, 0)

    def l1_loss(coeffs, x, y):
        poly_vals = np.polyval(coeffs, x)
        return np.sum(np.abs(poly_vals - y))
    
    initial_guess = np.polyfit(xs, ys, d)
    result = minimize(l1_loss, initial_guess, args=(xs, ys), method='Powell')
    coef = result.x
    
    print(coef)

    m = 0
    for i in range(len(xs)):
        # a = coef_3[0] * (xs[i] ** 3) + coef_3[1] * xs[i] * xs[i] + coef_3[2] * xs[i] + coef_3[3]
        x = xs[i]
        a = 0
        for j in range(d + 1):
            a += coef[j] * (x ** (d - j))
        b = max(0.0, xs[i])
        m += abs(a - b)
    print(m / len(xs))
    
    return coef



def zero(x):
    xx = CTile(x)
    xx.multiply_scalar(0)
    return xx
    

def one(x):
    xx = CTile(x)
    xx.multiply_scalar(0)
    xx.add_scalar(1)
    return xx


def multiply_scalar(x, v):
    xx = CTile(x)
    xx.multiply_scalar(v)
    return xx

def multiply(x, y):
    xx = CTile(x)
    xx.multiply_raw(y)
    return xx

def pow(x, n):
    xx = one(x)
    for i in range(n):
        xx.multiply(x)
    
    return xx

def add_scalar(x, v):
    xx = CTile(x)
    xx.add_scalar(v)
    return xx

def add(x, y):
    xx = CTile(x)
    xx.add(y)
    return xx

def sub(x, y):
    xx = CTile(x)
    xx.sub(y)
    return xx

def sum(x):
    xx = CTile(x)
    xx.inner_sum()
    return xx


def avg(x):
    xx = CTile(x)
    xx.inner_sum()
    xx.multiply_scalar(1.0 / (N * M))
    return xx

def square(x):
    xx = CTile(x)
    xx.square()
    return xx


def var(x):
    n = 2**13
    a = sum(x)
    an = multiply_scalar(a, 1.0 / n)
    d = sub(x, an)
    c = square(d)
    d = sum(c)
    dn = multiply_scalar(d, 1.0 / n)
    return dn

def rotate(x, n):
    xx = CTile(x)
    xx.rotate(n)
    return xx


def rev(x, d):
    coef = calc_coef(d)
    
    res = zero(x)
    
    for i in range(d + 1):
        if i == 0:
            res = multiply_scalar(x, coef[i])
        else:
            res.add_scalar(coef[i])
            if i != d:
                res.multiply(x)
    
    return res


def relu(x, d, lo, hi):
    coef = calc_coef(d, lo, hi)
    
    res = zero(x)
    
    for i in range(d + 1):
        res.add_scalar(coef[i])
        if i != d:
            res.multiply(x)
    
    return res

def debug(v, encoder):
    plain = encoder.decrypt_decode_double(v)
    print(plain[:3])


def main():
    pubkeypath = sys.argv[1]
    he_context = pyhelayers.SealCkksContext()
    he_context.load_from_file(pubkeypath)
    if len(sys.argv) >= 3:
        he_context.load_secret_key_from_file(str(sys.argv[2]))
    encoder = pyhelayers.Encoder(he_context)
    buf = open("./enc", "rb").read()
    a = pyhelayers.load_ctile(he_context, buf)
    b = CTile(a)
    lo_hi = [
        (-3.0, 3.0),
        (-2.0, 4.0),
        (-1.2, 3.8),
    ]
    for i in range(0, 3):
        u = rotate(b, 2**i)
        d = sub(u, b)
        lo, hi = lo_hi[i]
        c = relu(d, 3, lo, hi)
        b = add(b, c)
    for i in range(3, 4):
        u = rotate(b, 2**i)
        d = add(u, b)
        b = multiply_scalar(d, 0.5)
    ans = b
        
    ans_buf = ans.save_to_buffer()
    with open('./ans.enc-4', 'wb') as f:
        f.write(ans_buf)
    
    if len(sys.argv) >= 3:
        debug(a, encoder)
        debug(b, encoder)
        debug(ans, encoder)
    

main()
```
:::

# まとめ
二日目で KotH に行って得点源にしたのだけはよかったが、その後一番可能性のある simple_reversing に行けなかったのが心残りである。このミスのせいで 3 位以内 (入賞圏内) を逃した。

# 参考資料
https://zenn.dev/sigma425/articles/39dbe84df2390f (sigma さんの writeup)
