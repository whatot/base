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

## command to remove all unused images

* https://docs.docker.com/engine/reference/commandline/system_prune/
* http://stackoverflow.com/questions/32723111/how-to-remove-old-and-unused-docker-images
* https://forums.docker.com/t/command-to-remove-all-unused-images/20

### system cmd from docker upstream, delete ALL unused data
```
sudo docker system prune
```

cmds for each resource type
```
docker container prune
docker image prune
docker network prune
docker volume prune
```

### removing all stopped docker containers
it will list all images (docker ps), but only show the id's.
And then run a docker rm command for each one of them.
```
sudo docker ps -q |xargs docker rm
```

### in order to get rid of all untagged images
```
sudo docker rmi -f $(docker images | grep "<none>" | awk "{print \$3}")
```

### remove weeks/months ago docker images
we can add `hours|days|weeks|months` to clean more, but is too dangerous.
```
sudo docker images | grep " [months|weeks]* ago" | awk '{print $3}' | xargs docker rmi
```
