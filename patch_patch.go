package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, _ := ioutil.ReadFile("srcs/server/orchestration/tasks.go")
	fmt.Println(strings.Contains(string(b), "TaskManager"))
}
