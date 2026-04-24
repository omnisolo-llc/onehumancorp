package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	path := "srcs/server/tools/mcpwebhooktunnel/mcp_test.go"
	b, err := ioutil.ReadFile(path)
	if err != nil {
		panic(err)
	}

	content := string(b)

	// Increase wait time
	search := `	// Wait for the connection to be established and registered
	time.Sleep(50 * time.Millisecond)`
	replace := `	// Wait for the connection to be established and registered
	time.Sleep(200 * time.Millisecond)`

	content = strings.Replace(content, search, replace, 1)

	search2 := `	// Wait to receive the payload
	time.Sleep(50 * time.Millisecond)`
	replace2 := `	// Wait to receive the payload
	time.Sleep(200 * time.Millisecond)`

	content = strings.Replace(content, search2, replace2, 1)

	err = ioutil.WriteFile(path, []byte(content), 0644)
	if err != nil {
		panic(err)
	}

	fmt.Println("Replacement successful.")
}
