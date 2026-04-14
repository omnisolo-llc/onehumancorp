with open("srcs/server/orchestration/sip_test.go", "r") as f:
    content = f.read()

# Replace err := db.DelegateMission with err = db.DelegateMission in the EmptyContextRoot test
search_text = "	err := db.DelegateMission(ctx, \"mission-test-empty-root\", \"TEST_ENGINEER\", msg)"
replace_text = "	err = db.DelegateMission(ctx, \"mission-test-empty-root\", \"TEST_ENGINEER\", msg)"

content = content.replace(search_text, replace_text)

with open("srcs/server/orchestration/sip_test.go", "w") as f:
    f.write(content)
