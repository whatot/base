# cockroach usage

ensure docker is running, network is ok, then in shell:

## start the cockroach cluster
```
make up
```

## use the 'cockroach sql', CockroachDB's lightweight SQL client

login in the 'cockroach sql'
```
sudo docker exec -it cockroach_roach0_1 ./cockroach sql
sudo docker exec -it cockroach_roach2_1 ./cockroach sql
```

cmds in `cockroach sql`:
```
show databases;
CREATE DATABASE bank;
CREATE TABLE bank.accounts (id INT PRIMARY KEY, balance DECIMAL);
INSERT INTO bank.accounts VALUES (1, 1000.50);
SELECT * FROM bank.accounts;
```

## use the postgresql's standard psql

in psql:
```
psql -h 127.0.0.1 -p 26257 -U root
show databases;
\c bank;
show tables;
SELECT * FROM accounts;
SELECT * FROM bank.accounts;
```

but `\d` and `\l` return error like below:
```
root=> \d
ERROR:  unknown function: pg_catalog.pg_table_is_visible()
root=> \l
ERROR:  unknown function: pg_catalog.pg_encoding_to_char()
```
