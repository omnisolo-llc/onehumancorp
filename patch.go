package main

import (
	"bytes"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	path := "srcs/server/orchestration/health_test.go"
	content, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	insertion := `
func (m *mockProvider) Ping(ctx context.Context) error {
	return m.execErr
}
`
	if !strings.Contains(string(content), "func (m *mockProvider) Ping") {
		content = bytes.Replace(content, []byte("func (m *mockProvider) IsSQLite() bool {"), []byte(insertion+"\nfunc (m *mockProvider) IsSQLite() bool {"), 1)
		err = ioutil.WriteFile(path, content, 0644)
		if err != nil {
			log.Fatal(err)
		}
		log.Println("Patched health_test.go")
	} else {
		log.Println("Already patched")
	}
}
