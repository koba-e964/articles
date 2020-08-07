# 概要

serde の内部構造を解説するシリーズの第2回です。今回は serialize に焦点を当てて、解説します。
かなり長くなりましたが、コードは多めなので、楽しんで読んでいただけると思います。

[1. serde の基本](https://qiita.com/kobae964/items/e5f2f979af876712a23b)
**2. serialize を読む**
3. deserialize を読む 1 (予定)
4. deserialize を読む 2 (予定)
5. deserialize を読む 3 (予定)
6. formats (予定)

# まとめ

- Rust の強みである「単相化・インライン化を使ったゼロコスト抽象化」により、パフォーマンスを犠牲にすることなく serialize の実装を行うことができる。

# serializer プロトタイピング: ゼロコスト抽象化編
## 他の言語の serializer と比較した時の serde の serializer
serialize は、実はさほど難しい処理ではありません。
直感的には、例えば動的型付き言語の場合には、以下のように単に再帰を行うだけでよさそうです。(以下は JSON への変換を行う処理の擬似コード)

```javascript

function marshal(object) {
  if (object is array) {
    return marshal_array(object);
  }
  if (object is map) {
    return marshal_map(object);
  }
  if (object is int) {
    return int_to_string(object);
  }
  ...
}
function marshal_array(array) {
  len = length(array);
  s = "[";
  for (i = 0; i < len; i++) {
    s += marshal(array[i]);
    if (i + 1 < len) s += ",";
  }
  s += "]";
  return s;
}
...
```
実際この方法でも serialize はできますし、何らかのリフレクションが使えるのであれば、おそらく楽です。Go ではこのような実装がされています (https://github.com/golang/go/blob/go1.13.7/src/encoding/json/encode.go#L392-L437) が、Rust は違うアプローチを取り、静的型付き言語の強みを活かしています。

以下では、Rust で serialize ライブラリを作ろうとしたらどうなるか、の[プロトタイピング](https://ja.wikipedia.org/wiki/%E3%83%97%E3%83%AD%E3%83%88%E3%82%BF%E3%82%A4%E3%83%94%E3%83%B3%E3%82%B0)をしたいと思います。つまり、ごく一部の例について、「ゼロコスト抽象化」という基本原理を元に serialize 処理を汎用化していって、どういうことになるかを見ていきたいと思います。`serde` にはところどころ触れながらも、あくまで必要になったことだけをやります。また、あとで `serde` の実際の設計と照らし合わせて答え合わせをします。

## serialize ライブラリ、自作編

例えば以下の構造体 `A` を (インデントなしの) JSON に serialize するというタスクを考えます。このタスクのことだけを考えるのであれば、以下のような `serialize_A_to_json` という関数を作るのが最善です。[^trait_write]

[^trait_write]: [serde-json] では Write を実装する型に対して書き込むという形式で serialize を実装しているため、今回もそれに倣いました。[playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=64d22845c523bbb12713cad879f10886)

```rust
use std::io::{Result, Write};

struct A(i64, i32);

#[allow(non_snake_case)]
fn serialize_A_to_json<W: Write>(a: A, mut writer: W) -> Result<()> {
    let A(x, y) = a;
    write!(writer, "[{},{}]", x, y)?;
    Ok(())
}
```
この関数を使えば、構造体 `A` については JSON に serialize することができます。しかし、ここで2つの問題が現れます:

- 他の型 (構造体、列挙型) についてはどうやって serialize するのか?
- JSON 以外のフォーマットにはどうやって serialize するのか?

前者については、他の型についても同じように実装すればいいです。特に何か考えることはありません。[playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=14355c202ca2bf97e067a3986a28cbef)

```rust

use std::io::{Result, Write};

struct A2(i64, i32, i16);

#[allow(non_snake_case)]
fn serialize_A2_to_json<W: Write>(a: A2, mut writer: W) -> Result<()> {
    let A2(x, y, z) = a;
    write!(writer, "[{},{},{}]", x, y, z)?;
    Ok(())
}
```

後者について、例えば YAML へ serialize するコードは以下のようになるかと思われます。[^sorry-serde-yaml-creates-term][playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=991c8afd389b402005c6d1713af989b5)

[^sorry-serde-yaml-creates-term]: 実は [serde_yaml] は `Write` を受け取る設計ではなく、YAML 用の型を用意して、その型の値を吐き出す設計になっています。YAML ファイルを作るところは [yaml-rust] という別のライブラリに任せています。

```rust
use std::io::{Result, Write};

struct A(i64, i32);

#[allow(non_snake_case)]
fn serialize_A_to_yaml<W: Write>(a: A, mut writer: W) -> Result<()> {
    let A(x, y) = a;
    write!(writer, "- {}\n- {}", x, y)?;
    Ok(())
}
```

しかし、これを、「全ての型」と「すべてのデータフォーマット」について列挙するのはあまりにも苦痛です。というのは、データ型の個数を `N` とし、データフォーマットの個数を `M` としたとき、 `NM` 個の関数を定義しなければならないからです。せめて、それぞれの型について1回、それぞれのデータフォーマットについて1回だけ何かを定義すればいいようにしたいです。つまり `N+M` 回何かを定義すれば良いようにしたいです。

ここで、serde は Rust らしい解決法をとっています。つまり、

- トレイトによるジェネリクスを使った、ゼロコスト抽象化 (単相化、インライン化に依存する)
- マクロを用いてボイラープレートを自動生成

です。これを模倣していきたいと思います。
### トレイトによるジェネリクスを使った、ゼロコスト抽象化
上の構造体 `A` の serialize を使って説明します。`A` のような、シークエンスとして serialize されるものを抽象化するために、以下のようなヘルパートレイト `ToySerializer` を用意することにしましょう。これは `serialize_int_seq` を実装します。[playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c3c8aeb18d6350119d8e410e90b6c3a6)

```rust
use std::io::{Result, Write};

// 整数の配列を serialize する能力のあるオブジェクト
trait ToySerializer {
    // 整数の配列を serialize する関数
    fn serialize_int_seq<W: Write>(self, a: impl IntoIterator<Item = i64>, writer: W) -> Result<()>;
}

struct ToJson;

impl ToySerializer for ToJson {
    fn serialize_int_seq<W: Write>(self, a: impl IntoIterator<Item = i64>, mut writer: W) -> Result<()> {
        write!(writer, "[")?;
        let mut count = 0;
        for element in a {
            if count >= 1 {
                write!(writer, ",")?;
            }
            write!(writer, "{}", element)?;
            count += 1;
        }
        write!(writer, "]")?;
        Ok(())
    }
}

struct A(i64, i32);

#[allow(non_snake_case)]
fn serialize_A_to_json<W: Write>(a: A, writer: W) -> Result<()> {
    let int_serializer = ToJson;
    let A(x, y) = a;
    let seq = vec![x, y as i64];
    int_serializer.serialize_int_seq(seq, writer)?;
    Ok(())
}
```

この方法により、例えば YAML へと serialize するときは、YAML 向けの `ToySerializer` を実装し、`serialize_A_to_json` 内部で `ToJSON` の代わりにそれを使えばいいことになります。[playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a3bc37b54ecdb97fefc6fb35b5eeb357)

```rust
use std::io::{Result, Write};

trait ToySerializer {
    fn serialize_int_seq<W: Write>(self, a: impl IntoIterator<Item = i64>, writer: W) -> Result<()>;
}

struct ToYaml;

impl ToySerializer for ToYaml {
    fn serialize_int_seq<W: Write>(self, a: impl IntoIterator<Item = i64>, mut writer: W) -> Result<()> {
        for element in a {
            write!(writer, "- {}\n", element)?;
        }
        Ok(())
    }
}

struct A(i64, i32);

#[allow(non_snake_case)]
fn serialize_A_to_yaml<W: Write>(a: A, writer: W) -> Result<()> {
    let int_serializer = ToYaml;
    let A(x, y) = a;
    let seq = vec![x, y as i64];
    int_serializer.serialize_int_seq(seq, writer)?;
    Ok(())
}
```

`A` に関する実装を少なくするために、 `ToySerializer` を外部から受け取る方式にしてもよいでしょう。[playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9fd86a41f61458c18e594ffd4e06268d)

```rust
use std::io::{Result, Write};

trait ToySerializer {
    fn serialize_int_seq<W: Write>(self, a: impl IntoIterator<Item = i64>, writer: W) -> Result<()>;
}

// JSON 特有の処理。serialize されるデータ型は関係ない。
struct ToJson;
impl ToySerializer for ToJson {
    fn serialize_int_seq<W: Write>(self, a: impl IntoIterator<Item = i64>, mut writer: W) -> Result<()> {
        write!(writer, "[")?;
        let mut count = 0;
        for element in a {
            if count >= 1 {
                write!(writer, ",")?;
            }
            write!(writer, "{}", element)?;
            count += 1;
        }
        write!(writer, "]")?;
        Ok(())
    }
}

// YAML 特有の処理。serialize されるデータ型は関係ない。
struct ToYaml;
impl ToySerializer for ToYaml {
    fn serialize_int_seq<W: Write>(self, a: impl IntoIterator<Item = i64>, mut writer: W) -> Result<()> {
        for element in a {
            write!(writer, "- {}\n", element)?;
        }
        Ok(())
    }
}

struct A(i64, i32);

#[allow(non_snake_case)]
fn serialize_A<S: ToySerializer, W: Write>(a: A, int_serializer: S, writer: W) -> Result<()> {
    let A(x, y) = a;
    let seq = vec![x, y as i64];
    int_serializer.serialize_int_seq(seq, writer)?;
    Ok(())
}


fn main() {
    serialize_A(A(4,3), ToJson, std::io::stdout()).unwrap(); // `[4,3]` が出力される
    println!();
    serialize_A(A(4,3), ToYaml, std::io::stdout()).unwrap(); // `- 4\n- 3\n` が出力される
}
```

ここまでくると、本物の serde の設計にかなり近くなります。ただ、まだ不満点が残ります。

- serialize_A で `Vec` を作っているが、元の `serialize_A_to_json` にはそんなものはなかった。
- int だけ特別扱いせずに、serialize できるものをまとめたトレイトが欲しい。(仮にそのトレイトが `ToySerialize` という名前だとする。)
- int の配列だけでなく `ToySerialize` の配列を扱おうとすると、引数として `<T: ToySerialize> impl IntoIterator<Item = T>` のようなものを持つ必要が出てくる。しかし、要素の型が全部同じとは限らないので、それはできない。~~よって、`impl Iterator<Item = Box<dyn ToySerialize>>` みたいな型を受け取るより他ないと思われるが、これだと box 化による実行時オーバヘッドのせいで多少重くなる。~~ ToySerialize は [object safe](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects) なトレイトではない (serialize を行うメソッドが必然的に型パラメータ `S: ToySerializer` を持つ) ため、このような方法はそもそも使えない。(2020-08-07 追記)

#### serialize_A で Vec を作りたくない
これについては、`ジェネリックなシークエンスを扱うためにどうする?` でまとめて扱います。
#### int だけ特別扱いせずに、serialize できるものをまとめたトレイトが欲しい
これは、`serialize_A` のシグニチャを真似て、以下のようなトレイトを定義すればいいでしょう。

```rust
trait ToySerialize {
    fn serialize<S: ToySerializer, W: Write>(self, serializer: S, writer: W) -> Result<()>;
}
```
#### ジェネリックなシークエンスを扱うためにどうする?
上の実装では int のシークエンスに決め打ちしてメソッドを定義していました。実際の利用時には、 serialize できる全ての型について serialize できるようにしたいでしょう。単純に思いつくのは、以下のようなメソッドを定義することです。

```rust
fn serialize_seq<T: ToySerialize, W: Write>(self, a: impl IntoIterator<Item = T>, writer: W) -> Result<()>;
```
しかし、これでは要素の型がすべて同じではない配列 (`[2, "a", {}]` など) の serialize に失敗してしまいます。上の構造体の例でいうと、フィールドの型が同じである保証はどこにもないので、`struct A(i64, String)` のような型で使うことはできません。ちなみに、こうした要素の型がすべて同じではない配列のことを、<b>ヘテロジーニアス</b>な配列と呼びます。対義語は<b>ホモジーニアス</b>です。

次に思いつくのは、`Box` で包むことでしょう。

```rust
fn serialize_seq<W: Write>(self, a: impl IntoIterator<Item = Box<dyn ToySerialize>>, writer: W) -> Result<()>;
```
~~これはヘテロジーニアスな配列を問題なく受け取れますが、`Box` で包んでいるために動的ディスパッチが発生し、遅いという問題があります。元の `serailize_A_to_json` にはこのようなオーバヘッドはなかったはずです。~~ ToySerialize は [object safe](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects) なトレイトではない (`serialize` が型パラメータ `S: ToySerializer, W: Write` を持つ) ため、このような方法はそもそも使えません。(2020-08-07 追記)

そこでどうするか? この問題は serializer 側でリストの要素の処理をさせようとしたことによって発生したので、serialize 側でリストの要素の処理をさせればよいのです!

serialize 側でリストの要素の処理をさせるために、考えるべきことは以下の 2 個です。

1. serialize 側は、要素を `Vec` に包んで渡す代わりに、要素を渡して"消費"してもらえるオブジェクトを受け取り、そのオブジェクトに要素を次々に渡していく。要素を消費するメソッドは `ToySerialize` を実装している任意のオブジェクトを受け取れないといけないので、必然的に `fn name<T: ToySerialize>(self, element: T, ...)` というシグニチャを持つことになる。この要請を `ToySeqSerializer` というトレイトで表すことにし、要素の消費をするメソッドの名前を `receive_an_element` とする。
2. JSON の出力を見ればわかるように、データフォーマットの種類によっては、要素の消費の開始時と終了時に特別な処理をする必要があることもある。開始時はどちらにせよ `ToySeqSerializer` を返す必要があるのでついでにやればよいが、終了時には明示的に処理を行う必要がある。これを `ToySeqSerializer` の `finish_receiving` というメソッドにやらせることにする。

また、`i32`, `i64`, `A` に `ToySerialize` を実装します。`i32` と `i64` は今後 `ToySeqSerializer` で使うため、 `A` には実装する必要はないが、できるならした方が扱いやすいためです。

以上の工夫を実装したコードは以下のようになります。[playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6cdd1898e8d7245e20113ba26f64f8cb)

```rust
use std::io::{Result, Write};

trait ToySerializer {
    type Seq: ToySeqSerializer;
    fn serialize_seq<W: Write>(self, writer: W) -> Result<Self::Seq>;
}

// 要素を渡すと serialize してくれる。要素を渡し終わったら finish_receiving を呼ぶ。
trait ToySeqSerializer {
    fn receive_an_element<T: ToySerialize, W: Write>(&mut self, element: T, writer: W) -> Result<()>;
    fn finish_receiving<W: Write>(self, writer: W) -> Result<()>;
}

trait ToySerialize {
    fn serialize<S: ToySerializer, W: Write>(self, serializer: S, writer: W) -> Result<()>;
}

impl ToySerialize for i64 {
    fn serialize<S: ToySerializer, W: Write>(self, _serializer: S, mut writer: W) -> Result<()> {
        write!(writer, "{}", self)
    }
}

impl ToySerialize for i32 {
    fn serialize<S: ToySerializer, W: Write>(self, _serializer: S, mut writer: W) -> Result<()> {
        write!(writer, "{}", self)
    }
}

// JSON 特有の処理。serialize されるデータ型は関係ない。
struct ToJson;
struct SeqToJson { first: bool }
impl ToySerializer for ToJson {
    type Seq = SeqToJson;
    fn serialize_seq<W: Write>(self, mut writer: W) -> Result<Self::Seq> {
        write!(writer, "[")?;
        Ok(SeqToJson { first: true })
    }
}
impl ToySeqSerializer for SeqToJson {
    fn receive_an_element<T: ToySerialize, W: Write>(&mut self, element: T, mut writer: W) -> Result<()> {
        if !self.first {
            write!(writer, ",")?;
        }
        element.serialize(ToJson, writer)?;
        self.first = false;
        Ok(())
    }
    fn finish_receiving<W: Write>(self, mut writer: W) -> Result<()> {
        write!(writer, "]")
    }
}

// YAML 特有の処理。serialize されるデータ型は関係ない。
struct ToYaml;
struct SeqToYaml;
impl ToySerializer for ToYaml {
    type Seq = SeqToYaml;
    fn serialize_seq<W: Write>(self, _writer: W) -> Result<Self::Seq> {
        Ok(SeqToYaml)
    }
}
impl ToySeqSerializer for SeqToYaml {
    fn receive_an_element<T: ToySerialize, W: Write>(&mut self, element: T, mut writer: W) -> Result<()> {
        write!(writer, "- ")?;
        element.serialize(ToYaml, &mut writer)?;
        write!(writer, "\n")?;
        Ok(())
    }
    fn finish_receiving<W: Write>(self, _writer: W) -> Result<()> {
        Ok(())
    }
    
}

// データ型固有の処理。
struct A(i64, i32);
impl ToySerialize for A {
    fn serialize<S: ToySerializer, W: Write>(self, serializer: S, mut writer: W) -> Result<()> {
        let A(x, y) = self;
        let mut accessor = serializer.serialize_seq(&mut writer)?;
        accessor.receive_an_element(x, &mut writer)?;
        accessor.receive_an_element(y, &mut writer)?;
        accessor.finish_receiving(&mut writer)?;
        Ok(())
    }
}

// 実際の使い方
fn main() {
    A::serialize(A(4,3), ToJson, std::io::stdout()).unwrap(); // `[4,3]` が出力される
    println!();
    A::serialize(A(4,3), ToYaml, std::io::stdout()).unwrap(); // `- 4\n- 3\n` が出力される
}
```

#### serialize 側に Write いる?
いらないです。全部 serializer に押し付けましょう。やるべきことは以下の4つです。

- `ToySerialize`, `ToySerializer`, `ToySeqSerializer` すべてから `Write` への言及を消す。そのかわり、 `ToJson` と `ToYaml` に `Write` のインスタンスを持たせる。
- `Write` への言及が消えたことで、エラーが `std::io::Error` だけとは限らなくなった。このため、`Serializer` に `Error` 型を定義し、エラー時にはそれを返すことにする。
- 「`Write` に書き出す」以外の処理をさせたくなるかもしれない。そのために、成功時に任意の値を返せるようにする。そのため、`Serializer` に `Ok` 型を定義し、成功時にはそれを返すことにする。
- `i64`, `i32` の `ToySerialize` 実装が `Write` を参照できなくなったので、serializer に serialize してもらうしかなくなった。このため、`ToySerializer` に `serialize_i64`, `serialize_i32` というメソッドを追加する。とはいえ、整数型を serialize する方法を知っているのは serializer だけなので、これはあるべき姿と言える。

以上を実装すると以下のようになります。だいぶ複雑になりましたね。[playground へのリンク](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=55be023055f185bb065fd03da87b8c9e)

```rust
use std::io::Write;

trait ToySerializer {
    type Ok;
    type Error;
    // ToySeqSerializer でも同じエラー型を使うことを強制する。呼び出し側のエラー処理を楽にするため。
    type Seq: ToySeqSerializer<Ok = Self::Ok, Error = Self::Error>;
    fn serialize_seq(self) -> Result<Self::Seq, Self::Error>;
    fn serialize_i64(self, x: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(self, x: i32) -> Result<Self::Ok, Self::Error>;
}

// 要素を渡すと serialize してくれる。要素を渡し終わったら finish_receiving を呼ぶ。
trait ToySeqSerializer {
    type Ok;
    type Error;
    fn receive_an_element<T: ToySerialize>(&mut self, element: T) -> Result<(), Self::Error>;
    fn finish_receiving(self) -> Result<Self::Ok, Self::Error>;
}

trait ToySerialize {
    fn serialize<S: ToySerializer>(self, serializer: S) -> Result<<S as ToySerializer>::Ok, <S as ToySerializer>::Error>;
}

impl ToySerialize for i64 {
    fn serialize<S: ToySerializer>(self, serializer: S) -> Result<<S as ToySerializer>::Ok, <S as ToySerializer>::Error> {
        serializer.serialize_i64(self)
    }
}

impl ToySerialize for i32 {
    fn serialize<S: ToySerializer>(self, serializer: S) -> Result<<S as ToySerializer>::Ok, <S as ToySerializer>::Error> {
        serializer.serialize_i32(self)
    }
}

// JSON 特有の処理。serialize されるデータ型は関係ない。
struct ToJson<W> { writer: W }
struct SeqToJson<W> { first: bool, writer: W }
impl<W: Write> ToySerializer for ToJson<W> {
    type Ok = ();
    type Error = std::io::Error;
    type Seq = SeqToJson<W>;
    fn serialize_seq(mut self) -> Result<Self::Seq, std::io::Error> {
        write!(self.writer, "[")?;
        Ok(SeqToJson { first: true, writer: self.writer })
    }
    fn serialize_i64(mut self, x: i64) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", x)
    }
    fn serialize_i32(mut self, x: i32) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", x)
    }
}
impl<W: Write> ToySeqSerializer for SeqToJson<W> {
    type Ok = ();
    type Error = std::io::Error;
    fn receive_an_element<T: ToySerialize>(&mut self, element: T) -> Result<(), std::io::Error> {
        if !self.first {
            write!(self.writer, ",")?;
        }
        element.serialize(ToJson { writer: &mut self.writer})?;
        self.first = false;
        Ok(())
    }
    fn finish_receiving(mut self) -> Result<(), std::io::Error> {
        write!(self.writer, "]")
    }
}

// YAML 特有の処理。serialize されるデータ型は関係ない。
struct ToYaml<W> { writer: W }
struct SeqToYaml<W> { writer: W }
impl<W: Write> ToySerializer for ToYaml<W> {
    type Ok = ();
    type Error = std::io::Error;
    type Seq = SeqToYaml<W>;
    fn serialize_seq(self) -> Result<Self::Seq, std::io::Error> {
        Ok(SeqToYaml { writer: self.writer })
    }
    fn serialize_i64(mut self, x: i64) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", x)
    }
    fn serialize_i32(mut self, x: i32) -> Result<(), std::io::Error> {
        write!(self.writer, "{}", x)
    }
}
impl<W: Write> ToySeqSerializer for SeqToYaml<W> {
    type Ok = ();
    type Error = std::io::Error;
    fn receive_an_element<T: ToySerialize>(&mut self, element: T) -> Result<(), std::io::Error> {
        write!(self.writer, "- ")?;
        element.serialize(ToYaml { writer: &mut self.writer })?;
        write!(self.writer, "\n")?;
        Ok(())
    }
    fn finish_receiving(self) -> Result<(), std::io::Error> {
        Ok(())
    }
    
}

// データ型固有の処理。
struct A(i64, i32);
impl ToySerialize for A {
    fn serialize<S: ToySerializer>(self, serializer: S) -> Result<<S as ToySerializer>::Ok, <S as ToySerializer>::Error> {
        let A(x, y) = self;
        let mut accessor = serializer.serialize_seq()?;
        accessor.receive_an_element(x)?;
        accessor.receive_an_element(y)?;
        let result = accessor.finish_receiving()?;
        Ok(result)
    }
}

// 実際の使い方
fn main() {
    A::serialize(A(4,3), ToJson { writer: std::io::stdout() }).unwrap(); // `[4,3]` が出力される
    println!();
    A::serialize(A(4,3), ToYaml { writer: std::io::stdout() }).unwrap(); // `- 4\n- 3\n` が出力される
}
```

お疲れ様でした! これでほとんど余計なことをしなくなり、オーバーヘッドがなくなりました。また、 serialize の結果を `Write` への書き込みだけでなく、他のことにすることも可能になりました。(例えば、受け取ったタームを [`serde_json::Value`](https://docs.serde.rs/serde_json/enum.Value.html) のような項に変換するなど。)

### マクロを用いてボイラープレートを自動生成
Rust には procedural macro という機能があり、コンパイラがパースした構文木に対してかなり広範な処理を行うことができます。自作 derive マクロもその一つです。
上の `A` に対する `ToySerialize` 実装はほとんど典型的な処理しか行なっていないので、実装を自動で生成できたら楽です。実際に自動でできます。


# serde の基本・答えあわせ
上で苦労して導いた実装が実際に合理的であることを見ていきましょう。

実際の `serde` では、データ型側は `Serialize` を実装して、データフォーマット側は `Serializer` を実装するのでした。この2者が協働してデータの serialize を行うというのは第1回で説明した通りです。
## `Serialize`
`Serialize` を実装するためには、以下のメソッドを実装する必要があります:

```rust
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
```
このメソッドで受け取った `serializer` に対して、自分のフィールドをこれこれこういう順番で、こういうフォーマットで serialize してほしい、という要求を出すわけです。要求の出し方は、`Serializer` に定義されている `serialize_xxx` という名前のメソッドを呼ぶことです。`Serializer` には以下の30個[^isnt-it-29]のメソッドが提供されています ([ser/mod.rs](https://github.com/serde-rs/serde/blob/v1.0.104/serde/src/ser/mod.rs#L330-L1452))[^wheres-data-model]:

[^isnt-it-29]: `serialize_none` と `serialize_some` が同じ `Option` 型という扱いなので、データ型は 29 個です。
[^wheres-data-model]: serde のデータモデルは、公式には explicit なデータ型として提供されていません。 ([serde-value](https://github.com/arcnmx/serde-value/blob/0.6.0/src/lib.rs#L23-L49) というクレートにはありますが、例えば `serialize_unit_struct` に対応する enum variant がないなど、微妙に差異があります。)

```rust:ser/mod.rs
// コメントなど省略
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize;
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>;
```
とはいえこれらは `Self::Ok` を返すか、上で扱ったような 「`Serialize` を受け取って何かをするオブジェクト」を返すかのどちらかなので、取り立てて目新しいことはないでしょう。
ちなみに、`serialize_seq` が返す `SerializeSeq` は以下のようになっています ([ser/mod.rs](https://github.com/serde-rs/serde/blob/v1.0.104/serde/src/ser/mod.rs#L1502-L1516)):

```rust:mod.rs
pub trait SerializeSeq {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok;

    /// Must match the `Error` type of our `Serializer`.
    type Error: Error;

    /// Serialize a sequence element.
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize;

    /// Finish serializing a sequence.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
```
ほとんど上の `ToySeqSerializer` と同じです。このことからも、ゼロコスト抽象化を推し進めると必然的にこのような設計になるであろうということがわかります。

## データ型 <=> 呼ばれる関数 の対応

|Rust での表現 | データ型の例 | serialize 時に呼ばれる関数 | JSON での表現例 | `Serialize` の実装場所 |
|---|---|---|---|---|
| 整数型 | i64 | serialize_i64 | `1` | [ser/impls.rs @ serde](https://github.com/serde-rs/serde/blob/v1.0.104/serde/src/ser/impls.rs#L7-L19) |
| newtype struct | `struct A(i64)` | serialize_newtype_struct | `1`| [ser.rs @ serde_derive](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L221-L245) |
| tuple struct | `struct A(i64, i32, String)` | serialize_tuple_struct | `[1, 2, "a"]`| [ser.rs @ serde_derive](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L247-L284) |
| named struct | `struct A { x: i64, y: i32, z: String }` | serialize_struct | `{"x": 1, "y": 2, "z": "a"}`| [ser.rs @ serde_derive](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L286-L294)
| enum variant (newtype struct) | `enum A { V(i64) }` | serialize_newtype_variant | `{"V": 0}` | [ser.rs @ serde_derive](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L520-L538)
| enum variant (tuple struct) | `enum A { V(i64, i32, String) }` | serialize_newtype_variant | `{"V": [1, 2, "a"]}` | [ser.rs @ serde_derive](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L539-L547)
| enum variant (struct) | `enum A { V { x: i64, y: i32, z: String }}` | serialize_newtype_variant | `{"V": {"x": 1, "y": 2, "z": "a"}}` | [ser.rs @ serde_derive](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L548-L556)

## `serde-derive` で derive される `Serialize` 実装の例

```rust
struct B<T>(i64, i32, String, T);
```
に対して、`#[derive(Serialize)]` をかけてみた結果です。
`cargo-expand` で展開した結果なので、正確とは限りません。雰囲気だけ楽しんでください。
また、`try!` マクロまで展開されていて読みにくいです。
なお、完全なコードは https://github.com/koba-e964/cargo-serde-test/tree/a7cc2c2130a36630ba592c9889bbf12922c942c7 にあります。

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use serde::Serialize;

struct B<T>(i64, i32, String, T);
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_B: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[allow(unused_macros)]
    macro_rules! try {
        ($ __expr : expr) => {
            match $__expr {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            }
        };
    }
    #[automatically_derived]
    impl<T> _serde::Serialize for B<T>
    where
        T: _serde::Serialize,
    {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_tuple_struct(
                __serializer,
                "B",
                0 + 1 + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.0) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.1) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.2) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.3) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeTupleStruct::end(__serde_state)
        }
    }
};

fn main() {}
```

一つずつ見ていきましょう。

### `_IMPL_SERIALIZE_FOR_B`
```rust
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_B: () = { ... }
```
[生成している箇所](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/dummy.rs#L14-L17)
これは `B` についての自動生成された実装を隠蔽するためのブロックです。
この隠蔽は [serde-rs#159](https://github.com/serde-rs/serde/issues/159) で導入されました。`extern crate serde;` がトップレベル以外の場所に書かれていても良いようにするための変更ですが、ここでは触れません。

### `try!` マクロの自前定義

```rust
    #[allow(unused_macros)]
    macro_rules! try {
        ($ __expr : expr) => {
            match $__expr {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            }
        };
    }
```
[生成している箇所](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/try.rs)
コメントを読むとわかるように、この `try!` マクロはコンパイル時間の短縮のための工夫です。`into` を挟まないことで型推論の手間が少なくなり、コンパイルが速くなります。

### トレイト境界
```rust
    #[automatically_derived]
    impl<T> _serde::Serialize for B<T>
    where
        T: _serde::Serialize,
```
[生成している箇所](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/bound.rs#L83-L217)
`#[automatically_derived]` という属性と `T: Serialize` という境界に目が行くと思います。順番に見ていきましょう。

`#[automatically_derived]` という属性がついている要素は、未使用でも警告が出ません。[参考](https://stackoverflow.com/questions/51481551/what-does-automatically-derived-mean)
`T: Serialize` という境界は [bound.rs @ serde_derive](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/bound.rs#L83-L217) で生成されます。`bound::with_bound` は、構造体の型パラメータのうち、実際に serialize されるもの (つまり、 `#[skip_serialize]` がついていないもの) すべてを見つけて、それらに `Serialize` を要請する、という処理を行います。この場合、フィールドに `T` があり、serialize されるので、`T: Serialize` という境界が自動で付加されます。

### begin
```rust
            let mut __serde_state = match _serde::Serializer::serialize_tuple_struct(
                __serializer,
                "B",
                0 + 1 + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
```
[生成している箇所](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L791-L848)

[`Serializer::serialize_tuple_struct`](https://github.com/serde-rs/serde/blob/v1.0.104/serde/src/ser/mod.rs#L1070-L1074) を呼んで、serialize を開始する処理が書かれています。

`Serializer::serialize_tuple_struct` のシグニチャは以下のようになっています:

```rust
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
```
今回の場合は `name = "B", len = 4` です。`name` が必要なのはそうだろうという気がします (構造体の名前 serialize したい場合があるはずなので) ですが、長さを serialize する必要があるというのは謎です。実は、[serde-json] では[長さ 0 の場合とそれ以外の場合を場合分けしています](https://github.com/serde-rs/json/blob/v1.0.44/src/ser.rs#L326-L338)。長さ 0 の場合は単に `[]` を出力して終わり、長さが 1 以上の場合は `[` だけを出力して残りは別の状態 `Compound::Map` に委ねています。
(追記: 例えば msgpack では長さを serialize しています。 ([https://github.com/3Hren/msgpack-rust/blob/84fb34b57b6df35813779cd54735240ba1115558/rmp-serde/src/encode.rs#L531-L537](https://github.com/3Hren/msgpack-rust/blob/84fb34b57b6df35813779cd54735240ba1115558/rmp-serde/src/encode.rs#L531-L537)))

### serialize_field

```rust
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.0) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.1) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.2) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.3) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
```
[生成している箇所](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L1043-L1064)
フィールドを1個ずつ調べて、serialize_field しています。[serde-json] では tuple_struct はシークエンスへと serialize されます。

### end
```rust

            _serde::ser::SerializeTupleStruct::end(__serde_state)
```
[生成している箇所](https://github.com/serde-rs/serde/blob/v1.0.104/serde_derive/src/ser.rs#L282)
始めたものは終えないといけません。これが最後に呼ばれる関数なので、`Err` の場合に early return をする必要はありません。

# まとめ

- Rust の強みである「単相化・インライン化を使ったゼロコスト抽象化」により、パフォーマンスを犠牲にすることなく serialize の実装を行うことができる。

# 今後の構成予定

今後は deserialize を解説していきたいと思っています。Deserialize を理解するには以下の物事を理解しなくてはならないため、少なくとも 3 倍は難しいです。

- `Visitor` パターン
- in-place deserialization
- deserializer 本体

[serde-json]: https://docs.serde.rs/serde_json/
[serde_yaml]: https://docs.serde.rs/serde_yaml/
[yaml-rust]: https://github.com/chyh1990/yaml-rust
