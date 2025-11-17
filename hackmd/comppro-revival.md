# 競プロ参加記 (2025-09 から)

最終的な目標: ARC 20th 以内 (perf ~= 3100)

## テンプレート
### よかったところ

### 改善できそうなところ

### 二段上のレベルのために何が必要か

### パフォーマンス予測
- 実態: XXXX
- うまく行った場合: XXXX (0%)
- 期待値 (exp をとる): XXXX

## [ARC210](https://atcoder.jp/contests/arc210/tasks) (2025-11-16)
### よかったところ
- A,B,C の速解きができた

### 改善できそうなところ
- D を解きたかった
  - スーパーセットのグラフの認識について、考察も実装も苦手

### 二段上のレベルのために何が必要か
- E を解きたかった
  - マージの過程で元々 1.01 倍以上離れていたところに挿入するときにどうすれば良いのか不明
  - perf: [29th, 2928](https://atcoder.jp/users/Yukkku/history/share/arc210)

### パフォーマンス予測
- 実態: [53rd, 2793](https://atcoder.jp/users/kobae964/history/share/arc210)
- うまく行った場合: [39th, 2915](https://atcoder.jp/users/SakuraCat/history/share/arc210) (5%)
- 期待値 (exp をとる): 2800
 
```python
>>> math.log2(2**(2793/400)*0.95+2**(2915/400)*0.05)*400
2799.753078509965
```

## [ARC209](https://atcoder.jp/contests/arc209/tasks) (2025-11-09)
### よかったところ
- A,B は解けた
- D で詰まったところが考察ではなく実装だった
- 戦略は正しかった

### 改善できそうなところ
- A, B を解くのに時間がかかった
  - 43m + 31m = 74m だが、15m + 15m = 30m くらいにすべきだった
  - A: 実験したムーブそのものは正しかったようだが、後手の戦略が対応する括弧を取るしかないのに明示的に気付くのが遅くなった
  - B: 誤読などで時間が無駄になった
  - B: (可能な時に)同じ文字が隣り合わないように並び替える の実装に手間取った (バケットの個数を b として、ソートした文字列を s としたとき s[i] を i%b に入れる で ok)
- D で実装をバグらせた
  - two-pass でやるべきだったかもしれない
  - ランダムテストをすべきだった

### 二段上のレベルのために何が必要か
- A,B,C,D が時間内に解ける
  - perf = [15th, 3150](https://atcoder.jp/users/hitonanode/history/share/arc209)

### パフォーマンス予測
- 実態: [221st, 2138](https://atcoder.jp/users/kobae964/history/share/arc209)
- うまく行った場合: [113th, 2440](https://atcoder.jp/users/hint908/history/share/arc209) (30%)
- 期待値 (exp をとる): 2246

```python
>>> math.log2(2**(2138/400)*0.7+2**(2440/400)*0.3)*400
2246.230463026757
```

## [ARC207](https://atcoder.jp/contests/arc207/tasks) (2025-10-05)
### よかったところ
- 全力を出せた
- B を実験してサクッと解いた

### 改善できそうなところ
- A の箱根駅伝 DP について、何も知らなかった
- D が実験すれば見える系だったのに行かなかった

### パフォーマンス予測
- 実態: [235th, 2255](https://atcoder.jp/users/kobae964/history/share/arc207)
- うまく行った場合: [90th, 2654](https://atcoder.jp/users/hitoare/history/share/arc207) (5%)
- 期待値 (exp をとる): 2283

```python
>>> math.log2(2**(2255/400)*0.95+2**(2654/400)*0.05)*400
2283.06056768513
```

## [AGC073](https://atcoder.jp/contests/agc073/tasks) (2025-09-28)
### よかったところ
- 競プロを再開した
### 改善できそうなところ
- 全力ではなかった

### パフォーマンス予測
- 実態: [284th, 1798](https://atcoder.jp/users/kobae964/history/share/agc073)
- うまく行った場合: [284th, 1798](https://atcoder.jp/users/kobae964/history/share/agc073) (100%)
- 期待値 (exp をとる): 1798
