# https://hub.docker.com/_/python/
# https://github.com/docker-library/python/blob/master/3.6/alpine/Dockerfile
FROM python:3-alpine
COPY . /webapp
WORKDIR /webapp
RUN pip install --no-cache-dir -r requirements.txt
EXPOSE 8201
ENTRYPOINT ["python", "flask_todo_server.py"]
