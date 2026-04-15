import sys

with open('srcs/server/orchestration/mesh/BUILD.bazel', 'r') as f:
    content = f.read()

# Make sure broker.go and friends are only in srcs ONCE.
import re
new_srcs = '''srcs = [
        "broker.go",
        "http_handler.go",
        "local_broker.go",
        "redis_broker.go",
    ],'''
# Use simple replace if there are duplicates
if content.count('srcs = [') > 2:
    pass # Wait, BUILD file might be malformed

# Let's just rewrite the whole file
new_build = """load("@rules_go//go:def.bzl", "go_library", "go_test")

go_library(
    name = "mesh",
    srcs = [
        "broker.go",
        "http_handler.go",
        "local_broker.go",
        "redis_broker.go",
    ],
    importpath = "github.com/onehumancorp/mono/srcs/server/orchestration/mesh",
    visibility = ["//visibility:public"],
    deps = [
        "@com_github_redis_rueidis//:rueidis",
    ],
)

go_test(
    name = "mesh_test",
    srcs = ["broker_test.go"],
    embed = [":mesh"],
    deps = [
        "@com_github_redis_rueidis//:rueidis",
    ],
)
"""

with open('srcs/server/orchestration/mesh/BUILD.bazel', 'w') as f:
    f.write(new_build)
