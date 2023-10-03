---
title: 大学の数学の入試問題を量化子消去でサッと解く
tags: 数学 論理学 入試問題
author: kobae964
slide: false
---
# 編集履歴
- 2023-10-03 インストール方法を編集しました。また todai-2012-1, todai-2023-3-1 を追加しました。
- 2021-03-10 初稿を発行しました。

# はじめに

大学の入試問題の数学の問題の中には、コンピュータプログラムを実行することで簡単に解くことができるものもあります。この記事では、実際にそれらの問題を解いてみます。

## 前提知識
- 高校数学
- 量化子、[量化](https://ja.wikipedia.org/wiki/%E9%87%8F%E5%8C%96)

## 量化子消去

量化子消去 (quantifier elimination, QE) というのは、量化子を含んだ論理式を、それと同値な量化子を含まない論理式に変形することを指します。例えば、高校数学で典型的な問題として、 2 次関数が常に x 軸の上にある (つまり $x^2 + ax + b > 0$ が常に成り立つ) のはいつか? という問題を考え、対応する論理式を見てみましょう。これは以下のようになります。
$$ \forall x. x^2 + ax + b > 0$$
これと同値な、量化子を含まない論理式は以下です。
$$ a^2 - 4b < 0$$

この変形により変数が少なくなり、簡単な式になりました。一般に量化子を消去することは問題を「解く」ことに相当し、この記事で見ていくのは量化子消去によって解ける問題です。

詳しくは [Quantifier Elimination - Wikipedia](https://en.wikipedia.org/wiki/Quantifier_elimination) を見てください。[^video]

[^video]: 私が量化子消去を最初に知ったのは http://www.kmonos.net/wlog/129.html#_2300130307 です。そこでも紹介されていますが、https://www.youtube.com/watch?v=IHkbXpjiX5g&ab_channel=OginoNobuyaOginoNobu という面白い動画があるので観ましょう。

## QEPCAD について
実数に関する[冠頭標準形](https://ja.wikipedia.org/wiki/%E5%86%A0%E9%A0%AD%E6%A8%99%E6%BA%96%E5%BD%A2)の論理式の量化子消去を行うことのできるソフトウェアです。冠頭標準形というのは、$\forall x. \exists y. x > y$ のように、論理式の先頭だけに量化子があるような論理式のことです。

### インストール方法
~~https://ashiato45.hatenablog.jp/entry/2018/04/25/070040 に詳しいです。~~
M1 Mac でコンパイルできない問題、およびインストールに手間がかかりすぎる問題があるので、記事を書きました: [QEPCAD を M1 Mac で実行する](https://qiita.com/kobae964/items/8f2597f15372ba1a984a)


# 問題
## 2020 年度東京大学 数学 (理科[^rika]) 第 1 問 
[^rika]: 数学なのに理科ってどういうことだよ、と思われるかもしれませんが、東大入試においては文系・理系という区分ではなく文科・理科という区分が用いられています。多分実態は同じで名前が違うだけだと思いますが。
[問題へのリンク](http://server-test.net/math/php.php?name=tokyo&v1=1&v2=2020&v3=1&v4=1&y=2020&n=1)
ここでは (3) のみを扱います。((1) と (2) も同様の手順で扱えます。)
以下の命題の量化子消去を行い、これが常に成り立つことを示したいです。
$$(\forall x. ax^2+bx+c>0 \wedge bx^2+cx+a>0 \wedge cx^2+ax+b>0 \Leftrightarrow x > p) \Rightarrow p = 0$$
しかしこれは冠頭標準形ではないので、以下の変形を用いて冠頭標準形に変換する必要があります。

- (∀x. φ(x)) -> ψ は ∃x. (φ(x) -> ψ) と同値。

この変形を行うと以下のような論理式になります。
$$\exists x. ((ax^2+bx+c>0 \wedge bx^2+cx+a>0 \wedge cx^2+ax+b>0 \Leftrightarrow x > p) \Rightarrow p = 0)$$

これを bin/qepcadd に与えると、以下のような結果が得られます。出力のところに `TRUE` とありますので、この論理式は常に成立する、つまり問題の条件から $p=0$ が言えることがわかりました。
https://gist.github.com/koba-e964/2eedd09699c1f531f300abcff80dffe9

```text:todai-2020-1.txt
[todai-2020-1]
(a,b,c,p,x)
4
(E x)[
    [[a x^2 + b x + c > 0 /\ b x^2 + c x + a > 0 /\ c x^2 + a x + b > 0] <==>
    [x > p]] ==> [p = 0]
].
finish
```

```text:出力抜粋
An equivalent quantifier-free formula:

TRUE


=====================  The End  =======================
```

(なお、証明問題ではなく、「$p$ についての条件を求めよ」という形式だと、$\exists a. \exists b. \exists c. \forall x. ax^2+bx+c>0 \wedge bx^2+cx+a>0 \wedge cx^2+ax+b>0 \Leftrightarrow x > p$ を量化子消去すればよいことになりますが、これを bin/qepcadd に与えるとメモリ不足のエラーになりました。一応 `+N<セル数>` というオプションはありますが、`+N10000000` を与えても解決しません。)

## 2021 年度東京大学 数学 (理科) 第 1 問
[問題へのリンク](http://server-test.net/math/php.php?name=tokyo&v1=1&v2=2021&v3=1&v4=1&y=2021&n=1)
(1) を扱います。
問題の条件を論理式で表すと以下の通りになります。

- 「方程式 $x^2+ax+b=-x^2$ は $-1<x<0$ の範囲にただ一つの解を持つ」→ $\exists!x_1. x_1^2+ax_1+b=-x_1^2 \wedge -1<x_1<0$
- 「方程式 $x^2+ax+b=-x^2$ は $0<x<1$ の範囲にただ一つの解を持つ」→ $\exists!x_1. x_2^2+ax_2+b=-x_2^2 \wedge 0<x_2<1$

ここで $\exists! a$ というのは、「条件を満たす $a$ がただ一つ存在する」という意味です。
これを表現するための記法が QEPCAD にはあり、`(X1 a)` という文字列で表現できます。

入出力は以下の通りです。出力によると答えは $|a|-2 < b < 0$ だそうです。

https://gist.github.com/koba-e964/9e4d77785702a3dbc80dc463e3733306

```text:todai-2021-1.txt
[todai-2021-1]
(a,b,x1,x2)
2
(X1 x1)(X1 x2)[
    [x1^2 + a x1 + b = -x1^2 /\ -1 < x1 /\ x1 < 0] /\
    [x2^2 + a x2 + b = -x2^2 /\ 0 < x2 /\ x2 < 1]
].
finish
```

```text:出力抜粋
An equivalent quantifier-free formula:

b < 0 /\ b + a + 2 > 0 /\ b - a + 2 > 0


=====================  The End  =======================
```

## 2021 年度東京大学 数学 (理科) 第 3 問
[問題へのリンク](http://server-test.net/math/php.php?name=tokyo&v1=1&v2=2021&v3=1&v4=3&y=2021&n=3)

(1) を扱います。
途中まで手計算をします。
$f(1) = 1/4 で f'(1) = 1/8$ なので、接線の方程式は $y = (x+1)/8$ です。これと $y = f(x)$ の共有点を求めるためには方程式 $x/(x^2+3) = (x+1)/8$ が解ければ良いですが、QEPCAD は除算を扱うことができません。そこで分母を払い、$8x = (x+1)(x^2+3)$ の解を求めることにします。$x \neq 1$ という条件があるので、これは $x = -3$ というのが出力からわかります。

https://gist.github.com/koba-e964/84372990b9d00b9ca5342e05f0c0ef34

```text:todai-2021-3.txt
[todai-2021-3]
(x)
1
[
    8 x = (x + 1) (x^2 + 3) /\
    x /= 1
].
finish
```

```text:出力抜粋
An equivalent quantifier-free formula:

x + 3 = 0


=====================  The End  =======================
```

## 2021 年度東京大学 数学 (理科) 第 6 問
[問題へのリンク](http://server-test.net/math/php.php?name=tokyo&v1=1&v2=2021&v3=1&v4=6&y=2021&n=6)

またもメモリ不足。

https://gist.github.com/koba-e964/923063ef272c5bbe256d03b029d42001

```text:todai-2021-6.txt
[todai-2021-6]
(b,c,p,q,r,x)
5
(A x)[
    x^4 + b x + c = (x^2 + p x + q) (x^2 - p x + r) /\ [p > 0 \/ p < 0]
].
finish
```

## 2021 年度京都大学 数学 (理系) 第 2 問
[問題へのリンク](http://server-test.net/math/php_q.php?name=kyoto&v1=1&v2=2021&v3=1&y1=2021&n1=1&y2=2021&n2=2&y3=2021&n3=3&y4=2021&n4=4&y5=2021&n5=5&y6=2021&n6=6&y7=0000&n7=0)

途中まで手計算をします。
$y = (x^2+1)/2$ の $(t, (t^2+1)/2)$ における接線は $y = tx + (-t^2+1)/2$ です。この接線と x 軸との交点 Q は $(t/2 - 1/2t, 0)$ です。よって $\mathrm{PQ}^2 = (t/2+1/2t)^2 + ((t^2+1)/2)^2$ です。
この $\mathrm{PQ}^2$ の取りうる範囲を量化子消去で求めれば、最小値がわかるわけです。

この式を論理式に直すため、以下のステップを踏みます。

- $s = 1/t$ と置く。これは QEPCAD は除算を認識できないため。この式自体は $\exists s. ts = 1 \wedge ....$ という式で表現できる。
- $dx = t/2+1/2t, dy = (t^2+1)/2, l = \mathrm{PQ}$ と置く。

最終的な論理式は以下のようになります。
$$\exists t.\exists s. \exists dx. \exists dy.
t s = 1 \wedge
2 dx = t + s \wedge
2 dy = t^2 + 1 \wedge
l^2 = dx^2 + dy^2$$

これに対し量化子消去を実行すると、$16 l^2 - 27 \ge 0$ が得られます。($l$ は非負なので、) つまり $l \ge 3\sqrt{3}/4$ ということです。めでたしめでたし。

https://gist.github.com/koba-e964/68b864f4f11c1ed0fd5b13007c6ac845

```text:kyodai-2021-2.txt
[kyodai-2021-2]
(l,t,s,dx,dy)
1
(E t)(E s)(E dx)(E dy)[
    [t s = 1] /\
    [2 dx = t + s] /\
    [2 dy = t^2 + 1] /\
    [l^2 = dx^2 + dy^2]
].
finish
```

```text:出力抜粋
An equivalent quantifier-free formula:

16 l^2 - 27 >= 0


=====================  The End  =======================
```

## 2012 年度東京大学 数学 (理科) 第 1 問
[問題へのリンク](http://server-test.net/math/php.php?name=tokyo&v1=1&v2=2012&v3=1&v4=1&y=2012&n=1)

途中まで手計算をします。直線 $l$ を $y = tx$ とすると、直線 $x=\sqrt{2}/3$ および 円 $x^2 + (y-1)^2=1$ について、$l$ との交点の x 座標はそれぞれ $\sqrt{2}/3$ と $2t/(1+t^2)$ です。なのでこれらの間の距離 $L$ は
$$f(t) = \left(\frac{2t}{1+t^2} - \frac{\sqrt{2}}{3}\right)\sqrt{1+t^2}$$
です。これの最大値を求めたいので $\exists t\ldotp L = f(t)$ の量化子消去を試みたいのですが、QEPCAD は平方根や分数を扱えないので、いつものように変数を導入してそれらを消します:
$$ q := \frac{2t}{1+t^2}
\Longrightarrow  q(1+t^2)=2t$$
$$ s := \sqrt{1+t^2}
\Longrightarrow  s^2 = 1+t^2 \wedge s \ge 0$$
$$ s_{23} := \frac{\sqrt{2}}{3}
\Longrightarrow  9s_{23}^2 = 2 \wedge s_{23} > 0$$ 

最終的に $3L^2-2\le 0 \vee L<0$ が得られるので、$L \le \sqrt{2/3}$ が結論です。$L = \sqrt{2/3}$ を実現する $t$ の値も計算すると、$t = 1/\sqrt{2}$ であることがわかります。

https://gist.github.com/koba-e964/a8671abd91887051dc93e16b31a48783

```
An equivalent quantifier-free formula:

3 L^2 - 2 <= 0 \/ L < 0


=====================  The End  =======================
```

## 2023 年度東京大学 数学 (理科) 第 3 問
[問題へのリンク](http://server-test.net/math/php.php?name=tokyo&v1=1&v2=2023&v3=1&v4=3&y=2023&n=3)

(1) がそのまま論理式に変換できます。最終的に $4a - 5 > 0$ という出力が得られるので、$a > 5/4$ が答えです。


https://gist.github.com/koba-e964/8f944172bc5e577198cd68ded4bf870b

```
An equivalent quantifier-free formula:

4 a - 5 > 0


=====================  The End  =======================
```

# まとめ
いかがでしたか?
量化子除去は強力なツールで、実数についての加減乗除についての式であれば理論的には完全に解けることが証明されています。とはいえ、実用上は変数の個数が多いと計算時間・メモリ両方の観点で難しいのが現実です。
それでは皆様、楽しい量化子除去ライフを。
