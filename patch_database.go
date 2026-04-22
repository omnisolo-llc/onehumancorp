package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/db/database.go"
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	content := string(contentBytes)

	if strings.Contains(content, "return &DB{Provider: NewPgProvider(pool)}, nil") {
		content = strings.Replace(content, "return &DB{Provider: NewPgProvider(pool)}, nil", "return &DB{Provider: NewPgProvider(pool, nil)}, nil", 1)
		err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully modified database.go")
	} else {
		fmt.Println("database.go already patched")
	}
}
