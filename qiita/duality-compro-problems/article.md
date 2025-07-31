<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">毎日min-max, ∃-∀型の双対問題を投げて行きます<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1935558355353047459?ref_src=twsrc%5Etfw">June 19, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

[Segtree さん](https://atcoder.jp/users/ynymxiaolongbao)が競プロの双対の問題を出しているので解説する。


# (1)-(10)

## (1)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さNの数列Aについて、以下の最小化問題と等しい最大化問題は？<br><br>任意のiについてX[i]&gt;=A[i]であり、要素が相異なるような長さNの整数列Xに対して、Xの最大値の最小値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1935558358590960054?ref_src=twsrc%5Etfw">June 19, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (2)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">各頂点に整数重みA[i]が付いたN頂点の根付き木について、以下の存在命題と同値な全称命題は？<br><br>「iがjの先祖であるような(i,j)を選び、A[i]から1を引き、A[j]に1を足す」操作を繰り返して、Aの全ての要素を0にできる<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1935906007823286670?ref_src=twsrc%5Etfw">June 20, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

類別: `最小費用流 (MCF) の双対` `フローの実行可能性の双対`

<details>
<summary>解法</summary>
<a href="https://www.slideshare.net/wata_orz/ss-91375739">wata さんのスライド</a>の p.47 を参照する。元の命題は以下のように言い換えられる:

> $N$ 頂点のネットワークがあり、頂点 $i$ の流入は $A[i]$ である。また元々の根付き木の親から子へ、容量 $\infty$ の辺がある。このネットワークでフローを流すのは実行可能である。

まず、流入の和が 0 である必要があるので、 $\sum_v A[v] = 0$ である。今回のように実行可能性を調べる場合は、辺のコスト $w_{uv}$ は 0 としてよい。また $p_v = 0 (\forall v)$ が常に実行可能解 (値が 0) を与え、 $p$ を $k \ge 0$ 倍すると値も $k$ 倍になるので、双対問題 $\min \sum_v b_v p_v+ \sum_{uv} c_{uv}\max(0, p _ v - p _ u)$ の解としてあり得るのは以下の 2 パターンのみである:
1. 最小値は 0 である。つまり $\sum_v b_v p_v+ \sum_{uv} c_{uv}\max(0, p _ v - p _ u) \ge 0$ が任意の $p$ に対して成り立つ。
2. 最小値は存在せずいくらでも小さい値をとることができる。 $-\infty$ と言ってもよい。

`1.` が元の命題と同値である。今回の場合、 $c_{uv} = \infty$ であるため、 $p _ v - p _ u$ がちょっとでも 0 を上回ると最小化問題の答えにはならなくなる。よって、 $(\forall uv \in E\ldotp p_u \ge p_v) \Rightarrow \sum_v A[v] p_v \ge 0$ と同値である。

ここで、天才考察 (TODO) を行うと、$p$ として考えるべきものはある部分根付き木の上で $p_i = -1$、そうでないところで $p_i = 0$ のものだけであることがわかるので、以下の命題と同値であることが結論できる。

> $\sum_v A[v] = 0$ かつ 全ての部分根付き木に対し、 $\sum_v A[v] \le 0$

</details>

## (3)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さNの整数列Xについて、以下の存在命題と同値な全称命題は？<br><br>「任意のiについて、i番目の頂点の入次数-出次数がX[i]である」 ようなN頂点の単純有向グラフが存在する。<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1936264117205282968?ref_src=twsrc%5Etfw">June 21, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

類別: `最小費用流 (MCF) の双対` `フローの実行可能性の双対`

<details>
<summary>解法</summary>
(2) と同じように考察する。同値なフローの問題は以下である:

> $N$ 頂点のネットワークがあり、頂点 $i$ の流入は $X[i]$ である。また各ペア $i \ne j$ に対し、 $i$ から $j$ へ容量 $1$ の辺がある。このネットワークでフローを流すのは実行可能である。

流入の和は 0 なので $\sum_i X[i] = 0$ である。双対問題をとると以下のようになる。

> 任意の $p$ に対して $\sum_i X[i] p_i+ \sum_{ij} \max(0, p _ j - p _ i) \ge 0$

ここで、天才考察 (TODO) を行うと、 $p$ として考えるべきものはある頂点の部分集合 $S$ の上で $p_i = 1$、そうでないところで $p_i = 0$ のものだけであることがわかるので、以下の命題と同値であることが結論できる。

> $\sum_i X[i] = 0$ かつ 任意の頂点の部分集合 $S$ に対して $\sum_{i \in S} X[i] + |S|(N - |S|) \ge 0$

</details>

## (4)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さN+Mの非負整数列Xについて、以下の存在命題と同値な全称命題は？<br><br>「任意のiについて、i番目の頂点の次数がX[i]である」 ような、頂点1,…,Nのいずれかと頂点N+1,…,N+Mのいずれかを結ぶ辺のみが存在する単純無向二部グラフが存在する。<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1936621991152451763?ref_src=twsrc%5Etfw">June 22, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

類別: `最小費用流 (MCF) の双対` `フローの実行可能性の双対`

<details>
<summary>解法</summary>
(2) と同じように考察する。同値なフローの問題は以下である:
> $N + M$ 頂点のネットワークがあり、頂点 $i$ の流入は $X[i]$ ($1\le i \le N$) あるいは $-X[i]$ ($N+1 \le i \le N+M$) である。また各ペア $1 \le i \le N, 1 \le j \le M$ に対し、 $i$ から $N + j$ へ容量 $1$ の辺がある。このネットワークでフローを流すのは実行可能である。

流入の和は 0 なので $\sum_i X[i] = \sum_j X[N+j]$ である。双対問題をとると以下のようになる。
> 任意の $p$ に対して $\sum_i X[i] p_i - \sum_j X[N+j] p_{N+j}+ \sum_{ij} \max(0, p _ {N+j} - p _ i) \ge 0$

(3) と同じ天才考察により以下が同値であることがわかる。
> $\sum_i X[i] = \sum_j X[N+j]$ かつ 任意の頂点の部分集合の対 $(S \subseteq \lbrace 1,\ldots, N\rbrace, T \subseteq \lbrace N+1,\ldots, N+M\rbrace)$ に対し、 $\sum_{i\in S} X[i] - \sum_{j \in T} X[j] + |T|(N - |S|) \ge 0$
</details>

## (5)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">二次元平面上にN個の直線A[i]x+B[i]y=C[i]がある(C[i]!=0) 。以下の最大化問題と等しい最小化問題は？<br><br>原点を中心とした、どの直線とも交差しないような円の半径の最大値　<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1937000655794954499?ref_src=twsrc%5Etfw">June 23, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

<details>
<summary>意図がわかりかねている</summary>
円の半径 $r$ は直線の原点からの距離以下 ($r \le C[i]/\sqrt{A[i]^2 + B[i]^2}$) であるため、 $C[i]/\sqrt{A[i]^2 + B[i]^2}$ の最小値が答えである。
この問題をどうやって線型計画問題として定式化するかは謎。
</details>

## (6)
<details>
<summary>まだ解けていない</summary>

<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N個の食塩水があり、i番目は濃度A[i]で質量B[i]である。<br>以下の存在命題と同値な全称命題は？<br><br>「左の濃度&lt;=右の濃度であるような隣接する二つの食塩水を選んで取り除き、その中身を混ぜた新しい食塩水を元の位置に挿入する」ことを繰り返して、食塩水を一つにできる<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1937356279229612403?ref_src=twsrc%5Etfw">June 24, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (7)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N頂点の根付き木と、組(u,v,d) の形の制約M個に対して、以下の最大化問題と等しい最小化問題は？<br><br>各制約について、u→vパスの重みがd以下となるように、各辺に非負の値w[i]を与える。<br>ただし、各辺の重みは根に向かう方向に辿るときw[i], そうでないとき-w[i]。<br>1→Nパスの重みの最大値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1937721800189968485?ref_src=twsrc%5Etfw">June 25, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

類別: `最小費用流 (MCF) の双対`

<details>
<summary>解法</summary>
あらかじめ以下のように言い換えておく。答えを -1 倍して最小化問題に変換する。

> $N$ 頂点の根付き木の各頂点にポテンシャル $p_v$ を定める。 $uv$ が親子のとき $p _ u \le p _ v$ であり、制約 $(u, v, d)$ に対して $p _ v \le p _ u + d$ である。このとき $p _ 1 - p _ N$ を最小化せよ。

これは $p _ 1 - p _ N + \sum_{uv: \text{親子}} \infty \max(0, p _ u - p _ v) + \sum_{(u, v, d): \text{制約}} \infty \max(0, p _ v - p _ u - d)$ という形に変換できる。<a href="https://www.slideshare.net/wata_orz/ss-91375739">wata さんのスライド</a>の p.47 を参照する。言い換えた後の問題の双対は以下である:

> $N$ 頂点のネットワークがあり、頂点 1 への流入は 1、頂点 N への流入は -1 である。
> - 元々の根付き木の子から親へ、容量 $\infty$ コスト $0$ の辺がある。
> - 制約 $(u, v, d)$ に対して、$u$ から $v$ へ容量 $\infty$ コスト $d$ の辺がある。
>
> このネットワークにおける最小費用流を求めよ。

</details>

## (8)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さNで総和が0の整数列Aについて、以下の最大化問題と等しい最小化問題は？<br><br>任意のiで|B[i]-B[(i+1)%N]|&lt;=1であるような長さNの数列Bについて、ΣA[i]*B[i] の最大値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1938074738737484224?ref_src=twsrc%5Etfw">June 26, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (9)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さNの相異なる非負整数の列Aについて、以下の最小化問題と等しい最大化問題は？<br><br>各時刻 t=0,1,… で、各iで同時に、A[i]を1減らすか何もしないか選ぶ。ただし、A[i]-1と等しい要素が現在の列にある場合は減らす選択はできない。<br><br>全ての値が0以下になる時刻の最小値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1938434100177821748?ref_src=twsrc%5Etfw">June 27, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (10)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">2×Nグリッドにいくつかの赤いマスと青いマスが存在する。赤いマスは、x座標とy座標が共に自分以下の青いマスとマッチングできる。以下の存在命題と同値な全称命題は？<br><br>全ての赤いマスを相異なる青いマスにマッチングできる<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1938798748865569168?ref_src=twsrc%5Etfw">June 28, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

# (11)-(20)
## (11)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N頂点の無向グラフについて、以下の存在命題と等しい全称命題は？<br><br>奇数長の単純閉路が存在する。<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1939170725761556810?ref_src=twsrc%5Etfw">June 29, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

<details>
<summary>解法</summary>
任意の色割り当て $f \colon V\to \lbrace 0,1\rbrace$ に対して、 $f$ は 2-彩色ではない。つまり $f(u) = f(v)$ なる辺 $uv$ が存在する。
</details>

## (12)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">正整数Nについて、以下の最大化問題の答えは？<br><br>1~Nの整数の部分集合であって、約数/倍数関係にあるペアが存在しないようなもののサイズの最大値は？<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1939529280800895347?ref_src=twsrc%5Etfw">June 30, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (13)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N個の文字列S1,…,Snについて、以下の最大化問題の答えは？<br><br>どの文字列どうしも「片方が片方のsuffix」という関係にないような添字集合のサイズの最大値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1939906166978732281?ref_src=twsrc%5Etfw">July 1, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (14)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さNの整数列Aについて、以下の最小化問題と等しい最小化問題は？<br><br>「ある要素に+1,または-1する」ことを繰り返してAを広義単調増加にするための、操作回数の最小値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1940282890593722805?ref_src=twsrc%5Etfw">July 2, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

類別: `最小費用流 (MCF) の双対`

<details>
<summary>解法</summary>
<a href="https://www.slideshare.net/wata_orz/ss-91375739">wata さんのスライド</a>の p.47 を参照する。元の問題は以下のように言い換えられる:

> $N$ 個の変数 $X_i$ がある。$X_i \le X_{i+1}$ の条件付きで $\sum_i |X_i - A_i|$ を最小化せよ。

以下のように書き換えることができる。
> $N + 1$ 個の変数 $X_0, \ldots, X_N$ がある。$\sum_{1 \le i \le N-1} \infty \max(0, X_i - X_{i+1}) + \sum_i 1\max(0, X_i - X_0 - A_i) + \sum_i 1\max(0, X_0 - X_i + A_i)$ を最小化せよ。

これの双対問題はこのようなグラフにおける最小費用流である ($A = [5,4,1]$ の例):

![graph.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/67404/69b2af31-88ff-4186-9f79-056550769942.png)

さらに言い換えると以下のような問題になる。
> 長さ $N$ の整数列 $A$ に対して、$\sum_i A[N+1-i]b_i$ を最小化せよ。ただし、
> - $b$ は [-1, 0, 1] のいずれか
> - $\sum_{1 \le i \le N} b_i = 0$
> - $\sum_{1 \le i \le j} b_j \ge 0$

この問題は下に凸な折れ線を管理すれば良い。
- 各 $0 \le k \le N, 0 \le l \le k$ に対して、 $\sum_{1 \le i \le k} b_j = l$ のときの部分和の最小値を $\mathrm{dp}[k][l]$ と呼ぶ。
  - $\mathrm{dp}[k]$ は $l$ の関数として下に凸である。そのため、 $l=0$ から右に見ていくと増分列は単調増加であり、min を取り出せるタイプの優先度キュー + グローバルな差分 で管理できる。
  - 更新時には、「優先度キューに値を 2 回追加」→「グローバルな差分を調整」→「先頭を削除」を行う。
  - [Slope Trick](https://maspypy.com/slope-trick-1-%E8%A7%A3%E8%AA%AC%E7%B7%A8) と同じ考え方だが、残すのは右側だけで良い。

優先度キューを使えば $O(N \log N)$ 時間で解ける。

<details>
<summary>コード (Rust)</summary>

```rust
use std::cmp::*;
use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn solve(a: &[i64]) -> Vec<i64> {
    let mut que = BinaryHeap::new();
    let mut global = 0;
    for a in a {
        global -= a;
        que.push(Reverse(a));
        que.push(Reverse(a));
        let Reverse(x) = que.pop().unwrap();
        global += x;
    }
    let v = que.into_sorted_vec();
    let mut ans = vec![global];
    for Reverse(v) in v.into_iter().rev() {
        let new = ans[ans.len() - 1] + v;
        ans.push(new);
    }
    ans
}

fn main() {
    let a: Vec<i64> = getline().trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    a.reverse();
    let ans = solve(&a);
    eprintln!("{:?}", ans);
    println!("{}", -ans[0]);
}
```

</details>

<details>
<summary>実行結果</summary>

```console
$ rustc sol-14.rs 
$ ./sol-14 <<<"5 4 1"
[-4, 0, 5, 10]
4
```
</details>
</details>

## (15)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">正整数Kについて、以下の最大化問題の答えは？<br><br>どの二要素(a,b)についてもa&amp;b &lt; min(a,b)であるような、0以上2^K未満の整数の集合のサイズの最大値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1940648567229895006?ref_src=twsrc%5Etfw">July 3, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (16)
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">整数Xについて、以下の最大化問題と等しい最小化問題は？<br><br>Xの最大値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1941011933806973037?ref_src=twsrc%5Etfw">July 4, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

<details>
<summary>解法</summary>
制約がない状態で $1 \cdot X$ を最大化せよという問題。よって双対問題は

> $0 = 1$ という条件下で $0$ を最小化せよ

という問題である。これは実行可能解が存在しないので答えは $\infty$ であり、元の問題の答え (際限がないので $\infty$) と等しい。
</details>

## (17)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">各辺に整数重みw[e]が付いたN頂点の平面グラフと、そのある平面埋め込みについて、以下の最小化問題と等しい最大化問題は？<br><br>平面上のどの二点も互いに行き来可能にするために、削除する辺の重みの総和の最小値<br><br>(special thanks: <a href="https://twitter.com/Katu2ou?ref_src=twsrc%5Etfw">@Katu2ou</a> )<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1941378980004995438?ref_src=twsrc%5Etfw">July 5, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (18)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N個の禁止頂点を持つH×Wのグリッドグラフについて、以下の最大化問題と等しい最小化問題は？<br><br>(1,1)から(H,W)への内点素なパスの個数の最大値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1941745509368201672?ref_src=twsrc%5Etfw">July 6, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (19)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">H×Wのグリッドがあり、そのうちNマスが黒く塗られている。以下の最小化問題と等しい最大化問題は？<br><br>いくつかの行•列を選び、全ての黒いマスが選ばれた行•列の少なくとも一つに含まれているようにするための、選ぶ行•列の総数の最小値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1942067275399680009?ref_src=twsrc%5Etfw">July 7, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (20)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N頂点の根付き木があり、各頂点には非負整数A[v], B[v]が書かれている。以下の最小化問題と等しい最大化問題は？<br><br>いくつかの頂点を選び、任意の頂点vについてその部分木からA[v]個以上が選ばれているようにするとき、選んだ頂点のBの値の和の最小値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1942469997894255018?ref_src=twsrc%5Etfw">July 8, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

# (21)-(30)
## (21)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さNの整数列Aと整数Kについて、以下の最小化問題と等しい最大化問題は？<br><br>A[i]&gt;=Kなるiの最小値<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1942792544670343208?ref_src=twsrc%5Etfw">July 9, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (22)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">長さNの正整数列Aと、長さMの1以上N以下の整数列Bについて、以下の存在命題と等しい全称命題は？<br><br>N個の容器に対してM回操作を行う。j回目の操作では、N個の容器のうちB[j]個を選び、球を一つずつ入れる。このようにして、最終的に各容器に入っている球の数をA[i]にすることができる。<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1943161989020434433?ref_src=twsrc%5Etfw">July 10, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (23)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N次元ベクトルx,yに対して、以下の存在命題と等しい全称命題は？<br><br>xの要素を並び替えたN!通りのベクトルの非負の加重平均としてyが表せる<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1943507904335589587?ref_src=twsrc%5Etfw">July 11, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (24)
<details>
<summary>まだ解けていない</summary>
<blockquote class="twitter-tweet"><p lang="ja" dir="ltr">N頂点の森に対して、以下の最大化問題の答えは？<br><br>補グラフの最大マッチングのサイズ<a href="https://twitter.com/hashtag/%E6%AF%8E%E6%97%A5Duality?src=hash&amp;ref_src=twsrc%5Etfw">#毎日Duality</a></p>&mdash; ⋆꙳.*･ (@Segtree) <a href="https://twitter.com/Segtree/status/1943884466298232879?ref_src=twsrc%5Etfw">July 12, 2025</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
</details>

## (25)
<details>
<summary>まだ解けていない</summary>
</details>

## (26)
<details>
<summary>まだ解けていない</summary>
</details>

## (27)
<details>
<summary>まだ解けていない</summary>
</details>

## (28)
<details>
<summary>まだ解けていない</summary>
</details>

## (29)
<details>
<summary>まだ解けていない</summary>
</details>

## (30)
<details>
<summary>まだ解けていない</summary>
</details>

# (31)-(40)
