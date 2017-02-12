# flask webservice example

## flask steps

```
pip install Flask
```

use curl check the basic flask todo server
```
curl -X GET http://0.0.0.0:8201/todo
curl -X POST http://0.0.0.0:8201/todo -d content=add-a-todo
curl -X PUT http://0.0.0.0:8201/todo/f69bbf8c-cdc6-4d24-b8b6-00b26aa347b7/finish
curl -X DELETE http://0.0.0.0:8201/todo/f69bbf8c-cdc6-4d24-b8b6-00b26aa347b7
```
