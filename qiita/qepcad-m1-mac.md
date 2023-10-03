---
title: QEPCAD を M1 Mac で実行する
tags: 数学 論理学 Docker M1
author: kobae964
slide: false
---
## 概要
QEPCAD は、量化子消去を実行してくれるソフトウェアです。([大学の数学の入試問題を量化子消去でサッと解く](https://qiita.com/kobae964/items/30dcb77837068b2638be)に詳しい解説があります。)
ただし、QEPCAD のインストール方法として https://ashiato45.hatenablog.jp/entry/2018/04/25/070040 の方法は M1 Mac では実行できません。(コンパイルに失敗します。)
この記事では、これを回避し、M1 Mac でも QEPCAD を実行できるようにします。

## 対象読者
- M1 Mac などを使っていて、自分のマシーンで QEPCAD をコンパイルできないが QEPCAD を動かしたい人
- x86 マシーンを使用していて QEPCAD のインストールは可能ではあるものの、手順が面倒なので楽をしたい人

## 事前準備
Docker for Mac をインストールします。

筆者の環境は以下です:
```
$ docker version
Client:
 Cloud integration: v1.0.35
 Version:           24.0.2
 API version:       1.43
 Go version:        go1.20.4
 Git commit:        cb74dfc
 Built:             Thu May 25 21:51:16 2023
 OS/Arch:           darwin/arm64
 Context:           desktop-linux

Server: Docker Desktop 4.21.0 (113844)
 Engine:
  Version:          24.0.2
  API version:      1.43 (minimum version 1.12)
  Go version:       go1.20.4
  Git commit:       659604f
  Built:            Thu May 25 21:50:59 2023
  OS/Arch:          linux/arm64
  Experimental:     false
 containerd:
  Version:          1.6.21
  GitCommit:        3dce8eb055cbb6872793272b4f20ed16117344f8
 runc:
  Version:          1.1.7
  GitCommit:        v1.1.7-0-g860f061
 docker-init:
  Version:          0.19.0
  GitCommit:        de40ad0
```
## 実行方法

ターミナルからデータを入力する場合
```bash
docker run --rm --platform linux/amd64 -it kobae964/qepcad +N10000000
```

ファイルからデータを入力する場合
```bash
docker run --rm --platform linux/amd64 -i kobae964/qepcad +N10000000 <filename.txt
```

(これをそのまま実行すると、初回実行時に Docker イメージを取得する時間がかかります。それを実行時間に含めたくない場合は、あらかじめ
```bash
docker pull kobae964/qepcad
```
を実行しておけば、docker run ではすぐに実行が始まります。)

## 何をやっているのか
https://github.com/koba-e964/qepcad-image で、実際に QEPCAD をインストールする手順を Dockerfile に書き、Docker イメージを作りました。
イメージは https://hub.docker.com/repository/docker/kobae964/qepcad/general で公開されており、上の手順ではそれを取得していたのです。

## 関連記事
https://qiita.com/kobae964/items/30dcb77837068b2638be
