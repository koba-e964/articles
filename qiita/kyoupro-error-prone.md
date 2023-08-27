---
title: 競プロ 間違いやすいところまとめ
tags: 競プロ Rust
author: kobae964
slide: false
---
## 個数の二分探索
### 問題
0 が $a_0$ 個、1 が $a_1$ 個、…、さて $k$ 番目は何? ($k \ge 1$ で k は 0-origin)

### 答え
$k$ 番目が $i$ <=> $a_0 + \cdots + a_{i-1} \le k$ かつ $a_0 + \cdots + a_{i} > k$
<=> $\mathrm{acc}[i] \le k$ かつ $\mathrm{acc}[i+1] > k$
<=> `i == acc.upper_bound(&k) - 1`
