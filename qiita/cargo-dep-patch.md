## 状況説明
Rust で開発中に serde の一部機能を置き換えてテストしたくなり、改造した serde に依存させるべく以下のように Cargo.toml を書いた。(依存ライブラリ名、Cargo.toml の中身は理解できるように作られたフェイクで、実際はもっと別のライブラリです。)

```Cargo.toml
[dependencies]
serde = "0.9"

[patch.crates-io]
serde = { git = "https://github.com/serde-rs/serde", branch = "master" }
```
https://github.com/koba-e964/crate-version-diff-patch-test

コンパイルしてみた。

```
$ cargo c
warning: Patch `serde v1.0.117 (https://github.com/serde-rs/serde#8084258a)` was not used in the crate graph.
Check that the patched package version and available features are compatible
with the dependency requirements. If the patch has a different version from
what is locked in the Cargo.lock file, run `cargo update` to use the new
version. This may also occur with an optional dependency that is not enabled.
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
```

すると、patch セクションで取り込んだはずの変更が一向に取り込まれない！

## 原因
serde は現行バージョンが "1.0" になっており、semver の点からは互換性が失われていた。

- cargo は依存するクレートを `名前:バージョン` のペアとして扱い、**たとえ名前が同じでもバージョンが違うクレートは別として扱われ**、両方に依存することができる。この仕様は [semver trick](https://github.com/dtolnay/semver-trick) などのテクニックのために欠かせない仕様である。
- 今回の場合、serde:0.9 と serde:1.0 という 2 種のクレートに依存しているものとして扱われており、単に patch を追加しただけなので serde:1.0 の方はどこでも使用されておらず、警告が出た。

最終的には、serder:0.9 に依存しているクレート全ての依存性を serde:1.0 にアップグレードすることによって解決した。

## 教訓
- 警告メッセージは無視しない
- ツールの仕様を理解しておくとこういったコーナーケースで役に立つ
- **警告メッセージは無視しない**

