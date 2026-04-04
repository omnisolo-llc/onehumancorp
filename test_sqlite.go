package main

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func main() {
	pool, err := db.New(context.Background())
	if err != nil {
		fmt.Printf("failed to init db: %v\n", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		fmt.Printf("failed migrations: %v\n", err)
	}
}
