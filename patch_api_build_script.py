import re

with open("srcs/server/api/BUILD.bazel", "r") as f:
    content = f.read()

old_content = """    deps = [
        "//srcs/server/orchestration",
        "//srcs/server/orchestration/hybrid_sync",
        "//srcs/server/domain",
    ],
)"""

new_content = """    deps = [
        "//srcs/server/orchestration",
        "//srcs/server/orchestration/hybrid_sync",
        "//srcs/server/domain",
        "@com_github_redis_rueidis//:rueidis",
    ],
)"""

content = content.replace(old_content, new_content)

with open("srcs/server/api/BUILD.bazel", "w") as f:
    f.write(content)
