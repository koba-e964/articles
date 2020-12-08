
## 概要
競プロの作問ヘルパーツール [creo](https://github.com/koba-e964/creo) を作りました。

## 使い方

最初に creo をインストールします。

```bash
git clone https://github.com/koba-e964/creo
cd creo
cargo install --path .
```

続いて問題用のディレクトリを作ります。

```bash
creo init problem-directory
```

これで `problem-directory` にファイル・ディレクトリ一式ができます。

```console
$ cd problem-directory/
$ ls -l
total 8
-rw-r--r--  1 koba_mac  xxx  315 11 26 01:38 creo.toml
drwxr-xr-x  2 koba_mac  xxx   64 11 26 01:38 etc
drwxr-xr-x  2 koba_mac  xxx   64 11 26 01:38 in
drwxr-xr-x  2 koba_mac  xxx   64 11 26 01:38 out
drwxr-xr-x  2 koba_mac  xxx   64 11 26 01:38 sol
drwxr-xr-x  2 koba_mac  xxx   64 11 26 01:38 task
```

`creo.toml` が唯一の設定ファイルです。リポジトリの sample_aplusb を参考に、`creo.toml` に然るべき行を追加したり、然るべきディレクトリにファイルを追加することで、動かすことができます。 (ドキュメントに書かれている `creo add ...` は現状では動きません!)

## 設計
### 言語・使用ライブラリ
開発言語は Rust でした。コマンドラインツールを書くことができ、ある程度の規模のコードを書くことができる言語として Go や Rust など様々な言語がありますが、以下の理由で Rust を選びました。[^i-prefer-rust]
[^i-prefer-rust]: Go と Rust のどちらも妥当な選択だったと思います。
- 個人的に慣れ親しんでいる言語であった
- Rust で DI を試したかった
- Go での実装がすでにあり (https://github.com/camypaper/spica) 、それとの差別化を図りたかった (記事: [作問ヘルパーツールvirgo,spica](https://qiita.com/Camypaper/items/5b5683a96d013b99d2c3))

また、ライブラリとしては以下を使っています。
- `clap`: CLI アプリケーションのコマンドライン引数を解析する機能を提供します。
- `serde`: データ構造の直列化・脱直列化機能を提供します。定番です。設定ファイルの読み込みのために使っています。
- `thiserror`: エラー型の変換を簡単に記述できるマクロを提供します。Rust のエラーは、各リポジトリごとに固有のエラー型を定義して、依存ライブラリのエラー型を自分のライブラリのエラー型に変換する、という設計になっているリポジトリが多いです。それが簡単に書けるようになります。



### 設計思想
以下が達成したかったことです。

- ツールを利用しない人たちとの interoperability
  - ツールをインストールしなくても、テストを自動で実行できる
  - ツールを使わない人のために、設定ファイルを 1 個だけにとどめる
- 単純性
    - 扱える問題は 1 設定ファイルごとに 1 個
    - 実行するとき、一時ファイルをカレントディレクトリではなく、テンポラリディレクトリに配置する
        - 問題用のファイルは Dropbox で管理されることを想定しており、Dropbox に余計なファイルを認識させたくないため

### DI (Dependency Injection)
DI については以下を非常に参考にしました。 https://ryym.tokyo/posts/rust-di/
かいつまんで要点を説明します。

creo リポジトリの中の `Project` というコンポーネントを例にして説明します。`Project` は `IoUtil` および `RunUtil` に依存しています。

(1) Interface trait を定義し、必要なメソッドのデフォルト実装をすべてダミーとして実装する。 
  https://github.com/koba-e964/creo/blob/51333da5f804f5c2178b2f572d9c618725032fbf/src/entity/project.rs#L10-L69 
  
```rust
pub trait Project {
    fn check(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
    // ...その他のメソッド
}
```
(2) `Project` を継承したトレイト `ProjectExt` を定義し、依存するコンポーネントの interface trait を継承する。これが implementation trait となる。
   https://github.com/koba-e964/creo/blob/51333da5f804f5c2178b2f572d9c618725032fbf/src/entity/project.rs#L39-L69

```rust
pub trait ProjectExt: IoUtil + RunUtil {
    fn read_config(&mut self, proj: &Path) -> Result<CreoConfig> { /* */ }
    // ...その他関数
}
```
(3) interface trait の各メソッドの実装を、`ProjectExt` のデフォルト実装として与える。つまり、`ProjectExt` を実装する型は無条件で `Project` の本実装が手に入る。
https://github.com/koba-e964/creo/blob/51333da5f804f5c2178b2f572d9c618725032fbf/src/entity/project.rs#L71-L285

```rust
impl<T: ProjectExt> Project for T {
    fn check(&mut self, proj_dir: &str) -> Result<()> { /* */ }
}
```

(4) `ProjectImpl` という型を定義し、依存するトレイトに対応する implementation trait および `ProjectExt` を impl する。implementation trait は全てがデフォルト実装を持つように設計されているので、ここでは明示的に実装する必要はない。
https://github.com/koba-e964/creo/blob/51333da5f804f5c2178b2f572d9c618725032fbf/src/entity/project.rs#L287-L291

```rust
pub struct ProjectImpl;

impl IoUtilExt for ProjectImpl {}
impl RunUtilExt for ProjectImpl {}
impl ProjectExt for ProjectImpl {}
```
(5) テストを書く時は、implementation trait ではないトレイト (`IoUtil` や `RunUtil`) を実装したモックオブジェクト `MockProject` を作り、それに ProjectExt を実装することでテストを行う。 
implementation trait を実装するとデフォルト実装が手に入り、interface trait を実装すると自分で好きなように実装する自由が手に入るため、`Project` については前者を、`IoUtil` や `RunUtil` については後者を得ることで、モッキングをするテストができる。
https://github.com/koba-e964/creo/blob/51333da5f804f5c2178b2f572d9c618725032fbf/src/entity/project.rs#L326-L507

これに対して、愚直にコンストラクタに依存ライブラリを渡す設計だと以下のようになります。
(1) interface trait と implementation struct を用意する。

```rust
trait Project {
    fn check(&mut self, proj_dir: &str) -> Result<()> {
        unreachable!();
    }
    // ...その他のメソッド
}

struct ProjectImpl<'a, 'b> { /* ... */ }

impl Project for ProjectImpl<'a, 'b> { /* ... / }
```

(2) 依存ライブラリのトレイトオブジェクトを作り、`ProjectImpl` のコンストラクタがそれらを受け取れるようにする

```rust
impl<'a, 'b> ProjectImpl<'a, 'b> {
    fn new(io_util: &'a dyn IoUtil, run_util: &'b dyn RunUtil) -> Self {
        /* ... */
    }
}
```

(3) main 関数内で wiring を行う。

```rust
fn main() {
    let io_util = IoUtil::new();
    let run_util = RunUtil::new(&io_util);
    let project = Project::new(&io_util, &run_util);
}
```

以上 2 通りのやり方を比較して、implementation trait を定義する方法は以下の点で、愚直なやり方に比べてよかったように思います:
1. DI を行うときに必要な wiring 処理を Rust のトレイト実装解決の仕組みに任せることができる。愚直なやり方では wiring をどこかで手動で書く必要がある。
2. アプリケーションを作る際にありがちである、「トレイトのメインの実装 (アプリケーション本体で使うもの) が 1 つあり、トレイトの他の実装は全てテストのために使われる」という状況にうまく対応できている。implementation trait のデフォルト実装が一番「えらい」実装であるため、自然に特別扱いでき、あるコンポーネントのデフォルト実装をべつのコンポーネントで利用する時も implementation trait の実装だけでよく楽である。愚直なやり方ではこのような特別扱いはされず、デフォルト実装を利用する時も自分でデフォルト実装をコンストラクタに与える必要がある。
3. `trait XXXExt: XXX` と `impl<T: XXX> XXXExt for T { /* */ }` の組み合わせは標準・準標準ライブラリでよく見られるパターンなので[^example]、何をやりたいのか名前から直感的に理解できる。

[^example]: [Future](https://docs.rs/futures/0.3.8/futures/prelude/trait.Future.html) / [FutureExt](https://docs.rs/futures/0.3.8/futures/future/trait.FutureExt.html) など

逆にデメリットとして考えられるのは以下です。
1. 状態を持てない。これは implementation trait のデフォルト実装として機能を実装する都合上、フィールドを参照できないという問題があるため。愚直な実装では機能は struct の関数として実装されるため、このような問題は発生しない。 
   今回のプロジェクトでは各コンポーネントが状態を持たないため、問題ではない。


## まとめ
Rust を使って DI を行いながら CLI プロジェクトを作りました。
