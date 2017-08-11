.PHONY: up daemon stop clean distclean

up:
	docker-compose up

daemon:
	docker-compose up -d

stop:
	docker-compose stop

clean:
	docker-compose rm

distclean: clean
	sudo rm ./pgdata -rf
