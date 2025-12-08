<div align="center">

<h1>第2回: Rustを導入する</h1>

<b>QEMUクロスビルド環境の準備、TOPPERS/ASP3やFreeRTOSなどのMakefileベースのビルドシステムへの組み込み、Cコードとの連携<br>
Cベースの既存プロジェクト・開発プラットフォームでRustを導入する方法を学ぶ</b>

</div>

> 既存システムにRustを導入する際は、部分的な移行から始めるのが現実的ですが、最初からRustで開発する場合とは異なる課題が伴います。そこで今回は、TOPPERS/ASP3アプリケーションをベースにRustコードを組み込む方法と、クロスコンパイルやFFIなどの必要な知識を学びます。

## ディレクトリ構成

- `asp3/` には、TOPPERS/ASP3リアルタイムカーネルのソースコードが含まれています。

- `app/` には、サンプルアプリケーションのソースコードが含まれています。
ここには、一般的な開発タスク（例: `make run`でQEMU上でのビルドと実行）を行うための`Makefile`が含まれています。

    - `out/` には、アプリケーションをビルドするための`Makefile`が含まれています。
    このディレクトリには、ビルド完了後のビルド生成物も格納されます。

## はじめに

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

- [TOPPERS/ASP3をQEMUで動作させ](https://qiita.com/exshonda/items/bab3921161919feaba6f) - Ubuntu 22.04 LTS上でTOPPERS/ASP3アプリケーションをビルド・実行するための詳細な手順が記載されています。
