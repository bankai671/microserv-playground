package handler

import (
	"log"

	"github.com/bankai671/microserv-playground/user-service/database"
	"github.com/bankai671/microserv-playground/user-service/database/sqlc"
	"github.com/gofiber/fiber/v3"
)

type CreateUserRequest struct {
    Email       string  `json:"email"`
    Username    string  `json:"username"`
    Password    string  `json:"password"`
}

func CreateUser(c fiber.Ctx) error {
    ctx := c.Context()
    
    user := CreateUserRequest {}

    if err := c.Bind().JSON(&user); err != nil {
        return err
    }

    createdUser, err := database.Queries.CreateUser(ctx, sqlc.CreateUserParams {
        Email: user.Email,
        Username: user.Username,
        Password: user.Password,
    })

    if err != nil {
        log.Println(err)
        return c.Status(fiber.StatusConflict).SendString("User already exist!")
    }
    
    return c.Status(fiber.StatusOK).JSON(createdUser)
}

