---
title: 木の個数
tags:
  - 競プロ
  - 数え上げ
  - FPS
  - 母関数
private: true
id: 3f8bc67c8f464f57df4c
slide: false
---
# FPS についての性質

$A = \sum_{i \ge 1} i^{i-2}x^i/i! = x + x^2/2 + x^3/2 + 2x^4/3 + 25x^5/24 + \cdots$
(レーベル付き木の指数型母関数)

$Y = \sum_{i \ge 1} i^{i-1}x^i/i! = x + x^2 + 3x^3/2 + 8x^4/3 + 125x^5/24+ \cdots$
(レーベル付き木 + その上の頂点 の指数型母関数)

- $Y = x \exp(Y)$
  - レーベル付き木 + その上の頂点 は、その頂点に「レーベル付き木 + その上の頂点」を順不同で任意個付けたもの
- $Y = xA'$
- $$\frac{Y^m}{x^m} = \exp(mY) = \sum_{i \ge 0} \frac{m(m+i)^{i-1}}{i!}x^i$$
  - ガジェットの付け先が $m$ 種類あるとき、頂点番号 1 だけ $m$ 重になっているかのように数え上げできる ($m$ 倍を忘れずに)
    - 証明: Prüfer code で
    - 関連問題: [CF 529-1D (Sasha and Interesting Fact from Graph Theory)](https://codeforces.com/contest/1109/problem/D)

## 関連問題
- [CF 529-1D (Sasha and Interesting Fact from Graph Theory)](https://codeforces.com/contest/1109/problem/D)
- [ケイリーの公式の証明6種類](https://joisino.hatenablog.com/entry/2017/08/20/200000)
