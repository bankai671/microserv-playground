curl -X POST \
    -v \
    -H "Content-Type: application/json" \
    --url localhost:3001/login \
    -d '{"email":"test@gmail.com","username":"test","password":"password"}'
