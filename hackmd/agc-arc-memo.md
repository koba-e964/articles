# AGC/ARC などメモ (ネタバレあり)
AGC/ARC などで、解法が他の問題に流用できそうなものをメモする。ABC に出てきそうな実装法の典型というよりは、考察方法の典型を書く。

[AGC 手筋まとめ(AGCの多くの問題のネタバレを含みます)](https://www.dropbox.com/scl/fi/pi1um3izq07czb9ykfylp/AGC-AGC.paper?dl=0) の書き方を踏襲する。

あくまで自分用なので、**橙**以上の人は**強い人の解説を読んでください**。

## 更新履歴
|日付|イベント|
|--|--|
|2025-10-29|v1.0.0 公開|

## 典型
### 順列・完全マッチングの典型
- 最小値・最大値に着目する
  - 例: [AGC043-D Merge Triplets](https://atcoder.jp/contests/agc043/tasks/agc043_d) で $3N$ が現れた後のことを考えると、考察が進む (https://betrue12.hateblo.jp/entry/2020/03/22/005046)
- 挿入 DP
  - <https://ricky-pon.hatenablog.com/entry/2023/07/03/120540>
  - 問題
    - [CPSCO2019 Session3 Flexible Permutation](https://atcoder.jp/contests/cpsco2019-s3/tasks/cpsco2019_s3_f)
- 箱根駅伝 DP
  - <https://drken1215.hatenablog.com/entry/2019/10/05/173700>
  - (完全) マッチングの数え上げで、右が左より大きい・小さいなどの条件が掛かっているものに使える。
  - 一番プレインなものは 5 通りの遷移がある。両者等しい (1)、左が上下に行く (2) $\times$ 右が上下に行く (2) で $1+2\times 2=5$。
  - 「予約」した時には将来のペアの個数などを勘定したりせず、あくまでも決まった時に勘定することに注意。「上下の条件を満たす $p$ の個数」などではなく、マッチングの個数を数えていることを意識しよう。
    - マッチングの個数を数えるので、当然完全マッチング以外の数え上げもできる。
  - 問題
    - [ABC134-F Permutation Oddness](https://atcoder.jp/contests/abc134/tasks/abc134_f)
    - <https://www.mathenachia.blog/agc005-d-usereditorial/>
    - [CF 1035 (Div. 2) Token Removing](https://codeforces.com/contest/2119/problem/D)
    - [CPSCO2019 Session3 Flexible Permutation](https://atcoder.jp/contests/cpsco2019-s3/tasks/cpsco2019_s3_f)
    - [ARC207-A Affinity for Artifacts](https://atcoder.jp/contests/arc207/tasks/arc207_a)
  - JSON: <https://github.com/koba-e964/learning-trees/blob/01eb1749b2afff41346b6e696233037e386709f8/comppro-algo/%E9%A0%86%E5%88%97.json5#L29-L39>

### 区間の典型
- 区間の交差[^intersection-is-not-set-theoretical] + クエリー問題
  - 平面走査で、元々ある区間を長方形に、テスト用の区間を点にすることができる。2 個の長方形に分かれ、それらは共通部分を持たない。
  - 問題
    - [ABC360-F InterSections](https://atcoder.jp/contests/abc360/tasks/abc360_f)
- 区間スケジューリング
  - <https://algo-method.com/tasks/363/editorial>

[^intersection-is-not-set-theoretical]: 区間の**交差**というとき、それは共通部分が非空であることではなく、共通部分が非空かつどちらも包含しないことをいう。

### 凸最適化・フロー・マトロイドの典型
- 凸最適化
  - DP の加速
    - Monge 性
      - <https://qiita.com/kobae964/private/69ce90f4a3f8943bccbe>
    - monotone minima, Knuth-Yao speedup
      - <https://topcoder-g-hatena-ne-jp.jag-icpc.org/spaghetti_source/20120915/1347668163.html>
    - Alien DP
    - Convex Hull Trick
    - Slope Trick
  - 問題
    - <https://drken1215.hatenablog.com/entry/2020/01/13/011000>
    - [yukicoder 1122 Plane Tickets](https://yukicoder.me/problems/no/1122)
- フロー
  - 兆候
    - 割り当て問題 (あるものを k 通りの方法で使用できる)
      - <https://drken1215.hatenablog.com/entry/2023/05/01/171905>
    - 線型計画問題
      - <https://qiita.com/kobae964/items/7bba7dbfe242b602fa4f> に問題例がある
  - 種別
    - 最大フロー
    - 最小費用流・最小費用循環流
    - 二部グラフのあれこれ
      - 最大マッチング
      - 最大独立集合・最小頂点被覆
    - Dilworth の定理・推移的 DAG のパス被覆
  - 計算量
    - <https://misawa.github.io/others/flow/dinic_time_complexity.html>
  - 双対
    - [競プロの双対性: Segtree さんの問題編](https://qiita.com/kobae964/items/7bba7dbfe242b602fa4f)
      - 最小費用流 (MCF) の双対
      - フローの実行可能性の双対
      - Lagrange 双対
      - Dilworth の定理
      - 最大フロー最小カットの双対
      - 二部グラフの双対
  - 残余ネットワーク
    - 問題
      - [yukicoder 1123 Afforestation](https://yukicoder.me/problems/no/1123)
      - <https://drken1215.hatenablog.com/entry/2021/08/05/173900>
- マトロイド
  - 基本性質
    - 定義: [【月刊組合せ論 Natori】マトロイドに入門しよう【2024 年 7 月号】](https://combinatorics-fun.vercel.app/natori/202407/)
    - 貪欲ができる
    - 極大な集合が全部同じ大きさ
    - <https://combinatorics-fun.vercel.app/natori/202412/>
    - <https://maspypy.com/atcoder-jsc2019%E4%BA%88%E9%81%B8-e-card-collector-%EF%BC%88%E3%83%9E%E3%83%88%E3%83%AD%E3%82%A4%E3%83%89%EF%BC%89>
    - [マトロイドの例と構成法 - Katu math](https://katu2oumath.hatenablog.com/entry/2025/04/02/202928)
  - 閉路マトロイド
    - 最小全域木をクラスカル法で求めるときに使われるやつ (例: [解説](https://zenn.dev/convers39/articles/6126e22dd116fb))
  - ベクトルマトロイド
    - ベクトル空間の独立集合を独立集合とする。
    - 問題例: [ABC236-F Spices](https://atcoder.jp/contests/abc236/tasks/abc236_f)
  - bicircular matroid
    - 無向グラフについて、以下のようなペアは最大何個取り出せるか?
      - 頂点 $v$ とそれに接続する辺 $(v, w)$ のペア。頂点も辺も重複は許さない。
      - 接続関係についての最大二部マッチング
    - 連結成分ごとに見て、辺の本数と頂点数の min が自明な上限かつ答え。
      - 頂点数を $N$ とする。木であれば辺は $N-1$ 本で上限の $N-1$ は葉を貪欲にとることで達成できる。 (任意の頂点を残せることに注意。) そうでなければ全域木を任意に取り、全域木に含まれない辺を 1 本とって $(u, v)$ としたときに、 $u$ を使わないペアを全域木から $N-1$ 個取って、残り 1 個を $(u, (u, v))$ とする。 
    - 実はマトロイド
      - 無向グラフに対して、独立集合を「上のペアに含まれる辺の集合」とする。
      - 辺に適当に向きをつけて有向辺ということにすると、
        - (i) 各 $v$ に対して $v$ を始点とする一つの辺 $v \to w$ を取れるマトロイドと
        - (ii) 各 $v$ に対して $v$ を終点とする一つの辺 $w \to v$を取れるマトロイド
      - の[合併](https://hitonanode.github.io/cplib-cpp/combinatorial_opt/matroid_union.hpp.html)とみなせる
      - [横断マトロイド (transversal matroid)](https://combinatorics-fun.vercel.app/natori/202412/)でもある。左側を頂点全体、右側を辺全体とし、 $u$ や $v$ と $(u,v)$ に辺を張り、右側に着目する。
        - 横断マトロイドであることからもわかるように、実装時は辺だけに着目することに注意。各連結成分が木か[疑似森/なもりグラフ](https://ei1333.github.io/library/graph/others/namori-graph.hpp.html)であれば OK。
    - 頭の中で「れく太」と呼んだら強烈に印象に残った
      - 問題の一つ、Card Collector の名前から
      - <https://dmwiki.net/%E8%A7%92%E5%8F%A4+%E3%82%8C%E3%81%8F%E5%A4%AA>
    - 問題
      - [第一回日本最強プログラマー学生選手権-予選-E Card Collector](https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_e)
      - [Chokudai SpeedRun 002-K 種類数 β](https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_k)
  - 罰金付きスケジューリング問題
    - [クリスマスですし、罰金付きスケジューリング問題でマトロイドと貪欲法の基本に入門します！](https://qiita.com/ngtkana/items/ec9319619c41b1a77572)
    - [エイシング プログラミング コンテスト 2020-E Camel Train](https://atcoder.jp/contests/aising2020/tasks/aising2020_e)
  - 未分類
    - 線形マトロイド交差 [ABC399-G Colorful Spanning Tree](https://atcoder.jp/contests/abc399/tasks/abc399_g)
    - マトロイド交差がマトロイドになってるやつ [パ研合宿コンペティション 3日目-G 落単の危機](https://atcoder.jp/contests/pakencamp-2018-day3/tasks/pakencamp_2018_day3_g)
    - カタランマトロイド <https://x.com/Katu2ou/status/1926279713082712378> [ABC407-E Most Valuable Parentheses](https://atcoder.jp/contests/abc407/tasks/abc407_e)
    - マトロイド交差 <https://storage.googleapis.com/wp-content.icpc.jp/sites/12/2024/12/all_with_cover_web.pdf> の H
  - 考えている対象がマトロイドであることが分かっても、効率的に独立集合かどうか判定できるかどうかは別問題であることに注意。
    - 独立集合オラクル、ランクオラクル、閉路オラクルなどについて、一つから別のものを構築するのは多項式時間で可能。
    - 競プロでは多項式時間でできるだけだと意味がないことが多く、 $O(1)$ か $O(N)$ かの差が重要になりやすい。
    - 独立集合オラクルなどをインクリメンタルに適用することもある。
      - 例: 閉路マトロイドで辺の部分集合に閉路がないか一回判定するには $O(E\alpha(V))$ 時間かかるが、閉路がない辺の部分集合に一つの辺を追加できるかは $O(\alpha(V))$ 時間で判定できる。

### グラフの典型
- 部分グラフの検出・数え上げ
  - <https://qiita.com/kobae964/private/4a43f4bf6d0eae7839b1>

### 操作で状態遷移できるか判定する系
- 共通
  - 前処理してわかりやすい処理にする
  - 例: [第5回 ドワンゴからの挑戦状 本選-B XOR Spread](https://atcoder.jp/contests/dwacon5th-final/tasks/dwacon5th_final_b)
    - $(a_{i-1}, a_{i+1}) \leftarrow (a'_{i-1}, a'_{i+1}) = (a_{i-1} \oplus a_i, a_{i+1} \oplus a_i)$  ($2 \le i \le N-1$) という操作を何回もできる
    - $b_i := a_1 \oplus \cdots \oplus a _ i$ ($2 \le i \le N-1$) とする。元々の操作は $(b _ {i-1}, b _ i, b _ {i+1}) \leftarrow (b' _ {i-1}, b' _ i, b' _ {i+1}) = (b_i, b_{i-1}, b_{i+1})$ となる。つまり $[1, N-1]$ の範囲で $b$ の隣接 swap ができる。
- 操作が可逆な場合
  - 不変量を見つける
    - xor
    - 列を圧縮
    - 列を伸長
    - 置換の偶奇
  - 問題
    - <https://drken1215.hatenablog.com/entry/2020/09/16/180100_1>
    - [AGC055-B ABC Supremacy](https://atcoder.jp/contests/agc055/tasks/agc055_b)
- 操作が非可逆な場合
  - 考察例
    - マッチング
  - 問題
    - [yukicoder 1654 Binary Compression](https://yukicoder.me/problems/no/1654)

### 括弧列系
- 妥当な括弧列を作る
  - delta, min
    - [AOJ 2681 Parentheses](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2681) 
      - <https://drken1215.hatenablog.com/entry/2020/10/23/172800>
      - 並べ替えるとき、推移律が成立しないので注意 (<https://chatgpt.com/share/68f84596-84a8-8010-a03e-e77ab95b55c3>) TODO: 変数変換をどうやったか説明する
      - 全体では推移律が成立しないが、パターンに区切ると成立するのがポイント

### 文字列系
- 文字列で DP
  - 問題
    - <https://drken1215.hatenablog.com/entry/2020/01/28/121600>

### 数学系
- 問題
  - <https://drken1215.hatenablog.com/entry/2020/10/23/025800>

### 指数系
- $O(2^N)$ とか $O(3^N)$ とか
  - 問題
    - [ARC078-D](https://atcoder.jp/contests/arc078/tasks/arc078_d)
- 枝刈りで計算量が落ちる
  - 最大独立集合
  - 3-SAT
    - <https://drken1215.hatenablog.com/entry/2020/01/15/221000>

### 二人ゲームの典型
- 真似っこ戦略
- 実験
- 石取りゲーム
  - [競プロの Nim 問題まとめ (ネタバレあり)](https://qiita.com/kobae964/items/775c10877763a65a0328)
- グラフ
  - 問題
    - <https://drken1215.hatenablog.com/entry/2020/10/16/055000>

### 苦行系
- 実装が辛い
  - 地道に単純化するしかなさそう…
- 問題
  - <https://drken1215.hatenablog.com/entry/2020/11/05/123600>
    - 指数の塔の最大化、なおかつ辞書順最小化

### その他 (分類できていないものを入れる)
- 包除原理
  - <https://qiita.com/kobae964/private/ccb05edb05802853200f>
- グリッドで縦横の次数が決まっているマッチング
  - [yukicoder 1123 Afforestation](https://yukicoder.me/problems/no/1123)
  - [ABC424-G Set list](https://atcoder.jp/contests/abc424/tasks/abc424_g)

## 問題集

### [yukicoder 3305 Shift Sort](https://yukicoder.me/problems/no/3305) (2025-10, 550?) [配列の回転操作]
配列の回転 (Aa -> aA) は、好きな要素を右から削除して左に挿入する操作と言い換えることができる。そのため、それぞれの要素がソートするために動かす必要があることと左側に自分より大きい要素があるかどうかは同値。

実装 (Rust): <https://yukicoder.me/submissions/1126857>

### [ABC360-F InterSections](https://atcoder.jp/contests/abc360/tasks/abc360_f) [区間の交差 + クエリー問題]
平面走査で解ける。
別解法: 平面走査をし、 $t$ を増やして $[t, x)$ との交差状況を考えることにする。$t = l$ になったときに $[l, r)$ との交差状況が変わる ($x < r$ で交差するようになる)。
  - ref: <https://drken1215.hatenablog.com/entry/2024/07/06/172000>

### [ABC338-G evall](https://atcoder.jp/contests/abc338/tasks/abc338_g) (2024-01, 600) [複雑な状態]

#### 方針 1 (+ で分ける)

TODO: 書く

実装 (Rust): <https://atcoder.jp/contests/abc338/submissions/70308745>

#### 方針 2 ([ユーザー解説](https://atcoder.jp/contests/abc338/editorial/11651)に近い)
文字列 $S$ を左から見ていき、位置 $j$ を見る直前に、文字 $c$ を引数に取る関数 $c \mapsto \sum_{i=0}^{j-1} f(s[i,j) + c)$ を管理しておき、$c = s[j]$ として適用し、その後関数を更新する、という方針をとる。
管理しておくべき関数は $c \mapsto A + B(10C + c) + (10E + Dc)$ という形で書ける。(c は数字に対応する整数としても解釈することにする)
- 実際には、$j$ から見て (i) 前の `*` まで、(ii) 前の `+` まで、(iii) その後 で分けて足すことになる。例えば (iii) は $A$ に相当し、 $j$ の直前の `+` の位置を $k$ とすると単に $s[0,k)$ の suffix に対する $f$ の和である。 
- TODO: わかりやすく項を分割する

実装 (Rust): <https://atcoder.jp/contests/abc338/submissions/70314785>

### [ABC425-F Inserting Process](https://atcoder.jp/contests/abc425/tasks/abc425_f) (2025-09, 700?) [操作木を考える, 区間 DP]
実は多項式解法があるのでそれについて書く。
文字列の先頭に `$` を追加して操作を逆側から見ると、 $s[i] \neq s[i+1]$ のときに $s[i+1]$ を消去する問題とみなせる。操作木とトポロジカルソートの対を考えることになるので、区間 DP で区間のマージの際に C(全体の操作回数, 左の操作回数) を掛けることになる。計算量は $O(N^3 \sigma^2)$ だが、この問題に限っては $O(N^3)$ でできる。

実装 (Rust): <https://atcoder.jp/contests/abc425/submissions/69720430>

### [AGC071-A XOR Cross Over](https://atcoder.jp/contests/agc071/tasks/agc071_a) (2025-03, 700) [区間 DP]
累積 xor の配列を $c = (c[0], \ldots, c[N])$ と呼ぶ。

多項式時間にするパート: 配列の中で隣同士の xor というのは不変である。そのため、分割された区間は「全体に何を xor するか」で特徴づけることができる。この値を $x$ と置く。
ここで、二項演算 $\cdot$ を、

$$
i \cdot x := \begin{cases} x & (i \text{が奇数}) \\
0 & (i \text{が偶数}) \end{cases}
$$

で定義することにする。また $+$ で xor を表すことにする。区間 $[l, r)$ が値 $x$ を持っていて、これを $[i,k), [k,j)$ に分割する場合を考える。新しい $x$ を $x'\ ([i,k)),x''\ ([k,j))$ と置くと $x' = (k+j)x + x + c[k] + c[i]$ である。ここで 区間 $[i,j)$ に対して $y := x + c[i] + c[j]$ で定めることにすると $y' = (k + j)(y + c[i] + c[j]) + y$ が成り立つ ($c[k] + c[i]$ が消せる)。この式を眺めると $y'$ は $c[i] + c[j]$ か $y$ のいずれかであることがわかるし、区間全体では $x = 0$ だから $y = c[0] + c[N]$ であるため、$y$ としてあり得るのは $c[i] + c[j]$ の形の値のみ。これと区間の両端を状態として持って状態数が $O(N^4)$ で合計 $O(N^5)$-time。

$O(N^5)$ から $O(N^4)$ にするパート: 最終的に区間の長さが $1$ のときは $y = x + c[i] + c[i+1] = (i \text{番目の値})$ となり、さらに $y$ は親の値が踏襲されるか新しく設定されるかのため、$y$ についての一次式の min が答えである。つまり、各区間の答えは $\min_{0 \le i \le ?} i y + b_i$ という形で、持っておくべきなのは $b_i$ であり、これで状態数が $O(N^3)$ になる。

$O(N^4)$ から $O(N^3)$ にするパート: 実は、最適解は偶数長ならスカラー値で奇数長なら $y + d_{i,j}$ の形である。これで状態数が $O(N^2)$ になる。

実装 (Rust): <https://atcoder.jp/contests/agc071/submissions/69818279>

### [AGC073-A Chords and Checkered](https://atcoder.jp/contests/agc073/tasks/agc073_a) (2025-09, 700) [弦, $\sum_i \lfloor i/2\rfloor C(N,i)$]

弦で囲まれた領域の問題は、領域ごとにちょうど一つの弦に紐づけるのが重要 (一番左など)。

途中出てくる式について、$\sum_i \lfloor i/2\rfloor C(N,i) = \sum_i \frac{i - i \bmod 2}{2} C(N,i)$ だし、$\sum i C(N,i)$ も $\sum (i \bmod 2)C(N,i)$ も $N$ についての閉じた式で表せるので、元の式も閉じた式で表せる。

実装 (Rust): <https://atcoder.jp/contests/agc073/submissions/69719793>

### [ARC155-D Avoid Coprime Game](https://atcoder.jp/contests/arc155/tasks/arc155_d) (2023-03, 800) [ゲーム, 真似っこ戦略]

うまくいかなかった戦略: 偶数個は無視できるから、単に cnt[i] = i の倍数の個数 % 2 でよい。
うまくいかなかった理由: ある値の状態に行くためにはその値が存在する必要がある。そのため、単に偶数個同じ要素があったら消すだけではうまくいかず、 (i) 0 個、 (ii) 1 個以上の奇数個、 (iii) 2 個以上の偶数個 を区別する必要がある。
i -> d の遷移をする時、何らかの手法で gcd(i, g) = d なる f[i] の総和がわかる必要がある。約数包除など。

TODO: i % d == 0 なる i -> d の遷移それぞれで rad(i / d) の約数を全探索するときの計算量

実装 (Rust): <https://atcoder.jp/contests/arc155/submissions/69935622>

### [ARC207-A Affinity for Artifacts](https://atcoder.jp/contests/arc207/tasks/arc207_a) (2025-10, 800) [箱根駅伝 DP]

箱根駅伝 DP。 $i = 0,\ldots,N-1$ の値が左には $f[i]$ 個、右には $1$ 個あるとしたとき、マッチングを作っていく。今までに作られたマッチングの大きさを $j$ とすると、右はもちろん左のマッチング予定の頂点が何個残っているかも $j$ から計算できることに注意。

実装の失敗 1: [提出 2](https://atcoder.jp/contests/arc207/submissions/70185730) -> [提出 3](https://atcoder.jp/contests/arc207/submissions/70186183)
- DP の遷移で、「左 $f[i]$ 右 $b$ の完全二部グラフにおける大きさlt のマッチングの個数」が欲しいことがあったが、間違えて $C(f[i], \mathrm{lt})C(b,\mathrm{lt})$ にしてしまった ($\mathrm{lt}!$ を掛け忘れた)。
- 対策?: 完全二部グラフにおける大きさ $k$ のマッチングの個数をスニペットにしておく。因子をうっかり忘れてしまうことへの根本的な対策は分からない。慣れるしかないか…?

実装の失敗 2: [提出 1](https://atcoder.jp/contests/arc207/submissions/70185590) -> [提出 2](https://atcoder.jp/contests/arc207/submissions/70185730)
- 箱根駅伝 DP において、 `=` の場合だけ lt を見るのを忘れた
- 対策: 実装をなるべく共通化する

実装 (Rust): <https://atcoder.jp/contests/arc207/submissions/70188088>

### [AGC072-A Rhythm Game](https://atcoder.jp/contests/agc072/tasks/agc072_a) (2025-04, 900) [操作, 順列, スケジューリング, swap argument, 部分区間を自明な問題にする]

x < y < z のとき、 操作列で z,y,x という並びがあったら z,x,y にできるという性質がある。
これがどのような場合に成立するかは考察の必要あり。この問題では ($D$ がタスクごとに一定であるため) 成り立つ。
- 締切時刻 $T_i+X_i+D$ でソートすると、自動的に $T_i + X_i$ でソートされることになる。$z$ をやると、やった後は $T_z + X_z$ 以降の時刻であるため、$T_x - X_x \le T_x + X_x \le T_z + X_z$ から、$x < z$ である $x$ についてはすべて締切時刻だけのスケジューリング問題になる。
  - このように、$i \to j$ のジャンプをしたら $[i+1,j-1]$ の区間は自明な問題になる、という構造が大事かも。 

これで $O(N^3)$ にはなって、そこから $O(N^2)$ に落とすのは ABC。
