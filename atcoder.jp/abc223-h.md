## 前置き
https://atcoder.jp/contests/abc223/editorial/2784 の前置きと同じです。

## 解法
与えられる整数のサイズを $W := \lceil \log_2 \max A_i \rceil (\le 60)$ とします。

2 個の基底のマージにかかる時間は、それぞれのサイズを $a, b$ とおくと $O(ab)$ で、基底のサイズが $W$ 以下であることから $O(W^2)$ です。

平方分割の考え方を使います。
数列を大きさ $B$ のブロックに分け、ブロック間は sparse table で、ブロック内は愚直に計算することにすると、計算量は以下の通りです:

- sparse table の構築: $O((N/B)W^2 \log (N/B))$
- 1 個のクエリ処理:  $O(BW + W^2)$

これらのバランスが取れるのは $B = O(\sqrt{NW \log N/Q})$ のときで、全体の計算量は $O(\sqrt{NQW^3 \log N} + QW^2)$ です。

提出: https://atcoder.jp/contests/abc223/submissions/30147315
