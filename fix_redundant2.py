import re

with open("srcs/server/orchestration/sip.go", "r") as f:
    content = f.read()

search_text = """	root := s.ContextRoot
	if root == "" {
		root = "."
	}

	if root != "" {
		s.groundingOnce.Do(func() {"""

replace_text = """	root := s.ContextRoot
	if root == "" {
		root = "."
	}

	s.groundingOnce.Do(func() {"""

content = content.replace(search_text, replace_text)
# We also need to remove the closing brace.
# Let's see the context.
