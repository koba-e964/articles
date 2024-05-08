コード例はすべて Rust で書かれている。問題のネタバレあり。

## 基本

$\sum_{i = 0}^n i = n(n+1)/2$
$\sum_{i = 0}^n i^2 = n(n+1)(2n+1)/6$
$\sum_{i = 0}^n i(i-1) = n(n+1)(n-1)/3$ ($\sum_{i = 0}^n i(i+1) = n(n+1)(n+2)/3$)

## 絶対値の和
$0 \le a \le n$ のとき、
```math
\begin{align}
\sum_{i=0}^n |i-a| &= a + \cdots + 1 + 0 + 1 + \cdots + (n-a) \\
&= (a(a+1) + (n-a)(n-a+1))/2
\end{align}
```

```rust:diffsum.rs
fn diffsum(n: i64, a: i64) -> i64 {
    let mut tot = a * (a + 1) / 2;
    tot += (n - a) * (n - a + 1) / 2;
    tot
}
```

```math
\begin{align}
\sum_{i=0}^n \sum_{i=0}^n |i-j| &= 2\sum_{a=0}^n a(a+1)/2 \\
&= n(n+1)(n+2)/3
\end{align}
```

## 一部だけ取り除く

https://kmyk.github.io/cp-unspoiler/?q=aHR0cHM6Ly9hdGNvZGVyLmpwL2NvbnRlc3RzL2FyYzE3Ni90YXNrcy9hcmMxNzZfZA%3D%3D から例題。
<details><summary>ネタバレ (問題へのリンク)</summary>
ARC176-D https://atcoder.jp/contests/arc176/tasks/arc176_d
</details>

部分問題として以下のような問題を解く必要がある。

1. $\sum_{1\le i \le n, i \neq b} |i-a|$
2. $\sum_{\\{i, j\\} \subseteq \\{1,\ldots,n\\}, \\{i,j\\} \cap \\{a,b\\} = \emptyset} |i-j|$ ($a \neq b$)

`1.` は `diffsum(n, a) - a - (b - a).abs()` で簡単。`2.` について、包除原理を使ってみよう。
考慮すべき条件は $i=a$, $i=b$, $j=a$, $j=b$ の 4 種類。これらの部分集合について包除原理を使えば良いが、考慮すべき部分集合は
|条件|重み|
|--|--|
|$\mathrm{true}$ (条件なし) |1|
|$i=a$|-1|
|$i=b$|-1|
|$j=a$|-1|
|$j=b$|-1|
|$i=a \wedge j=b$|1|
|$i=b \wedge j=a$|1|

の 7 種類である。(4 頂点 4 辺のサイクルの独立集合の個数と同じ。) $i=a \wedge j=a$ のような矛盾する条件の下での和は 0 であり、あってもなくても一緒なので包除原理を考える時はあるとみなしてやれば普通の包除原理と同じ係数 ($(-1)^{\mathrm{条件の個数}}$) になることがわかる。
