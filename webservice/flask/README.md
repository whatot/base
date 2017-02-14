# flask webservice example

## flask

```
pip install Flask
```

use curl check the basic flask todo server
```
curl -X GET http://0.0.0.0:8201/todo
curl -X POST http://0.0.0.0:8201/todo -d content=add-a-todo
curl -X PUT http://0.0.0.0:8201/todo/f69bbf8c-cdc6-4d24-b8b6-00b26aa347b7/finish
curl -X DELETE http://0.0.0.0:8201/todo/f69bbf8c-cdc6-4d24-b8b6-00b26aa347b7
```

## docker

```
sudo docker build --no-cache --rm -t whatot/flask-todo .
sudo docker run -i --rm -p 8201:8201 --name flask-todo whatot/flask-todo
sudo docker ps
sudo docker inspect --format '{{ .NetworkSettings.IPAddress }}' ID
```

`-d` means `detached mode`, more restriction for long-time running
```
sudo docker run -d --cpuset-cpus 1 --name flask-todo --restart=always whatot/flask-todo
```

## docker-compose

```
sudo docker-compose up  # run in foreground
sudo docker-compose up -d  # run in background
sudo docker-compose ps
sudo docker-compose run web env
sudo docker-compose stop
sudo docker-compose down --volumes  # also remove the data-volumes
```
