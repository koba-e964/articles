# AGC/ARC などメモ (ネタバレあり)
AGC/ARC などで、解法が他の問題に流用できそうなものをメモする。ABC に出てきそうな実装法の典型というよりは、考察方法の典型を書く。

[AGC 手筋まとめ(AGCの多くの問題のネタバレを含みます)](https://www.dropbox.com/scl/fi/pi1um3izq07czb9ykfylp/AGC-AGC.paper?dl=0) の書き方を踏襲する。

あくまで自分用なので、**橙**以上の人は**強い人の解説を読んでください**。

## 更新履歴
|日付|イベント|
|--|--|
|2025-11-15|v1.1.1 公開、ミスの修正|
|2025-11-14|v1.1.0 公開、パターンが限られる系・弦・区間 DP を追加、様々な問題の解法を追加|
|2025-10-29|v1.0.0 公開|

## 典型
### 順列・完全マッチングの典型
- 最小値・最大値に着目する
  - 例: [AGC043-D Merge Triplets](https://atcoder.jp/contests/agc043/tasks/agc043_d) で $3N$ が現れた後のことを考えると、考察が進む (https://betrue12.hateblo.jp/entry/2020/03/22/005046)
- 挿入 DP
  - <https://ricky-pon.hatenablog.com/entry/2023/07/03/120540>
  - 問題例
    - [CPSCO2019 Session3 Flexible Permutation](https://atcoder.jp/contests/cpsco2019-s3/tasks/cpsco2019_s3_f)
    - [ABC431-F Almost Sorted 2](https://atcoder.jp/contests/abc431/tasks/abc431_f)
- 箱根駅伝 DP
  - <https://drken1215.hatenablog.com/entry/2019/10/05/173700>
  - (完全) マッチングの数え上げで、右が左より大きい・小さいなどの条件が掛かっているものに使える。
  - 一番プレインなものは 5 通りの遷移がある。両者等しい (1)、左が上下に行く (2) $\times$ 右が上下に行く (2) で $1+2\times 2=5$。
  - 「予約」した時には将来のペアの個数などを勘定したりせず、あくまでも決まった時に勘定することに注意。「上下の条件を満たす $p$ の個数」などではなく、マッチングの個数を数えていることを意識しよう。
    - マッチングの個数を数えるので、当然完全マッチング以外の数え上げもできる。
  - 問題例
    - [ABC134-F Permutation Oddness](https://atcoder.jp/contests/abc134/tasks/abc134_f)
    - <https://www.mathenachia.blog/agc005-d-usereditorial/>
    - [CF 1035 (Div. 2) Token Removing](https://codeforces.com/contest/2119/problem/D)
    - [CPSCO2019 Session3 Flexible Permutation](https://atcoder.jp/contests/cpsco2019-s3/tasks/cpsco2019_s3_f)
    - [ARC207-A Affinity for Artifacts](https://atcoder.jp/contests/arc207/tasks/arc207_a)
  - JSON: <https://github.com/koba-e964/learning-trees/blob/01eb1749b2afff41346b6e696233037e386709f8/comppro-algo/%E9%A0%86%E5%88%97.json5#L29-L39>

### 区間の典型
- 区間の交差[^intersection-is-not-set-theoretical] + クエリー問題
  - 平面走査で、元々ある区間を長方形に、テスト用の区間を点にすることができる。2 個の長方形に分かれ、それらは共通部分を持たない。
  - 問題例
    - [ABC360-F InterSections](https://atcoder.jp/contests/abc360/tasks/abc360_f)
- 区間スケジューリング
  - <https://algo-method.com/tasks/363/editorial>
- 区間分割
  - 問題例
    - [ARC189-D Takahashi is Slime](https://atcoder.jp/contests/arc189/tasks/arc189_d)
      -  <https://drken1215.hatenablog.com/entry/2025/01/04/101100>
      - $a_i \ge a_{i+1}$ なら左から $a_i$ を吸収できるなら $a_{i+1}$ も吸収できる。よって、$a_i$ が最大値で吸収できるなら $a_j$ ($j\ge i$) も吸収できる。
    - [yukicoder 3258 Xor Division Game](https://yukicoder.me/problems/no/3258)
- 弦
  - 切り開いて区間の形にしよう
    - 弦が交差しない <=> 区間に直しても交差しない
- 区間 DP の高速化
  - 遷移が限られて $O(N^3)$ から $O(N^2)$ に
    - 問題例: [ARC204-B Sort Permutation](https://atcoder.jp/contests/arc204/tasks/arc204_b)

- その他
  - 2 つの区間の関係は 4 パターンしかない [yukicoder 3313 Matryoshka](https://yukicoder.me/problems/no/3313)

[^intersection-is-not-set-theoretical]: 区間の**交差**というとき、それは共通部分が非空であることではなく、共通部分が非空かつどちらももう一方を包含しないことをいう。

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
  - 問題例
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
    - 問題例
      - [yukicoder 1123 Afforestation](https://yukicoder.me/problems/no/1123)
      - <https://drken1215.hatenablog.com/entry/2021/08/05/173900>
- マトロイド
  - 基本性質
    - 定義: [【月刊組合せ論 Natori】マトロイドに入門しよう【2024 年 7 月号】](https://combinatorics-fun.vercel.app/natori/202407/)
    - 貪欲ができる (最良優先貪欲法)
    - 極大な集合が全部同じ大きさ
    - <https://combinatorics-fun.vercel.app/natori/202412/>
    - <https://maspypy.com/atcoder-jsc2019%E4%BA%88%E9%81%B8-e-card-collector-%EF%BC%88%E3%83%9E%E3%83%88%E3%83%AD%E3%82%A4%E3%83%89%EF%BC%89>
    - [マトロイドの例と構成法 - Katu math](https://katu2oumath.hatenablog.com/entry/2025/04/02/202928)
  - 閉路マトロイド
    - 最小全域木をクラスカル法で求めるときに使われるやつ (例: [解説](https://zenn.dev/convers39/articles/6126e22dd116fb))
    - TODO: 閉路マトロイドでの特殊事情をまとめる
      - TODO: 最小全域木の別の解き方をまとめる
        - <https://drken1215.hatenablog.com/entry/2019/01/15/081500>
        - クラスカル法
          - 分割統治などで辺の本数を減らす
        - プリム法
        - Borůvka 法
        - Voronoi 図を使う方法
      - 基 $X$ と $e \not \in X$ に対して、 $e \in Y, |Y| = |X|, |Y \cap X| = |X| - 1$ を満たす最適な $Y$ を見つけることが <$O(N)$, $O(\log N)$> でできる
        - [ARC093-E Bichrome Spanning Tree](https://atcoder.jp/contests/arc093/tasks/arc093_c)
        - $X + e - f$ が独立集合であるような $f \in X$ の中で最適なものを見つける、と言っても良い
      - 辺数を減らすテク
        - クリークをパスグラフやスターグラフで代用など [ABC352-E Clique Connect](https://atcoder.jp/contests/abc352/tasks/abc352_e)
      - TODO: 未分類
        - [ARC181-E Min and Max at the edge](https://atcoder.jp/contests/arc181/tasks/arc181_e)
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
    - 問題例
      - [第一回日本最強プログラマー学生選手権-予選-E Card Collector](https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_e)
      - [Chokudai SpeedRun 002-K 種類数 β](https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_k)
  - 罰金付きスケジューリング問題
    - [クリスマスですし、罰金付きスケジューリング問題でマトロイドと貪欲法の基本に入門します！](https://qiita.com/ngtkana/items/ec9319619c41b1a77572)
    - [エイシング プログラミング コンテスト 2020-E Camel Train](https://atcoder.jp/contests/aising2020/tasks/aising2020_e)
  - 未分類
    - 線形マトロイド交差 [ABC399-G Colorful Spanning Tree](https://atcoder.jp/contests/abc399/tasks/abc399_g)
    - マトロイド交差がマトロイドになってるやつ [パ研合宿コンペティション 3日目-G 落単の危機](https://atcoder.jp/contests/pakencamp-2018-day3/tasks/pakencamp_2018_day3_g)
    - カタランマトロイド
      - 括弧列の開き括弧の位置としてあり得るものが基になっているマトロイド
      - 最初の `(` を取った後、優先度付きキューに 2 個ずつ入れる貪欲ができる
        - 一般のマトロイドの最良優先貪欲法を使うと遅延セグメント木などが必要
        - <https://qiita.com/kobae964/items/7bba7dbfe242b602fa4f> の (14) と似ている
      - 問題例
        - <https://x.com/Katu2ou/status/1926279713082712378> [ABC407-E Most Valuable Parentheses](https://atcoder.jp/contests/abc407/tasks/abc407_e)
        - [AGC053-B Taking the middle](https://atcoder.jp/contests/agc053/tasks/agc053_b)
    - マトロイド交差 <https://storage.googleapis.com/wp-content.icpc.jp/sites/12/2024/12/all_with_cover_web.pdf> の H
    - $k$-全域森問題 <https://topcoder-g-hatena-ne-jp.jag-icpc.org/spaghetti_source/20121124/1353741121.html>
    - 有向全域木 <https://topcoder-g-hatena-ne-jp.jag-icpc.org/spaghetti_source/20121110/1352528267.html> <https://joisino.hatenablog.com/entry/2017/01/11/230141>
  - 考えている対象がマトロイドであることが分かっても、効率的に独立集合かどうか判定できるかどうかは別問題であることに注意。
    - 独立集合オラクル、ランクオラクル、閉路オラクルなどについて、一つから別のものを構築するのは多項式時間で可能。
    - 競プロでは多項式時間でできるだけだと意味がないことが多く、 $O(1)$ か $O(N)$ かの差が重要になりやすい。
    - 独立集合オラクルなどをインクリメンタルに適用することもある。
      - 例: 閉路マトロイドで辺の部分集合に閉路がないか一回判定するには $O(E\alpha(V))$ 時間かかるが、閉路がない辺の部分集合に一つの辺を追加できるかは $O(\alpha(V))$ 時間で判定できる。
    - マトロイド研究者向け
      - <https://matroidunion.org/>

### グラフの典型
- 部分グラフの検出・数え上げ
  - <https://qiita.com/kobae964/private/4a43f4bf6d0eae7839b1>

### 操作で状態遷移できるか判定する系
- 共通
  - 前処理してわかりやすい処理にする
  - 例: [第5回 ドワンゴからの挑戦状 本選-B XOR Spread](https://atcoder.jp/contests/dwacon5th-final/tasks/dwacon5th_final_b)
    - $(a_{i-1}, a_{i+1}) \leftarrow (a'_{i-1}, a'_{i+1}) = (a_{i-1} \oplus a_i, a_{i+1} \oplus a_i)$  ($2 \le i \le N-1$) という操作を何回もできる
    - $b_i := a_1 \oplus \cdots \oplus a _ i$ ($2 \le i \le N-1$) とする。元々の操作は $(b _ {i-1}, b _ i, b _ {i+1}) \leftarrow (b' _ {i-1}, b' _ i, b' _ {i+1}) = (b_i, b_{i-1}, b_{i+1})$ となる。つまり $[1, N-1]$ の範囲で $b$ の隣接 swap ができる。
  - 例: [ARC202-A Merge and Increment](https://atcoder.jp/contests/arc202/tasks/arc202_a)
    - ランレンクス圧縮
- 操作が可逆な場合
  - 不変量を見つける
    - xor
    - 列を圧縮
    - 列を伸長
    - 置換の偶奇
  - 問題例
    - <https://drken1215.hatenablog.com/entry/2020/09/16/180100_1>
    - [AGC055-B ABC Supremacy](https://atcoder.jp/contests/agc055/tasks/agc055_b)
- 操作が非可逆な場合
  - 考察例
    - マッチング
      - 問題例
        - [yukicoder 1654 Binary Compression](https://yukicoder.me/problems/no/1654)
    - 区間をマージする過程の木
      - 操作の順番が関係ない
      - 問題例
        - [AGC009-E Eternal Average](https://atcoder.jp/contests/agc009/tasks/agc009_e)
      - [AOJ 1458 Tree Generators](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1458)

### 括弧列系
- 妥当な括弧列を作る
  - delta, min
    - [AOJ 2681 Parentheses](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2681) 
      - <https://drken1215.hatenablog.com/entry/2020/10/23/172800>
      - 並べ替えるとき、推移律が成立しないので注意 (<https://chatgpt.com/share/68f84596-84a8-8010-a03e-e77ab95b55c3>) TODO: 変数変換をどうやったか説明する
      - 全体では推移律が成立しないが、パターンに区切ると成立するのがポイント

### 文字列系
- 文字列で DP
  - 問題例
    - <https://drken1215.hatenablog.com/entry/2020/01/28/121600>

### 数学系
- 問題例
  - <https://drken1215.hatenablog.com/entry/2020/10/23/025800>

### 約数系
- 問題例
  - [CF613-2F Classical?](https://codeforces.com/contest/1285/problem/F) <https://drken1215.hatenablog.com/entry/2020/01/14/023500>

### パターンが限られる系
- パターンが数通りしかない
  - mod を見るべきところで商が数通りしかない
    - $a \bmod b = c$ という条件があるとき、 $\lfloor a/b \rfloor = 0,1$ であれば $a = c, a = b + c$ くらいに絞れる
    - 問題例: [ARC208-C Mod of XOR](https://atcoder.jp/contests/arc208/tasks/arc208_c)
- パターン数のオーダーが落ちる
  - 商の個数が sqrt
    - $\lbrace N/i : i \in \mathbb{Z}_+ \rbrace$ の大きさが $O(\sqrt{N})$ (実際には $\le 2\sqrt{N}$)
  - 約数の個数が sqrt 未満
  - 累積 OR, AND, GCD の種類数が log
    - 長さ $n$ の $2^k$ 未満の非負整数列 $a$ に対して、$\lbrace \bigvee_{i=0}^{k} a_i : 0 \le i \le n\rbrace$ の大きさが $O(k)$ (実際には $\le k + 1$)

### 指数系
- $O(2^N)$ とか $O(3^N)$ とか
  - 問題例
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
  - 問題例
    - <https://drken1215.hatenablog.com/entry/2020/10/16/055000>
- 一方の戦略が固定
  - 自分のムーブが可能かどうかを考える
  - 問題例
    - [ARC209-A Bracket Game](https://atcoder.jp/contests/arc209/tasks/arc209_a)
    - [AGC053-B Taking the middle](https://atcoder.jp/contests/agc053/tasks/agc053_b)
    - [KEYENCE2021-E Greedy Ant](https://atcoder.jp/contests/keyence2021/tasks/keyence2021_e)
    - [EPIC Institute of Technology Round Summer 2024-D World is Mine](https://codeforces.com/contest/1987/problem/D)

### スタック系
- 一点から左に見えるビルを列挙する
  - <https://drken1215.hatenablog.com/entry/2024/11/04/011552>
- 数列に挿入を繰り返した結果の構文解析
  - 括弧列のように、複数文字で終わりの文字が明確だとやりやすい
  - [AGC063-B Insert 1, 2, 3, ...](https://atcoder.jp/contests/agc063/tasks/agc063_b) <https://drken1215.hatenablog.com/entry/2023/07/31/013655>
  - <https://drken1215.hatenablog.com/entry/2023/09/08/235429>
  - <https://drken1215.hatenablog.com/entry/2020/12/19/230000>
- 高速化で $\log N$ を落とす
  - [ARC115-E LEQ and NEQ](https://atcoder.jp/contests/arc115/tasks/arc115_e) <https://drken1215.hatenablog.com/entry/2021/03/21/235000_1>

### 苦行系
- 実装が辛い
  - 地道に単純化するしかなさそう…
  - ランダムテスト
    - namespace で挟んで include し、別ファイルで呼ぶ
- 問題例
  - <https://drken1215.hatenablog.com/entry/2020/11/05/123600>
    - 指数の塔の最大化、なおかつ辞書順最小化

### その他 (分類できていないものを入れる)
- 包除原理
  - <https://qiita.com/kobae964/private/ccb05edb05802853200f>
- グリッドで縦横の次数が決まっているマッチング
  - [yukicoder 1123 Afforestation](https://yukicoder.me/problems/no/1123)
  - [ABC424-G Set list](https://atcoder.jp/contests/abc424/tasks/abc424_g)
- 多項式で数え上げ
  - [AOJ 4007 ゲームブック](https://onlinejudge.u-aizu.ac.jp/challenges/sources/PCK/Final/4007)
- DAG 上単一終点の DP
  - DAG なので辺を逆向きにすれば単一始点になる

## 問題集
### [yukicoder 3305 Shift Sort](https://yukicoder.me/problems/no/3305) (2025-10, 550?) [配列の回転操作]
配列の回転 (Aa -> aA) は、好きな要素を右から削除して左に挿入する操作と言い換えることができる。そのため、それぞれの要素がソートするために動かす必要があることと左側に自分より大きい要素があるかどうかは同値。

実装 (Rust): <https://yukicoder.me/submissions/1126857>

### [yukicoder 3376 Rectangle in Circle](https://yukicoder.me/problems/no/3376) (2025-11, 550?) [弦]
多項式時間解法: 長方形ができる <=> 直径が 2 個埋まる なので、直径が半分埋まっているのを a　個、全部埋まっているのを b 個、孤立点の残りが c 個として $O(N^2)$ 状態の DP になる。 ($0 \le b \le 1$ に注意)

$O(N^2)$ から $O(N)$ にするパート: 直径が 2 個以上ある時はつねに長方形ができてゲームが終了するので、$c$ は実は終了条件に影響しない。$c$ を取り除けば状態数は $O(N)$ である。

実装 (Rust): <https://yukicoder.me/submissions/1137263>

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

### [ARC212-E Drop Min](https://atcoder.jp/contests/arc212/tasks/arc212_e) (2026-01, 700 -> 600) [分割統治, Cartesian tree]
大きい方から見ていくと、例えば $N$ の左右は互いに干渉しないことがわかる。このことから分割統治ができることがわかる。
この分割統治のやり方は [Cartesian tree](https://nyaannyaan.github.io/library/tree/cartesian-tree.hpp.html) の作成過程と同じである。(ただし min ではなく max である。) なのでマージのやり方だけ考えれば良い。各要素について、自分より小さい要素を無視した場合に一番近くに見えるものを考える。両端のことを壁と呼ぶことにする。
また、一番近くに見えるものまでの区間で、左右にある要素の個数を $i,j$ と置く。(つまり、Cartesian tree において自分の子に対応する区間の要素数が $i,j$ である。)

- 両側が壁の場合: その要素は $N$ であり、そもそも $A$ には入らない。$C(i+j,i)$ を掛ける。
- 片側が壁の場合: 要素がある側の子に対応する区間を全て $A$ に入れた後 $A$ に入れることができる。要素を右側 ($j$ 側) とすると、 $i+j+1$ 個から $j+1$ 個選んで最後の 1 個を自分自身にすれば良いので $C(i+j+1,j+1)$ を掛ければ良い。
- 両側が要素の場合: 左右どちらかは $A$ に入れ尽くしたあとで自分自身を入れることができる。 $\sum_{1 \le k \le i}C(i+j-k, i-k) + \sum_{1 \le k \le j}C(i+j-k, j-k) + C(i+j,i) = C(i+j,i)+C(i+j,i+1)+C(i+j,j+1)$ が[成立し](https://qiita.com/kobae964/items/b665c1f8fc8402219316)、これが掛けるべき値である。

実装 (Rust): <https://atcoder.jp/contests/arc212/submissions/72577664>

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

### [パソコン甲子園 2021年 本選 12 平方連続部分文字列](https://onlinejudge.u-aizu.ac.jp/challenges/sources/PCK/Final/0477) (2021-11, 700?) [分割統治]

$S = S[:m] + S[m:]$ と分割した時、 $m$ を跨ぐ平方連続部分文字列が $O(f(|S|))$-time でカウントできれば、分割統治によって全体のカウントは $O(f(|S|) \log |S|)$-time でできる。
$m$ を跨ぐ平方連続部分文字列は、AmBAB か ABAmB か AmA の形である。
- AmA を数えるのは Z-algorithm で簡単。
- AmBAB は、 mBA の位置 ($p = m + |B| + |A|$) で全探索し、$S[m:] + S[:m]$ に対しての Z-algorithm で $p$ から右に、$\mathrm{rev}(S[:m]) + \mathrm{rev}(S[m:])$ に対しての Z-algorithm で $p$ から左に、それぞれ伸ばせるだけ伸ばして長さの合計が $p-m$ 以上であれば良い。

これらは $O(|S|)$-time でできるので、全体は $O(|S| \log |S|)$-time である。

### [yukicoder 3370 AB → BA](https://yukicoder.me/problems/no/3370) (2025-11, 700?) [分割統治]
<https://noshi91.hatenablog.com/entry/2023/07/21/235339> 一発なので ABC 的でもある。ここに入れるかどうかは迷った。

$0 \le B[i] \le A[i]$ である弱増加な $B$ を数え上げる問題である。これは分割統治でできる。

### [ARC208-C Mod of XOR](https://atcoder.jp/contests/arc208/tasks/arc208_c) (2025-10, 700 -> 600?) [パターンが数通りしかない, mod を見るべきところで商が数通りしかない]
実験していると以下のことがわかる:
- $C = X$ の場合、 $n = 2^{30}$ などで構成できる。
- $C \oplus X > X$ の場合、 $n = C \oplus X$ とすれば構成できる。
- $C = 3, X = 6$ の場合などは構成できない。 $n \oplus C$ としてあり得るものは $n+3, n+1, n-1, n-3$ だが、これらのどれだとしても条件を満たさない。 $n+3,n+1$ は $n$ より大きく $n + X$ よりも小さいため、 $\bmod\ n$ で $X$ になることはない。 $n-3, n-1$ とした場合、 $n \oplus C=X$ なのだから $n = C \oplus X$ である必要がある。

以上の考察をまとめると以下のようになる。

- $C = X$ の場合、 $n = 2^{30}$ で構成できる。
- $C \oplus X > X$ の場合、 $n = C \oplus X$ とすれば構成できる。
- 以上のどれでもなく $C < X$ の場合、構成はできない。
- 以上のどれでもない場合、 $C \oplus X \le X < C$ が成立する。このとき、 $n > X$ であれば $n \oplus C < 2n$ が成り立つので、 $n \oplus C = n + X$ である必要がある。
  - $C$ も $X$ も最上位ビットが $2^k$ であるとしてよい。 $X < n < 2^{k+1}$ の場合は $n \oplus C < 2^k < n$ である。 $n \ge 2^{k+1}$ の場合、$n > C$ であるため、 $n \oplus C \le n + C < 2n$ が成り立つ。
- $n \oplus C = n + C - 2(n \cap C)$ であるため、 $(n \cap C) = (C - X) / 2$ である必要がある。逆に、 $(C - X) / 2 \subseteq C$ であれば、 $n = 2^{30} + (C - X) / 2$ とすることで構成できる。

難しいケースで $\lfloor (n \oplus C) / n\rfloor$ が 0,1 しかありえないことがキーだった。

実装 (Rust): <https://atcoder.jp/contests/arc208/submissions/70716621>

### [ARC204-A Use Udon Coupon](https://atcoder.jp/contests/arc204/tasks/arc204_b) (2025-08, 700) [操作で状態遷移できるか判定する系, 前処理してわかりやすい処理にする]
dp[a][b] = (操作 1 を a 回、操作 2 を b 回やった時の条件を満たす場合の数) と言った $O(N^2)$ の DP にしたくなる。
dp[a][b] を見るとき、$C$ は基本 $-\sum_{i = 0}^{a-1} A_i + \sum_{j=0}^{b-1} B_i$ であって、 $\max(0, C-A_i)$ の 0 側が取られた場合にだけそこから逸脱するという事実に注目する。そうすると $D = C + \sum_{i = 0}^{a-1} A_i - \sum_{j=0}^{b-1} B_i$ という値を考えたくなる。
- 操作 1 では、 $D' = \max(D, \sum_{i = 0}^{a-1} A_i - \sum_{j=0}^{b-1} B_i)$ という更新が走る。
- 操作 2 では $D$ は変わらない。

$D \le {}$ (特定の値) となるパターンを数えればいいので、操作 1 で (特定の値) を越えるものを踏まなければ良い。

実装 (Rust): <https://atcoder.jp/contests/arc204/submissions/70923515>

### [KEYENCE2021-E Greedy Ant](https://atcoder.jp/contests/keyence2021/tasks/keyence2021_e) (2021-01, 700 -> 800?) [区間 DP, 一方の戦略が固定, DAG 上単一終点の DP]
多項式時間解法: 蟻の隣以外をとる場合、「とる権利」を保持しておいて後でまとめてとることにしても結果は変わらない。個数を状態に入れれば、開始地点ごとに $O(N^3)$-time で、全体では $O(N^4)$-time。

$O(N^4)$ から $O(N^3)$ にするパート: この DP は DAG 上の最長距離 ($(i, i, 1) \to (0, N, \mathrm{any})$) と見なすことができる。終点が同じ状態 $(0, N, \mathrm{any})$ であるため、辺の向きを逆にすると単一始点の DP となり、全体で $O(N^3)$-time になる。

実装 (Rust): <https://atcoder.jp/contests/keyence2021/submissions/70993127>

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

### [ARC203-D Insert XOR](https://atcoder.jp/contests/arc203/tasks/arc203_d) (2025-08, 800 -> 600?) [セグメント木, オートマトン]

オートマトン上の DP、およびそれに付随するセグメント木で殴れるので配点ミス。

うまくいかなかった戦略: 単純に左から右に見る DP (およびそれをセグメント木に載せたもの) で正解できると思ったが、サンプルも合わない。
うまくいかなかった理由:
- 1 1 1 0 のようなパターンで、右に 0 があるから左の 1 1 を消せる、というパターンを見落としていた。
- 1 0 1 1 1 のようなパターンで、先に 1 番目 の 0 を消してしまうと 1 1 1 1 になり、これ以上操作できないという問題もある。この場合は先に [2,3] の 1 1 を消すべきである。

うまくいかなかった戦略: 以下のような状態を持つオートマトンで最適解を計算できる。

0: 初期状態。何も読んでいない
1: 先頭から 1+ を読んだ
2: 先頭から 0 を読んだ
3: 先頭から 00+ を読んだ
4: 0 も 1 も読んだことがあり、直近で 10 を読んだ
5: 0 も 1 も読んだことがあり、直近で 100+ を読んだ
6: 0 も 1 も読んだことがあり、直近で 1 を読んだ

うまくいかなかった理由: よく考えたら、 1 0 1 0 1 のときに、最後 1 1 から減らすことはできなかった。このような場合は 2 と max をとることで対処できる。

うまくいかなかった実装: naïve に 7 次正方行列をセグメント木に載せると TLE。定数倍高速化 (i <= 4, i > j の箇所を無視する) などをして AC。

### [ARC207-D Devourers and Cake](https://atcoder.jp/contests/arc207/tasks/arc207_d) (2025-10, 800 -> 700?) [実験, 偶奇性, 極小なものだけ見る]

実験 + 偶奇性 + 極小なものだけ見る

### [ARC204-B Sort Permutation](https://atcoder.jp/contests/arc204/tasks/arc204_b) (2025-08, 800 -> 900?) [弦, 区間 DP, 区間 DP の高速化]
多項式時間解法: まず、サイクルを分割する過程 <=> 円において交わらない (端点を共有するのは ok) 弦による全域木 というのがある。

弦の問題は切り開いて区間の問題にする。こうすると交差しない区間が何個あるかの問題になる。

以降サイクルサイズを $C$ とする。 $A[i] \equiv A[j] \pmod{N}$ のとき重み 1 として、最大全域木を求めれば良いのだが、普通に区間 DP をやると $O(C^3)$-time であり間に合わない。

$O(C^3)$ から $O(KC^2)$ にするパート: 重み 1 の辺の集合を考える。 $a < b < c$ のとき、$(a, b), $(a, c)$ は $(a, c), (b, c)$ に変換できることを考えると、連結成分は全て一番右の頂点を中心とするスターグラフだと思うことができる。辺の次数は $K$ 以下なので、遷移 $([l + 1, m), [m, r)) \to [l, r)$ の本数の合計は $KC^2 = o(C^3)$ 以下である。
$C \le 5000, K \le 10$ なので ok。

実装 (Rust): <https://atcoder.jp/contests/arc204/submissions/70930824>

### [AGC072-A Rhythm Game](https://atcoder.jp/contests/agc072/tasks/agc072_a) (2025-04, 900) [操作, 順列, 区間スケジューリング, swap argument, 部分区間を自明な問題にする]

x < y < z のとき、 操作列で z,y,x という並びがあったら z,x,y にできるという性質がある。
これがどのような場合に成立するかは考察の必要あり。この問題では ($D$ がタスクごとに一定であるため) 成り立つ。
- 締切時刻 $T_i+X_i+D$ でソートすると、自動的に $T_i + X_i$ でソートされることになる。$z$ をやると、やった後は $T_z + X_z$ 以降の時刻であるため、$T_x - X_x \le T_x + X_x \le T_z + X_z$ から、$x < z$ である $x$ についてはすべて締切時刻だけのスケジューリング問題になる。
  - このように、$i \to j$ のジャンプをしたら $[i+1,j-1]$ の区間は自明な問題になる、という構造が大事かも。 

これで $O(N^3)$ にはなって、そこから $O(N^2)$ に落とすのは ABC。

### [ARC186-B Typical Permutation Descriptor](https://atcoder.jp/contests/arc186/tasks/arc186_b) (2024-10, 900 -> 700?) [区間の交差]

区間 $[A_i, i]$ ($1 \le i \le N$) を考えると、$N$ 個の区間は交差しない。つまり、 $l_1 = A_{r_1} < l_2 = A_{r_2} < r_1 < r_2$ となった場合、 $P_{l_2} < P_{r_2}, P_{l_2} > P_{r_1}, P_{r_1} > P_{r_2}$ となってしまい矛盾する。つまり $A_j < i < j$ に対して $A_i \ge A_j$ である。

区間は入れ子状になるのだから、再帰的に考えてどうにかなりそう。
区間 $[A_i, i]$ に対しては、 $P_i$ は $i <j$ なる $P_j$ に対して「義理立て」する必要がある ($P_i < P_j$ である必要がある) が、 $(A_i, i)$ についてはそのような義理立ては必要ない。 $[A_i, i]$ の内部では、 $[A_i, i]$ だけが世界の全てであるかのように計算すれば良くて、「義理立て」しない部分を勘定するのは $[A_i, i]$ の役目である。($(A_i, i)$ の要素を $(i, \infty)$ の要素と組み合わせて並べる。普通の combination でできる。)

「条件を満たす順列が存在することが保証されます」の部分で考察の道筋が明らかになり、かなり難易度が下がっている気がする。

実装 (Rust): <https://atcoder.jp/contests/arc186/submissions/70719446>

### [ARC210-E Subset Sum Gaps](https://atcoder.jp/contests/arc210/tasks/arc210_e) (2025-11, 1000 -> 800?) [マージソート, パターンが限られる系]
考察は一発ギャグだが、実装が普段見ない形で面倒で間違えやすい。

考察: 一要素ずつ見ていく $O(2^N)$ の全探索を軸に考える。
$\log_{1.01} (10^{13}N) \le \log_{1.01} (5 \times 10^{16}) \simeq 3864$ が成立するので、その各ステップで保持される状態は 4165 個以下である。
1.01 倍離れていない区間は、他の区間とマージしたり定数を足したりしても離れていないままなので、1.01 倍離れている区間だけに注目して考えることができる。

実装: $O(2^N)$ 全探索をマージソートのマージと組み合わせて高速化する手法を流用する。
1. まず、最大の要素 $a$ に対して処理をし、$[0, a]$ を得る。
2. 現在持っている区間の和集合を $L$ とし、$L$ の最大値を $M$ とする。現在見ている要素を $a$ とする。$L+a$ で、 $\lbrace x + a \mid x \in L \rbrace$ を表す。
  - $[0, a] \cap L$ を計算する。$L$ 内部の区間について、ほとんどそのまま採用すればよい。
  - $L \cap (L+a)$ を計算する。尺取り法で、区間ごとに共通部分を割り出せば良い。
  - $[M, a] \cap (L + a)$ を計算する。$L+a$ 内部の区間について、ほとんどそのまま採用すればよい。

1.01 倍以上離れているものだけ採用することを忘れないこと。

実装 (Rust): <https://atcoder.jp/contests/arc210/submissions/71409891>
