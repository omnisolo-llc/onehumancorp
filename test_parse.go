package main

import (
	"fmt"
	"strings"
	"regexp"
)

func main() {
	query := "SELECT id, payload FROM agent_missions WHERE payload::json->>'role' = $1 AND status = 'PENDING'"
	fmt.Println(convertBindVars(query))
}

func convertBindVars(query string) string {
	query = strings.ReplaceAll(query, "FOR UPDATE SKIP LOCKED", "")

	// Maps json paths (e.g. col::json->>'key' to json_extract(col, '$.key'))
	re := regexp.MustCompile(`([a-zA-Z0-9_]+)::json->>'([a-zA-Z0-9_]+)'`)
	query = re.ReplaceAllString(query, "json_extract($1, '$$.$2')")

	var result strings.Builder
	result.Grow(len(query))

	inQuotes := false
	for i := 0; i < len(query); i++ {
		c := query[i]

		if c == '\'' {
			inQuotes = !inQuotes
			result.WriteByte(c)
			continue
		}

		if !inQuotes && c == '$' {
			// Look ahead for numbers
			j := i + 1
			for j < len(query) && query[j] >= '0' && query[j] <= '9' {
				j++
			}
			if j > i+1 {
				result.WriteByte('?')
				result.WriteString(query[i+1 : j])
				i = j - 1
				continue
			}
		}

		result.WriteByte(c)
	}

	return result.String()
}
