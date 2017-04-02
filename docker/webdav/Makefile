.PHONY: clean distclean up daemon

clean:
	sudo rm -rf webdav-log

distclean: clean
	sudo docker-compose rm

up:
	sudo docker-compose up

daemon:
	sudo docker-compose up -d
