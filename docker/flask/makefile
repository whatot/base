.PHONY: up clean distclean

up:
	sudo docker-compose up

clean:
	sudo rm -rf pgdata/
	sudo rm -rf nginx-log/

distclean: clean
	sudo docker-compose rm
