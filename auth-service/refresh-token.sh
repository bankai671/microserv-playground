curl -X POST \
    -v \
    -H "Content-Type: application/json" \
    --url localhost:8001/refresh-token \
    -d '{"refresh_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzNSIsImV4cCI6MTcyNDI5ODU4MX0.SdMdBhOmN6X4OcMWgAIrMCt0bPUtnD4aujb9s6Yezh4"}'

