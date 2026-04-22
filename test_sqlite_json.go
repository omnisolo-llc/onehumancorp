package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"

	_ "modernc.org/sqlite"
)

func main() {
	db, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			dependencies TEXT NOT NULL DEFAULT '[]'
		);
	`)
	if err != nil {
		log.Fatal(err)
	}

	_, err = db.Exec(`
		INSERT INTO shared_tasks_decomposition (id, status, dependencies) VALUES
		('task1', 'COMPLETED', '[]'),
		('task2', 'PENDING', '["task1"]'),
		('task3', 'PENDING', '["task1", "task4"]'),
		('task4', 'PENDING', '[]');
	`)
	if err != nil {
		log.Fatal(err)
	}

	query := `
		SELECT
			id,
			CASE WHEN json_array_length(dependencies) = 0 THEN 1
				 WHEN (SELECT COUNT(*) FROM json_each(dependencies) j LEFT JOIN shared_tasks_decomposition d ON j.value = d.id WHERE d.status IN ('COMPLETED', 'SUCCESS')) = json_array_length(dependencies) THEN 1
				 ELSE 0
			END AS all_met
		FROM shared_tasks_decomposition
	`
	rows, err := db.QueryContext(context.Background(), query)
	if err != nil {
		log.Fatal(err)
	}
	defer rows.Close()

	for rows.Next() {
		var id string
		var allMet int
		if err := rows.Scan(&id, &allMet); err != nil {
			log.Fatal(err)
		}
		fmt.Printf("id: %s, allMet: %d\n", id, allMet)
	}
}
