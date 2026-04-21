package main

import (
	"context"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/services/growth"
)

func main() {
	ctx := context.Background()
	os.Setenv("DATABASE_URL", "sqlite://:memory:")

	database, err := db.New(ctx)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	if err := database.RunMigrations(ctx); err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	it := growth.NewInviteTracker(database)

	database.Close()

	err = it.RecordInvite(ctx, "team1", "user1", "user2")
	fmt.Printf("Error after close: %v\n", err)
}
