.PHONY: up clean distclean

up:
	sudo docker-compose up

clean:
	rm -rf web/__pycache__/
	sudo rm -rf pgdata

distclean: clean
	sudo docker-compose rm
