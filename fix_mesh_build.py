import sys

with open('srcs/server/orchestration/mesh/BUILD.bazel', 'r') as f:
    content = f.read()

# Replace srcs
import re
new_srcs = '''srcs = [
        "broker.go",
        "local_broker.go",
        "redis_broker.go",
        "http_handler.go",
    ],'''
content = re.sub(r'srcs = \[[^\]]*\],', new_srcs, content)

# Replace test srcs
new_test_srcs = '''srcs = [
        "broker_test.go",
    ],'''
content = re.sub(r'srcs = \["mesh_test.go"\],', new_test_srcs, content)

# update deps to use rueidis
content = content.replace('"@com_github_redis_go_redis_v9//:go-redis",', '"@com_github_redis_rueidis//:rueidis",')

with open('srcs/server/orchestration/mesh/BUILD.bazel', 'w') as f:
    f.write(content)
