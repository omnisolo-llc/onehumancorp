with open("srcs/server/memory/autodream/BUILD.bazel", "r") as f:
    content = f.read()

deps_patch = """
        "//srcs/server/memory",
        "//srcs/server/telemetry",
"""

content = content.replace('        "//srcs/server/memory",\n', deps_patch)

with open("srcs/server/memory/autodream/BUILD.bazel", "w") as f:
    f.write(content)
