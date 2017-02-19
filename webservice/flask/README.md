# flask webservice module

## post project

### install deps

in a python3 virtualenv
```
pip install Flask
pip install --no-cache-dir -r requirements.txt
```

### init postgres

set test config in config.py, then init db
```
sudo docker run -i --rm -p 5432:5432 --name postgres postgres:latest
python create_db.py
psql -h 127.0.0.1 -p 5432 -U postgres
```

### run with gunicorn

default with `sync` worker, `-k gevent` set `gevent` async worker
```
gunicorn -w 4 -b :8201 post:app
gunicorn -w 4 -k gevent -b :8201 post:app
```

## todo project

### basic tests for todo by curl

use curl check the basic flask todo server
```
curl -X GET http://0.0.0.0:8201/todo
curl -X POST http://0.0.0.0:8201/todo -d content=add-a-todo
curl -X PUT http://0.0.0.0:8201/todo/f69bbf8c-cdc6-4d24-b8b6-00b26aa347b7/finish
curl -X DELETE http://0.0.0.0:8201/todo/f69bbf8c-cdc6-4d24-b8b6-00b26aa347b7
```


