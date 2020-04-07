独立集合問題は $W[1]$ 困難です。これの証明 [1] に感服したので、概要をまとめます。

## 想定読者
$W[t]$ の定義や FPT の基本的な定義を理解しているが、独立集合問題の $W[1]$ 困難性の証明をしようとして挫折した人。
論理式と回路を同一のものとして扱っても問題ない人。[^0]
[^0]: 深さが固定であれば論理式と回路は相互変換可能であるため、問題ない。

## 準備
導入
$W[t, h, s]$: weft が t 以下、深さが h 以下、fan-in の最大個数が s 以下であるような回路の重み付き充足可能性問題に FPT 帰着できる問題の集合。[^1]
[^1]: ここでいう $W[1, 2, s]$ が、元論文の $W[1, s]$ にあたる。元論文では (おそらく、ほとんどの場合で深さは 2 以下であることを理由に) 深さのパラメタを省略しているが、ここではわかりやすさのため毎回律儀に書くことにする。

$W_{\mathrm{anti}}[t, h, s]$: $W[t, h, s]$ の定義に登場する回路に、「全ての変数には直後に not ゲートが適用され、それ以外に not ゲートは登場しない」という条件を課した場合の問題の集合。[^2][^top-is-always-and]
[^2]: 元論文では antimonotone $W[1, s]$ などと書かれている。毎回 antimonotone と表記するのは煩雑であるため、簡略化した記法を用いる。
[^top-is-always-and]: 以下で証明することなく、「回路の出力に繋がっているのが big and gate であり、それの入力は small or gate であると仮定して良い」という事実を使う。これは $s$-RED/BLUE NONBLOCKER を論理式として解釈するとこの形になることから示せる上、示さなくても議論の流れに問題は生じない。
重み付き充足可能性問題: 論理式 C と非負整数 k が与えられる。重み[^weight] k の変数割り当てであって、C を満たすものが存在するか?[^sat-spoiler]
[^weight]: 割り当ての中で true である変数の個数
[^sat-spoiler]: ちなみに、(2-SAT は線形時間で解けるが) 重み付き 2-SAT は $W[1]$ 困難であることが知られている。正体現したね
独立集合問題: 無向グラフと非負整数 k が与えられる。大きさ k の独立集合が存在するか?
FPT 帰着: 入力 $(I, k)$ を、$f(k) \times \mathrm{poly}(|I|)$ 時間で別の問題の入力 $(J, k')$ に変換して、両者の答えが同じになるようにできるとき、その変換のことを帰着と呼ぶ。このとき、ある関数 $g$ に対して $k' \le g(k)$ となっていなければならない。($|I|$ などに依存してはいけない。)

## 方針の概略
(1) 深さを 2 以下にする
(2) antimonotone 性を仮定しても問題ないことを示す (一番技巧的)
(3) fan-in の最大個数が 2 であることを仮定して問題ないことを示す
(4) $W_{\mathrm{anti}}[1, 2, 2]$ の回路は簡単に独立集合問題に FPT 帰着できるので、FPT 帰着する。

## (1) 深さを 2 以下にする
以下の Lemma 2.1 を証明すれば良い。
Lemma 2.1: $W[1, h, 2] \subseteq W[1, 2, s]$ ($s = 2^h + 1$ とする)
(証明)
重み付き充足可能性問題のインスタンスとして、
深さ h 以下、fan-in 最大 2 の回路が与えられた時に、それを深さ 2 以下、fan-in 最大 s の回路の重み付き充足可能性問題に FPT 帰着する。
### Step 1. tree circuit にする
まあやればよい。ここで諸々 $O(2^{2^h})$ 倍にはなるはず。
### Step 2. not を入力側に移動する
これもやるだけ
### Step 3. depth 4 normalization を行う
詳しい方法は省略。これにより、fan-in $2^h$ 、深さ 4 の回路ができる。この回路は$\Pi\Sigma({\prod}\Sigma \vee {\sum}\Pi)$ の形をしているため、これの weft は 1 である。(${\prod}$ と ${\sum}$ は big node を表す。)
ここまで、与えられた論理式と同値な論理式であるため、充足可能性も同値。
### Step 4. 深さを 2 にする
ここで、与えられた論理式と同値な論理式ではないが充足可能性は同値であるような論理式を作る。
補助変数を導入し、$\prod (x_1 \vee \cdots \vee x_s)$ の形の回路を作る。
典型パターンとして、元の問題の変数を増幅して、補助変数の影響が皆無になるようにするテクニックが使われる。

- 元の変数 $x$ に対して $x[0] \Rightarrow x[1], x[1] \Rightarrow x[2], \ldots, x[A -1] \Rightarrow x[0]$[^imply-is-clause] という $A$ 個の CNF 節を用意すれば、$A$ 個の同値な変数ができる。A は k によって決まる値とすれば、いくら増やしても FPT 帰着の範疇に収まる。

実際にどういう論理式を作るのかは元論文を参照されたい。
[^imply-is-clause]: $x \Rightarrow y$ は $\neg x \vee y$ のことであるため CNF 節である。
これによって $W[1, h, 2] \subseteq W[1, 2, 2^h + 1]$ が言えるため、
$$
W[1] = \bigcup_{h=0}^{\infty} W[1, h, 2] \subseteq \bigcup_{h=0}^{\infty} W[1, 2, 2^h + 1] \subseteq \bigcup_{s=2}^{\infty} W[1, 2, s]
$$が言える。

## (2) antimonotone 性を仮定しても問題ないことを示す (一番技巧的)
$s$-RED-BLUE NONBLOCKER という問題を導入する。
$W_{\mathrm{anti}}[1, 2, s]$ に属する $s$-RED-BLUE NONBLOCKER が $W[1, 2, s]$ 困難であることを示すことで、$W[1, 2, s] \subseteq W_{\mathrm{anti}}[1, 2, s]$ を示す。逆の包含関係は自明。

### s-RED-BLUE NONBLOCKER
無向グラフ[^bipartite] $G = (V, E)$、頂点集合の分割 $V = V_{\mathrm{red}} + V_{\mathrm{blue}}$ および非負整数 $k$ が与えられる。$V_{\mathrm{blue}}$ 側の最大次数は $s$ 以下である。[^degree]大きさ $k$ の部分集合 $W \subseteq V_{\mathrm{red}}$ であって、以下の条件を満たすものが存在するか?

- 任意の $v \in V_{\mathrm{blue}}$ に対し、$v$ と隣接する頂点 $w$ であって $w \not \in W$ を満たすものが1個以上存在する。

[^bipartite]: 無向二部グラフに制限しても複雑性は変わらない。
[^degree]: 元論文では $V_{\mathrm{red}}$ 側にも最大次数 $s$ 以下の制約が課されているが、おそらく誤りである。というのは、以下で構築するグラフが条件を満たさないため。
上の条件を満たす $W$ を <b>nonblocker</b> と呼ぶ。

上の条件を論理式で表記すると $\bigwedge_{v \in V_{\mathrm{blue}}} \bigvee_{w: v\text{と隣接}} \neg w$ という形になるため、これが $W_{\mathrm{anti}}[1, 2, s]$ に属することは明らか。

### 証明
$s$-CNF $C_1 \wedge C_2 \wedge \cdots \wedge C_m$ と非負整数 $k$ が与えられたとする。$k' = 2k$ とする。
与えられた $s$-CNF を充足する重み $k$ の変数の割り当てが存在することと、以下で構築する二部グラフ $G = (V_{\mathrm{red}}, V_{\mathrm{blue}}, E)$ に大きさ $k'$ の nonblocker が存在することは同値である。

$V_{\mathrm{red}} := V_1 + V_2, V_{\mathrm{blue}} := V_3 + V_4 + V_5 + V_6 + V_7$
$E := E_1 + E_2 + E_3 + E_4 + E_5 + E_6 + E_7 + E_8$

- 赤い頂点
 - $V_1 := \\{a[r, v] \mid 0 \leq r \lt k, 0 \leq v \lt n\\}$
     - 意図: $a[r, v] = \mathrm{true} \Leftrightarrow $ 元の変数割り当てにおいて、$r$ 番目の true は $x_v$
 - $V_2 := \\{b[r, v, p] \mid 0 \leq r \lt k, 0 \leq v \lt n, 1 \leq p \leq n-k+1 \\}$
     - 意図: $b[r, v, p] = \mathrm{true} \Leftrightarrow $ 元の変数割り当てにおいて、$r$ 番目 の true は $x_v$ で $(r + 1)\%k$ 番目の true は $x_{(v + p)\%n}$ である。特に、$x_{v+1}, \ldots, x_{(v+p-1)\%n}$ は全て false である。
- 青い頂点、および辺
  - $V_1$ のうち、選択されるのは各行高々 1 個
      - $V_3 := \\{c[r, v_1, v_2] \mid 0 \leq r \lt k, 0 \leq v_1 \lt v_2 \lt n\\}$
      - $E_1 := \\{a[r, q]c[r, v_1, v_2] \mid q \in \\{v_1, v_2\\}\\}$
  - $V_2$ のうち、選択されるのは各行高々 1 個
      - $V_4 := \\{d[r, v_1, p_1, v_2, p_2] \mid (v_1, p_1) \neq (v_2, p_2)\\}$
      - $E_2 := \\{b[r, q_1, q_2]d[r, v_1, p_1, v_2, p_2] \mid (q_1, q_2) \in \\{(v_1, p_1), (v_2, p_2)\\}\\}$
  - $a[r, v_1]$ と $b[r, v_2, p]$ が同時に選択される場合、$v_1 = v_2$
      - $V_5 := \\{e[r, v_1, v_2, p] \mid v_1 \neq v_2 \\}$
      - $E_3 := \\{a[r, v_1]e[r, v_1, v_2, p]\\}$
      - $E_4 := \\{b[r, v_2, p]e[r, v1, v_2, p]\\}$
  - 差分を強制
      - $V_6 := \\{f[r, v_1, v_2, p] \mid v_1 + p \not \equiv v_2 \pmod n\\}$
      - $E_5 := \\{b[r, v_1, p]f[r, v_1, v_2, p]\\}$
      - $E_6 := \\{a[(r + 1)\%k, v_2, p]f[r, v_1, v_2, p]\\}$
  - 元の論理式を強制 ($R(j, j')$ に含まれる $s$ 個の点は同時にとってはいけない)
      - $V_7 := \\{g[j, j'] \mid 1 \le j \le m, 1 \le j' \le m_j\\}$
      - $E_7 := \\{a[r, v]g[j, j'] \mid a[r, v] \in R(j, j')\\}$
      - $E_8 := \\{b[r, v, p]g[j, j'] \mid b[r, v, p] \in R(j, j')\\}$

$R(j, j')$ を定義する。与えられた論理式の各節 $C_i = l_{j1} \vee l_{j2} \vee \cdots \vee l_{js}$ について、$l_{jq} = x_{u}$ であれば $P_{q} := \\{b[r, v, p] \mid 0 \le r \lt k, 0 \le v \le n, 1 \le p \le n-k+1, 1\le (q-v)\%n \lt p\\}$ と、$l_{jq} = \neg x_{u}$ であれば $P_{q} := \\{a[r, u] \mid 0 \le r \lt k\\}$ と置く。$P_{q}$ の直積[^tuple-as-set]の要素を並べたものを $R(j, 1), R(j, 2), \ldots, R(j, m_j)$ とする。
$|P_{q}| \le n^3$ であるため、$m_j \le n^{3s}$ である。
[^tuple-as-set]: 通常直積はタプルを要素に持つように定義するが、ここでは集合を要素に持つように定義する。つまり、 $\\{1, 2\\} \times \\{3, 4\\} = \\{\\{1, 3\\}, \\{1, 4\\}, \\{2, 3\\}, \\{2, 4\\}\\}$ などとなる。

同値性を示す。元の論理式を充足する重み $k$ の変数割り当てにおいて、true である変数を $x_{h_0}, \ldots, x_{h_{k-1}}$ としたとき、$A := \\{a[i, h_i] \mid 0 \le i \lt k\\} + \\{b[i, h_i, (h_{(i + 1)\%k} - h_i) \% n] \mid 0 \le i \lt k\\}$ が $G$ の重み $k' = 2k$ の nonblocker であることは明らか。($V_7$ が一番難しいパートだが、元の論理式の節 $C_j$ が充足されていることから、$C_j$ の中で true であるリテラルに相当する $R(j, j')$ の頂点が $A$ に含まれないことがわかる。)
$G$ の重み $k'$ の nonblocker $A$ が存在したとする。nonblocker の頂点は $a$ と $b$ の各行に高々 1 個しかないという条件から、すべての行にちょうど1個存在することが言える。$V_7$ の条件から、各 $j$ について、全ての $j'$ に対して $R(j, j')$ に含まれる頂点のうち少なくとも1つが $A$ の要素でないことがいえるため、上の議論においてどれか一つの $P_q$ の頂点がすべて $A$ の要素でないことが言える。($Q_q := P_q \cap A$ と置くと、$R(j, 1), \ldots, R(j, m_j)$ の中で全ての要素が $A$ に含まれるものはちょうど $|Q_1| \times |Q_2| \times \cdots |Q_s|$ 個あり、これが 0 個なのだからどれか一つの $|Q_q|$ は 0 である。) よって、$A$ から自然に定まる変数割り当ては元の論理式を充足する。

以上までにわかったことから、$$
W[1] \subseteq \bigcup_{s=2}^{\infty} W[1, 2, s] \subseteq \bigcup_{s=2}^{\infty} W_{\mathrm{anti}}[1, 2, s]
$$が言える。
## (3) fan-in の最大個数が 2 であることを仮定して問題ないことを示す (Proposition 3.3)
$s \ge 2$ のとき $W_{\mathrm{anti}}[1, 2, s] \subseteq W[1, 2, 2]$ を示す。
登場するリテラルがすべて変数の否定であるような $s$-CNF、および非負整数 $k$ が与えられたとする。
元の論理式と充足可能性が同値になるように、以下のように 2-CNF を構成する。また $k' = k 2^k + \sum_{i=2}^s C(k, i)$ と置く。

- 変数:
 - 元の変数 $x_i$ に対し、$2^k$ 個に増幅させた変数 $x[i, 0], \ldots, x[i, 2^k - 1]$
 - 元の変数の 2 点以上 s 点以下の集合 V に対し、$V$ に対応する変数 $c[V]$
- 2-CNF 節:
 - 変数 $x_i$ と $0 \leq a \lt 2^k$ に対し、$x[i, a] \Rightarrow x[i, (a + 1) \% 2^k]$
 - $V \subseteq V'$ なる集合ペアに対し、$c[V'] \Rightarrow c[V]$
 - $x_i \in V$ なる $x_i, V$ に対し、$c[V] \Rightarrow x[i, 0]$
 - 元の CNF が $\bigvee \\{\neg x \mid x \in V\\}$ の論理積であるとしたとき、それぞれの V に対して $\neg c[V]$

これを充足する重み $k'$ の割り当てがあることと、元の論理式を充足する重み $k$ の割り当てがあることは同値。(元の論理式の割り当てで true であるような変数の集合を $T$ としたとき、$x[i, 0] = \mathrm{true} \Leftrightarrow x_i \in T, c[V] = \mathrm{true} \Leftrightarrow V \subseteq T$ となるように 2-CNF 節と $k'$ を決めている)

以上までにわかったことから、$$
W[1] \subseteq \bigcup_{s=2}^{\infty} W_{\mathrm{anti}}[1, 2, s] \subseteq W[1, 2, 2] \subseteq W_{\mathrm{anti}}[1, 2, 2] 
$$が言える。

## (4) W_anti[1,2,2] の回路を独立集合問題に FPT 帰着する。 (Theorem 3.1)
$W_{\mathrm{anti}}[1, 2, 2]$ の回路を論理式で表記すると、必然的に $(\neg x_{11} \vee \neg x_{12}) \wedge \cdots \wedge (\neg x_{m1} \vee \neg x_{m2})$ の形である。登場する変数を頂点とし各 $i$ について $x_{i1}$ と $x_{i2}$ の間に辺を張った無向グラフに大きさ $k$ の独立集合が存在することと、元の回路に重み $k$ の解が存在することは同値。

## まとめ
深さや fan-in の大きさの制限は基本手筋といえるので特に驚きはありませんが、$W[1, 2, s] \subseteq W_{\mathrm{anti}}[1, 2, s]$ の証明で「ある変数が false である」という条件をエンコードするために差分をとって、「その差分の間はいかなる変数も存在しない」という制約を作ることで対応したのは非常に頭の良い方法だと思いました。ナイーブに $W[1]$ 問題を最大独立集合に FPT 帰着しようとして変数の否定をエンコードする方法が思いつかず、私もここで苦しみました。

## 参考文献
[1] [Downey, Rod G., and Michael R. Fellows. "Fixed-parameter tractability and completeness II: On completeness for W [1]." Theoretical Computer Science 141.1-2 (1995): 109-131.](https://www.sciencedirect.com/science/article/pii/0304397594000973)