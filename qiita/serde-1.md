概要[^1]: Rust の serde は面白いです。そこで、serde の内部構造を解説します。何回かに分けて詳しく見ていきたいと思います。

[^1]: タイトルと概要は https://qiita.com/qnighy/items/4bbbb20e71cf4ae527b9 をパクりました。ありがとう…

**1. serde の基本**
2. serialize を読む (予定)
3. deserialize を読む 1 (予定)
4. deserialize を読む 2 (予定)
5. deserialize を読む 3 (予定)
6. formats (予定)

# はじめに
serde は直列化 (serialize) および直列化復元 (deserialize) を行うためのライブラリです。様々なデータ型、様々なデータフォーマットに対して直列化処理というものがジェネリクスを用いて統一的に記述できます。

ジェネリクスを使い、データフォーマット側とデータ型側で分離した設計にしている[^2]理由は、公式のドキュメントにあります。以下 [serde 公式][serde-rs] より引用:


> Design
>
> Where many other languages rely on runtime reflection for serializing data, Serde is instead built on Rust's powerful trait system. A data structure that knows how to serialize and deserialize itself is one that implements Serde's `Serialize` and `Deserialize` traits (or uses Serde's derive attribute to automatically generate implementations at compile time). This avoids any overhead of reflection or runtime type information. In fact in many situations the interaction between data structure and data format can be completely optimized away by the Rust compiler, leaving Serde serialization to perform the same speed as a handwritten serializer for the specific selection of data structure and data format.

訳:
> 設計
>
> 他の多数の言語がデータの serialize に実行時リフレクションを使う一方で、 serde は Rust の強力なトレイトの仕組みの上に築かれている。データ構造であって、自分自身を serialize および deserialize する方法を知っているものは、 serde の `Serialize` と `Deserialize` を実装した (あるいは、 serde の derive 属性を使って、コンパイル時に自動で実装を生成した) データ構造である。これにより、リフレクションや実行時の型情報によるオーバヘッドが避けられる。実際に多くの状況下で、データ構造とデータフォーマットの協調は完全に Rust コンパイラの最適化で消える。そのため、serde の serialize は、特定のデータ構造とデータフォーマットに向けて手書きで書いた serializer と同等のスピードで動く。

実際に serialize, deserialize の速度は速く、例えば JSON フォーマット向けの serde 実装 [serde_json](https://github.com/serde-rs/json) について、serialize は 500 ~ 1000 MB/s、deserialize は 600 ~ 900 MB/s という測定結果があります。これは C++ による実装とも遜色ない速度です。[ソース](https://github.com/serde-rs/json#performance)


[^2]: ちなみに、 [miniserde](https://github.com/dtolnay/miniserde) というライブラリもあります。これは serde とは真逆の設計思想で作られており、JSON だけ・`String` への変換だけ、`struct` だけ・エラーは `()` 型・カスタマイズ不可という、一点集中型の単純さに特化した設計です。

さて、serde の内部構造について読んでいきたいわけですが、読むためにもそれなりの下準備が必要だと思われます。
そのため、この記事では serde の概観を説明し、個々の概念の詳しい説明は次回以降に譲ることとします。特に、ソースコードを詳しく読むことはしません。

# まとめ

- Serde はジェネリクスにより、仮に手書きで書いたと仮定した場合の実装と同等の速度を出すフレームワークを実現している。
- `#[derive(Serialize)]` や `#[derive(Deserialize)]` を使うことで、任意の型に対して serialize や deserialize の処理を自動で導出することができる。
- 一般的に serialize の方が deserialize より簡単。これは、考えるべきことが少ないため。
- JSON への serialize、JSON からの deserialize は直感的。

# 基本
## serialize
serialize を行う時、`Serialize` と `Seralizer` という2種類のトレイトが基本的な役割を果たします。

データ型側は `Serialize` を実装します。`Serialize` の仕事は、データフォーマットに合わせてなんらかの出力をする人 (`Serializer`) に向けて、どういうデータを出力して欲しいかを逐一送ることです。

以下のような基本的な型に対する実装は serde 内部でされています。 ([ser/impls.rs](https://github.com/serde-rs/serde/blob/master/serde/src/ser/impls.rs))

```
impl Serialize for i64 { ... }
impl Serialize for String { ... }
...
```
また、ユーザ定義型については、 `#[derive(Serialize)]` を使うことで `Serialize` を自動で実装することができます。

```rust:my_struct.rs
use serde::Serialize;

#[derive(Serialize)]
struct MyStruct {
    a: i64,
    b: i32,
    c: String,
}
```

データフォーマット側は `Serializer` を実装します。`Serializer` の仕事は、 `Serialize` を実装する型から送られてきたデータを変換し、特定のデータフォーマットに合わせて出力することです。実装例としては、データを JSON フォーマットで出力する [`serde_json::Serializer`](https://github.com/serde-rs/json/blob/v1.0.44/src/ser.rs#L14-L18) などがあります。

```rust:ser.rs
use serde::ser::{self, Serialize}; // これ以外のインポートは省略

/// A structure for serializing Rust values into JSON.
pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}

(省略)

impl<'a, W, F> ser::Serializer for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    (必要な型、メソッドの定義...)
}
```

Serialize の詳しい流れなどは次回以降に譲ります。

## deserialize
基本的には serialize の逆です。今回の登場人物は `Deserialize` と `Deserializer` です。

データ型側は `Deserialize` を実装します。`Deserialize` の仕事は、データフォーマットをパースしてくれる人 (`Deserializer`) からデータを受け取って、自分のデータ構造を順番に復元することです。

以下のような基本的な型に対する実装は serde 内部でされています。 ([de/impls.rs](https://github.com/serde-rs/serde/blob/master/serde/src/de/impls.rs))

```
impl Deserialize for i64 { ... }
impl Deserialize for String { ... }
...
```
また、ユーザ定義型については、 `#[derive(Deserialize)]` を使うことで `Deserialize` を自動で実装することができます。

```rust:my_struct.rs
use serde::Deserialize;

#[derive(Deserialize)]
struct MyStruct {
    a: i64,
    b: i32,
    c: String,
}
```

データフォーマット側は `Deserializer` を実装します。`Deserializer` の仕事は、データフォーマットを構文解析し、`Deserialize` を実装した型が欲しいデータを、要求に応じて返すことです。

Deserialize の詳しい流れなどは次回以降に譲ります。

## テキストフォーマットとバイナリフォーマット

テキストフォーマットの例:

- [JSON][serde-json]
- [YAML][serde-yaml]
- [TOML][toml-rs]

バイナリフォーマットの例:

- [bincode](https://github.com/TyOverby/bincode)
- [MessagePack](https://github.com/3Hren/msgpack-rust)

一般にテキストフォーマットとバイナリフォーマットでは、以下の点でデータのエンコード方法が違います。

- テキストフォーマットは自己記述的 (データを見れば型がわかる) であることが多い。バイナリフォーマットはそうでないことが多い。
- 配列の長さやマップの大きさについて、テキストフォーマットはデータを見ればわかることが多いが、バイナリフォーマットはあらかじめ埋め込んでおく必要があることが多い。

どちらにも対応できるように、 `Serialize`, `Deserialize` の方で調整をする必要があります。

これについては、次回以降で詳しく書きたいと思います。

# JSON の変換例
おそらく最もよく使われているデータフォーマットである JSON について、serde がどのように動くかを見てみましょう。[^other_format_excuse]
[^other_format_excuse]: [play.rust-lang.org][rust-play] は [serde_json][serde-json], [serde_yaml][serde-yaml], [toml-rs] くらいしかサポートしていないため、特にバイナリフォーマットへの serialize / バイナリフォーマットからの deserialize を [play.rust-lang.org][rust-play] で試すのは困難です。

## serialize
実際に JSON へ serialize を行う例を見てみましょう。`serde_json::to_string` を呼ぶことで試せます。

https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c4e12d8cd249f43df941da352e9b8bdf

```rust:main.rs
use serde::Serialize;

#[derive(Serialize)]
struct MyStruct {
    a: i64,
    b: i32,
    c: String,
    d: Vec<MyStruct>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = MyStruct {
        a: 3,
        b: 2,
        c: "a".to_owned(),
        d: vec![MyStruct {
            a: 0,
            b: -1,
            c: "inner".to_owned(),
            d: Vec::new(),
        }],
    };
    println!("{}", serde_json::to_string(&value)?);
    Ok(())
}
```

```text:output.txt
{"a":3,"b":2,"c":"a","d":[{"a":0,"b":-1,"c":"inner","d":[]}]}
```

ここで以下の事実に注目しましょう:

1. struct の各フィールドは JSON のマッピングとして serialize されている。
2. `Vec` の要素はシークエンスとして serialize されている。
3. 入れ子になったデータ構造は、入れ子になったデータとして serialize される。
 - この例では、トップレベルの `d` は MyStruct 型の値を 1 個含むので、それが `{"a":0,"b":-1,"c":"inner","d":[]}` として serialize されている。

とくに目立って不自然なところはありません。

## deserialize
deserialize も同様に試せます。

https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=135b42a8315fe970b2386d15389e9197

```rust:main.rs
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyStruct {
    a: i64,
    b: i32,
    c: String,
    d: Vec<MyStruct>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = r#"{"a":3,"b":2,"c":"a","d":[[0,-1,"inner",[]]]}"#;
    println!("{:?}", serde_json::from_str::<MyStruct>(&s)?);
    Ok(())
}
```

```text:output.txt
MyStruct { a: 3, b: 2, c: "a", d: [MyStruct { a: 0, b: -1, c: "inner", d: [] }] }
```
deserialize には一点注目すべきことがあります。`[0,-1,"inner",[]]` が `MyStruct { a: 0, b: -1, c: "inner", d: [] }` に deserialize されていることです。
これは、serde_json の deserialize に見られる特有の挙動で、マッピングもシークエンスも `struct` へ deserialize することができます。(実装は[ここ](https://github.com/serde-rs/json/blob/v1.0.45/src/de.rs#L1607-L1628))

- マッピングの場合、フィールド名とキー名の対応が取られ、deserialize される。
- シークエンスの場合、`struct` の中で宣言された順番が早いフィールドから順番に deserialize される。(この場合、`a` -> `b` -> `c` -> `d` という順番)



なお他のデータフォーマットについては、以下のようになっています:

- [YAML][serde-yaml] についてはそう (実装は[ここ](https://github.com/dtolnay/serde-yaml/blob/0.8.11/src/de.rs#L922-L923))
- [TOML][toml-rs] はそうではない (実装は[ここ](https://github.com/alexcrichton/toml-rs/blob/0.5.6/src/de.rs#L303))

ここでの話はただの一例ですが、一般に deserialize の方は、ユーザ入力を扱うという性質上、あらかじめデータの中身について保証をしにくく、色々な場合に対応する必要があり実装が複雑になる傾向があります。

# まとめ

- Serde はジェネリクスにより、仮に手書きで書いたと仮定した場合の実装と同等の速度を出すフレームワークを実現している。
- `#[derive(Serialize)]` や `#[derive(Deserialize)]` を使うことで、任意の型に対して serialize や deserialize の処理を自動で導出することができる。
- 一般的に serialize の方が deserialize より簡単。これは、考えるべきことが少ないため。
- JSON への serialize、JSON からの deserialize は直感的。

[rust-play]: play.rust-lang.org
[serde-rs]: https://serde.rs/
[serde-json]: https://github.com/serde-rs/json
[serde-yaml]: https://github.com/dtolnay/serde-yaml
[toml-rs]: https://github.com/alexcrichton/toml-rs
