package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/orchestration/health_test.go")
	if err != nil {
		panic(err)
	}

	content := string(b)

	toReplace := `func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execErr != nil {
		return 0, m.execErr
	}
	return 1, nil
}


func (m *mockProvider) Ping(ctx context.Context) error {
	if m.execErr != nil {
		return m.execErr
	}
	return nil
}`

	newContent := `func (m *mockProvider) Ping(ctx context.Context) error {
	return m.execErr
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execErr != nil {
		return 0, m.execErr
	}
	return 1, nil
}`

	if !strings.Contains(content, toReplace) {
		fmt.Println("toReplace not found!")
	} else {
		content = strings.Replace(content, toReplace, newContent, 1)
		err = ioutil.WriteFile("srcs/server/orchestration/health_test.go", []byte(content), 0644)
		if err != nil {
			panic(err)
		}
		fmt.Println("health_test.go updated!")
	}
}
