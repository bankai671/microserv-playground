curl -X POST \
    -v \
    -H "Content-Type: application/json" \
    --url localhost:8002/user \
    -d '{"email":"user@gmail.com","username":"user","password":"password"}'

