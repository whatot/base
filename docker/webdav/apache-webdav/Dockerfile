FROM ubuntu:16.10

RUN sed -i 's/archive/cn.archive/g' /etc/apt/sources.list
RUN apt-get update \
    && apt-get install -y apache2 apache2-utils \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

RUN a2enmod dav dav_fs
RUN a2dissite 000-default

ENV APACHE_RUN_USER www-data
ENV APACHE_RUN_GROUP www-data
ENV APACHE_LOG_DIR /var/log/apache2
ENV APACHE_PID_FILE /var/run/apache2.pid
ENV APACHE_LOCK_DIR /var/lock/apache2
ENV APACHE_RUN_DIR /var/run/apache2

RUN mkdir -p /var/lock/apache2 \
    && chown www-data /var/lock/apache2 \
    && mkdir -p /var/webdav \
    && chown www-data /var/webdav

ADD webdav.conf /etc/apache2/sites-available/webdav.conf
RUN a2ensite webdav

ADD run.sh /

EXPOSE 80

VOLUME /var/webdav

CMD ["/run.sh"]
