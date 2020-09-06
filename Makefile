# 起動
up:
	docker-compose up

# コンテナを作り直して起動
init:
	docker-compose stop
	docker-compose rm -f
	docker-compose build
	docker-compose up

# コンテナに加え、それらに結びつくボリューム(DB, registry等)も削除
clean:
	docker-compose stop
	docker-compose rm -v -f

# コンテナ、イメージ、ボリューム、noneのイメージを削除
clean_all:
	docker-compose down --rmi all --volumes
	docker system prune

# backendのコンテナに入る
back:
	docker-compose exec backend bash

# make for=create_users migration のようにしてdieselでmigrationを作成
migration:
	docker-compose exec backend diesel migration generate ${for}

# migrationを実行
migrate:
	docker-compose exec backend diesel migration run

# redoで最後のmigrationに対してrollbackとrunを行い、動作確認する
migrate_check:
	docker-compose exec backend diesel migration redo

# frontendのコンテナに入る
front:
	docker-compose exec frontend bash
