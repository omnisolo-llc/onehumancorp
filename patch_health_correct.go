package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/orchestration/health_test.go")
	if err != nil {
		fmt.Println(err)
		return
	}

	newContent := string(content)

	// Simply replace the first Ping implementation with an empty string, leaving the second one intact
	firstPing := `func (m *mockProvider) Ping(ctx context.Context) error {
	return m.execErr
}`

	newContent = strings.Replace(newContent, firstPing, "", 1)

	ioutil.WriteFile("srcs/server/orchestration/health_test.go", []byte(newContent), 0644)
}
