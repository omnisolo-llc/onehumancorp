with open("srcs/server/orchestration/sip.go", "r") as f:
    lines = f.readlines()

new_lines = []
for i in range(len(lines)):
    if lines[i].strip() == "if root != \"\" {":
        pass # Skip
    elif lines[i].strip() == "if s.cachedGrounding != \"\" {" and lines[i-1].strip() == "task.Content += s.cachedGrounding":
        # Wait, let's just use string replace.
        pass

with open("srcs/server/orchestration/sip.go", "r") as f:
    content = f.read()

# Replace the beginning
search_start = """	root := s.ContextRoot
	if root == "" {
		root = "."
	}

	if root != "" {
		s.groundingOnce.Do(func() {"""
replace_start = """	root := s.ContextRoot
	if root == "" {
		root = "."
	}

	s.groundingOnce.Do(func() {"""
content = content.replace(search_start, replace_start)

# Replace the end
search_end = """		if s.cachedGrounding != "" {
			task.Content += s.cachedGrounding
		}
	}"""
replace_end = """	if s.cachedGrounding != "" {
		task.Content += s.cachedGrounding
	}"""
content = content.replace(search_end, replace_end)

with open("srcs/server/orchestration/sip.go", "w") as f:
    f.write(content)
