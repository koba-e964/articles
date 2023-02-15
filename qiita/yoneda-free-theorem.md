# まとめ
黒魔術を使うと米田の補題による制限を回避して自由闊達に関数が書けます。

# 前提知識
Haskell の知識、多相関数に関する知識を仮定します。圏論の知識はほとんど仮定しません。

# 説明
## 記法
`F` や `A` などの大文字始まりの型名はすべてなんらかの具体的な型がそこに入ります。
`x` や `a` などの小文字始まりの型名は型変数です。(Haskell での流儀と一緒です。)

## 自然変換
Functor インスタンスが実装されている 2 種類の型 `F` と `G`、および多相関数 `u :: forall x. F x -> G x` について、以下のような法則が成り立つ時、`u` は自然変換であるといいます。
```
a, b: 型
m :: F a
f: a -> b
```
としたとき、
```Haskell
fmap f (u m) = u (fmap f m)
```
が常に成り立つ。

直感的にいえば、これは `u` が型変数 `x` で指定されている型の中身には一切触らない、ということを指します。`u` が中身に触れないから、中身の変換を行う `f` を `u` の計算前と計算後のどちらでやっても同じ、ということです。
詳しくは[もう諦めない圏論入門―関手と自然変換―](https://qiita.com/norkron/items/1cc1b593e04184f0fe98)などを参照してください。

## 米田の補題

ある種の自然変換は中身が大幅に制限されるという定理です。正しい主張は [Wikipedia](https://ja.wikipedia.org/w/index.php?title=%E7%B1%B3%E7%94%B0%E3%81%AE%E8%A3%9C%E9%A1%8C&oldid=83239674) を参照してください。
以下のような定理です。
```Haskell
t :: forall x. (A -> x) -> F x
```

という値があったとき、仮に `t` が自然変換であるとすればそれは `F A` と同じ個数しか存在せず、`u :: F A` としたとき `t` は `\g -> fmap g u` という形に限定される。

### 具体例
以下のような多相関数であり、自然変換でもある関数を考えます。
```Haskell
t :: forall x. (x, x) -> x
```

この関数は何通りあり得るでしょうか? 上の式に `A` = `Bool`, `F` = `Identity` を代入すると、`Bool -> x` は `(x, x)` と同一視できるので、`Identity Bool` と同じ個数、つまりちょうど 2 個存在することが言えます。

具体的な中身を見ましょう。`(x1, x2)` は `\b -> if b then x2 else x1` と同一視されます。また `Identity` について、`fmap f` と `f` は同一です。
したがって、上の `\g -> fmap g u` というのは `\(x1, x2) -> fmap (\b -> if b then x2 else x1) u` = `\(x1, x2) -> if u then x2 else x1` ということになります。`u` は `False` か `True` かのどちらかなので、結局以下の 2 パターンに分類できます。

- `u` が `False` のとき: `\(x1, x2) -> x1`
- `u` が `True` のとき: `\(x1, x2) -> x2`

## Free theorem
関数の表現能力が制限されている場合、多相関数はすべて自然変換である、という定理です。詳しくは [Wadler の論文](https://people.mpi-sws.org/~dreyer/tor/papers/wadler.pdf) を参照してください。

例えば、`u :: forall a. [a] -> [a]` という項は、`u` が性質のよい部品から作られている場合は必ず自然変換です。つまり、`u . map f = map f . u` です。
ここでいう「性質のよい部品」というのは以下のような関数です:
- [`id`](https://hackage.haskell.org/package/base-4.16.2.0/docs/Prelude.html#v:id) (恒等関数)
- `:` (要素とリストから新しいリストを作る)
- [`head`](https://hackage.haskell.org/package/base-4.16.2.0/docs/Prelude.html#v:head) (リストの先頭 1 要素だけをとる)
- [`tail`](https://hackage.haskell.org/package/base-4.16.2.0/docs/Prelude.html#v:tail) (リストの先頭 1 要素以外をとる)

逆に「黒魔術」とは以下のような関数です:
- [`typeOf`](https://hackage.haskell.org/package/base-4.16.2.0/docs/Data-Typeable.html#v:typeOf) (`Typeable` が提供する関数。型での場合分けなどができる)
- [`unsafeCoerce`](https://hackage.haskell.org/package/base-4.14.0.0/docs/Unsafe-Coerce.html) (型合わせパズルでチートができる。)

## 病的な関数
さて、ここで少し逸脱してみましょう。
ほとんどすべての型の値はそのまま返しますが、`Bool` については否定した結果を返すような関数があったとしたらどうなるでしょう?
Haskell ではそのような関数は以下のように実装できます。

```Haskell
Prelude> import Unsafe.Coerce
Prelude Unsafe.Coerce> import Data.Typeable
Prelude Unsafe.Coerce Data.Typeable> pathological x = if typeOf x == typeOf False then unsafeCoerce (not (unsafeCoerce x)) else x
```

型シグニチャは以下の通りです。`Typeable` というのを除けばほとんど `forall a. a -> a` であることが分かると思います。

```Haskell
Prelude Unsafe.Coerce Data.Typeable> :t pathological 
pathological :: Typeable p => p -> p
```

動作確認は以下です。

```Haskell
Prelude Unsafe.Coerce Data.Typeable> pathological 1
1
Prelude Unsafe.Coerce Data.Typeable> pathological "string"
"string"
Prelude Unsafe.Coerce Data.Typeable> pathological False
True
```

この `pathological` という関数は多相関数ですが自然変換ではありません。`Bool` を絡ませるとおかしくなります。(`(==1)` は `\x -> x == 1` の略記で、x が与えられた時に 1 と等しいか判定する関数です。)

```Haskell
Prelude Data.Typeable Unsafe.Coerce> (==1) $ pathological 1
True
Prelude Data.Typeable Unsafe.Coerce> pathological $ (==1) 1
False
```

米田の補題から、`forall a. a -> a` という型シグニチャを持ち自然変換である多相関数は `id` に限られます。
- 上の例で `A = (), F = Identity` とすると、`F A = Identity () = ()` なので、1 通りしか存在しません。この 1 通りに対応するのが `id` です。


しかし `pathological` は自然変換ではないので `id` と食い違っていても何の問題もありません。また自然変換でないので、Free theorem から分かりますが性質のよい部品だけから作ることはできず、`typeOf` や `unsafeCoerce` などの黒魔術が必要となります。

### 追記 (2023/02/16)
viercc さんから以下のような提案をいただきました:
https://twitter.com/viercc/status/1546791629843238917

このような工夫をすると `Typeable b` という制約なしで `forall b. b -> b` 型の値を定義できます。

```Haskell
$ ghci
GHCi, version 9.4.4: https://www.haskell.org/ghc/  :? for help
ghci> true = True
ghci> false = False
ghci> import System.Mem.StableName
ghci> import System.IO.Unsafe
ghci> pathEq a b = unsafePerformIO $ do; x <- makeStableName $! a; y <- makeStableName $! b; return (eqStableName x y)
ghci> import Unsafe.Coerce 
ghci> pathological x = if pathEq x true || pathEq x false then unsafeCoerce (not (unsafeCoerce x)) else x
ghci> pathological true
False
ghci> pathological false
True
ghci> pathological 1
1
ghci> :t pathological 
pathological :: b -> b
```
