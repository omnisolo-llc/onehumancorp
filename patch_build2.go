package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/db/BUILD.bazel"
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	content := string(contentBytes)

	if strings.Contains(content, "\"@com_github_redis_rueidis//mock\",") {
		content = strings.Replace(content, "        \"@com_github_redis_rueidis//mock\",\n", "", 1)
		err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully modified BUILD.bazel")
	} else {
		fmt.Println("BUILD.bazel already patched")
	}
}
