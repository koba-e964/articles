## 概要
yukicoder の問題にソースコードを提出する機能を持つ、コマンドラインツールを作成した。
https://github.com/koba-e964/contest/blob/master/yukicoder/submit.py

## 作成した経緯
もともと提出を手動でやるのが面倒だと思っていた。
yukicoder が API を公開していることを知り、せっかくだから Python での開発の練習がてら自動で提出できるようにしてみた。

## 仕様
### 設定ファイル
`yukicoder_config`: yukicoder 用の秘密情報ファイル (自分のプロフィールの「APIキー (β)」の情報を格納する)

```yaml:yukicoder_config
api_key: 'API-KEY-XXXX'
```

[`languages.yml`](https://github.com/koba-e964/contest/blob/master/languages.yml): 言語ごとの設定情報 (言語名、yukicoder での管理名、拡張子、AtCoder での管理名などを格納する。すべてのジャッジで共用)

```yaml:languages.yml
- name: C++
  extension: cpp
  yukicoder_name: cpp17
- name: Rust
  extension: rs
  yukicoder_name: rust
- name: Python3
  extension: py
  yukicoder_name: pypy3
```

### 使い方

```
./submit.py [問題番号].[拡張子]
```

使用例

```bash
./submit.py 1245.rs
```

![使用例 スクリーンショット](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/67404/0a918c94-1c36-c5b1-ebdf-e570fdc412d8.png)


## 工夫した点
### 設定ファイル
設定ファイルのフォーマットとして、大まかに以下の2通りが存在する:

1. Python の式そのものが書けるようにし、読み込みの際は設定ファイルを直接実行し、必要な値が読まれている状態にする
2. YAML, JSON, TOML, XML などの設定ファイル記述言語で記述する。読み込みの際はライブラリを使用し、設定ファイルをパースして値に変換し、その値から必要なデータを読み込む。

これらには以下のようなメリット・デメリットが存在する:

- 1. 
  - メリット
     - ファイルを読み込んで、式や文として評価すればよいので、実装が楽 (Python なら `exec` 関数、Ruby なら `Module.module_eval` など)
     - 任意の値を計算して設定できる。
  - デメリット
     - チューリング完全である (表現力が高すぎる) ため、設定ファイルの読み込みが停止する保証がなく、また理論的には読み込みが停止するかどうかの判定もできない
     - 同じく、表現力が高すぎるため、正しい情報を与えるような設定ファイルかどうかを事前に検証できない
     - 一部のスクリプト言語以外の言語では、そもそも記述できない
         - 実行時に式をパースして実行する機能がない場合があるため
     - 設定が言語に縛られる
         - これは 2. の形式でも、 例えば [RON](https://github.com/ron-rs/ron) や [Starlark](https://github.com/bazelbuild/starlark) や [Erlang の file:consult/1 で読める形式](https://erlang.org/doc/man/file.html#consult-1) などで起こり得る[^do-you-know-about-json]
- 2.
  - メリット
     - 表現力が制限されるため、読み込みは必ずいつか停止する
     - 同じく表現力が制限されるため、正しい情報を与えるような設定ファイルかどうか事前に検証できる (スキーマに従っているかどうかを検証できる)
     - (YAML, JSON, XML などの汎用的な設定記述言語を使っている場合) 言語によらずほとんど同一の手間で読み込みができる
         - リファクタリングで言語を変えたり、協調するためのツールを別の言語で書く場合に役に立つ
  - デメリット
     - 表現力が弱いため、設定ファイル内部で計算を行うことはできない
     - 設定ファイルをライブラリを使って値に変換し、そこから必要な値を取得する手間が発生する

以前作った [AtCoder 用の提出ツール](https://github.com/koba-e964/contest/blob/2eef5531b3fd95225c67e679450bf3041da25b98/atcoder/submit.rb)では 1. を採用した。これは以下の理由がある:

- 当時は技術力が低く、設定ファイルの表現力を制限することによるメリットを知らなかった
- 当時は技術力が低く、設定ファイルの実現方法としてこれしか思いつかなかった (JSON や YAML についての知識が乏しかった)
- 当時は技術力が低く、設定ファイル用のスキーマを定義することのメリットを知らなかった

今回は 2. を採用した。理由は上記の逆および個人の好みである。
なお、スキーマ定義および検証には [Cerberus](https://docs.python-cerberus.org/en/stable/) を使用した。[^why-not-json-schema]

[^do-you-know-about-json]: JSON が JavaScript 由来なのを知ってますか?
[^why-not-json-schema]: スキーマ定義言語として JSON Schema を使用しなかった理由は、JSON Schema は JSON 用であり YAML に使うためには微妙に追加の手間が必要であるため、そして今回の設定ファイルの規模 (数項目) に対してスキーマがかなり大きくなってしまい管理が大変になることが予測されたためである。Cerberus を使うとスクリプト内にスキーマを埋め込む格好になり、今回の規模であれば楽。
