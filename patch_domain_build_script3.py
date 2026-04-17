import re

with open("srcs/server/domain/BUILD.bazel", "r") as f:
    content = f.read()

old_content = """    deps = [
        "@com_github_redis_rueidis//:rueidis",
        "@com_github_redis_rueidis//mock:mock",
        "@org_uber_go_mock//gomock:gomock",
    ],"""

new_content = """    deps = [
        "@com_github_redis_rueidis//:rueidis",
    ],"""

content = content.replace(old_content, new_content)

with open("srcs/server/domain/BUILD.bazel", "w") as f:
    f.write(content)
