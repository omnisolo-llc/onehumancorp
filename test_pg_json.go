package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"

	_ "github.com/lib/pq"
)

func main() {
	// Not actually running against a real db here unless we have one. But let's look at the query string format.
	fmt.Println("Tested SQLite, Postgres syntax is roughly `jsonb_array_length(dependencies) = 0 OR (SELECT COUNT(*) FROM jsonb_array_elements_text(dependencies) d JOIN shared_tasks_decomposition dep ON dep.id = d WHERE dep.status IN ('COMPLETED', 'SUCCESS')) = jsonb_array_length(dependencies)`")
}
