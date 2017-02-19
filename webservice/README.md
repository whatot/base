# webservice explain

## docker usage

check the files inside a image
```
sudo docker run -i --entrypoint /bin/bash -t image-name
```

```
sudo docker build -f flask/Dockerfile -t flask-post .
sudo docker run -i --rm -p 8201:8201 --name flask-post whatot/flask-post
sudo docker run -i --rm -p 5432:5432 --name postgres postgres:latest
sudo docker ps
sudo docker inspect --format '{{ .NetworkSettings.IPAddress }}' ID
```

`-d` means `detached mode`, more restriction for long-time running
```
sudo docker run -d --cpuset-cpus 1 --name flask-todo --restart=always whatot/flask-todo
```

## docker-compose usage

```
sudo docker-compose up  # run in foreground
sudo docker-compose up -d  # run in background
sudo docker-compose ps
sudo docker-compose run web env
sudo docker-compose stop
sudo docker-compose down --volumes  # also remove the data-volumes
```
