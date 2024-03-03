## はじめに
趣味で数学を勉強している kobae964 です。
置換群の位数を求めるための[Schreier-Simsのアルゴリズム](https://en.wikipedia.org/wiki/Schreier%E2%80%93Sims_algorithm)というアルゴリズムを実装したので、それの解説を書いていきたいと思います。

[GAP](https://www.gap-system.org/) などの計算代数システムを使うと、色々な計算をたちどころに行うことができて非常に便利です。しかし、その内部でどのような処理が実行されているのかは、知らない方も多いのではないでしょうか?
置換群の位数の計算もその一つのように思えます。例えば 3x3x3 ルービックキューブの可能な配置は有限群をなすので、その総数は、GAP を使用すれば[簡単に計算できます](http://staff.fukuoka-edu.ac.jp/fujimoto/rubik.pdf)。しかし、そのために内部でどのような処理を必要とするのかは決して自明ではありません。ルービックキューブの配置の総数は $43252003274489856000 \simeq 4.3 \cdot 10^{19}$ であり、愚直に一つ一つの配置を見ていくやり方では決して全数検査を行うことはできません。
これの内部で実行されているのが Schreier-Sims のアルゴリズムです。$n$ 点の置換群について、$n$ についての多項式時間で位数を計算することができます。

これから数回に分けて、Schreier-Sims のアルゴリズムのアイディア、実装を追っていけたらと思います。今回はアルゴリズムの基本方針についてです。
なお、実装は[このレポジトリ](https://github.com/koba-e964/rust-schreier-sims)に置いてあるので、興味のある方は読んでいただけたらと思います。

## 前提知識
群論の初歩的な知識、およびグラフ理論の初歩的な知識を仮定する。
群論: https://ja.wikipedia.org/wiki/%E7%BE%A4%E8%AB%96
グラフ理論
 - 深さ優先探索 https://ja.wikipedia.org/wiki/%E6%B7%B1%E3%81%95%E5%84%AA%E5%85%88%E6%8E%A2%E7%B4%A2
 - 幅優先探索 https://ja.wikipedia.org/wiki/%E5%B9%85%E5%84%AA%E5%85%88%E6%8E%A2%E7%B4%A2

## 記法
群の作用は右作用とする。つまり、$x$ に対する $g$ の作用を $x^g$ と表記する。これは GAP における流儀と合わせている。普通の記法と逆であることに注意されたい。
作用は $x^{g \cdot h} = (x^g)^h$ を満たす必要がある。
## 導入
計算群論において、[Schreier-Simsのアルゴリズム](https://en.wikipedia.org/wiki/Schreier%E2%80%93Sims_algorithm)は[置換群](https://ja.wikipedia.org/wiki/%E5%AF%BE%E7%A7%B0%E7%BE%A4)の位数を計算したり、置換群にある要素が含まれているかを高速に判定したり…などのクエリへの高速な応答を行うための前計算アルゴリズムである。
今回は置換群 $G$ の位数 $|G|$ を求めるところに焦点を当てて議論する。形式的に表記すると、対称群 $S_n$ が集合 $\\{0, 1, \ldots, n - 1\\}$ の上に作用するとしたとき、我々の興味は生成元の集合 $X = \\{ x_1, ..., x_k\\} \subseteq S_n$によって生成された群 $G = \langle X \rangle \subseteq S_n$ の位数を計算することである。
これにより例えば以下のことが可能になる:
- 3x3x3 ルービックキューブのありうる配置の総数を求める。キューブの面は合計 54 個あり、各面の中心は動かさないとしてよいので、合法的な操作は $S_{48}$ の部分群をなすとみなすことができる。(もっと精密な議論をすると、例えばコーナー・キューブとエッジ・キューブが互いに移り変わることはないため $S_{24} \times S_{24}$ の部分群であるということも可能であるが、ここではこの事実は使用しない。)


## 軌道安定化群定理
一般に部分群の位数や指数を求めるのは簡単なことではない。しかし[安定化群](https://ja.wikipedia.org/wiki/%E7%BE%A4%E4%BD%9C%E7%94%A8#%E8%BB%8C%E9%81%93%E3%81%A8%E7%AD%89%E6%96%B9%E9%83%A8%E5%88%86%E7%BE%A4)に限って言えば指数を求めるのは比較的簡単である。それには以下の定理が重要である。
> $G$ を $S_n$ の部分群とし、集合 $\\{0, 1, \ldots, n - 1\\}$ の上の自然な作用を考える。$x \in \\{0, \ldots, n - 1 \\}$ の[軌道](https://ja.wikipedia.org/wiki/%E7%BE%A4%E4%BD%9C%E7%94%A8#%E8%BB%8C%E9%81%93%E3%81%A8%E7%AD%89%E6%96%B9%E9%83%A8%E5%88%86%E7%BE%A4)を $\mathrm{Orb}\_G(x) := \\{ x^g \mid g \in G\\}$, $x$ の安定化群を $\mathrm{Stab}\_G(x) := \\{g \in G \mid x^g = x \\}$ と表記することにする。このとき、$|\mathrm{Orb}_G(x)| \cdot |\mathrm{Stab}_G(x)| = |G|$ が成り立つ。

$\mathrm{Orb}_G(x)$ はグラフ理論的なアルゴリズム(深さ優先探索)で効率的に計算できるため、安定化群の大きささえわかれば $|G|$ がわかる。

## 安定化群を用いた減少列の構成
軌道安定化群定理を用いて次々に安定化群を作っていくことを考える。
$H_0 = G$ として、 $H_1 = \mathrm{Stab}\_{H_0}(u_0), H_2 = \mathrm{Stab}\_{H_1}(u_1), \ldots, H_k = \mathrm{Stab}\_{H_{k-1}}(u_{k-1}) = \\{e\\}$ として、$G = H_0 \supseteq H_1 \supseteq H_2 \cdots \supseteq H_k = \\{e\\}$ という部分群の列を構成したとする。このとき $|G| = (H_0 : H_1) (H_1 : H_2) \cdots (H_{k - 1} : H_k)$ が成り立つ。各 $(H_i : H_{i+1}) = |\mathrm{Orb}\_{H_i}(u_i)|$ が計算できれば $|G|$ も計算できる。
このようにして順番に全ての点を固定して、各時点での軌道の大きさを求め、それら全てを掛け合わせることで $|G|$ を計算する、というのが Schreier-Sims のアルゴリズムによる位数計算の基本的なアイディアである。

以上の議論において、各ステップで計算すべきものは二つある。
- $G$ の生成元が与えられたとき、$\mathrm{Orb}\_G(x)$
- $G$ の生成元が与えられたとき、 $\mathrm{Stab}\_G(x)$ の生成元
$\mathrm{Orb}\_G(x)$ は $G$ の作用によって到達可能な点全体の集合であるため、幅優先探索か深さ優先探索で計算できる。$\mathrm{Stab}\_G(x)$ の生成元は難しいが以下のアイディアによって計算できる。

> $\mathrm{Orb}\_G(x)$ の計算中にグラフの探索によって $x^p = x^q$ である $p, q \in G$ が見つかったとする。この時、$x^{p^{-1} \cdot q} = x$ であるため、$p^{-1} \cdot q \in \mathrm{Stab}\_G(x)$ である。逆に、$\mathrm{Stab}\_G(x)$ の元はすべてこのような元の積で表せる。 (Schreier's Theorem, [HoltSlide] の p.8)

これを認めれば $\mathrm{Stab}\_G(x)$ の生成元も計算できる。

実装には幅優先探索を用いた。以下に擬似コードを与える。 ([HoltSlide] の p.6)
実装本体は [main.rs@754d0ad#L76-L110](https://github.com/koba-e964/rust-schreier-sims/blob/754d0ad18676db142436eee22955e0908749c049/src/main.rs#L76-L110) である。

```
function orbit_stabilizer(n: 要素数, gen: G の生成元のリスト, v: 固定する点) -> (軌道, 安定化群の生成元) {
  que: キュー
  stabilizer_gen: 安定化群の生成元のリスト
  (u, e) を que に積む
  stabilizer_gen <- []
  for (y, g) in que {
    if y に訪れたことがある {
      p(u) = y という情報が記録されていれば、 p^{-1} * g は y を固定する。
      gen.push(p^{-1} * g);
      continue;
    }
    y に訪れたことにし、g(u) = y という情報を記録する
    for x in gen {
      (x(y), g * x) を que に積む
    }
  }
  return (orb, stabilizer_gen)
}
```


## Schreier-Simsのアルゴリズム (愚直)
0 から $n - 1$ までの点を順番に固定して、徐々に群を小さくするというのが基本方針である。
上の数式でいうと $u_0 = 0, u_1 = 1, \ldots, u_{n - 1} = n - 1$ である。

```
function schreier_sims(n: 点数, x: 生成元の集合) -> 位数 {
  gen := x;
  ord := 1;
  for i in 0..n {
    (orb, stab) := calc_orbit_stabilizer(n, i, x);
    ord *= |orb|;
    gen = stab;
  }
  return ord;
}
```

ソースコード: [main.rs@754d0ad](https://github.com/koba-e964/rust-schreier-sims/blob/754d0ad18676db142436eee22955e0908749c049/src/main.rs)
これの計算量はどうなるだろうか? 各ステップで、生成元の個数は最悪 (軌道の大きさ) 倍になる。$i$ を固定するときの軌道の大きさは最大で $n - i$ であるため、これらの積は最悪 $O(n!)$ 程度である。当初の目標は $n = 48$ の場合に群の位数を求めることだったため、これでは到底間に合わない。

多項式時間バージョンの Schreier-Sims のアルゴリズムはより賢い工夫をしている。それの解説は次回以降に譲る。

## 参考文献
[HoltSlide] https://blogs.cs.st-andrews.ac.uk/codima/files/2015/11/CoDiMa2015_Holt.pdf このスライドを主に利用して実装しました。
