.PHONY: init clean distclean up

init:
	touch minio1.log minio2.log minio3.log minio4.log

clean:
	rm -rf minio1.log minio2.log minio3.log minio4.log
	sudo rm -rf minio1-data minio2-data minio3-data minio4-data

distclean: clean
	sudo docker-compose rm

up: init
	sudo docker-compose up
