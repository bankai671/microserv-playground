FROM golang:latest

WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download

COPY *.go .

RUN go build -o user-service main.go

EXPOSE 8002

CMD ["./user-service"]

