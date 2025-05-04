[TsukuCTF 2025](https://tsukuctf.org/) に [ZK Lovers](https://tsukuctf.org/teams/431) で参加した。16 位だった。

## [crypto] a8tsukuctf (241/882)
### 問題
途中から暗号文を使うタイプのシーザー暗号で暗号化された文が与えられる。復号せよ。

### 解法
ソースコードを読むと、アルファベットだけシーザー暗号で暗号化して、それ以外の文字はそのままであることがわかる。
tsukuctf が保存されていることから、plaintext[30:38] において key あるいは cipher_without_symbols は a が連続していたことがわかる。a の連続によって ciphertext[30:] を復号すると 8 個のときにうまくいくことがわかる。

問題は ciphertext[:30] をどう復号するかだが、以下の復号された文を読むと実際はこの部分だけで十分であることがわかる。
```
$ python3 solve.py 
# tsukuctf, or both? the flag is concatenate the seventh word in the first sentence, the third word in the second sentence, and 'fun' with underscores.
```
tsukuctf が第一文の 7 語目であることに注意して言われた通りにフラグを作成すると `TsukuCTF25{tsukuctf_is_fun}` になる。


## [crypto] PQC0 (149/882)
### 問題
耐量子暗号で暗号化された shared key と、shared key を使って AES で暗号化したデータがある。復号せよ。

### 解法
配布された output.txt に**秘密鍵**と暗号文があるので、秘密鍵を使って復号する。

::: details solve.py

```python
import os
from Crypto.Cipher import AES
from Crypto.Util.Padding import unpad

if __name__ == '__main__':
    with open('output.txt', 'r') as f:
        lines = f.readlines()
        priv_idx = lines.index('==== private_key ====\n')
        if priv_idx == -1:
            raise ValueError("Private key not found in output.txt")
        ciphertext_idx = lines.index('==== ciphertext(hex) ====\n')
        if ciphertext_idx == -1:
            raise ValueError("Ciphertext not found in output.txt")
        tmp = lines[priv_idx + 1:ciphertext_idx - 1]
        with open('priv.pem', 'w') as f:
            f.writelines(tmp)
        encrypted_flag_idx = lines.index('==== encrypted_flag(hex) ====\n')
        if encrypted_flag_idx == -1:
            raise ValueError("Encrypted flag not found in output.txt")
        tmp = lines[ciphertext_idx + 1:encrypted_flag_idx]
        with open('ciphertext.dat', 'wb') as f:
            f.write(bytes.fromhex(''.join(tmp)))
        tmp = lines[encrypted_flag_idx + 1:]
        encrypted_flag = bytes.fromhex(''.join(tmp))

    os.system('openssl pkeyutl -decap -inkey priv.pem -in ciphertext.dat -out shared.dat')
    with open("shared.dat", "rb") as f:
        shared_secret = f.read()

    flag = AES.new(shared_secret, AES.MODE_ECB).decrypt(encrypted_flag)
    flag = unpad(flag, 16)
    print(flag.decode())
```
:::
## [crypto] PQC1 (21/882)
### 問題
PQC0 と同じだが、秘密鍵の PEM データの先頭 128 文字しか与えられない。それでも復号できるか?

### 解法
与えられた鍵の方式は ML-KEM-768 である。

残っている秘密鍵のデータを見ると、 [0x1b, 0x5b) の 64 バイトに seed が書かれていることがわかる。
```
$ <test.txt base64 --decode | hexdump -C
00000000  30 82 09 be 02 01 00 30  0b 06 09 60 86 48 01 65  |0......0...`.H.e|
00000010  03 04 04 02 04 82 09 aa  30 82 09 a6 04 40 69 ad  |........0....@i.|
00000020  87 4f 24 26 da f9 a6 0e  6c 39 7a 17 8c 26 9e a0  |.O$&....l9z..&..|
00000030  26 6d ec 72 b2 25 a0 d6  59 17 67 8a 9d c4 19 7c  |&m.r.%..Y.g....||
00000040  fc a9 a4 d3 4d 3b 00 15  dd 66                    |....M;...f|
0000004a
```

<https://www.ietf.org/id/draft-ietf-lamps-kyber-certificates-10.html#section-6> や <https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf> の論文を見ると、その 64 バイトの前半 32 バイトが d であること、後半 32 バイトの z はなんでも良さそうなこと、与えられたファイルには 0x2c = 44 バイト残っていることがわかる。

openssl には seed を設定して ML-KEM-768 の鍵を作るオプションがあるのでそれを使って AC。

::: details test.sh (seed から鍵を作る)
```
SEED=69ad874f2426daf9a60e6c397a178c269ea0266dec72b225a0d65917678a9dc4197c02030405060708090A0B0C0D0E0F000102030405060708090A0B0C0D0E0F
openssl genpkey -algorithm ML-KEM-768 -pkeyopt hexseed:${SEED} -out priv.pem
```
:::

::: details solve.py

```python
import os
from Crypto.Cipher import AES
from Crypto.Util.Padding import unpad

if __name__ == '__main__':
    with open('output.txt', 'r') as f:
        lines = f.readlines()
        ciphertext_idx = lines.index('==== ciphertext(hex) ====\n')
        if ciphertext_idx == -1:
            raise ValueError("Ciphertext not found in output.txt")
        encrypted_flag_idx = lines.index('==== encrypted_flag(hex) ====\n')
        if encrypted_flag_idx == -1:
            raise ValueError("Encrypted flag not found in output.txt")
        tmp = lines[ciphertext_idx + 1:encrypted_flag_idx]
        with open('ciphertext.dat', 'wb') as f:
            f.write(bytes.fromhex(''.join(tmp)))
        tmp = lines[encrypted_flag_idx + 1:]
        encrypted_flag = bytes.fromhex(''.join(tmp))

    os.system('openssl pkeyutl -decap -inkey priv.pem -in ciphertext.dat -out shared.dat')
    with open("shared.dat", "rb") as f:
        shared_secret = f.read()

    flag = AES.new(shared_secret, AES.MODE_ECB).decrypt(encrypted_flag)
    flag = unpad(flag, 16)
    print(flag.decode())

```
:::

## [crypto] xortsukushift (34/882)
### 問題
サーバーとジャンケン勝ち抜きを 300 回やる権利が与えられるので、どこかの勝ち抜きで 294 連勝せよ。
### 解法
サーバーの乱数生成器が破りやすいことを利用する。サーバーの乱数生成器は以下のようになっている:
```cpp
uint64_t next(uint64_t x) {
  x ^= x << 17
  x ^= x >> 9
  x ^= x << 18
  return x
}
```

実験によると、これは 280 回周期で同じ値を繰り返すため、280 回分の乱数がわかればその後の乱数を予知できる。

::: details exp.py (実験用スクリプト)
```python
s = 2

def get_period(s_init: int) -> int | None:
    s = s_init
    for i in range(100000):
        s ^= (s << 17) & ((1 << 64) - 1)
        s ^= s >> 9
        s ^= s << 18 & ((1 << 64) - 1)
        if s == s_init:
            return i + 1

for s in range(0, 100):
    p = get_period(s)
    print('s:', s, 'period:', p)

for i in range(0, 64):
    s = 1 << i
    p = get_period(s)
    print('s:', s, 'period:', p)
```
:::

::: details solve.py
```python
import sys
from pwn import remote, process
import z3

io = process(['python3', 'server.py']) if len(sys.argv) == 1 else remote(sys.argv[1], int(sys.argv[2]))

def go(x: int, expecting_flag: bool = False) -> int:
    io.recvuntil(b"Rock, Paper, Scissors... Go! (Rock: 0, Paper: 1, Scissors: 2): ")
    io.sendline(str(x).encode())
    l = io.recvline()
    result = None
    if b"Tsukushi: You win!" in l:
        result = 1
    elif b"Tsukushi: Draw!" in l:
        result = 0
    elif b"Tsukushi: You lose!" in l:
        result = -1
    if expecting_flag:
        print(io.recvall(timeout=1))
        sys.exit(0)
    return (x - result) % 3


if __name__ == "__main__":
    recon_count = 280
    dat = []
    for i in range(recon_count):
        dat.append(go(0))
    go((dat[0] + 2) % 3) # lose on purpose
    for i in range(294):
        go((dat[(i + 1) % recon_count] + 1) % 3, i == 293)
```
:::

## [crypto] PQC2 (0/882)
### 問題
PQC0 と同じだが、秘密鍵の PEM データの先頭 294 文字が落とされている。それでも復号できるか?

### 解法?
落とされた先頭部分は、PQC0 の鍵で言うと大体以下の部分に相当する。 0x62 からの 0x960 = 2400 バイトが秘密鍵だが、その先頭 0x64 = 100 バイト近くが PQC2 の鍵には欠けていることになる。

::: details 結果
```
$ cat head.txt
MIIJvgIBADALBglghkgBZQMEBAIEggmqMIIJpgRAv9B0xN9H9VxT9h6t98wqSuqJ
Byif6N8+FqaTBY9y86Rxbi14UAsxBvzbSZ7aVElR9zdXlYp1OYKbCyYo1Fl5twSC
CWB8y5x69sGKKZUUOsGolY+HO2KMuIKwAKk/IxyuaCWJM8MqJaTVMZkWainb2Ylg
4YjVJCUvELUbCnImMIgbNxktNEKuumnWkadyw7/kQHpkuQ90lMW4qDhZw2whrJ2B
Y0LaWFXF
$ <head.txt base64 --decode | hexdump -C
00000000  30 82 09 be 02 01 00 30  0b 06 09 60 86 48 01 65  |0......0...`.H.e|
00000010  03 04 04 02 04 82 09 aa  30 82 09 a6 04 40 bf d0  |........0....@..|
00000020  74 c4 df 47 f5 5c 53 f6  1e ad f7 cc 2a 4a ea 89  |t..G.\S.....*J..|
00000030  07 28 9f e8 df 3e 16 a6  93 05 8f 72 f3 a4 71 6e  |.(...>.....r..qn|
00000040  2d 78 50 0b 31 06 fc db  49 9e da 54 49 51 f7 37  |-xP.1...I..TIQ.7|
00000050  57 95 8a 75 39 82 9b 0b  26 28 d4 59 79 b7 04 82  |W..u9...&(.Yy...|
00000060  09 60 7c cb 9c 7a f6 c1  8a 29 95 14 3a c1 a8 95  |.`|..z...)..:...|
00000070  8f 87 3b 62 8c b8 82 b0  00 a9 3f 23 1c ae 68 25  |..;b......?#..h%|
00000080  89 33 c3 2a 25 a4 d5 31  99 16 6a 29 db d9 89 60  |.3.*%..1..j)...`|
00000090  e1 88 d5 24 25 2f 10 b5  1b 0a 72 26 30 88 1b 37  |...$%/....r&0..7|
000000a0  19 2d 34 42 ae ba 69 d6  91 a7 72 c3 bf e4 40 7a  |.-4B..i...r...@z|
000000b0  64 b9 0f 74 94 c5 b8 a8  38 59 c3 6c 21 ac 9d 81  |d..t....8Y.l!...|
000000c0  63 42 da 58 55 c5                                 |cB.XU.|
000000c6
```
:::

<https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf> でいうと $\mathrm{dk}_{\mathrm{PKE}} = \mathrm{encode}(\bf{\hat s})$ が 2400 バイト中の先頭 1152 バイトにあるが、その先頭が落ちている。

予想: $\bf{\hat t} = \bf{\hat A} \circ \bf{\hat s} + \bf{\hat e}$ を使って $\bf{\hat s}$ の欠けている部分を復元する。




## [web] len_len (451/882)
### 問題
```
"length".length is 6 ?

curl http://challs.tsukuctf.org:28888
```

### 解法
server.js の L17-21 を見ると以下のようになっている。
```javascript
  const array = JSON.parse(sanitized);
  if (array.length < 0) {
    // hmm...??
    return FLAG;
  }
```
`length` というキーを持っているオブジェクトを渡すことにすれば、値として任意の値を渡せる。

::: details コマンド
```console
$ curl -X POST http://challs.tsukuctf.org:28888 -d 'array={"length":-1}'
TsukuCTF25{l4n_l1n_lun_l4n_l0n}
```
:::

## [web] flash (170/882)
### 問題
フラッシュ暗算ができる Web サービスがある。ただし途中の数値は表示されない。合計を正しく正解せよ。

### 解法
セッションが JWT である。JWT には以下のような情報が入っている。
```json
{
  "round": 0,
  "session_id": "aec2f5b945e7a3d566a40f80261d01bd"
}
```
session_id が決まれば合計の値も一位に決まることに注意。(特に、複数回同じ session_id で遊ぶこともできる。)

数値が 10 個出た後に解答できる画面に遷移するが、そのあとの流れは以下。
1. GET すると次のページ用の JWT と token が払い出される
2. JWT と token を使って POST をする。データとして token と answer を渡す。
3. answer が正しければフラグが得られ、間違っていたら正しい値が得られる。

このプロセスを 1 回やって正しい値を得て、2 回目でフラグを得ればよい。

::: details 解答できる画面で情報を得るコマンド例 (1.)
```
curl http://challs.tsukuctf.org:50000/result -b session=.eJwNxzEOgCAMAMC_dHYo2kLxMwZKTYwGEtHJ-He97R44rd_HtVxttwozZA1MoXijSXmiFINH87SSODHOCAOc7a4FZocDdOt9a3XZ_oOMyByzcnGKI6k445ATxUQmCRneDzcUICM.aBYbmg.iiA4AW6szG1IlCD401NkP2CWtXo -v
```
:::

::: details run.sh (2.)
```bash
curl -X POST http://challs.tsukuctf.org:50000/result \
  -d 'token=ed6e526bb83da9b4204ec8f48f655dc2&answer=44192675' \
  -b session=.eJwNxzEOwzAIAMC_MGfAFFycz0Q2EClqZUtxMlX9e3vbfeCMeb-v7Rqv6LBCeA6h3Jo-vJbGhBymO-ueRdwIFjjH3R3WhAvMmPMYfTv-ByUUKc3EkyGxaQp5tsqlcmhFge8PcJsg4A.aBa0yw.dwflq_kI_ZNtSvF6xcPBszxYPSo
```
:::
