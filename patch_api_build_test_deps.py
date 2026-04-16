import re

with open("srcs/server/api/BUILD.bazel", "r") as f:
    content = f.read()

content = content.replace(
    '"//srcs/server/orchestration/hybrid_sync",',
    '"//srcs/server/orchestration/hybrid_sync",\n        "@org_golang_google_grpc//credentials",\n        "@org_golang_google_grpc//peer",'
)

with open("srcs/server/api/BUILD.bazel", "w") as f:
    f.write(content)
