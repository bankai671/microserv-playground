package handler

import (
    "github.com/gofiber/fiber/v3"
)

func CreateUser(c fiber.Ctx) error {
    return c.SendString("POST: hello from user-service /user endpoint")
}
