# Do not use nginx as base since extras are not included.
FROM      debian:stable

RUN       sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list && \
          sed -i 's|security.debian.org/debian-security|mirrors.ustc.edu.cn/debian-security|g' /etc/apt/sources.list && \
          apt-get update && \
          apt-get install -y nginx nginx-extras apache2-utils && \
          rm -rf /var/lib/apt/lists/*

COPY      set_htpasswd.sh /set_htpasswd.sh
COPY      webdav-site.conf /etc/nginx/sites-enabled/
RUN       rm /etc/nginx/sites-enabled/default
# Overwrite mimetypes to add rss format.
COPY      mime.types /etc/nginx/

# forward request and error logs to docker log collector
RUN       ln -sf /dev/stdout /var/log/nginx/access.log
RUN       ln -sf /dev/stderr /var/log/nginx/error.log

# Create folder where webdav files end up.
RUN       mkdir -p /var/www/.temp
RUN       chown -R www-data:www-data /var/www
RUN       chmod -R a+rw /var/www

# Share the volume with the files to other dockers
VOLUME    /var/www

CMD       /set_htpasswd.sh && nginx -g "daemon off;"
