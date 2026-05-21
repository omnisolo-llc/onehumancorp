package main

import (
    "database/sql"
    "log"
    "net/http"
    "os"

    _ "github.com/lib/pq"
    "github.com/onehumancorp/mono/srcs/server/api"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func main() {
    dbURL := os.Getenv("DATABASE_URL")
    if dbURL == "" {
        dbURL = "postgres://postgres:postgres@localhost:5432/test?sslmode=disable"
    }

    database, err := sql.Open("postgres", dbURL)
    if err != nil {
        log.Fatalf("Failed to connect to db: %v", err)
    }

    store := db.NewAutoDreamStore(database)
    apiHandler := api.NewAutoDreamAPI(store)

    http.HandleFunc("/api/autodream/query", apiHandler.HandleQuery)

    port := os.Getenv("PORT")
    if port == "" {
        port = "8081"
    }

    log.Printf("Starting Go AutoDream API on port %s", port)
    if err := http.ListenAndServe(":"+port, nil); err != nil {
        log.Fatalf("Failed to start server: %v", err)
    }
}
