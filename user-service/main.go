package main

import (
	"log"

	"github.com/bankai671/microserv-playground/user-service/router"
    "github.com/bankai671/microserv-playground/user-service/database"

	"github.com/gofiber/fiber/v3"
    "github.com/gofiber/fiber/v3/middleware/logger"
)

func init() {
    database.ConnectDB()
}

func main() {
    app := fiber.New()
    
    app.Use(logger.New())

    router.SetupRoutes(app)

    log.Print("Server is started and listening port: 8002")
    log.Fatal(app.Listen(":8002"))
}
