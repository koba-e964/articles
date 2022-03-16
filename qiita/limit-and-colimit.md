2018/11/12 (月) 発表分です。

## limit と colimit の定義

圏 $\mathcal{C}$ の部分圏 $\mathcal{D}$ に対して、対象 $X$ が $\mathcal{D}$ の **極限** (_limit_) であるとは、 $X$ から $\mathcal{D}$ の全ての対象への射があり ($X$ と $X$ から $\mathcal{D}$ の対象への射の族を合わせたものを **錐** (_cone_) と呼ぶ)、他の任意の錐からその錐への射が存在することをいう。 (詳しくは https://ja.wikipedia.org/wiki/%E6%A5%B5%E9%99%90_(%E5%9C%8F%E8%AB%96)#%E6%A5%B5%E9%99%90 を参照してください。図式 (関手) ではなく部分圏を考えていること以外は一緒です。) このとき $X = \lim_{\leftarrow} \mathcal{D}$ と表記する。教科書では錐の射は $\nu_i \colon X \to D_i$ と表記される。

**余極限** (_colimit_) については、双対圏の極限という形で定義ができる。 $X$ が 部分圏 $\mathcal{D}$ の余極限であるとき、 $X = \lim_{\rightarrow} \mathcal{D}$ と表記する。このとき双対圏の錐に当たるものを **余錐** (_cocone_) と呼ぶ。教科書では余錐の射は $\mu_i \colon D_i \to X$ と表記される。

## 例
(1) $\mathcal{D} = \\{D_0 \to D_1 \to D_2 \to \cdots, f_{ij} \colon D_i \to D_j\\}$ の場合
limit $$\lim_{\leftarrow} \mathcal{D} = D_0$$ は自明。
colimit $$\lim_{\rightarrow} \mathcal{D}$$ は自明ではない。集合の圏 $\mathbf{Sets}$, 群の圏 $\mathbf{Gr}$, アーベル群の圏 $\mathbf{Ab}$ については、これは **帰納的極限** (_inductive limit_) と呼ばれるものになる。帰納的極限とは以下のようなものである:
$$ D_\infty := \left(\coprod_{i = 0}^{\infty} D_i\right) / \mathord{\sim}$$
ただし、$\coprod_{}$ は集合の[非交和](https://ja.wikipedia.org/wiki/%E9%9D%9E%E4%BA%A4%E5%92%8C)であり、同値関係 $\mathord{\sim}$ は次のように定義される: 0以上の整数 $i, j$ と $x \in D_i, y \in D_j$ に対して、
$$(i, x) \sim (j, y) :\Leftrightarrow \exists k \ldotp k \ge i, k \ge j, f_{ik}(x) = f_{jk}(y)$$

以下のように余錐を定めることにより、 $D_\infty = \lim_{\rightarrow} \mathcal{D}$ が証明できる:
$$ \mu_i \colon D_i \to D_\infty; x \mapsto [(i, x)]$$
証明は $\mathbf{Sets}$ においては普遍性を確かめればよく、 $\mathbf{Gr}, \mathbf{Ab}$ においてはさらに $\mu_i$ および $\mathbf{Sets}$ での議論において普遍性によって定まる射 (教科書では $\sigma$ と呼ばれる) が群の準同型写像であることを証明すれば良い。
