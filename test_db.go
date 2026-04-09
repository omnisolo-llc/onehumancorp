package main

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func main() {
	p, err := db.NewSqliteProvider("file::memory:?cache=shared")
	if err != nil {
		fmt.Println(err)
		return
	}
	ctx := context.Background()
	tx, _ := p.Begin(ctx)
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, "CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT NOT NULL, status TEXT NOT NULL);")
	if err != nil {
		fmt.Println("Error 1", err)
	}

	_, err = tx.Exec(ctx, "CREATE INDEX idx_test ON shared_tasks(organization_id, status);")
	if err != nil {
		fmt.Println("Error 2", err)
	}

	fmt.Println("Success")
}
