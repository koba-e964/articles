$k$ は頂点数。(パスの場合は長さ + 1 であることに注意。)

## 検出
数え上げでカバーされている場合もある。
|問題設定|計算量|問題・解説リンク|実装リンク|
|--|--|--|--|
|$k=4$ サイクル|$O(V^2)$ 時間、$O(V^2)$ 空間 ($V^2/8$ バイト程度)|http://lealgorithm.blogspot.com/2019/06/blog-post.html||
|$k=5$ サイクル|$O(V(V+E))$ 時間|https://yukicoder.me/problems/no/408||
|$k$ が小さいパス|$O(2^{O(k)}E)$ 時間、$O(2^k V)$ 空間 ($2^k V/8$ バイト程度)||
|$k$ が小さい木|$O(2^{O(k)}E)$ 時間、$O(2^k V)$ 空間 ($2^k V/8$ バイト程度)||

## 数え上げ
|問題設定|計算量|問題・解説リンク|実装リンク|
|--|--|--|--|
|$k=3$ パス|$O(E)$ 時間|$\sum_i C(d_i,2)$||
|$k=3$ サイクル|$O(E^{3/2})$ 時間、$O(E)$ 空間|https://www.slideshare.net/catupper/trianguler||
|$k=5$ パス||https://atcoder.jp/contests/tkppc2/tasks/tkppc2016_h, https://codeforces.com/gym/102028/problem/L||
|$k=5$ 木||https://codeforces.com/gym/102028/problem/L||
|$k=4$ サイクル・なもり||https://codeforces.com/gym/102028/problem/L||
