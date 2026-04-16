import re

with open('srcs/app/BUILD.bazel', 'r') as f:
    content = f.read()

content = content.replace('''    deps = [
        ":router_lib",
        "@pub_mocktail//:mocktail",
    ],''', '''    deps = [
        ":router_lib",
        "//srcs/app/lib/widgets:widgets_lib",
        "@pub_mocktail//:mocktail",
    ],''')

with open('srcs/app/BUILD.bazel', 'w') as f:
    f.write(content)
