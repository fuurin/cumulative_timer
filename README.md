# 積み上げタイマー

Rust + Postgres + Vue + Nginx 環境での開発練習。
開発手順は [こちら](https://scrapbox.io/programming-technology/docker-compose%E3%81%A7Vue%E3%83%95%E3%83%AD%E3%83%B3%E3%83%88%EF%BC%8BRust%E3%83%90%E3%83%83%E3%82%AF%EF%BC%8BPostgreSQL)

## 環境変数の設定
- sample.envをコピーして.envを作成
- .env内の空になっている環境変数を埋める

**環境変数を追加するときには、sample.envにも名前だけ追加すること！**

## 起動と終了
``` bash
$ make

↓起動したら別タブで
$ make migrate
```
- サービス：http://localhost
- API：http://localhost:8000

Ctrl + Cで終了
