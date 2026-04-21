package main

import (
	"io/ioutil"
	"strings"
	"fmt"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/orchestration/tasks_test.go")
	if err != nil { panic(err) }

	content := string(b)

	content = strings.ReplaceAll(content, "dbProvider, err := db.NewSqliteProvider(\":memory:\")", "dbProvider, err := db.NewSQLiteProvider(\":memory:\")")

	// Wait, the previous error was:
	// srcs/server/orchestration/tasks_test.go:557:24: undefined: db.NewSQLiteProvider (but have NewSqliteProvider)
	// And then I changed it to NewSqliteProvider, and got:
	// srcs/server/orchestration/tasks_test.go:557:21: assignment mismatch: 2 variables but db.NewSqliteProvider returns 1 value
	// srcs/server/orchestration/tasks_test.go:557:42: cannot use ":memory:" (untyped string constant) as *sql.DB value in argument to db.NewSqliteProvider
	// It seems NewSqliteProvider accepts a *sql.DB and returns just the provider.

	// Let's replace the whole test body to just mock a db provider instead.
	// Or we can just use NewMockDBProvider if it exists, or just use a dummy one.

	// Actually, let's look at how tasks_test.go usually creates a dbProvider.
	ioutil.WriteFile("srcs/server/orchestration/tasks_test.go", []byte(content), 0644)
	fmt.Println("tasks_test.go reverted")
}
