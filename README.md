<div align="center">

<h1>第2回: Rustを導入する</h1>

<b>QEMUクロスビルド環境の準備、TOPPERS/ASP3やFreeRTOSなどのMakefileベースのビルドシステムへの組み込み、Cコードとの連携<br>
Cベースの既存プロジェクト・開発プラットフォームでRustを導入する方法を学ぶ</b>

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)][2]

</div>

> 既存システムにRustを導入する際は、部分的な移行から始めるのが現実的ですが、最初からRustで開発する場合とは異なる課題が伴います。そこで今回は、TOPPERS/ASP3アプリケーションをベースにRustコードを組み込む方法と、クロスコンパイルやFFIなどの必要な知識を学びます。

## ディレクトリ構成

- [`asp3/`](asp3) には、TOPPERS/ASP3リアルタイムカーネルのソースコードが含まれます。

- [`app/`](app) には、サンプルアプリケーションのソースコードが含まれています。

    - 一般的な開発タスク (例: `make run`でQEMU上でのビルドと実行) を行うための [`Makefile`](app/Makefile) が含まれています。

    - [`main.c`](app/main.c) にメインタスク (`main_task`) の実装が含まれます。

    - [`main.h`](app/main.h) に `app.cfg` から参照される関数宣言などが含まれます。

    - [`app.cfg`](app/app.cfg) はカーネルコンフィギュレーションファイルです。

- [`app/out/`](app/out) にはアプリケーションのビルドに関係するファイルが含まれます。

    - アプリケーションをビルドするための [`Makefile`](app/out/Makefile) がここにあります。

    - ビルド完了後のビルド生成物はここに格納されます。

- [`app/rustapp/`](app/rustapp) にはCargoパッケージが含まれます。
    
    - [`Cargo.toml`](app/rustapp/Cargo.toml) にパッケージの定義が含まれます。
    
    - [`main.rs`](app/rustapp/src/main.rs)/[`lib.rs`](app/rustapp/src/lib.rs) がルートソースファイルです。

## はじめに

### 事前準備: GitHub Codespaces

[GitHub Codespaces](https://github.com/codespaces)を利用するとブラウザ上でLinuxベースの開発環境をすぐに準備できます。
2022年から[無料枠][1]が追加されました。

[**こちらのリンクからセットアップ済みのCodespaceを作成できます。**][2]

### 事前準備: Ubuntu

APTを使用して必要なツールをインストール:

```console
$ sudo update
$ sudo apt install wget ruby build-essential gcc-arm-none-eabi gdb-multiarch qemu-system-arm
```

### ビルドと実行

`app/`ディレクトリで`make run`を実行し、QEMU上でアプリケーションをビルド・実行:

```console
$ cd app
$ make run
```

## 便利なリンク

- [TOPPERS/ASP3をQEMUで動作させる](https://qiita.com/exshonda/items/bab3921161919feaba6f) - Ubuntu 22.04 LTS上でTOPPERS/ASP3アプリケーションをビルド・実行するための詳細な手順が記載されています。

[1]: https://github.blog/changelog/2022-11-09-codespaces-for-free-and-pro-accounts/
[2]: https://codespaces.new/ycst2025/part-2?quickstart=1
