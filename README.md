# 環境構築

前提事項：
- Windowsの場合、WSL2をインストールし、Ubuntu上で行うこと
- brewをインストールしておくこと

## zigのインストール

cargo lambda build時にzigを使用するため

```bash
brew install zig
```

## cargo-lambdaのインストール

Lambda用のバイナリにビルド、デプロイを行うために使用します。

```bash
brew tap cargo-lambda/cargo-lambda
```

```bash
brew install cargo-lambda
```

aarch64-unknown-linux-gnuツールチェインを追加します。

```bash
rustup target add aarch64-unknown-linux-gnu
```

# プロジェクトの新規作成

今回は rust-lambda という名称でプロジェクトを作成します。

```bash
cargo lambda new rust-lambda
```

すると、下記のように質問されます：

```bash
? Is this function an HTTP function? (y/N)
[type `yes` if the Lambda function is triggered by an API Gateway, Amazon Load Balancer(ALB), or a Lambda URL]
```

yesの場合、HTTP関数を作成します
noの場合、イベント駆動型の関数を作成します

# ビルド

```bash
cargo lambda build
```

Lambdaにデプロイする際には --release オプションを付けます。

```bash
cargo lambda build --release
```

ビルドして、zip化することも可能です。

```bash
cargo lambda build --output-format zip
```

デプロイ用にビルドしてzipすることも可能です。

```bash
cargo lambda build --output-format zip --release
```

# ローカル実行

ローカル上でLambdaのエミュレータサーバーを起動することができます。
※エミュレータサーバーでは function url を使用してエミュレートします。

```bash
cargo lambda watch
```

curl コマンドでトリガー

```bash
curl -L http://127.0.0.1:9000/lambda-url/rust-lambda
```

invokeでトリガー

```bash
cargo lambda invoke rust-lambda --data-example apigw-request
```

# デプロイ

IAMロールのARNを記述します。

```bash
cargo lambda deploy --enable-function-url my-function-1 --iam-role <IAMロールのARN>
```

後は、API Gatewayにて各パスを設定し、PostmanまたはVS Codeの拡張機能であるREST Client等でAPIコールできます。