# https://hub.docker.com/_/python/
# https://github.com/docker-library/python/blob/master/3.6/alpine/Dockerfile
FROM python:3.6-alpine

RUN mkdir -p /webapp
COPY requirements.txt /webapp
RUN pip install --no-cache-dir -r /webapp/requirements.txt
COPY flask_todo_server.py /webapp

ENV HOME /webapp
WORKDIR /webapp

EXPOSE 8201
ENTRYPOINT ["python", "flask_todo_server.py"]
