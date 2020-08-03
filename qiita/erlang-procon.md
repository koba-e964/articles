## 始めに
[AtCoder に登録したら解くべき精選 10 問](https://qiita.com/drken/items/fd4e5e3630d0f5859067) を、Erlang で解きました。[^style-excuse]
[^style-excuse]: 体裁は [AtCoder に登録したら解くべき精選過去問 10 問を Rust で解いてみた](https://qiita.com/tubo28/items/e6076e9040da57368845)を参考にさせていただきました。
### 対象
Erlang と競プロをどちらも知っていて、競プロを通じて Erlang のスキルを高めたい方。

### テンプレート

```erlang
-module('Main').
-export([main/1]).

solve() ->
    ok.

main(_) ->
    solve(),
    halt().

input(Pat) ->
    {ok, L} = io:fread("", Pat),
    L.
```
基本的に solve/0 の中に実装を書けば良いです。以降 module, export, main/1, input/1 は省略します。

### 入出力
io:fread を使うことで、 C の scanf と似た形式 (単語ごとに読む) で入力を取ることができます。
例えば、整数 N とその後に続く N 個の整数を読みたい場合は以下のようにします。

```erlang
solve() ->
    [N] = input("~d"),
    % 遅い
    A = [input("~d") || _ <- lists:seq(1, N)],
    % 速い
    Pat = lists:flatten(lists:duplicate(N, "~d")),
    A = input(Pat),

    ok.
```

基本的に io:fread を何回も呼ぶと遅いので、パターン文字列を連結して1回の呼び出しで済ませた方が速いです。なにこれ

出力は `io:format(Pattern, [Args...])` という関数呼び出しで行うことができます。例えば整数 A を出力したい場合は以下のようにすれば良いです:

```erlang
solve() ->
    A = 42,
    io:format("~B~n", [A]),
    ok.
```
| 型 | フォーマット文字列| 
|---|---|
| 整数 | `~B` (`~p` も可)|
| 文字列 | `~s` |
| Erlang 式表記 (型は何でもよい) | `~p` |

### 実行方法
```shell
escript ファイル名.erl
```

### 問題と解法の詳細について
解法の詳細については、この記事では一切書きません。そもそもこれは入門用の記事ではありません。

## 第 1 問: [ABC 086 A - Product (100 点)](https://atcoder.jp/contests/abc086/tasks/abc086_a)

A, B を受け取り、AB が偶数ならば "Even" を、奇数ならば "Odd" を出力せよ。

[提出](https://atcoder.jp/contests/abc086/submissions/13184025)

```erlang
solve() ->
    [A, B] = input("~d~d"),
    io:format("~s~n", [case A * B rem 2 of
        0 -> "Even"; 1 -> "Odd" end]),
    ok.
```

## 第 2 問: [ABC 081 A - Placing Marbles (100 点)](https://atcoder.jp/contests/abc081/tasks/abc081_a)

3 文字の文字列が与えられる。その中に含まれる '1' の個数は?

[提出](https://atcoder.jp/contests/abc081/submissions/13184036)

```erlang
solve() ->
    [S] = input("~s"),
    Len = length(lists:filter(fun (Elem) -> Elem =:= $1 end, S)),
    io:format("~B~n", [Len]),
    ok.
```

## 第 3 問: [ABC 081 B - Shift Only (200 点)](https://atcoder.jp/contests/abc081/tasks/abc081_b)

数列 A の要素が全て偶数の場合、全て 2 で割る。この操作は何回できる?

[提出](https://atcoder.jp/contests/abc081/submissions/13184041)

```erlang
rec(A, N) when A rem 2 =:= 0 -> rec(A div 2, N + 1);
rec(_, N) -> N.

solve() ->
    [N] = input("~d"),
    A = input(lists:flatten(["~d" || _ <- lists:seq(1, N)])),
    And = lists:foldl(fun (Elem, Acc) -> Elem bor Acc end, 0, A),
    io:format("~B~n", [rec(And, 0)]),
    ok.
```
ループが使えないので、lists:foldl などを使う必要があります。

## 第 4 問: [ABC 087 B - Coins (200 点)](https://atcoder.jp/contests/abc087/tasks/abc087_b)

[提出](https://atcoder.jp/contests/abc087/submissions/13184078)

```erlang
solve() ->
    [A, B, C, X] = input("~d~d~d~d"),
    io:format("~B~n", [length(
        [ ok || I <- lists:seq(0, A), J <- lists:seq(0, B), K <- lists:seq(0, C), 500 * I + 100 * J + 50 * K =:= X]
    )]),
    ok.
```

何重ループであろうとループする範囲が決まっているのであれば、リストの内包表記でサクッと書けます。

## 第 5 問: [ABC 083 B - Some Sums (200 点)](https://atcoder.jp/contests/abc083/tasks/abc083_b)

[提出](https://atcoder.jp/contests/abc083/submissions/13184090)

```erlang
sum(N) when N =< 0 -> 0;
sum(N) -> sum(N div 10) + N rem 10.

solve() ->
    [N, A, B] = input("~d~d~d"),
    io:format("~B~n", [lists:sum(
        [I || I <- lists:seq(1, N), A =< sum(I), sum(I) =< B]
    )]),
    ok.
```
桁和みたいなのは再帰関数でサクッと書けます。

## 第 6 問: [ABC 088 B - Card Game for Two (200 点)](https://atcoder.jp/contests/abc088/tasks/abc088_b)

[提出](https://atcoder.jp/contests/abc088/submissions/13184099)

```erlang
rec([]) -> 0;
rec([A | Rest]) -> A - rec(Rest).

solve() ->
    [N] = input("~d"),
    A = input(lists:flatten(["~d" || _ <- lists:seq(1, N)])),
    ASorted = lists:reverse(lists:sort(A)), % 逆順ソート
    io:format("~B~n", [rec(ASorted)]),
    ok.
```
普通の配列のソートが $O(N \log N)$ 時間でできるのは有名ですが、連結リストでも (マージソートで) $O(N \log N)$ 時間が達成できるのは面白いですね。

## 第 7 問: [ABC 085 B - Kagami Mochi (200 点)](https://atcoder.jp/contests/abc085/tasks/abc085_b)

[提出](https://atcoder.jp/contests/abc085/submissions/13184106)

```erlang
solve() ->
    [N] = input("~d"),
    A = input(lists:flatten(["~d" || _ <- lists:seq(1, N)])),
    io:format("~B~n", [length(lists:usort(A))]),
    ok.
```
lists:usort で重複除去 + ソートができます。なんで (いろいろな関数が足りていないのに) こんなピンポイントな関数だけ用意してあるんだ…?

以下から計算量の意識が必要な C 問題になるので、難易度が跳ね上がります。

## 第 8 問: [ABC 085 C - Otoshidama (300 点)](https://atcoder.jp/contests/abc085/tasks/abc085_c)

[提出](https://atcoder.jp/contests/abc085/submissions/13184108)

```erlang
check(A, B, C, N) ->
    case 10000 * A + 5000 * B + 1000 * C =:= N of
        true -> throw({A, B, C});
        false -> ok
    end.

solve() ->
    [N, Y] = input("~d~d"),
    {A, B, C} = try
        lists:foreach(fun (I) ->
            lists:foreach(fun (J) -> check(I, J, N - I - J, Y) end, lists:seq(0, N - I)) end,
            lists:seq(0, N)),
        {-1, -1, -1}
    catch
        throw:E -> E
    end,
    io:format("~B ~B ~B~n", [A, B, C]),
```

`escript c.erl` で実行したら 40 秒近くかかりましたが、AtCoder のコードテストで実行したら 300ms 以内に終わりました。

## 第 9 問: [ABC 049 C - 白昼夢 / Daydream (300 点)](https://atcoder.jp/contests/abc049/tasks/arc065_a)

[提出](https://atcoder.jp/contests/arc065/submissions/13184128)

```erlang
calc("maerd" ++ S) -> calc(S);
calc("remaerd" ++ S) -> calc(S);
calc("esare" ++ S) -> calc(S);
calc("resare" ++ S) -> calc(S);
calc([]) -> true;
calc(_) -> false.

solve() ->
    [S] = input("~s"),
    SRev = lists:reverse(S),
    io:format("~s~n", [case calc(SRev) of
        true -> "YES";
        false -> "NO"
    end]),
    ok.
```
リストに対する先頭のマッチングは定数時間なので、このコードは $O(|S|)$ 時間で動作します。

## 第 10 問: [ABC 086 C - Traveling (300 点)](https://atcoder.jp/contests/abc086/tasks/arc089_a)
[提出](https://atcoder.jp/contests/abc086/submissions/13184031)

```erlang
calc([T1, X1, Y1 | L = [T2, X2, Y2 | _]]) ->
    T = T2 - T1,
    Dist = abs(X2 - X1) + abs(Y2 - Y1),
    case Dist =< T andalso (Dist + T) rem 2 =:= 0 of
        true -> calc(L);
        false -> false
    end;
calc(_) -> true.

solve() ->
    [N] = input("~d"),
    List = input(lists:flatten(lists:duplicate(3 * N, "~d"))),
    io:format("~s~n", [case calc([0, 0, 0 | List]) of
        true -> "Yes";
        false -> "No"
    end]),
    ok.
```

いわゆる $10^5$ 系問題の入力は、Erlang だと注意する必要があります。なるべく `io:format` を呼ぶ回数を減らすことが大切です。

こういう先頭から順に見ていくタイプの問題は、連結リストでも問題なく書けますね。

## 終わりに
Erlang で競プロは、しちゃダメだろ。
