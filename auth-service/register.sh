curl -X POST \
    -H "Content-Type: application/json" \
    --url localhost:3001/register \
    -d '{"email":"test@gmail.com","username":"test","password":"password","confirm_password":"password"}'
