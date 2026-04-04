import re

with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    content = f.read()

# Add new files to srcs list
srcs_addition = """
        "autodream_kairos.go",
        "kairos_api.go",
"""
content = re.sub(r'(srcs = \[\n)', r'\1' + srcs_addition, content, count=1)

# Add new tests to tests srcs list
test_addition = """
        "autodream_kairos_test.go",
        "kairos_api_test.go",
"""
content = re.sub(r'(srcs = \[\n)(?!.*"autodream_kairos.go")', r'\1' + test_addition, content, count=1)

with open('srcs/server/orchestration/BUILD.bazel', 'w') as f:
    f.write(content)
