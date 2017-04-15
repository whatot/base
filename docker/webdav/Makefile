.PHONY: clean distclean up daemon stop

clean:
	sudo rm -rf webdav-log

distclean: clean
	sudo docker-compose rm

up:
	sudo docker-compose up

stop:
	sudo docker-compose stop

daemon:
	sudo docker-compose up -d
