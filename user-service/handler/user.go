package handler

import (
    "github.com/gofiber/fiber/v3"
)

func CreateUser(c fiber.Ctx) error {
    //ctx := c.Context()

    return c.SendString("POST: hello from user-service /user endpoint")
}
