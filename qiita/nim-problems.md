Nim についてまとめる。ネタバレ多いので注意。大体難易度の昇順に並べているつもり。

# 典型 (ゲーム)
## 普通の Nim
個数 $a_1, \ldots, a_n$ の山があり、交互に好きな個数取る。

解法: xor が非ゼロなら勝ち。xor が非ゼロ値 g の場合、 $a_i \oplus g < a_i$ なる $i$ が必ず存在する。その $i$ について $i$ 番目の山から $a_i \oplus g$ 個取る。

例題: スタンダード過ぎ、たくさんあり過ぎ

## 個数制限付き Nim
個数 $a_1, \ldots, a_n$ の山があり、交互に好きな個数取る。ただし個数は $1$ 個から $k$ 個まで。

解法: 各山の grundy 数は $a_i \bmod (k+1)$ なので、それらの xor が非ゼロなら勝ち。

例題:
- https://yukicoder.me/problems/no/669
- https://drken1215.hatenablog.com/entry/2019/03/07/091100

## Decreasing Nim (仮)[^decreasing-nim]
個数 $a_1, \ldots, a_n$ の山があり、交互に好きな個数取る。ただし 2 手目以降は前の人の取った個数**以下**しかとれない。

解法: xor が非ゼロなら勝ち。奇数なら 1 個取れば合計が偶数で 1 個しか取れない状況を相手に押し付けられるので勝ち、偶数ならどちらも偶数個取るしかないので、`a_i /= 2` を実行して同じ考察をする。

例題: https://yukicoder.me/problems/no/2666

[^decreasing-nim]: 特に名前がついていないようなので、元の問題名の部分列をとった。

## Staircase Nim
個数 $a_0, \ldots, a _ {n-1}$ の山があり、交互に添字 $1 \le i \le n-1$ を選んで好きな数の石を左に移す。移せなくなったら負け。

解法: $a_1 \oplus a_3 \oplus \cdots$ が非ゼロなら勝ち。勝っている側は普通に Nim をプレイすればよく、負けている側が $a_{2i} \to a_{2i-1}$ をやった場合は同じ個数だけ $a_{2i-1} \to a_{2i-2}$ をすれば良い。

資料: https://drken1215.hatenablog.com/entry/2019/03/15/114200

例題:
- https://yukicoder.me/problems/no/2726

## 実験してみたら周期性などがありましたよ Nim
例題:
- https://yukicoder.me/problems/no/715 (Dawson's chess, [octal game](https://en.wikipedia.org/wiki/Octal_game) 0o0.137, 周期 34)
- https://yukicoder.me/problems/no/2285 (周期 34)
- https://atcoder.jp/contests/nikkei2019-ex/tasks/nikkei2019ex_h (周期 9)
- https://www.hackerrank.com/contests/yfkpo5/challenges/g-exponential-banana-game (周期色々)

# 典型 (数え上げ)
ゲームを数え上げる問題もある。(例: 後手必勝の局面は何個あるか?)
たとえば以下のような問題:
> 山の個数が $n$ であり、 $i$ 番目の山の個数はそれぞれ指定された有限集合の要素
 ($0$ 以上 $a_i$ 以下など) である。このような Nim を行うとき、後手必勝の局面の個数は?

これは添字が xor の畳み込み ($C[i \oplus j] \leftarrow A[i] B[j]$) であり、[高速アダマール変換](https://sapphire15.hatenablog.com/entry/2021/09/13/114900)などで計算できる。

# 難しい問題
- https://yukicoder.me/problems/no/946
- https://atcoder.jp/contests/agc017/tasks/agc017_d
- https://drken1215.hatenablog.com/entry/2020/03/22/173500
