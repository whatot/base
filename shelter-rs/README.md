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
