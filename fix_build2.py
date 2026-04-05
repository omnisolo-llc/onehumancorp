with open("srcs/server/orchestration/BUILD.bazel", "r") as f:
    build = f.read()

# Make sure auth is in deps for both go_library and go_test
if '"//srcs/server/auth",' not in build:
    build = build.replace(
        '"//srcs/server/db",',
        '"//srcs/server/auth",\n        "//srcs/server/db",'
    )

with open("srcs/server/orchestration/BUILD.bazel", "w") as f:
    f.write(build)
