.PHONY: clean distclean up

clean:
	sudo rm -rf cockroach0-data cockroach0-log \
		cockroach1-data cockroach1-log \
		cockroach2-data cockroach2-log \
		cockroach3-data cockroach3-log \
		cockroach4-data cockroach4-log \

distclean: clean
	sudo docker-compose rm

up:
	sudo docker-compose up
