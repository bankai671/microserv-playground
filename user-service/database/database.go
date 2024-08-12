package database

import (
    "fmt"
    "log"
	"database/sql"
	"github.com/bankai671/microserv-playground/user-service/config"
)

var DB *sql.DB

func ConnectDB () {
    conn, err := sql.Open("posgres", config.Config("DB_URL"))
    
    if err != nil {
        log.Fatal(err)
    }

    if err := conn.Ping(); err != nil {
        log.Fatal(err)
    }

    DB = conn
    fmt.Println("Connection to database established!")
}
