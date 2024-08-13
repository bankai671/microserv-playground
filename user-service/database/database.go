package database

import (
	"context"
	"fmt"
	"log"

	"github.com/bankai671/microserv-playground/user-service/config"
	"github.com/bankai671/microserv-playground/user-service/database/sqlc"
	"github.com/jackc/pgx/v5"
)

var DB *pgx.Conn
var Queries *sqlc.Queries

func ConnectDB () {
    ctx := context.Background()
    conn, err := pgx.Connect(ctx, config.Config("DB_URL"))

    if err != nil {
        log.Fatal(err)
    }

    if err := conn.Ping(ctx); err != nil {
        log.Fatal(err)
    }

    DB = conn
    Queries = sqlc.New(DB)

    fmt.Println("Connection to database established!")
}
