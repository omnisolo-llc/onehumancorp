import re

with open("srcs/app/lib/widgets/BUILD.bazel", "r") as f:
    content = f.read()

# Add glass_container.dart
if "glass_container" not in content:
    content = content.replace("flutter_library(\n    name = \"slide_to_approve\",", """flutter_library(
    name = "glass_container",
    srcs = ["glass_container.dart"],
    pubspec = "//srcs/app:pubspec.yaml",
    workspace_pubspec = "//:pubspec.yaml",
    deps = [
        "@flutter_sdk//flutter/packages/flutter",
    ],
)

flutter_library(
    name = "slide_to_approve",""")
    content = content.replace("deps = [\n        \":org_tree_widget\",\n        \":slide_to_approve\",\n    ],", "deps = [\n        \":org_tree_widget\",\n        \":slide_to_approve\",\n        \":glass_container\",\n    ],")

with open("srcs/app/lib/widgets/BUILD.bazel", "w") as f:
    f.write(content)
