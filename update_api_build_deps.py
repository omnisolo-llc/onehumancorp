import re

with open("srcs/server/api/BUILD.bazel", "r") as f:
    content = f.read()

# Add to go_test
test_deps_idx = content.find('deps = [', content.find('go_test'))
if test_deps_idx != -1:
    end_idx = content.find('],', test_deps_idx)
    sub = content[test_deps_idx:end_idx]
    if '"@org_golang_google_grpc//peer"' not in sub:
        content = content[:end_idx] + '    "@org_golang_google_grpc//credentials",\n        "@org_golang_google_grpc//peer",\n    ' + content[end_idx:]

with open("srcs/server/api/BUILD.bazel", "w") as f:
    f.write(content)
