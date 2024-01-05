# shelter

## cmds

```shell
## start pg
docker-compose up -d
docker ps

## init migrate
cargo install sea-orm-cli
ea-orm-cli migrate init

## connect pg
docker exec -it shelter-rs-db-1 psql -U postgres -h 127.0.0.1

## in psql
create database shelter;
\c shelter

## execute migrate
./target/debug/shelter_main migrate
```

## curl

```shell

curl -v 127.0.0.1:8080/v1/hello

## no matched username
curl -X POST -v 127.0.0.1:8080/v1/login -H 'Content-Type: application/json' \
-d '{"username": "admin123", "password": "pass"}'

## error password
curl -X POST -v 127.0.0.1:8080/v1/login -H 'Content-Type: application/json' \
-d '{"username": "admin", "password": "pass"}'

## success password
curl -X POST -v 127.0.0.1:8080/v1/login -H 'Content-Type: application/json' \
-d '{"username": "admin", "password": "Pa$$wd123"}'
```
