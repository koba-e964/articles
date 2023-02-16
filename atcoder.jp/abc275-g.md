[wata さんのスライド](https://www.slideshare.net/wata_orz/ss-91375739)の 60 ページ以降で説明されているラグランジュ緩和を使用します。

元の問題は、
maximize
$\sum_i C_i x_i$

s.t.

1. $\sum_i A_i x_i \le 1$
2. $\sum_i B_i x_i \le 1$
3. $x_i \ge 0$

です。

これに対して、2 番目の制約に対してラグランジュ乗数 $\lambda$ を導入する、ラグランジュ緩和によって得られる問題は以下です。

$\min_{\lambda \ge 0} \max_{x} \sum_i (C_i - \lambda B_i) x_i + \lambda$

s.t.

1. $\sum_i A_i x_i \le 1$
2. $x_i \ge 0$

内側部分 ($\max_{x} \sum_i (C_i - \lambda B_i) x_i + \lambda$) を $f(\lambda)$ と置くと、$f(\lambda)$ は $(C_i-\lambda B_i)/A_i$ が最大である $i$ を選びそこにオールインすることで $O(N)$ 時間で計算できます。

外側部分($\min_{\lambda \ge 0} f(\lambda)$)ですが、これは $\lambda$ に関して下に凸なので三分探索で計算できます。三分探索の範囲は $[0, 10]$ で良いです。なぜなら制約から $\lambda \ge 10$ のとき  $C_i - \lambda B_i \le 0$ であり、$x_i=0$ が最適解で $f(\lambda) = \lambda$ であることから $f$ の最小値がこれより左にあることがわかるからです。

(傾きによる二分探索でもできます。$f(\lambda)$ の計算で得られた最適な $i$ を $j$ と呼ぶことにすると、$f$ はほとんどいたるところで微分可能で、微分可能なところで $f'(\lambda) = 1 - B_j/A_j$ であることがわかります。微分不可能な点でもこの値を採用しあたかも微分できるかのように扱って、$f'(\lambda) \ge 0$ なる最小の $\lambda$ を探索すれば良いです。)

以上で $O(N \log (\max C / \varepsilon \min B))$ 時間で計算できることがわかりました。(ただし $\varepsilon$ は要求される精度で、この問題では $\varepsilon = 10^{-6}$ です。)

提出 (Rust): https://atcoder.jp/contests/abc275/submissions/38918226
