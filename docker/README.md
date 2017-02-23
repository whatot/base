# webservice explain

## docker usage

check the files inside a image
```
sudo docker run -i --rm --entrypoint /bin/bash -t image-name
```

Use the `docker exec` command to use the command inside the container
```
sudo docker exec -it postgres-web /bin/bash
```

build and run example
```
cd flask/web/
sudo docker build -t flask-todo .
sudo docker run -i --rm -p 8201:8201 --name flask-todo whatot/flask-todo
sudo docker run -i --rm -p 5432:5432 --name postgres-web postgres:latest
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

### entrypoint and command in docker-compose.yml

whatever specified in the "command" in docker-compose should get appended
to the entrypoint defined in the dockerfile provided entrypoint is defined
in exec form in the DockerFile.

If the EntryPoint is defined in shell form, then any CMD arguments will be ignored.
