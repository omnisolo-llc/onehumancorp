# The problem is that test_provider_test.go is inside `db` and it exports NewTestProvider!
# BUT it's in the `srcs` list of `db_test` so it's only available to tests inside `db`.
# To make it available to `orchestration_test`, it must be in the `srcs` of `db` NOT `db_test` or we just create a local test_provider.go inside orchestration.
# Let's just create a local file `srcs/server/orchestration/test_provider_test.go` containing the mock.
# Wait I ALREADY did that above but it's not in the BUILD.bazel for orchestration!
import re
with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    content = f.read()

content = re.sub(r'("autodream_kairos_test\.go",\n)', r'\1        "test_provider_test.go",\n', content)
with open('srcs/server/orchestration/BUILD.bazel', 'w') as f:
    f.write(content)
