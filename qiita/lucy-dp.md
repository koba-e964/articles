# 概要
Lucy DP は、ナイーブには $O^*(N)$ 時間かかる計算を $o(N)$ 時間で済ませる手法である。
典型的な利用箇所は

- $N$ 以下の素数の個数の計算
- $N$ 以下の素数の和の計算

などである。

# 計算量とその理屈
## 空間計算量
$O(\sqrt{N})$
## 時間計算量
$O(N^{3/4})$

以下の DP を行う。
- 添字 1: $\lfloor N/i \rfloor$ 全体。合計 $O(\sqrt{N})$ 個。
- 添字 2: $p \le \sqrt N$ なる素数 $p$。
- $S(v, p) := p$ 以下の素数に対して篩をしたときに、 $v$ 以下で残っている個数。

これだけだと naïve には $O(N)$ だが、 $v \le p^2$ の範囲は操作がいらないことに留意すると $O(N^{3/4})$ となる。

# 実装
<https://github.com/koba-e964/contest/blob/HEAD/comm/math/QuoDP.rs>

# 参考文献
- <https://qiita.com/masa0599/items/2704047579eaafb2a322>
- <https://rsk0315.hatenablog.com/entry/2021/05/18/015511>
- <https://github.com/koba-e964/contest/blob/HEAD/comm/math/QuoDP.rs>
