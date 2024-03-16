## 概要
正規表現のコンパイルの方法はあまり知られていません。そのため、自分の学習も兼ねて説明します。何回かに分けて、様々なトピックについて詳しく説明していきます。

[1. コンパイルの理論](https://qiita.com/kobae964/items/81058a229dced09dd2ab)
**2. 各言語・ライブラリーにおけるコンパイル処理の実装**
3. 自分でコンパイル処理を実装する (予定)

今回は Go, Rust について実装を見ていきたいと思います。

## Go

Go には標準で [regexp](https://pkg.go.dev/regexp@go1.21.6) というパッケージがあり、これが正規表現の担当をします。正規表現のコンパイルは [regexp/syntax](https://pkg.go.dev/regexp/syntax@go1.21.6) が担当しています。(Go のバージョンは 1.21.6 です。)

regexp/syntax の構造は以下の図で表されます。

![go-regexp-syntax-dep.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/67404/a0455af1-058a-2e31-cbd9-36f8bcefd49a.png)

以下のようなコードで実際に正規表現のコンパイルを試すことができます。
```go
package main

import (
	"fmt"
	"regexp/syntax"
)

func main() {
	result, err := syntax.Parse("a{2,}.*", 0)
	if err != nil {
		panic(err)
	}
	result = result.Simplify()
	fmt.Printf("%#v\n", result)
	prog, err := syntax.Compile(result)
	if err != nil {
		panic(err)
	}
	fmt.Println(prog.String())
}
```

実行結果は以下のようになります。
```console
$ go run .
&syntax.Regexp{Op:0x12, Flags:0x0, Sub:[]*syntax.Regexp{(*syntax.Regexp)(0x140000bc230), (*syntax.Regexp)(0x140000bc150)}, Sub0:[1]*syntax.Regexp{(*syntax.Regexp)(0x140000bc230)}, Rune:[]int32(nil), Rune0:[2]int32{0, 0}, Min:0, Max:0, Cap:0, Name:""}
  0     fail
  1*    rune1 "a" -> 2
  2     rune1 "a" -> 3
  3     alt -> 2, 5
  4     anynotnl -> 5
  5     alt -> 4, 6
  6     match

```

`a{2,}.*` にマッチする文字列 s = `aaab` を例にして、どのように実行されるか見ましょう。
- ptr = 0, pc = 1: rune1 "a" があり、s[0] = 'a' なので、成功。pc = 2 にする。
- ptr = 1, pc = 2: rune1 "a" があり、s[1] = 'a' なので、成功。pc = 3 にする。
- ptr = 2, pc = 3: alt -> 2, 5 があるので、次の行き先を 2 または 5 から選べる。ここでは 2 を選ぶ。
- ptr = 2, pc = 2: rune1 "a" があり、s[2] = 'a' なので、成功。pc = 3 にする。
- ptr = 3, pc = 3: alt -> 2, 5 があるので、次の行き先を 2 または 5 から選べる。ここでは 5 を選ぶ。
- ptr = 3, pc = 5: alt -> 4, 6 があるので、次の行き先を 4 または 6 から選べる。ここでは 4 を選ぶ。
- ptr = 3, pc = 4: anynotnl -> 5 があり、s[3] = 'b' は改行文字ではないので、成功。pc = 5 にする。
- ptr = 4, pc = 5: alt -> 4, 6 があるので、次の行き先を 4 または 6 から選べる。ここでは 6 を選ぶ。
- ptr = 4, pc = 6: match があるので、マッチが見つかったとみなす。

プログラム中で 1,2,3 が a{2,} に、 4,5,6 が .* に対応しているとみなすことができます。

これ以上の詳しい説明は https://github.com/koba-e964/code-reading/tree/master/algorithm/go~go1.21.6-regexp にあります。

## Rust
Rust の標準には正規表現を扱うモジュールはありませんが、regex というライブラリーがありこれが正規表現を扱います。
https://github.com/rust-lang/regex/tree/1.10.3

流れは https://docs.rs/regex-automata/0.4.5/regex_automata/nfa/thompson/index.html の通りです。

担当は以下の通りです。
- パースは https://docs.rs/regex-syntax/0.8.2/regex_syntax/
  - &str -> Ast -> Hir をやる
- コンパイルは https://github.com/rust-lang/regex/blob/1.10.3/regex-automata/src/meta/regex.rs#L3543-L3560
  - Hir -> NFA -> PikeVM などをやる
  - https://docs.rs/regex-automata/0.4.5/regex_automata/nfa/thompson/index.html
- オートマトンは https://docs.rs/regex-automata/latest/regex_automata/


- Ast: https://docs.rs/regex-syntax/0.8.2/regex_syntax/ast/enum.Ast.html
  - 普通の enum
- Hir: TODO
- meta::Regex: TODO

$O(nL)$ 時間で動作することは保証されています。

実際に正規表現 `a|bcd*` をコンパイルしてみた結果を読みましょう。

```
PikeVM { config: Config { match_kind: None, pre: None }, nfa: thompson::NFA(
>000000: binary-union(7, 1)
 000001: \x00-\xFF => 0
 000002: a => 8
 000003: b => 4
 000004: c => 5
 000005: binary-union(6, 8)
 000006: d => 5
^000007: binary-union(2, 3)
 000008: MATCH(0)

transition equivalence classes: ByteClasses(0 => [\x00-`], 1 => [a], 2 => [b], 3 => [c], 4 => [d], 5 => [e-\xFF], 6 => [EOI])
)
 }
```

`>` は start_unanchored (文字列の任意の位置から開始可能) の位置を表します。

`a|bcd*` にマッチする文字列 s = `ebcdd` を例にして、どのように実行されるか見ましょう。
- ptr = 0, pc = 0: binary-union(7, 1) があるので、次の行き先を 7 または 1 から選べる。ここでは 1 を選ぶ。
- ptr = 0, pc = 1: \x00-\xFF はすべてのバイトにマッチする。当然 `e` にもマッチするので pc = 0 にする。
- ptr = 1, pc = 0: binary-union(7, 1) があるので、次の行き先を 7 または 1 から選べる。ここでは 7 を選ぶ。
- ptr = 1, pc = 7: binary-union(2, 3) があるので、次の行き先を 2 または 3 から選べる。ここでは 3 を選ぶ。
- ptr = 1, pc = 3: b は `b` にマッチする。pc = 4 にする。
- ptr = 2, pc = 4: c は `c` にマッチする。pc = 5 にする。
- ptr = 3, pc = 5: binary-union(6, 8) があるので、次の行き先を 6 または 8 から選べる。ここでは 8 を選ぶ。
- ptr = 3, pc = 8: MATCH(0) があるので、マッチして完了。

この実行結果だと、 a**bc**dd の部分にマッチしたことになります。

これ以上の詳しい説明は https://github.com/koba-e964/code-reading/tree/master/algorithm/regex~1.10.3 にあります。
