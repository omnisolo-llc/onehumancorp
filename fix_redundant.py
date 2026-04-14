import re

with open("srcs/server/orchestration/sip.go", "r") as f:
    content = f.read()

# Replace redundant check
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

if search_text in content:
    content = content.replace(search_text, replace_text)

    # We also need to fix the closing brace for the `if root != ""`
    # Looking for:
    # 		if s.cachedGroundErr != nil {
    # It was inside the if block, wait, no, the if block enclosed groundingOnce.Do.
    # Let me check the original file structure.
