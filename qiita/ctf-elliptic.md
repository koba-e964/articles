最近 (2024/9/15) [A Dance of Add and Mul](https://alpacahack.com/ctfs/round-3/challenges/a-dance-of-add-and-mul) という問題で楕円曲線が扱われたので、いい機会と捉え CTF プレイヤー向けに簡単に背後の理論の説明をする。

# 前書き
## この記事で扱うこと
楕円曲線暗号について、CTF で扱う範囲に絞って背後の理論の説明を行う。ただし証明は一切しない。

具体的な問題の紹介および解き方は [CTF crypto 逆引き#楕円曲線暗号](https://furutsuki.hatenablog.com/entry/2021/03/16/095021#楕円曲線暗号) を参考にすること。

## 対象読者
CTF の Crypto 問をやって、楕円曲線の問題でそもそも解法が理解できなかった人。

## 仮定する知識
アーベル群 (群であって積が交換可能なもの) の定義を知っていると有利。この記事では有限アーベル群について扱う。知らなくても途中までは読める。

## 事前準備
[SageMath](https://www.sagemath.org/) をインストールすること。これは CTF で (楕円曲線に限らず) 高度な Crypto 問題を解くときには非常に役に立つ。

# 本編
## 有限体
$\mathrm{GF}(q)$ (`\mathrm{GF}(q)`) で位数 $q$ の有限集合であるような体 (有限体) を表す。証明はしないが、$q$ が決まればこのような体は **1 通りに決まる**ことに注意。
- $q = p$ が素数であればこれは mod q で考えるのと同じとみなして全く問題ない。
- $q = p^k$ ($k \ge 2$) が素数でない素数ベキである場合、mod q で考えるのと同じとみなすと**大いに問題が発生する**。これについては「(発展) 体拡大」の節で扱う。
- $q$ が素数ベキでない場合はありえない。

他に $\mathbb{F}_{q}$ (`\mathbb{F}_{q}`) という表記の方法もある。ここでは SageMath に合わせた。

SageMath では `GF` で構築できる。具体例は以下。

```python
sage: try: GF(5)
....: except Exception as e: print(e)
Finite Field of size 5
sage: try: GF(25)
....: except Exception as e: print(e)
Finite Field in z2 of size 5^2
sage: try: GF(6)
....: except Exception as e: print(e)
the order of a finite field must be a prime power
```


## 有限体上の楕円曲線
### 3 次元空間の要素として
以下のような式で定義される点 $(X : Y : Z)$ の集合 ($(X, Y, Z) \neq (0, 0, 0)$) のことを[楕円曲線](https://crypto.stanford.edu/pbc/notes/elliptic/weier.html)と呼ぶ。
$$Y^2Z + a_1XYZ + a_3 YZ^2 \equiv X^3 + a_2X^2Z + a_4XZ^2 + a_6Z^3 \pmod {p}$$

CTF でよく見る曲線は Short Weierstrass 曲線 (ショート・ヴァイエルシュトラス・きょくせん) と呼ばれる、$a_1=a_2=a_3=0$ であるような以下の形式である。[^how-about-ed25519]
$$Y^2Z \equiv X^3 + aXZ^2 + bZ^3 \pmod{p}$$

[^how-about-ed25519]: Ed25519 や X25519 のような方式で使われる楕円曲線 (およびそれの変種) では $Y^2Z \equiv X^3 + 486662X^2Z + XZ^2 \pmod{2^{255}-19}$ だったりして、$X^2Z$ の項が 0 とは限らない。しかし Ed25519 のような比較的安全な方式が CTF に出題される可能性は低い (それ単体で使い物にならなくなるような重大な欠陥が埋め込めないと CTF の問題として出題できず、Ed25519 などはそうした欠陥を埋め込んでしまう余地が少なくなるように注意深く設計されている) ので、無視しても大きな問題はないと思われる。

たとえば以下の例は楕円曲線 $Y^2Z \equiv X^3 + 4Z^3 \pmod{7}$ である。$(0:1:0), (0:2:1), (0:5:1)$ の 3 個の点があることがわかる。

```
sage: E = EllipticCurve(GF(7), [0, 4])
sage: E.points()
[(0 : 1 : 0), (0 : 2 : 1), (0 : 5 : 1)]
```
楕円曲線の式が X, Y, Z の 3 次式であったことに注目してほしい。つまり X, Y, Z を同時に c 倍しても式が成り立つかどうかは変わらない。このことを数式で書くと以下のようになる。
$$(X:Y:Z) = (cX:cY:cZ)$$
このことから比の記号 `:` を使って座標を書くことが正当化される。(こういったスカラー倍を同一視する座標は**射影座標**と呼ばれる。)

実際に SageMath でも定数倍したものが等しいとみなされることは、以下のように確認できる。
```python
sage: E(0, 2, 1) == E(0, 4, 2)
True
sage: E(0, 6, 3) == E(0, 4, 2)
True
```
同じ点に複数通りの表現方法があるので、その複数通りの中で「標準的な」表現方法を決めなければならないが、SageMath では以下のようにしている。おそらく常にこうするのが良いだろう。
- $Z=0$ である点は $(0:1:0)$ と表現する。
- $Z \neq 0$ である点は $Z=1$ として表現する。たとえば $(0:2:1)$ など。

 
### (x, y) で表す方式との互換性について
他の記事で「無限遠点」や $O$ と呼んでいるものはこの記事だと (0 : 1 : 0) である。他の記事で $(x, y)$ と呼んでいるものはこの記事だと $(x : y : 1) = (xu : yu : u)$ である。($u$ は 0 でない任意の値)

楕円曲線の方程式は以下のようになる。
$$y^2 + a_1xy + a_3 y \equiv x^3 + a_2x^2 + a_4x + a_6 \pmod {p}$$

今回扱った楕円曲線は以下のようになる。
$$y^2 \equiv x^3 + 4 \pmod {7}$$
この形であれば $(0, 2) = (0 : 2 : 1)$ が含まれることも簡単に確かめられるだろう。

## 位数
楕円曲線 $E$ に含まれる点の個数のことを $E$ の**位数**と呼び、$|E|$ や $\\# E$ で表す。たとえば先ほどの $E_{7,0,4}: Y^2Z \equiv X^3 + 4Z^3 \pmod{7}$ に対して $\\# E_{7,0,4} = 3$ である。
[ハッセの定理](https://en.wikipedia.org/wiki/Hasse%27s_theorem_on_elliptic_curves) という位数についての定理がある。これによって、 $\mathrm{GF}(q)$ で定義された楕円曲線について以下が成り立つ。
$$ q - 2\sqrt{q} + 1 \le \\# E \le q + 2\sqrt{q} + 1$$
特に、$q = p$ (素数) のとき、つまり mod p で考えているときには以下が成り立つ。
$$ p - 2\sqrt{p} + 1 \le \\# E \le p + 2\sqrt{p} + 1$$

要するに $p$ に近いということである。SageMath で確かめると以下のようになる。($Y^2Z \equiv X^3 + XZ^2 + Z^3 \pmod{p}$ で $p$ を変えて試している)

```python
sage: EllipticCurve(GF(10007), [1, 1]).order()
10065
sage: EllipticCurve(GF(1000000007), [1, 1]).order()
999953006
sage: EllipticCurve(GF(998244353), [1, 1]).order()
998223252
```

この位数が以下の場合は攻撃が可能である。楕円曲線の問題を見たら初手で位数を確認しよう。
- $\\# E=p$ のとき: SSSA attack ([筆者の解説](https://github.com/koba-e964/code-reading/tree/master/algorithm/smart-attack)、[応用問題の解法コード](https://gist.github.com/elliptic-shiho/d13c2333adb4a94514753c8ca85a3f8e))
- $\\# E=p \pm 1$ のとき: MOV attack (有限体の上の離散対数問題になる。CTF では出題が難しそう)

## 群構造
楕円曲線は[アーベル群](https://ja.wikipedia.org/wiki/%E7%BE%A4_(%E6%95%B0%E5%AD%A6))とみなすことができる。

- 単位元は $(0:1:0)$ である。
- $(X:Y:Z)$ の逆元は $(X:-Y-a_1X-a_3Z:Z)$ である。Short Weierstrass 曲線の場合は $(X:-Y:Z)$ という非常に簡単な式になる。
- 演算は難しいので省略。知りたければ Google 検索とかでいくらでもヒットするのでそれを参考に。多分 CTF では要らん

楕円曲線がなす群については以下のいずれかに分類される。
- 1 点集合の群 (**自明な群**)
- [巡回群](https://ja.wikipedia.org/wiki/%E5%B7%A1%E5%9B%9E%E7%BE%A4) (0, 1, ..., s-1 mod s みたいなやつ)
- 巡回群 2 個の直和

今扱っている楕円曲線がどうなっているのかは、E.abelian_group() で表示できる。
```python
sage: EllipticCurve(GF(10007), [1, 1]).abelian_group()
Additive abelian group isomorphic to Z/10065 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + x + 1 over Finite Field of size 10007
sage: EllipticCurve(GF(1000000007), [1, 1]).abelian_group()
Additive abelian group isomorphic to Z/999953006 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + x + 1 over Finite Field of size 1000000007
sage: EllipticCurve(GF(998244353), [1, 1]).abelian_group()
Additive abelian group isomorphic to Z/998223252 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + x + 1 over Finite Field of size 998244353
sage: EllipticCurve(GF(131), [1, 1]).abelian_group()
Additive abelian group isomorphic to Z/64 + Z/2 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + x + 1 over Finite Field of size 131
sage: EllipticCurve(GF(3), [2, 2]).abelian_group()
Trivial group embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + 2*x + 2 over Finite Field of size 3
```

なお、先ほどのハッセの定理から、$E$ が自明な群となるためには、つまり $\\# E = 1$ となるためには $q$ が相当小さくなければいけないことに注意。$q - 2\sqrt{q} + 1 = (\sqrt{q} - 1)^2 \le 1$ が成立するから $q \le 4$ である。(つまりまともな暗号システムにおいてはこうなることはあり得ない。)

$E$ が巡回群であることはまあまあ多い。実用されている暗号システムでは基本的には素数位数の巡回群になるようにパラメーターが選ばれることが多いが、たまにそうでない場合がある。[A Dance of Add and Mul](https://alpacahack.com/ctfs/round-3/challenges/a-dance-of-add-and-mul) で扱われている [BLS12-381](https://hackmd.io/@benjaminion/bls12-381) の大元となる群も巡回群でない例である (BLS12-381 ではその一部だけ使って巡回群になるようにしている)。

## 群の生成元
SageMath の楕円曲線には [.gens()](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html#sage.schemes.elliptic_curves.ell_finite_field.EllipticCurve_finite_field.gens) と [.abelian_group()](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html#sage.schemes.elliptic_curves.ell_finite_field.EllipticCurve_finite_field.abelian_group) の 2 種類のメソッドがある。どちらも楕円曲線を群として見た時の構造を取得するために使われるメソッドだが、.gens() の場合値が「直交」しているとは限らない。
- .gens() も .abelian_group().gens() も巡回群の場合は 1 要素しか返さない。この場合特に考えることはない。
- .gens() が 2 個の値 $g_1, g_2$ を返してきた場合、この $g_1, g_2$ は確かに $E$ を生成する (つまり、任意の $h \in E$ に対して $h = j{g_1} + k{g_2}$ となる (j,k) が存在する) のだが、このような表し方が一意ではない可能性がある。
  - 人工的な例だが $G = \mathbb{Z}/11\mathbb{Z} \oplus \mathbb{Z}/33\mathbb{Z}$ を考えよう。$g_1 = (1, 1), g_2 = (0, 1)$ とする。このとき $g_1, g_2$ は $G$ を生成する ($g_1 - g_2 = (1,0)$ と $g_2$ が $G$ を生成するため) が、$11g_1 = 11g_2 = (0,11)$ となってしまうので、和としての表し方が一意ではない。(たとえば、$(1, 2) = g_1 + g_2 = -10g_1 + 12g_2 = \cdots$ など。$g_1, g_2$ は位数 (何倍したら単位元になるか) がどちらも 33 なので、本質的に異なる複数の表し方になっている。)
  - 一意でない場合は `g1.order() * g2.order() > E.cardinality()` となってしまう。
- 一方で .abelian_group().gens() の場合、必ず「直交」する。つまり和の表し方が一意になる (帰ってきた値は基底である)。
  - 「直交」させるために単なる .gens() に加えて余分な計算をする必要があり、それが非常に重い場合がある。しかし筆者はそのようなケースに遭遇したことがない。
  - 以下のように 2 番目の要素の位数 (`.order()`) が実現可能な最小の値になっている。
```python
sage: G = EllipticCurve(GF(131), [1, 1]).abelian_group()
sage: G
Additive abelian group isomorphic to Z/64 + Z/2 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + x + 1 over Finite Field of size 131
sage: G.gens()
((68 : 25 : 1), (75 : 0 : 1))
sage: G.gens()[0].order()
64
sage: G.gens()[1].order()
2
```
## (発展) 体拡大
たとえば $x^2 \equiv 2 \pmod{5}$ ($x^2 = 2 \in \mathrm{GF}(5)$) には $\mathrm{GF}(5)$ の中に根が存在しない。そのような場合により広い体を考えることによって根が存在するようになる場合がある。(実数の集合 $\mathbb{R}$ の内部にないから複素数の集合 $\mathbb{C}$ に議論領域を広げるのと同じノリ)

今回の場合は $\mathrm{GF}(25) = \mathrm{GF}(5^2)$ で考えると以下のように $4z_2+3$ (1 重), $z_2+2$ (1 重) という根が存在するようになる。
```python
sage: R.<x> = PolynomialRing(GF(5))
sage: (x^2 - 2).roots()
[]
sage: R.<x> = PolynomialRing(GF(25))
sage: (x^2 - 2).roots()
[(4*z2 + 3, 1), (z2 + 2, 1)]
```

なお、ここでいう $z_2$ とは $z_2^2 = z_2 + 3$ を満たす、$\mathrm{GF}(5)$ にはなかった謎の値である。
```python
sage: GF(25)
Finite Field in z2 of size 5^2
sage: K.<z2> = GF(25)
sage: z2
z2
sage: z2*z2
z2 + 3
sage: (4*z2+3)**2
2
```

また、$4z_2+3 = -(z_2+2)$ という事実に注意すること。移項すると $5z_2+5=0$ ということになるが、$\mathrm{GF}(5)$ と同様に $\mathrm{GF}(25)$ の中でも 5 倍したら全ての値が 0 になる ([標数](https://ja.wikipedia.org/wiki/%E6%A8%99%E6%95%B0)が 5 である) ことに注意しよう。一般に $\mathrm{GF}(p^k)$ の標数は $p$ である。

別の例を使ったより懇切丁寧な説明が [BLS12-381 For The Rest Of Us#Field extensions](https://hackmd.io/@benjaminion/bls12-381#Field-extensions)にある。

## (発展) 体拡大した時の要素の増え方
楕円曲線の定義式はそのままで体をコロコロ変えたい場合がある。そういった場合に $E: Y^2Z = X^3 + 4Z^3$ として $E(\mathrm{GF}(5))$ とか　$E(\mathrm{GF}(2^{128}))$ などという記法を使ったりする。
$k_1 \mid k_2$ のとき $\mathrm{GF}(p^{k_1}) \subseteq \mathrm{GF}(p^{k_2})$ である。そのため $E(\mathrm{GF}(p^{k_1})) \subseteq E(\mathrm{GF}(p^{k_2}))$ に決まっている。$K$ が体のとき $E(K)$ は群だったので、$E(\mathrm{GF}(p^{k_1}))$ の方が $E(\mathrm{GF}(p^{k_2}))$ の部分群ということである。つまり位数を見たときに $\\# E(\mathrm{GF}(p^{k_1})) \mid \\# E(\mathrm{GF}(p^{k_2}))$ が成立する。([ラグランジュの定理](https://ja.wikipedia.org/wiki/%E3%83%A9%E3%82%B0%E3%83%A9%E3%83%B3%E3%82%B8%E3%83%A5%E3%81%AE%E5%AE%9A%E7%90%86_(%E7%BE%A4%E8%AB%96))!)

疑う者は以下の実験結果を見よ。$a_1 \mid a_2 \mid a_4$ や $a_2 \not \mid a_3$ が成立している。
(ここで、 $a_k := \\# E(\mathrm{GF}(p^{k}))$ として `E.count_points(5)` は $[a_1, a_2, a_3, a_4, a_5]$ を返す。)
```python
sage: E = EllipticCurve(GF(7), [0, 4])
sage: E.count_points(5)
[3, 39, 324, 2379, 16833]
sage: 39 % 3
0
sage: 324 % 3
0
sage: 324 % 39
12
sage: 2379 % 39
0
```

楕円曲線 $E$ において、$m$ 倍すると単位元 $(0:1:0)$ になる点のことを $m$ 分点とよび、その集合を $E[m]$ と表す。以下のような定理が知られている。

**定理**: $p \not \mid m$ であり $E$ が十分大きい体[^explanation-sufficiently-large-field]で定義されていれば $E[m]$ は $\mathbb{Z}/m\mathbb{Z} \oplus \mathbb{Z}/m\mathbb{Z}$ と同型である。([講義資料](https://crypto.stanford.edu/pbc/notes/elliptic/torsion.html) や [Sil2016, Corollary III.6.4] など)

この定理を使えば $E(\mathrm{GF}(p^{k}))$ が巡回群 2 個以下の直和であることはほぼ明らか。[^explanation-gens-le-2]

[^explanation-sufficiently-large-field]: ここでいう「十分大きい体」とは、必要な値をすべて含んでいる拡大体を意味する。たとえば $\mathrm{GF}(7)$ で $x^2+1=0$ の根を考える時、 $\mathrm{GF}(7^2)$ はその根を含むし、 $\mathrm{GF}(7^{2k})$ はすべてその根を含む。代数閉包でもよいし普通は代数閉包を考えるが、今回は有限体の範囲で考察したかったため、このような議論にした。

[^explanation-gens-le-2]: 略証: $a := \\# E(\mathrm{GF}(p^{k}))$ とする。この時十分大きい体を $\mathrm{GF}(p^{l})$ とすれば $E(\mathrm{GF}(p^{l}))[a] \simeq \mathbb{Z}/a\mathbb{Z} \oplus \mathbb{Z}/a\mathbb{Z}$ である。$E(\mathrm{GF}(p^{k}))$ は巡回群 2 個の直和の部分群なのだから、それ自身巡回群 2 個以下の直和である。

実際に $Y^2Z = X^3 + 4Z^3$ において、定義体を $\mathrm{GF}(7)$ から $\mathrm{GF}(7^3)$ に変えることで 3 分点が $\mathbb{Z}/3\mathbb{Z} \oplus \mathbb{Z}/3\mathbb{Z}$ になることを確かめよう。

```python
sage: E = EllipticCurve(GF(7), [0, 4])
sage: try: E.torsion_basis(3)
....: except Exception as e: print(e)
curve does not have full rational 3-torsion
sage: F = E.division_field(3)
sage: F
Finite Field in t of size 7^3
sage: EE = E.change_ring(F)
sage: P, Q = EE.torsion_basis(3)
sage: P.order()
3
sage: Q.order()
3
sage: EE.abelian_group()
Additive abelian group isomorphic to Z/18 + Z/18 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + 4 over Finite Field in t of size 7^3
```

また、$a_k := \\# E(\mathrm{GF}(p^{k}))$ としたときに $b_k := p^k + 1 - a_k$ は以下のような 3 項漸化式に従う[^quadratic]:
$$b_{k+2} - b_1b_{k+1} + pb_k = 0$$
$k \ge 1$ だが $a_0 = 0, b_0 = 2$ によって外挿すれば $k \ge 0$ で成り立つ。

[^quadratic]: 二次方程式 $x^2 - b_1x + p = 0$ の根を $\alpha, \beta$ とすれば、$b_k = \alpha^k + \beta^k$ であって $a_k = p^k + 1 - \alpha^k - \beta^k = (\alpha^k - 1)(\beta^k - 1)$ である。こう見れば $k_1 \mid k_2$ のとき $a_{k_1} \mid a_{k_2}$ であることも自明に見えるかもしれない。

例によって証拠は以下の通り。
```python
sage: E = EllipticCurve(GF(7), [0, 4])
sage: arr = E.count_points(10)
sage: arr
[3, 39, 324, 2379, 16833, 117936, 824799, 5769075, 40366188, 282508239]
sage: b = [7**(i+1) + 1 - arr[i] for i in range(len(arr))]
sage: b
[5, 11, 20, 23, -25, -286, -1255, -4273, -12580, -32989]
sage: [b[i + 2] - b[0] * b[i + 1] + 7 * b[i] for i in range(len(b) - 2)]
[0, 0, 0, 0, 0, 0, 0, 0]
```
## (発展) $p^e$ 分点
$p \not \mid m$ のときの $m$ 分点の話はした。$p^e$ 分点の話をすれば網羅的になる。

**定理**: $E$ が十分大きい体で定義されていれば、$E$ ごとに次のいずれかが成り立つ ($e$ に依存しない):
- $E[p^e] = \lbrace O \rbrace$ (supersingular elliptic curve と呼ばれ、激レア)
- $E[p^e] \simeq \mathbb{Z}/p^e\mathbb{Z}$

これも[講義資料](https://crypto.stanford.edu/pbc/notes/elliptic/torsion.html) や [Sil2016, Corollary III.6.4] などを参考にされたい。

$E[p^e] = \lbrace O \rbrace$ である例:
```python
sage: E = EllipticCurve(GF(17), [0, 1])
sage: F = E.division_field(17)
sage: EE = E.change_ring(F)
sage: EE.abelian_group()
Additive abelian group isomorphic to Z/18 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + 1 over Finite Field of size 17
```

$E[p^e] \simeq \mathbb{Z}/p^e\mathbb{Z}$ である例 (`.abelian_group()` の直和分解において、一方だけが 7 の倍数であることに注意):
```python
sage: E = EllipticCurve(GF(7), [1, 1])
sage: F = E.division_field(7)
sage: EE = E.change_ring(F)
sage: EE.abelian_group()
Additive abelian group isomorphic to Z/29260 + Z/4 embedded in Abelian group of points on Elliptic Curve defined by y^2 = x^3 + x + 1 over Finite Field in t of size 7^6
sage: 29260 % 7
0
```

# 参考資料
## 初心者向け
- [楕円曲線暗号を実装して有名な攻撃を試してみる](https://zenn.dev/anko/articles/ctf-crypto-ellipticcurve)
- [楕円曲線暗号についてかじってみる](https://qiita.com/rinr0q/items/c1180bf2bc8ab9c7e62e)

## 中級者向けだが上級者も楽しめるスルメ記事
- [BLS12-381 For The Rest Of Us](https://hackmd.io/@benjaminion/bls12-381)
  - ペアリング曲線について丁寧に解説している。
- [SaveCurves](https://safecurves.cr.yp.to/)
  - あの djb が暗号技術における楕円曲線の選び方について詳細に解説している。

# 参考文献
- [Sil2016] Silverman, Joseph H. The Arithmetic Of Elliptic Curves. 2nd ed., Springer-Verlag, GTM 106, 2016.
