# まとめ
p が 256 bit 程度のとき、有限体上の離散対数問題 (FFDLP) は 30 分くらいで解けるため、CTF の時間制限の中ですら有効である。競技者は積極的にそのやり方を試すべきである。作問者は p を 512 bit 程度にしてその解法を封じるべきである。

# 注意
[IERAE CTF 2025 で出題された Cipher Brothers (medium)](https://gmo-cybersecurity.com/blog/ierae-ctf-2025-writeup-crypto/) のネタバレを含む。

# 理論
有限体上の離散対数問題 (FFDLP) とは以下のような問題である。

> 整数 $a, b$ と素数 $p$ がわかっている。このとき $a^x \equiv b \pmod{p}$ となる $x$ を求めよ。

この問題には [GNFS](https://ja.wikipedia.org/wiki/%E4%B8%80%E8%88%AC%E6%95%B0%E4%BD%93%E7%AF%A9%E6%B3%95) を用いた解法が知られている。[^1]

[^1]: GNFS は素因数分解のアルゴリズムとして有名だが、少し修正することで FFDLP も解ける。

$p$ が 256 bit の場合、必要な手間はおよそ 47 bit ($2^{47}$ 回の計算) 程度である。

# 実践

## インストールバトル
- <https://gitlab.inria.fr/cado-nfs/cado-nfs> を読んでがんばってくれ
- x86_64 なら動くはず
- 筆者は GitHub Codespaces で動かした

## 本丸
IERAE CTF 2025 で [Cipher Brothers (medium)](https://gmo-cybersecurity.com/blog/ierae-ctf-2025-writeup-crypto/) という問題が出題された。

この問題そのものは、ElGamal 暗号化の不適切な使用を咎める問題であるが、そもそも modulus が 256 bit 程度と非常に短いので、正攻法で解けてしまう。

以下は配布スクリプトをローカルで動かして得られる出力である。
```
$ python server.py 
1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
p: 79938156244279490196917570063539966343976157459476321233940429171848106787199
g: 66660761356875726919828279967833176091195706284787636435486590927603284420008
h: 44466765521008176715623609408764237450859969023369800232513568424301429892003
c1: 16776860000244332084040132710826842383815433486056474110987611840531063884646
c2: 17312389444159424432880742499318648086587714590988164668646265834663708400738
```

$x = \log_g h$ を求めれば正攻法で $m$ がわかる。 $\log_g h$ を求めるには $\log_u h$ と $\log_u g$ を求めれば良い。 (ここで、 $u$ は cado-nfs が勝手に決める生成元)

`./cado-nfs.py -dlp -ell ((p-1)/2) target=(h) (p)` と `./cado-nfs.py (一時ファイルの場所) target=(g)` を実行し、順に結果を $a_1, a_2$ としたとき $a_1 / a_2 \pmod{p-1}$ を計算すればそれが答えである。

```bash
./installed/bin/cado-nfs.py -dlp -ell 79938156244279490196917570063539966343976157459476321233940429171848106787198 target=44466765521008176715623609408764237450859969023369800232513568424301429892003 79938156244279490196917570063539966343976157459476321233940429171848106787199

# 同じ ell, p、異なる target に対して実行する。
./installed/bin/cado-nfs.py (前回の実行で教えてもらった一時ファイルの場所) target=66660761356875726919828279967833176091195706284787636435486590927603284420008
```

::: details 実行結果
```console
$ time ./installed/bin/cado-nfs.py -dlp -ell 39969078122139745098458785031769983171988078729738160616970214585924053393599 target=44466765521008176715623609408764237450859969023369800232513568424301429892003 79938156244279490196917570063539966343976157459476321233940429171848106787199

...

Info:API server: Shutting down server
Info:Complete Factorization / Discrete logarithm: Total cpu/elapsed time for entire Discrete logarithm: 2279.02/1572.72
Info:root: CADO_DEBUG is on, data kept in /tmp/cado.i1jt9oq_
Info:root: If you want to compute one or several new target(s), run ./installed/bin/cado-nfs.py /tmp/cado.i1jt9oq_/p75.parameters_snapshot.0 target=<target>[,<target>,...]
Info:root: logbase = 9864698054415101807980118848628032889566114535937230788623576759967959301768
Info:root: target = 44466765521008176715623609408764237450859969023369800232513568424301429892003
Info:root: log(target) = 26111141173836657074373772688926188824319793196585249990104109882942309631554 mod ell
26111141173836657074373772688926188824319793196585249990104109882942309631554

real    26m13.471s
user    26m59.535s
sys     3m9.708s

$ time ./installed/bin/cado-nfs.py /tmp/cado.i1jt9oq_/p75.parameters_snapshot.0 target=66660761356875726919828279967833176091195706284787636435486590927603284420008

...

Info:API server: Shutting down server
Info:Complete Factorization / Discrete logarithm: Total cpu/elapsed time for entire Discrete logarithm: 2368.92/1632.15
Info:root: If you want to compute one or several new target(s), run ./installed/bin/cado-nfs.py /tmp/cado.i1jt9oq_/p75.parameters_snapshot.1 target=<target>[,<target>,...]
Info:root: logbase = 9864698054415101807980118848628032889566114535937230788623576759967959301768
Info:root: target = 66660761356875726919828279967833176091195706284787636435486590927603284420008
Info:root: log(target) = 2180776442776793182028134003512608148750446183707217767951401121937094164886 mod ell
2180776442776793182028134003512608148750446183707217767951401121937094164886

real    0m59.843s
user    1m30.327s
sys     0m8.115s
```

:::

GitHub Codespaces (2 cores) の上では前者は約 26 分、後者は約 1 分で完了した。

なお、元の問題に戻ると、離散対数さえわかれば以下のようにして簡単にフラグが取得できる。

```python
$ python3         
Python 3.12.11 (main, Jun  3 2025, 15:41:47) [Clang 17.0.0 (clang-1700.0.13.3)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> p = 79938156244279490196917570063539966343976157459476321233940429171848106787199
>>> g = 66660761356875726919828279967833176091195706284787636435486590927603284420008
>>> h = 44466765521008176715623609408764237450859969023369800232513568424301429892003
>>> lg = 2180776442776793182028134003512608148750446183707217767951401121937094164886
>>> lh = 26111141173836657074373772688926188824319793196585249990104109882942309631554
>>> pow(h, (p - 1) // 2, p)
79938156244279490196917570063539966343976157459476321233940429171848106787198
>>> pow(g, (p - 1) // 2, p)
79938156244279490196917570063539966343976157459476321233940429171848106787198
>>> lgh = lh * pow(lg, -1, (p - 1) // 2) % ((p - 1) // 2)
>>> pow(g, lgh, p) == h
True
>>> 
>>> c1 = 16776860000244332084040132710826842383815433486056474110987611840531063884646
>>> c2 = 17312389444159424432880742499318648086587714590988164668646265834663708400738
>>> m = c2 * pow(c1, -lgh, p) % p
>>> m
117741614136557025042125087210689522154980137777162415462642813
>>> m.to_bytes(32)
b'\x00\x00\x00\x00\x00\x00IERAE{dummy flag for test}'
```
# 罠
## 小さい例だと動かない
F_7 でまず試そうとして実行しても固まる なんだこれ

::: details F_7 の素因数分解 実行
```console
$ ./installed/bin/cado-nfs.py parameters/factor/parameters.F7 340282366920938463463374607431768211457
nfo:root: No database exists yet
Info:root: Created temporary directory /tmp/cado.qn2mhtjs
Info:Database: Database URI is db:sqlite3:///tmp/cado.qn2mhtjs/F7.db
Info:Database: Opened connection to database /tmp/cado.qn2mhtjs/F7.db
Info:root: Set tasks.linalg.bwc.threads=1 based on detected physical cores
Info:root: Set tasks.threads=2 based on detected logical cpus
Info:root: tasks.threads = 2 [via tasks.threads]
Info:root: tasks.polyselect.threads = 2 [via tasks.polyselect.threads]
Info:root: tasks.sieve.las.threads = 2 [via tasks.sieve.las.threads]
Info:root: tasks.linalg.bwc.threads = 1 [via tasks.linalg.bwc.threads]
Info:root: tasks.sqrt.threads = 2 [via tasks.threads]
Info:root: Command line parameters: ./installed/bin/cado-nfs.py parameters/factor/parameters.F7 340282366920938463463374607431768211457
Info:root: If this computation gets interrupted, it can be resumed with ./installed/bin/cado-nfs.py /tmp/cado.qn2mhtjs/F7.parameters_snapshot.0
Info:API server: server whitelist is []
Info:API server: Running from werkzeug (1 thread(s))
Info:Lattice Sieving: param rels_wanted is 20000
Info:Complete Factorization / Discrete logarithm: Factoring 340282366920938463463374607431768211457
Info:API server: Running on https://localhost:45223 (Press CTRL+C to quit))
Info:API server: You can start additional cado-nfs-client.py scripts with parameters: --server=https://localhost:45223 --certsha1=f1d24a8e9813392c4dbee212227c05483c039fb2
Info:API server: If you want to start additional clients, remember to add their hosts to server.whitelist
Info:Polynomial Selection (size optimized): Starting
Info:Polynomial Selection (size optimized): 0 polynomials in queue from previous run
Info:Polynomial Selection (size optimized): Adding workunit F7_polyselect1_0-2500 to database
Info:Polynomial Selection (size optimized): Adding workunit F7_polyselect1_2500-5000 to database
```
:::

## `Number Theory for DLP: Starting` の後 404 が返ってくるんだが
ell = p-1 とすると以下のように進まなくなる:
```
./cado-nfs.py -dlp -ell 79938156244279490196917570063539966343976157459476321233940429171848106787198 target=44466765521008176715623609408764237450859969023369800232513568424301429892003 79938156244279490196917570063539966343976157459476321233940429171848106787199

...

Info:Polynomial Selection (root optimized): Aggregate statistics:
Info:Polynomial Selection (root optimized): Total time: 7.07
Info:Polynomial Selection (root optimized): Rootsieve time: 7.07
Info:Number Theory for DLP: Starting
Info:werkzeug: 127.0.0.1 - - [26/Jun/2025 02:38:04] "GET /workunit HTTP/1.1" 404 -
Info:werkzeug: 127.0.0.1 - - [26/Jun/2025 02:38:06] "GET /workunit HTTP/1.1" 404 -
Info:werkzeug: 127.0.0.1 - - [26/Jun/2025 02:38:08] "GET /workunit HTTP/1.1" 404 -
Info:werkzeug: 127.0.0.1 - - [26/Jun/2025 02:38:13] "GET /workunit HTTP/1.1" 404 -
```

この問題では $p$ は安全素数 ($(p-1)/2$ も素数であるような素数) であり、ell を $(p-1)/2$ にすれば動く。一般の場合に ell をどうすれば良いかは未調査。もしかしたら ell は素数である必要があるかもしれない。

# 背景情報
cado-nfs のバージョン
```console
(cado-nfs.venv) @koba-e964 ➜ /workspaces/ctf/cado-nfs (master) $ git log --max-count=1
commit 5f44a4936b014ae34d8e9d7bb636a7c74750a3c1 (grafted, HEAD -> master, origin/master, origin/HEAD)
Author: Emmanuel Thomé <emmanuel.thome@inria.fr>
Date:   Wed Jun 25 12:26:14 2025 +0200

    Merge branch 'las-fix-above-large-prime' into 'master'
    
    las: take into account composite special-q in check_whether_q_above_large_prime_bound
    
    See merge request cado-nfs/cado-nfs!213
```
