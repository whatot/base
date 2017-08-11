build:
	docker-compose build

up:
	docker-compose up

daemon:
	docker-compose up -d

scale:
	docker-compose scale sentinel=3
	docker-compose scale slave=2

stop:
	docker-compose stop

clean:
	docker-compose rm

bash:
	docker exec -it rediscluster_sentinel0_1 /bin/bash
