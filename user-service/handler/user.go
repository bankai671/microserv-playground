package handler

import (
	"log"
    "strconv"
	"github.com/bankai671/microserv-playground/user-service/database"
	"github.com/bankai671/microserv-playground/user-service/database/sqlc"
	"github.com/gofiber/fiber/v3"
)

type CreateUserRequest struct {
    Email       string  `json:"email"`
    Username    string  `json:"username"`
    Password    string  `json:"password"`
}

func GetUsers(c fiber.Ctx) error {
    email := c.Query("email")

    if email != "" {
        user, err := database.Queries.GetUserByEmail(c.Context(), email)
        if err != nil {
            log.Println("Error retrieving user by email:", err)
            return c.Status(fiber.StatusNotFound).SendString("User not found!")
        }

        return c.Status(fiber.StatusOK).JSON(user)
    }

    users, err := database.Queries.GetAllUsers(c.Context())

    if err != nil {
        return err
    }

    return c.Status(fiber.StatusOK).JSON(users)
}

func CreateUser(c fiber.Ctx) error {
    user := CreateUserRequest {}
    if err := c.Bind().JSON(&user); err != nil {
        return err
    }

    createdUser, err := database.Queries.CreateUser(c.Context(), sqlc.CreateUserParams {
        Email: user.Email,
        Username: user.Username,
        Password: user.Password,
    })

    if err != nil {
        log.Println(err)
        return c.Status(fiber.StatusConflict).SendString("User already exist!")
    }
    
    return c.Status(fiber.StatusCreated).JSON(createdUser)
}

func GetUserByID(c fiber.Ctx) error {
    query_id, parse_err := strconv.ParseInt(c.Params("id"), 10, 32)
    
    if parse_err != nil {
        log.Println(parse_err)
    }

    user, err := database.Queries.GetUserByID(c.Context(), int32(query_id))
    
    if err != nil {
        log.Println(err)
        return c.Status(fiber.StatusNotFound).SendString("User not found!")
    }

    return c.Status(fiber.StatusOK).JSON(user)
}

func DeleteUser(c fiber.Ctx) error {
    param_id, parse_err := strconv.ParseInt(c.Params("id"), 10, 32)
    
    if parse_err != nil {
        log.Println(parse_err)
    }

    if err := database.Queries.DeleteUser(c.Context(), int32(param_id)); err != nil {
        log.Println(err)
        return c.Status(fiber.StatusNotFound).SendString("User not found!")
    }
    
    return c.Status(fiber.StatusNoContent).SendString("User successfuly deleted!")
}

func HealthCheck(c fiber.Ctx) error {
    return c.Status(fiber.StatusOK).SendString("Server is running!")
}

