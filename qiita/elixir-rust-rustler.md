# この記事は何?
rustler のバージョンが上がって [Elixirから簡単にRustを呼び出せるRustler #1 準備編](https://qiita.com/twinbee/items/aabc11d0d667800fc0bb) のやり方では試せなくなったので、現在のバージョン (rustler 0.21.1) 向けの rustler の試し方を記載します。

基本的に元記事と同じ方法を辿るため、**元記事の文章をコピペしています**。ただしバージョンアップにより一部手順が異なります。

なお、rustler 0.21.x の API docs は生成が失敗しているので、代わりに [rustler 0.22.0-rc.0 の docs](https://docs.rs/rustler/0.22.0-rc.0/rustler/) を見ることをお勧めします。

(コピペ + 改変ここから)

## 導入手順
### mix に rustler を追加
まずは rustler のパッケージをインストールして、mix rustler.new コマンドが実行できるようにします。
rustler 0.21.x に依存させるために、deps に `{:rustler, "~> 0.21.0"}` を加えてください。


```bash
# 通常通り elixir のプロジェクトを作成
$ mix new phx_rust_21
$ cd phx_rust_21
```

```elixir:mix.exs
...
  defp deps do
    [
      {:rustler, "~> 0.21.0"}
    ]
  end
...
```

```
$ mix deps.get
```


### rustler のボイラープレートを生成

mix に rustler の命令が追加されたので、さっそく使います。

```bash
$ mix rustler.new
```

Elixir 側の Module 名と Rust でのユニット名 (ファイル名) をそれぞれ対話形式で聞いてくるので、`NifExample`, `example` と入力します。
これで準備は完了です。

ここで指定した「example」をAtom化した `:example` は Ruslter の[公式ドキュメント](https://hexdocs.pm/rustler/basics.html)内では **NIF ID** と呼ばれています。

```
$ mix rustler.new
==> toml
Compiling 10 files (.ex)
Generated toml app
==> rustler
Compiling 5 files (.ex)
warning: EEx.eval_string/3 defined in application :eex is used by the current application but the current application does not directly depend on :eex. To fix this, you must do one of:

  1. If :eex is part of Erlang/Elixir, you must include it under :extra_applications inside "def application" in your mix.exs

  2. If :eex is a dependency, make sure it is listed under "def deps" in your mix.exs

  3. In case you don't want to add a requirement to :eex, you may optionally skip this warning by adding [xref: [exclude: EEx]] to your "def project" in mix.exs

  lib/mix/tasks/rustler.new.ex:79: Mix.Tasks.Rustler.New.copy_from/3

warning: Toml.decode!/1 defined in application :toml is used by the current application but the current application does not directly depend on :toml. To fix this, you must do one of:

  1. If :toml is part of Erlang/Elixir, you must include it under :extra_applications inside "def application" in your mix.exs

  2. If :toml is a dependency, make sure it is listed under "def deps" in your mix.exs

  3. In case you don't want to add a requirement to :toml, you may optionally skip this warning by adding [xref: [exclude: Toml]] to your "def project" in mix.exs

  lib/mix/tasks/compile.rustler.ex:177: Mix.Tasks.Compile.Rustler.check_crate_env/1

Generated rustler app
==> phx_rust_21
This is the name of the Elixir module the NIF module will be registered to.
Module name > NifExample
This is the name used for the generated Rust crate. The default is most likely fine.
Library name (nifexample) > example
* creating native/example/.cargo/config
* creating native/example/README.md
* creating native/example/Cargo.toml
* creating native/example/src/lib.rs
Ready to go! See /Users/koba_mac/srcview/phx_rust_21/native/example/README.md for further instructions.
koba_mac@kobas-MacBook-Air:~/srcview/phx_rust_21 $ 
```

このような流れでボイラープレートが出来上がります。
Rust のテンプレートはこちらになります。

```rust:native/example/src/lib.rs
use rustler::{Encoder, Env, Error, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.NifExample",
    [
        ("add", 2, add)
    ],
    None
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}
```

## プロジェクト概要をmix.exsに定義

``mix.exs``に３箇所コードを追加します。

1. `compilers:` 行の追加
1. `rustler_crates:` 行の追加
1. `rustler_crates` 関数の追加

```elixir:mix.exs
...
  def project do
    [
      app: :phx_rust_21,
      version: "0.1.0",
      elixir: "~> 1.11",
      compilers: [:rustler] ++ Mix.compilers, # 1. 追加
      rustler_crates: rustler_crates(),  #2. 追加
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end
...

  # 3. この関数(rustler_crates)を追加
  defp rustler_crates() do
    [example: [ # 呼び出し側Elixirモジュール NifExampleのマクロで使用するAtom
      path: "native/example",
      mode: (if Mix.env == :prod, do: :release, else: :debug),
    ]]
  end
...
```

## Elixir側のコード実装

Elixirの呼び出し側モジュールを実装します。ElixirとRustのモジュールと関数を具体的にマッピングしていく作業になります。

``use Rustler``では、モジュールにrustler拡張を適用しています。
以下の２箇所の設定を、確認して下さい。

- ``otp_app:`` は``mix.exs``内の `project[ app: ~ ]`で指定してある App 名。通常はプロジェクト名 (ここでは `:phx_rust_21` )です。
- ``crate:``はmix.exsのrustler_crates関数で定義された、NIF ID（前述）。


```elixir:lib/example.ex
defmodule NifExample do
  use Rustler, otp_app: :phx_rust_21, crate: :example

  def add(_a, _b), do: exit(:nif_not_loaded)
end
```

ちなみに`add`関数は**最初からボイラープレートに含まれる**関数です。

## Rust側のコード実装

テンプレートとして作られるコードを多少改変します。
以下を設定しています。

- Elixir側のモジュール名
- ElixirとRustの関数の関連付けリスト (elixir関数名, アリティ, Rust関数名)


```rust:native/example/src/lib.rs
...

mod atoms {
    // 生成されるコードでは rustler_atoms! となっているが、
    // cannot find macro `rustler_atoms` in this scope というエラーが出るため、絶対パスで参照するように変更する。
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.NifExample", // Elixirのmodule名
    [
        ("add", 2, add) // Elixirとrustの関数の関連付け 
    ],
    None
}
```


---

## 実行してみる


プロジェクトを起動すると、Rustのコンパイラーが走ります。
警告は出ますが、無事完了。

```bash
$ iex -S mix
Erlang/OTP 23 [erts-11.1.1] [source] [64-bit] [smp:4:4] [ds:4:4:10] [async-threads:1] [hipe] [dtrace]

Compiling NIF crate :example (native/example)...
   Compiling example v0.1.0 (/Users/koba_mac/srcview/phx_rust_21/native/example)
    Finished dev [unoptimized + debuginfo] target(s) in 4.39s
Interactive Elixir (1.11.2) - press Ctrl+C to exit (type h() ENTER for help)
``` 

iex のプロンプトが起動するので、早速 add 関数を実行してみます。

```elixir
iex(1)> NifExample.add(1,2)
{:ok, 3}
```

無事動作しました！

注目すべきは、戻りがタプルになっていることです。Rust側のタプルの構造がそのままElixirに返ってきてます。素晴らしいですね。

mix.exsの追加とexample.exを追加しただけで、rustの関数を呼び出せてしまいました。

(コピペ + 改変ここまで)

# まとめ
使用したコードは https://github.com/koba-e964/phx_rust_21/tree/intro に置いてあります。
次回の[Elixirから簡単にRustを呼び出せるRustler #2 クレートを使ってみる](https://qiita.com/twinbee/items/54e8a4ec73bc27abd10e)はまだ試していないため、この記事の続きが必要かどうかは未定です。
