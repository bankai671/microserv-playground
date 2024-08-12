package main

import (
	"context"
	"log"
	"github.com/bankai671/microserv-playground/user-service/config"
	"github.com/bankai671/microserv-playground/user-service/internal/db"
	"github.com/bankai671/microserv-playground/user-service/router"
	"github.com/jackc/pgx/v5"

	"github.com/gofiber/fiber/v3"
)

func main() {
    ctx := context.Background()

    app := fiber.New()
    
    log.Printf("env url: %s", config.Config("DB_URL"))

    conn, err := pgx.Connect(ctx, config.Config("DB_URL"))
    if err != nil {
        log.Fatal(err)
    }

    defer conn.Close(ctx)
    
    queries := user.New(conn)
    
    createdUser, err := queries.CreateUser(ctx, user.CreateUserParams {
        Email: "pizdec",
        Username: "test",
        Password: "pohui",
    })

    log.Print(createdUser)

    if err != nil {
        log.Fatal(err)
    }

    router.SetupRoutes(app)

    log.Print("Server is started and listening port: 8002")
    log.Fatal(app.Listen(":8002"))
}
