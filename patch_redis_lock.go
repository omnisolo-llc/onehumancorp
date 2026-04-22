package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/db/postgres_provider.go"
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	content := string(contentBytes)

	// we need to add Redis distributed locking import
	if !strings.Contains(content, "\"github.com/redis/rueidis\"") {
		content = strings.Replace(content, "import (", "import (\n\t\"github.com/redis/rueidis\"", 1)
		err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully added rueidis to postgres_provider.go")
	}

    // add redisClient to PgProvider
    if !strings.Contains(content, "redisClient rueidis.Client") {
        content = strings.Replace(content, "pool *pgxpool.Pool", "pool *pgxpool.Pool\n\tredisClient rueidis.Client", 1)
        err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully added redisClient to PgProvider")
    }
}
