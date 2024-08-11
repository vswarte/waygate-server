// TODO: expand once more API endpoints make it in

Example healtcheck call:
```bash
$ curl -X GET http://localhost:10902/health \
    --header "X-Auth-Token: <API KEY>" \
    --header "Content-Type: application/json"         
```

Example announcement call:
```bash
$ curl -v -X POST http://localhost:10902/notify/message \
    --header "X-Auth-Token: <API KEY>" \
    --header "Content-Type: application/json" \
    --data '{"message":"Test Announcement"}'
```
