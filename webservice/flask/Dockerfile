# https://hub.docker.com/_/python/
FROM python:3
COPY . /webapp
WORKDIR /webapp
RUN pip install --no-cache-dir -r requirements.txt
EXPOSE 8201
ENTRYPOINT ["gunicorn", "-w", "4", "-k", "gevent", "-b", ":8201", "post:app"]
