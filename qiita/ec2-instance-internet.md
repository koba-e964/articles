AWS の EC2 instance からインターネットに繋ぐのにハマったのでメモ。

# 概要
AWS の EC2 instance からインターネットに繋ぎたい。以下の要請がある。
- 複数のインスタンスを立てるかもしれない。その時のためになるべく少ない個数の public IPv4 アドレスを利用したい
- Web サービスは立てないので、EC2 instance に public IPv4 アドレスが紐づいている必要はない
- インターネットに繋ぐには何らかの形で public IPv4 アドレスを 1 個以上要する

# 完成図
VPC の中に以下のようなものができる。
![スクリーンショット 2024-09-22 17.46.40.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/67404/632df6cf-bbf7-55fe-e9e3-93cf38caf651.png)
図からわかりにくいが、`koba-nat-gateway` が `subnet-for-internet-access` (pubilc な方の subnet) に関連づけられていることに注意。これによって 2 個の subnet の間のつながりが生まれる。

# 作るもの
全体の構造から順に説明する。構築する際は順番に従うこと。

- subnet 2 個 (1 番目)
  - 図の `subnet-for-internet-access`: インターネットへの接続を司る。Internet Gateway に接続する。また NAT Gateway もこの中に作る。大きさは 11 ホスト (CIDR block を /28 で設定する、5 ホスト予約されるので $2^{32-28}-5=11$) でよい。/29 (3 ホスト) が可能だったらそれでも良かったかもしれない (検証不能)
  - 図の `subnet-for-priv-dev`: 外に漏れたらまずいことをやる。インターネットに繋ぎたい時は NAT Gateway にリクエストを送信する
- route table 2 個 (3 番目)
  - 図の `rtb-public-dev-subnet-into-internet`: Explicit subnet associations を `subnet-for-internet-access` にし、「全て (0.0.0.0/0) のターゲットを Internet Gateway に繋げ」という指示を与える
  - 図の `rtb-public-dev-subnet-into-internet`: Explicit subnet associations を `subnet-for-priv-dev` にし、「全て (0.0.0.0/0) のターゲットを NAT Gateway に繋げ」という指示を与える
- NAT Gateway (2 番目)
   - Elastic IP Address を持つ。必然的に public subnet (Internet Gateway につながっている subnet) に作る必要があり、そうでないところに作るのはナンセンスである

# ハマったポイント
## public subnet と private subnet の違い
subnet そのものに public/private の設定項目があるわけではない。Internet Gateway につながっているものを public subnet と呼ぶ。

## 一つの subnet でできないのか?
多分できない。
