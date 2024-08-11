package main

import (
    "github.com/bankai671/microserv-playground/user-service/router"

    "github.com/gofiber/fiber/v3"
    "log"
)

func main() {
    app := fiber.New()
    
    router.SetupRoutes(app)

    log.Print("Server is started and listening port: 8002")
    log.Fatal(app.Listen(":8002"))
}
