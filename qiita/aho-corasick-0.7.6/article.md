[aho-corasick](https://github.com/BurntSushi/aho-corasick) クレイトの現在の最新バージョン ([0.7.6](https://github.com/BurntSushi/aho-corasick/tree/0.7.6)) のコードリーディングをしました。

## モジュール図
![aho-corasick-0.7.6-mods.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/67404/40850ed0-646f-71a6-d7fc-1395db3500ed.png)

## 各モジュールの責務
[docs.rs](https://docs.rs/aho-corasick/0.7.6/aho_corasick/) にあるのは詳細を省く。
### packed
ベクトル化した探索の実装
### state_id
状態の番号を表す型である StateID トレイトの定義
### error
エラー型 (`error::Error`, `error::Result`) の定義
### classes
u8 の同値類を表現する `classes::ByteClasses` の定義。これにより大文字小文字を同一視した探索などができる。
### buffer
u8 ロールバッファ (`buffer::Buffer`) の定義
### prefilter
マッチがありえない時にフィルタする機能を持つ `prefilter::Prefilter` トレイトの定義。偽陰性 (マッチがあるのにないと主張すること) は許されない
### automaton
Automaton トレイトの定義
### nfa
NFA (非決定性有限オートマトン) の定義
### dfa
DFA (決定性有限オートマトン) の定義、NFA -> DFA の変換 (決定化) 関数 [`Builder::build`](https://github.com/BurntSushi/aho-corasick/blob/0.7.6/src/dfa.rs#L619-L670) の実装
### ahocorasick
公開インタフェイスである `AhoCorasick`, `AhoCorasickBuilder` などの定義

### ソースコード
モジュール図 (dot) https://gist.github.com/koba-e964/5fa51a9426061d027142d55c7542acc7
