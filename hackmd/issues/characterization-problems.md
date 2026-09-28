# 必要条件を列挙したら十分条件でした系

特徴づけを探す系。

[ABC449-G Many Repunit Sum 2](https://atcoder.jp/contests/abc449/tasks/abc449_g) を基準に、「まず必要条件を列挙して、それが実は十分条件になる」感が強い順で並べる。

ABC449-G 自体は repunit 和を 10 冪和に言い換えたあと、`n >= N`, `f(n) <= N`, `f(n) == N mod 9` が必要十分条件になるタイプ。

## かなり近い

- [ABC449-G Many Repunit Sum 2](https://atcoder.jp/contests/abc449/tasks/abc449_g)
  - 個数・最小個数・mod 9 の必要条件が、そのまま十分条件になる。

- [ARC227-D Median of Binary Strings](https://atcoder.jp/contests/arc227/tasks/arc227_d)
  - 1 bit 条件、2 bit 条件を深掘りすると、実はそれで全体の構築可能性が特徴づけられる。
  - <https://atcoder.jp/contests/arc227/editorial/23784?lang=en>

- [ARC227-F Erase and Raise](https://atcoder.jp/contests/arc227/tasks/arc227_f)
  - 最終列 `B` について、長さの偶奇・値の相異性・変動量の不等式が必要で、逆操作を見ると十分。
  - <https://atcoder.jp/contests/arc227/editorial/23802>

- [ABC454-E LRUD Moving](https://atcoder.jp/contests/abc454/tasks/abc454_e)
  - 市松模様から `N` 偶数・`A+B` 奇数が必要になり、構成で十分性を示す。
  - <https://atcoder.jp/contests/abc454/editorial/19125>

- [ARC191-B XOR = MOD](https://atcoder.jp/contests/arc191/tasks/arc191_b)
  - `N <= X < 2N` と「`X` が `N` の立っている bit を全部含む」が必要十分条件になる。
  - <https://atcoder.jp/contests/arc191/editorial/12066>

## 構成・不変量寄り

- [ARC198-B Rivalry](https://atcoder.jp/contests/arc198/tasks/arc198_b)
  - `X >= 1`, `Y <= 2X`, `Z <= X`, 例外条件を並べると、それらが構成可能性の必要十分条件になる。
  - <https://atcoder.jp/contests/arc198/editorial/13116?lang=en>

- [AGC077-A Reverse A...B](https://atcoder.jp/contests/agc077/tasks/agc077_a)
  - `A` の個数、辞書順、反転後の辞書順という 3 条件で変換可能性を特徴づける。
  - <https://atcoder.jp/contests/agc077/editorial/16486>

- [AGC059-E Grid 3-coloring](https://atcoder.jp/contests/agc059/tasks/agc059_e)
  - 境界から整数ポテンシャルを復元し、距離制約などの必要条件が十分条件になる。
  - <https://atcoder.jp/contests/agc059/editorial/5325>

- [ARC135-D Add to Square](https://atcoder.jp/contests/arc135/tasks/arc135_d)
  - 行・列ごとの交代和不変量が必要条件で、左上から合わせる操作により十分性を示す。
  - <https://atcoder.jp/contests/arc135/editorial/3381>

## 軽め・典型確認用

- [ABC379-C Sowing Stones](https://atcoder.jp/contests/abc379/tasks/abc379_c)
  - prefix に十分な石があることが、最終的に各マス 1 個にできる必要十分条件。
  - <https://atcoder.jp/contests/abc379/editorial/11328?lang=en>

- [ABC287-C Path Graph?](https://atcoder.jp/contests/abc287/tasks/abc287_c)
  - 辺数 `N-1`、次数 2 以下、連結性という必要条件を列挙するとパスグラフである十分条件にもなる。
  - <https://atcoder.jp/contests/adt_hard_20260127_1/editorial/5608?lang=ja>

- [ABC466-G Segment Sum Constraints](https://atcoder.jp/contests/abc466/tasks/abc466_g)
  - 入力の区間和制約を、より小さい必要十分な制約集合へ落とすタイプ。
  - <https://atcoder.jp/contests/abc466/editorial/22631?lang=en>
