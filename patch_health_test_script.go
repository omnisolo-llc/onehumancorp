package main

import (
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/orchestration/health_test.go")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	var newLines []string
	skipCount := 0

	for i := 0; i < len(lines); i++ {
		line := lines[i]
		if strings.HasPrefix(line, "func (m *mockProvider) Ping(ctx context.Context) error {") {
			if skipCount > 0 {
				// The alternative Ping functions have different lengths. Let's look for the ending "}"
				for j := i; j < len(lines); j++ {
					if strings.HasPrefix(lines[j], "}") {
						i = j
						break
					}
				}
				continue
			}
			skipCount++
		}
		newLines = append(newLines, line)
	}

	err = ioutil.WriteFile("srcs/server/orchestration/health_test.go", []byte(strings.Join(newLines, "\n")), 0644)
	if err != nil {
		panic(err)
	}
}
