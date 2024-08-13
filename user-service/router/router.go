package router

import (
    "github.com/bankai671/microserv-playground/user-service/handler"
    "github.com/gofiber/fiber/v3"
)

func SetupRoutes(app *fiber.App) {
    app.Get("/", func (c fiber.Ctx) error {
        return c.SendString("hello from user-service / endpoint, port 8002")
    })

    app.Post("/user", handler.CreateUser)
}
