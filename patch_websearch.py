import sys

content = open("srcs/server/agents/builtin/websearch.go").read()

search = """	seen := map[string]bool{}
	count := 0
	for _, m := range matches {
		if count >= numResults {
			break
		}
		rawURL := strings.TrimSpace(m[1])"""

replace = """	seen := map[string]bool{}
	count := 0
	for _, m := range matches {
		if count >= numResults {
			break
		}
		rawURL := strings.TrimSpace(m[1])
		// handle relative duckduckgo urls if any
		if strings.HasPrefix(rawURL, "/") {
			rawURL = "https://duckduckgo.com" + rawURL
		}"""

content = content.replace(search, replace)
open("srcs/server/agents/builtin/websearch.go", "w").write(content)
