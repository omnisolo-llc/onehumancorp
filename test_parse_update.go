package main

import (
	"fmt"
	"strings"
	"regexp"
)

func main() {
	query2 := "UPDATE agent_missions SET payload = jsonb_set(payload, '{role}', '\"new_role\"') WHERE id = $1 RETURNING *"
	fmt.Println(convertBindVars(query2))
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
