# 競プロの高速化系メモ

解法は合っていそうなのに TLE する、あるいは実装すると重すぎる、という場面で考えることをまとめる。

高速化を「何を落とすか」という見方で整理する。

## 更新履歴
|日付|イベント|
|--|--|
|YYYY-MM-DD|v1.0.0 公開|

## 典型

### ループを落とす

- 典型
  - 全ての候補を試す代わりに、遷移先が限られていることを使う
  - 全ての区間を持つ代わりに、端点や極小なものだけを見る
  - 全ての状態を更新する代わりに、差分が出る場所だけを見る
- 区間 DP の高速化
  - naive には次の形で $O(N^3)$ になる: $dp[l][r] = \min_m f(l,m,r)$
  - 有効な遷移が少ない、あるいは遷移先に強い制約がある場合、$m$ のループを落として $O(N^2)$ にできることがある
  - 問題例
    - [ARC204-B Sort Permutation](https://atcoder.jp/contests/arc204/tasks/arc204_b)

### `log` を落とす

- 方向性
  - スタックで単調性を管理する
  - two pointers で区間を動かす
  - prefix sum / difference array に落とす
  - 座標を値そのものではなく「変化点」として見る
  - オフライン化して、時刻順・値順に一度だけ走査する
- 確認すること
  - 今ほしいのは本当に任意区間クエリか
  - 片側から伸びる prefix / suffix だけでよいか
- 問題例
  - [ARC115-E LEQ and NEQ](https://atcoder.jp/contests/arc115/tasks/arc115_e)

### 定数倍を落とす

- 基本方針
  - 使っていない一般性を捨てる

#### セグメント木 -> BIT

- `セグメント木 -> BIT` は「使っていない一般性を捨てる」代表例
- 2倍くらい速くなることもある
- 置き換えの判断
  - 差を取るなら逆元があるか
- 問題例
  - [ABC384-G Abs Sum](https://atcoder.jp/contests/abc384/tasks/abc384_g)
    - BIT 版: [提出 78675916](https://atcoder.jp/contests/abc384/submissions/78675916), `2335 ms`
    - SegTree 版: [提出 78675140](https://atcoder.jp/contests/abc384/submissions/78675140), `> 5000 ms`
    - `SegTree 版 / BIT 版 > 2.1`
    - BIT 版は SegTree 版の `< 0.47` 倍の時間で動いた

#### 疎な計算を密にしない

- 問題例
  - [ARC203-D Insert XOR](https://atcoder.jp/contests/arc203/tasks/arc203_d)
    - naive に 7 次正方行列をセグメント木に載せると TLE
    - 実際には見なくてよい成分がある
    - `i <= 4` や `i > j` の箇所を無視するような定数倍高速化で AC

### メモリを落とす

これをすると定数倍高速化にもなることがある。

- 観点
  - `Vec<Vec<T>>` ではなく、必要なら一次元 `Vec<T>` に潰す
  - `HashMap` ではなく、座標圧縮 + `Vec` で持つ
  - 全 DP 表ではなく、直前行・直前列だけ持つ
  - セグメント木 2 本ではなく、BIT 2 本や累積和 2 本にできないか見る
  - 構造体の配列より、配列の構造体化/構造体の配列化を考える
- 注意
  - メモリを落とすために添字変換が複雑になると、バグで時間を失う

### 実装量を落とす

- 実装量を落とすことも高速化の一部と見なせる
- 例
  - オンライン処理をオフライン処理にする
  - push 型更新を pull 型更新にする
  - 多方向の遷移を、片方向の走査にする
  - 複数ケースを同じ式にまとめる
  - ランダムテストしやすい小さい関数に切り出す

## 周辺高速化 pointers

### Rust の入出力

- 参考
  - [`std::io::read_to_string`](https://doc.rust-lang.org/std/io/fn.read_to_string.html)
  - [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html)
  - [`std::io::BufRead`](https://doc.rust-lang.org/stable/std/io/trait.BufRead.html)
  - [`std::io::Stdin`](https://doc.rust-lang.org/std/io/struct.Stdin.html)

### Rust のコンパイル設定

- ローカル計測では、必ず release build で測る
- AtCoder では提出環境のコンパイルコマンドや利用可能ライブラリが決まっている
- 参考
  - [The Cargo Book: Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
  - [AtCoder: 新ジャッジ運用開始のお知らせ](https://atcoder.jp/posts/1579?lang=ja)
  - [AtCoder で使用出来る言語とライブラリの一覧 - 2025/10](https://img.atcoder.jp/file/language-update/2025-10/language-list.html)
  - [Rust Blog: Announcing Rust 1.89.0](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/)
