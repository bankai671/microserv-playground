package router

import (
	"github.com/bankai671/microserv-playground/user-service/handler"
	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/logger"
)

func SetupRoutes(app *fiber.App) {
    app.Use(logger.New())
    app.Get("/", handler.HealthCheck)
    app.Get("/users", handler.GetUsers)
    app.Post("/users", handler.CreateUser)
    app.Get("/users/:id", handler.GetUserByID)
    app.Delete("/users/:id", handler.DeleteUser)
}
