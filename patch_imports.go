package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/orchestration/sip.go")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	s := string(b)
	if !strings.Contains(s, "\"strings\"") {
		s = strings.Replace(s, "\"time\"", "\"time\"\n\t\"strings\"", 1)
	}
	ioutil.WriteFile("srcs/orchestration/sip.go", []byte(s), 0644)
}
