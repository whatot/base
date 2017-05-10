FROM ubuntu:16.04

RUN sed -i 's/archive/cn.archive/g' /etc/apt/sources.list
RUN apt-get update \
    && apt-get install -y gdebi-core wget \
    && wget http://s1.music.126.net/download/pc/netease-cloud-music_1.0.0-2_amd64_ubuntu16.04.deb -O /tmp/netease.deb \
    && gdebi -n /tmp/netease.deb \
    && rm /tmp/netease.deb \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

# Add netease user
RUN groupadd -r netease && useradd -r -g netease -G audio,video netease \
&& mkdir -p /home/netease/Music && chown -R netease:netease /home/netease

USER netease

ENTRYPOINT [ "/usr/bin/netease-cloud-music" ]
