with open("srcs/server/orchestration/sip_test.go", "r") as f:
    content = f.read()

search_text = """	ctx := context.Background()

	ctx := context.Background()"""

replace_text = """	ctx := context.Background()"""

content = content.replace(search_text, replace_text)

with open("srcs/server/orchestration/sip_test.go", "w") as f:
    f.write(content)
