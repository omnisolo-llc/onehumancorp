package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "modernc.org/sqlite"
)

func main() {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	_, err = db.Exec("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
	if err != nil {
		log.Fatal(err)
	}

	_, err = db.Exec("INSERT INTO test (name) VALUES ($1)", "hello")
	if err != nil {
		log.Fatal(err)
	}

	var name string
	err = db.QueryRow("SELECT name FROM test WHERE name = $1", "hello").Scan(&name)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Success:", name)
}
