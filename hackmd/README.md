# articles/hackmd

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

リリースコミットには lightweight tag を付ける。

タグ名は次の形にする。

```text
agc-arc-memo-vX.Y.Z
```

例:

```text
agc-arc-memo-v1.1.1
```

更新履歴に書いた version、リリースコミットの version、タグ名の version は一致させる。

### version bump の目安

- 内容追加を含む公開は minor bump にする。
- ミス修正だけの公開は patch bump にする。

例えば、`v1.1.1` のあとに内容追加をまとめて公開する場合は `v1.2.0` が自然である。
