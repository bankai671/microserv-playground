package main

import (
	"log"

	"github.com/bankai671/microserv-playground/user-service/router"
    "github.com/bankai671/microserv-playground/user-service/database"

	"github.com/gofiber/fiber/v3"
)

func init() {
    database.ConnectDB()
}

func main() {
    app := fiber.New()
    
    router.SetupRoutes(app)

    log.Fatal(app.Listen(":8002"))
}
