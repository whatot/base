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

## invalid json request
curl -X POST -v 127.0.0.1:8080/v1/login -H 'Content-Type: application/json' -d '{"user": "admin123", "password": "pass"}'

## no matched username
curl -X POST -v 127.0.0.1:8080/v1/login -H 'Content-Type: application/json' -d '{"username": "admin123", "password": "pass"}'

## error password
curl -X POST -v 127.0.0.1:8080/v1/login -H 'Content-Type: application/json' -d '{"username": "admin", "password": "pass"}'

## success password
curl -X POST -v 127.0.0.1:8080/v1/login -H 'Content-Type: application/json' -d '{"username": "admin", "password": "Pa$$wd123"}'


curl -v -XPOST -d '{"name": "Fido", "description": "...", "date_of_birth": "2022-01-01", "chip_number": "1234", "gender": "male", "is_sterilized": true, "breed": "mixed", "size": "medium", "weight": 25, "hair": "brown"}' \
   -H 'Content-Type: application/json' \
   -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImlhdCI6MTcwNDQ3ODk3OSwiZXhwIjoxNzA0NDgyNTc5fQ.KRLduzwEm7HwKP-o-pAkURSfYbFJ_r3cxGjYEyEafHI' \
   http://127.0.0.1:8080/v1/dogs



curl -v -XPOST -d '{"name": "Fido","hair": "brown"}' \
   -H 'Content-Type: application/json' \
   -H 'Authorization: Bearer eyJ0eXAiOiJxNzA0NDgyNTc5fQ.KRLduzwEm7HwKP-o-pAkURSfYbFJ_r3cxGjYEyEafHI' \
   http://127.0.0.1:8080/v1/dogs
```
