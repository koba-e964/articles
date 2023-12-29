# 概要
2023 年 12 月に Terrapin attack と呼ばれる、SSH への新しい攻撃手法が発表された。
中間者攻撃の一種である。弱い認証を行わせることができる。(ダウングレード攻撃)

[[Atk-page]] と [[Atk-paper]] で著者らが攻撃の仕組みを紹介しているので、この記事ではそれの解説を行う。

# 基本用語
- パケット: SSH で送受信されるデータのまとまり。パケットは TCP のチャネルにそのまま流されるので、受信者がバイト列をパケットに分割することが一意にできるように、長さの情報が含まれている。中間者は以下のような方法で特定のパケットを取り除くことができる。
  - 暗号化されていないパケット: 長さが簡単にわかるので、その長さだけバイト列を除去すれば良い。
  - 暗号化されているパケット: 長さも暗号化されていることがある[^maybe-encrypted]が、定型文などで長さが予測できる場合はその長さだけ取り除くことができる。
- Sequence Number: SSH で今までに送信・受信したパケットの総数。送信・受信で別々にカウンターを持つ。
クライアントとサーバーで同じ値を持っていることが想定されているが、これを中間者がうまく操作してクライアント・サーバー双方を騙せることがこの攻撃のポイント。

[^maybe-encrypted]: 暗号化方式によって異なる。Chacha20-Poly1305 は暗号化する (パケットの他の部分と違う共通鍵で)。AES-GCM および *-EtM の場合は長さ部分を暗号化しない ([AES-GCM の実装](https://github.com/warp-tech/russh/blob/v0.40.2/russh/src/cipher/gcm.rs#L187), [*-ETM の実装](https://github.com/warp-tech/russh/blob/v0.40.2/russh/src/cipher/block.rs#L179))。E&M の場合は長さ部分ごと暗号化する ([実装](https://github.com/warp-tech/russh/blob/v0.40.2/russh/src/cipher/block.rs#L184))。

# 攻撃手法

![スクリーンショット 2023-12-26 12.03.39.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/67404/6b589597-1814-3500-c097-c4bf9bc689e3.png)
([[Atk-paper]] の Figure 4)

脆弱なポイントは以下。
- 暗号化していない通信時は sequence number のチェックが行われない
- 暗号化通信開始時、sequence number さえ正しければ整合性が保たれてしまう
  - 特に、最初のパケットがなくなっても sequence number 以外では気付けない

これらを踏まえての攻撃手段は以下。
- 暗号化通信時の最初の EXT_INFO パケットを落とす
  - そのとき (サーバーの Snd) = (クライアントの Rcv) となるようにするため、暗号化していないときにパケットを水増しする
  - これにより、サーバー・クライアントの認証などが弱い形式になる。EXT_INFO は自分が認識できる拡張機能などの伝達のために使われており、その伝達に失敗すると拡張機能が使えず弱い暗号化や弱い認証方式を強制される。これにより影響を受けるのは例えば以下:
    - OpenSSH 9.5p1 で導入された keystroke obfuscation (SSH_MSG_PING メッセージをサーバーが認識する必要があるので、サーバーは自分が認識できるということをクライアントに教える必要がある。プロトコルの仕様: https://github.com/openssh/openssh-portable/blob/V_9_6_P1/PROTOCOL#L129-L133)
    - `server-sig-algs`: クライアントの認証に使えるアルゴリズムの伝達のために、サーバーが使う。これがないと `ssh-rsa` (RSA 署名 + 内部でハッシュ関数として SHA-1 を使用) を強制される。 ([[RFC8332]] の Section 3.3)

# 脆弱・非脆弱なパターン
どんな方式であれ、パケットの暗号化は以下のような関数とみなすことができる:
- 入力
  - 鍵 (鍵交換で共有した値から導出される)
  - ノンス (nonce) (SSH の暗号化方式によって中身が異なる)
  - パケットの平文 (SSH の暗号化方式によって変わることはない)
- 出力
  - 暗号化されたパケット
  - 認証タグ (MAC)

これらはさらに以下に二分できる:
- 暗号化と認証タグの付与を、一つのアルゴリズムで一挙にやってしまうタイプ
  - AES-GCM, Chacha20-Poly1305 など
- 暗号化と認証タグの付与について、別のアルゴリズムを使うタイプ
  - (AES-CBC, AES-CTR) + (HMAC-SHA256, HMAC-SHA512) など
    - さらに、認証タグの計算方法も E&M と EtM の 2 通りある。説明は後の章で行う。

[[Atk-paper]] の著者らは脆弱であることを、認証タグを偽造できて妥当なメッセージとみなされるようにできることと定義している。これに従えば、以下のようになる:
- 認証タグが偽造できない: 脆弱でない
- 認証タグが偽造できるが、復号された結果がクライアントやサーバーに正しく認識される確率が低い: 脆弱であるが攻撃に利用できない
- 認証タグが偽造でき、復号された結果がクライアントやサーバーに正しく認識される確率がそれなりにある: 脆弱であり攻撃に利用可能

基本的に SSH で使われている暗号化方式・認証タグの付与方式は、それ自体はすべて安全である。つまり脆弱かどうかは暗号化方式そのものによって決まるのではなく、暗号化方式の使われ方によって決まる。
具体的には以下。
- パケットごとにノンスを何に設定するか
  - 特に、sequence number 以外の情報 (暗号化通信開始時からのパケット数、前のパケットの情報など) をノンスに含めるかどうか
- 認証タグをいつ、何に対して付けるか
  - 平文に対して付ければ認証タグが不正とみなされて攻撃が失敗する可能性が高く、暗号文に対して付ければ認証タグ自体は正しくなってしまい脆弱である可能性が高い

## 非脆弱
### AES-GCM
cipher + mac: `aes256-gcm@openssh.com`

(ノンス) = (固定値) || (invocation_counter) である ([[RFC5647]] の 7.1)。invocation_counter というのは暗号化通信が始まってからのパケット数であるため、暗号化通信が始まってから中間者がパケットを一つ取り除くと受信側で invocation_counter が食い違い、受信側が認証タグの検証時に不正を検出することで攻撃が防がれる。

### *-CBC + E&M, *-CTR + E&M
cipher:
- `*-cbc` という名前のもの (`aes256-cbc` など)
- `*-ctr` という名前のもの (`aes256-ctr` など)

mac:
- `hmac-sha1`
- `hmac-sha2-256`
- `hmac-sha2-512`

*-CBC, *-CTR という名前の暗号化は、どちらも状態を持っている。
- *-CBC は前のパケットの最後のブロック (暗号文) をノンスにする
- *-CTR は今まで暗号化したバイト数に相当するカウンターを持っており、これがノンスに相当する

認証方式の E&M は、暗号化される前の平文に対して認証タグを計算する。このため、中間者がパケットを除去した後受信者が復号を試みると、復号されたものはランダムなバイト列となるので、そこについている認証タグが正しい確率は低い。これにより攻撃が防がれる。

実装:
- https://github.com/warp-tech/russh/blob/v0.40.2/russh/src/cipher/block.rs#L76-L84
  - cipher というフィールドがあり、`self.cipher.apply_keystream` の呼び出しでこのフィールドが変更されるため、状態を持っていることがわかる。
- https://github.com/warp-tech/russh/blob/v0.40.2/russh/src/cipher/block.rs#L182-L186
  - 認証タグの計算 (`self.mac.compute`) のあとで暗号化 (`self.cipher.apply_keystream`) が行われることがわかる。
- https://github.com/RustCrypto/block-modes/blob/ctr-v0.9.2/ctr/src/ctr_core.rs#L19
  - CTR mode が実際にカウンターを持っていることがわかる。
## 脆弱
### Chacha20-Poly1305
cipher + mac: `chacha20-poly1305@openssh.com`

ノンスは 64 ビットであり、(ノンス) = (sequence number) である。(参照: https://github.com/openssh/openssh-portable/blob/V_9_6_P1/PROTOCOL.chacha20poly1305#L54-L60)
ノンスが sequence number の情報しか含んでいないので、脆弱である。
- 文書内の IV をここではノンスと呼んでいる。

実装:
- https://github.com/warp-tech/russh/blob/v0.40.2/russh/src/cipher/chacha20poly1305.rs#L162-L187

### CTR-EtM
認証方式の EtM は、暗号化された後の暗号文に対して認証タグを計算する。そのためパケットを取り除いても認証タグ自体は正しい可能性が高く、脆弱であると言える。

しかし、CTR-E&M の説明にもあったようにこの暗号化方式は内部でカウンターを持っており、パケットが取り除かれた後で受信者が復号で得るものは、(取り除かれたパケットの次のパケットに限らず) すべてがランダムである。このため攻撃は成功する可能性が極めて低い。

実装:
- https://github.com/warp-tech/russh/blob/v0.40.2/russh/src/cipher/block.rs#L176-L182
  - 暗号化 (`self.cipher.apply_keystream`) のあとで認証タグの計算 (`self.mac.compute`) が行われることがわかる。

### CBC-EtM
CTR と違い CBC は前のパケットの最後のブロック (暗号文) をノンスにするのだった。そのため、中間者は受信者に、中間者が取り除いたパケットの次のパケットだけを正しいものと誤認させるだけで、それ以降の全てのパケットを正しいものと誤認させることができる。

1 つのパケットだけを誤認させることができる確率は意外と高く、著者らはターゲットとなる実装に依存して 0.03%-83% 程度と見積もっている。

# 回避策
## Strict key exchange
説明: https://github.com/openssh/openssh-portable/blob/V_9_6_P1/PROTOCOL#L155-L164

まずクライアントとサーバーがどちらも strict key exchange に対応していることを確認するため、クライアントは `kex-strict-c-v00@openssh.com` を、サーバーは `kex-strict-s-v00@openssh.com` を、鍵交換前に送る。
- 具体的にはお互いに送り合う SSH_MSG_KEXINIT メッセージ内部の、対応する鍵交換アルゴリズムの列の中にそれぞれ入れる。
- 実装:
  - 送信: https://github.com/openssh/openssh-portable/blob/V_9_6_P1/kex.c#L355-L357
  - 確認: https://github.com/openssh/openssh-portable/blob/V_9_6_P1/kex.c#L1182-L1190

お互いに strict key exchange に対応していることがお互いに確認できたら、以下の 2 個の対抗策を行う:
- 最初の鍵交換時、予期しないパケットが来たら通信を終了する
  - 実装: https://github.com/openssh/openssh-portable/blob/V_9_6_P1/kex.c#L488-L492
- 鍵交換完了時に sequence number をリセットする
  - 実装: https://github.com/openssh/openssh-portable/blob/V_9_6_P1/packet.c#L1224-L1227

これらはどちらか片方だけでも Terrapin attack を防ぐはず。

# 参考文献

[[Atk-page]]: Bäumer, Fabian, Marcus Brinkmann, and Jörg Schwenk. *Terrapin Attack.* terrapin-attack.com.
[[Atk-paper]]: Bäumer, Fabian, Marcus Brinkmann, and Jörg Schwenk. "Terrapin Attack: Breaking SSH Channel Integrity By Sequence Number Manipulation." arXiv preprint arXiv:2312.12422 (2023).
[[RFC5647]]: Igoe, K. and J. Solinas, "AES Galois Counter Mode for the Secure Shell Transport Layer Protocol", RFC 5647, DOI 10.17487/RFC5647, August 2009, <https://www.rfc-editor.org/info/rfc5647>.
[[RFC8332]]: Bider, D., "Use of RSA Keys with SHA-256 and SHA-512 in the Secure Shell (SSH) Protocol", RFC 8332, DOI 10.17487/RFC8332, March 2018, <https://www.rfc-editor.org/info/rfc8332>.

[RFC5647]: https://datatracker.ietf.org/doc/html/rfc5647
[Atk-page]: https://terrapin-attack.com/
[Atk-paper]: https://arxiv.org/abs/2312.12422
[RFC8332]: https://datatracker.ietf.org/doc/html/rfc8332

# この記事での用語の使い方
- `暗号化` は `encryption` の訳語である。
- `暗号化方式` は `encryption mode` の訳語である。
- `暗号文` は `ciphertext` の訳語である。
- `RSA 署名` は `RSA signature` の訳語である。
- `認証タグ` は `message authentication code` の訳語である。
- 曖昧性を避けるために、`暗号` という単語は使用していない。
