.PHONY: rebuild build clean distclean up daemon stop

rebuild: distclean
	sudo docker rmi webdav_webdav:latest
	sudo docker-compose build

build:
	sudo docker-compose build

clean:
	sudo rm -rf webdav-log

distclean: clean
	sudo docker-compose rm -f

up:
	sudo docker-compose up

stop:
	sudo docker-compose stop

daemon:
	sudo docker-compose up -d
