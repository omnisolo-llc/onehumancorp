import re

def rewrite_minimax(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Let's replace the top of the file to ensure httptest is imported
    if "net/http/httptest" not in content:
        content = content.replace(
            "import (\n\t\"context\"",
            "import (\n\t\"context\"\n\t\"net/http\"\n\t\"net/http/httptest\""
        )

    with open(filepath, 'w') as f:
        f.write(content)

rewrite_minimax('srcs/server/integration/minimax_e2e_test.go')
