#!/usr/bin/env bash
set -xue

docker run -d \
-v /home/mir/git/base/docker/jellyfin/config:/config \
-v /home/mir/git/base/docker/jellyfin/cache:/cache \
-v /home/mir/other/pts/:/media \
--net=host \
--name=jellyfin \
--restart=always \
--runtime=nvidia \
--gpus all \
nyanmisaka/jellyfin:latest
