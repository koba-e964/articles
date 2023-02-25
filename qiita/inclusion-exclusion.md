Graded poset の族 $P_0, P_1, \ldots$ に対して以下の量を定める:

- $P_n$ に含まれる rank = $i$ の要素数: $S_{i,d}$


$\mu_{d,d} = 1$
$\mu_{i,d} = -\sum_{j=i+1}^d S_{j,d} \mu_{j,i}$

実用上は $i < j$ のとき $P_j$ の一部分に着目すると $P_i$ に同型なことも多く、それを利用して単純にできる。
具体的には $\mu_{i,j} = \mu_{0,j-i}$であるため $\mu_{0,d}$ を $\mu_d$　と書き直すことができる。

この時の包除原理の式は以下の通り:

- $f_d = \sum_{i \le d} S_{i,d} g_i$
- $g_d = \sum_{i \le d} S_{i,d} \mu_{d-i} f_i$

$\mu_0$ は 1 に決まっている。また $\mu_1$ は -1 であることが多い。

## imos
普通は包除原理扱いされないが一応包除原理の一例とみなすことができる。

$S_{i,d} = 1$
$\mu_0 = 1$
$\mu_1 = -1$
$\mu_d = 0 (d \ge 2)$

## powerset

$S_{i,d} = \binom{d}{i}$ (二項係数)
$\mu_d = (-1)^{d}$

常識


## F_2 ベクトル空間

$S_{i,d} = \binom{d}{i}_2$ (q-二項係数)

$\mu_d = (-1)^{d} 2^{d(d-1)/2}$
https://atcoder.jp/contests/abc278/tasks/abc278_h
