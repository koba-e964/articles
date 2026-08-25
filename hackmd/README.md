# articles/hackmd

## 共通の運用規則

### 1.0.0 未公開の記事の更新履歴

`## 更新履歴` を持つ記事では、最初に書く履歴は `v1.0.0` とする。`v1.0.0` より前の `v0.x.y` エントリーは置かない。

まだ `v1.0.0` を公開していないが履歴表だけ先に置く場合、日付は stub として `YYYY-MM-DD` を使ってよい。

```markdown
|YYYY-MM-DD|v1.0.0 公開|
```

初回公開時に `YYYY-MM-DD` を実際の日付へ置き換える。

### タグ

リリースコミットには lightweight tag を付ける。

タグ名は次の形にする。

```text
記事名-vX.Y.Z
```

例:

```text
agc-arc-memo-v1.1.1
comppro-speedups-v1.0.0
```

更新履歴に書いた version、リリースコミットの version、タグ名の version は一致させる。

### version bump の目安

- 内容追加を含む公開は minor bump にする。
- ミス修正だけの公開は patch bump にする。

例えば、`v1.1.1` のあとに内容追加をまとめて公開する場合は `v1.2.0` が自然である。

### 問題リンクの表記

問題へのリンクは、`<https://...>` だけで済ませず、基本的に問題名つきの Markdown リンクにする。裸 URL は参考記事・解説・提出・ライブラリなど、リンク先のタイトルを本文側で持たなくてもよいものに限る。

AtCoder の問題は次の形を基本にする。

```markdown
[ABC360-F InterSections](https://atcoder.jp/contests/abc360/tasks/abc360_f)
[ARC212-E Drop Min](https://atcoder.jp/contests/arc212/tasks/arc212_e)
[AGC071-A XOR Cross Over](https://atcoder.jp/contests/agc071/tasks/agc071_a)
```

- `ABC360-F` のように、コンテスト略称と問題記号を `-` でつなぐ。
- その後ろに半角スペースを入れて、AtCoder 上の問題タイトルを書く。
- 企業コンテストや特殊コンテストは、既存の短い呼び名が自然ならそれを使う。例: `KEYENCE2021-E Greedy Ant`
- 本文中で文脈上明らかな場合も、問題リンクだけは `ARC078-D` のような記号だけで終わらせず、できれば問題タイトルまで入れる。

Codeforces の問題は、URL の `/contest/数値/` だけを見てリンクテキストを決めない。Codeforces では contest ID と round number が基本的に一致しないので、問題ページにアクセスして、ページ上部の round 名・Div.・問題記号・問題タイトルを確認する。

通常ラウンドなら、次の短縮形を基本にする。

```markdown
[CF613-2F Classical?](https://codeforces.com/contest/1285/problem/F)
[CF1035-2D Token Removing](https://codeforces.com/contest/2119/problem/D)
```

- `CF613-2F` のように、round number、Div.、問題記号を入れる。
- `-2F` は `Div. 2` の `F` を表す。Div. 情報は落とさない。
- その後ろに半角スペースを入れて、Codeforces 上の問題タイトルを書く。
- ラウンド名自体に意味がある、または短縮形にすると分かりにくい特殊ラウンドでは、ページ上の round 名を使ってよい。例: `[EPIC Institute of Technology Round Summer 2024 (Div. 1 + Div. 2)-D World is Mine](https://codeforces.com/contest/1987/problem/D)`

## `comppro-speedups.md` の運用規則

`comppro-speedups.md` は、高速化の典型を自分用にまとめるメモである。

### 書き方

- `agc-arc-memo.md` と同じく、見出し、箇条書き、問題例、短い実装メモを中心にする。
- 青向けの一般説明や、読者向けの注意書きを足しすぎない。
- ユーザーが削った語や補足を戻さない。
- 「A と書かなくてよい」と判断した場合、「A とは限らない」のような反対向きの説明も書かない。

### 問題例

- 速度比較を書く場合は、提出リンクを併記する。
- 実行時間は提出ページ上の値をそのまま書く。推測で補わない。

### 更新コミット

通常更新のコミットメッセージは次の形にする。

- 追加: `Update comppro-speedups.md (add 内容)`
- 修正: `Update comppro-speedups.md (fix 内容)`

### リリースコミット

公開するタイミングでは、`comppro-speedups.md` の `## 更新履歴` に 1 行追加するだけのリリースコミットを作る。

リリースコミットのメッセージは次の形にする。

```text
comppro-speedups vX.Y.Z
```

タグ名は次の形にする。

```text
comppro-speedups-vX.Y.Z
```

## `agc-arc-memo.md` の運用規則

`agc-arc-memo.md` は、通常更新を積んだあとにリリースコミットを作り、そのリリースコミットに対応するタグを付ける。

### 通常更新コミット

通常更新のコミットメッセージは次の形にする。

- 追加: `Update agc-arc-memo.md (add 内容)`
- 修正: `Update agc-arc-memo.md (fix 内容)`

例:

- `Update agc-arc-memo.md (add ARC212-E)`
- `Update agc-arc-memo.md (fix 閉路マトロイド)`

### リリースコミット

公開するタイミングでは、`agc-arc-memo.md` の `## 更新履歴` に 1 行追加するだけのリリースコミットを作る。

リリースコミットのメッセージは次の形にする。

```text
agc-arc-memo vX.Y.Z
```

例:

```text
agc-arc-memo v1.1.1
```

### 更新履歴

`## 更新履歴` の表には、新しい行を上に追加する。

形式は次の通り。

```markdown
|YYYY-MM-DD|vX.Y.Z 公開、内容の要約|
```

例:

```markdown
|2025-11-15|v1.1.1 公開、ミスの修正|
|2025-11-14|v1.1.0 公開、パターンが限られる系・弦・区間 DP を追加、様々な問題の解法を追加|
```

更新履歴を書くときは、前回リリースタグから今回リリース直前までの通常更新コミットを確認する。

```sh
git log --reverse --format='- %s' agc-arc-memo-vX.Y.Z..HEAD -- agc-arc-memo.md
```

ここで `agc-arc-memo-vX.Y.Z` は前回リリースタグに置き換える。出力された通常更新コミットの一覧を見て、更新履歴には 1 行で要約を書く。

例えば、次のようなコミットが並んでいる場合:

```text
- Update agc-arc-memo.md (add DAG 上単一終点の DP)
- Update agc-arc-memo.md (add ランレンクス圧縮)
- Update agc-arc-memo.md (add 操作列を逆から見る)
```

更新履歴は次のようにまとめる。

```markdown
|2026-01-18|v1.2.0 公開、DAG 上単一終点の DP・ランレンクス圧縮・操作列を逆から見るを追加|
```

### タグ

タグ名は次の形にする。

```text
agc-arc-memo-vX.Y.Z
```

例:

```text
agc-arc-memo-v1.1.1
```
