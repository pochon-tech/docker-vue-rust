# docker-actix-web

## 開発環境の起動

```
docker-compose up -d
```

### DBへのアクセス

```
docker-compose exec db psql -U postgres dvdrental
```

### Vue.jsのためのパッケージ追加

```
docker-compose exec frontend yarn add <package-name>
```

### Rustのcrate追加

```
docker-compose exec backend cargo <crate-name>
```

## 本番環境のビルド

### フロントエンド

```
docker build -t frontend_production ./frontend
```

### バックエンド

```
docker build -t backend_production ./backend
```

### Own Git 

```
rm -rf .git
```

### Documents
https://gitlab.com/yamamoto_kazumasa/docker-actix-web
https://qiita.com/kyamamoto9120/items/c391f0ce5050c5ddcc87