package db

import (
	"strings"
	"regexp"
)

var forUpdateRegex = regexp.MustCompile(`(?i)\s*FOR UPDATE SKIP LOCKED`)
var jsonExtractRegex = regexp.MustCompile(`(?i)([a-zA-Z_0-9]+)::json->>'([^']+)'`)

func convertQuery(sql string) string {
	sql = forUpdateRegex.ReplaceAllString(sql, "")
	sql = jsonExtractRegex.ReplaceAllString(sql, "json_extract($1, '$$.$2')")

	var result strings.Builder
	inString := false

	for i := 0; i < len(sql); i++ {
		c := sql[i]
		if c == '\'' {
			inString = !inString
			result.WriteByte(c)
			continue
		}

		if !inString && c == '$' {
			// check if next chars are digits
			j := i + 1
			for j < len(sql) && sql[j] >= '0' && sql[j] <= '9' {
				j++
			}
			if j > i+1 {
				result.WriteString("?")
				result.WriteString(sql[i+1 : j])
				i = j - 1
				continue
			}
		}

		result.WriteByte(c)
	}

	return result.String()
}
