import re

with open('srcs/app/BUILD.bazel', 'r') as f:
    content = f.read()

content = content.replace('''# Tests for widgets
flutter_test(
    name = "widget_tests",
    embed = [":app_for_tests"],
    workspace_pubspec = "//:pubspec.yaml",
    test_files = glob(["test/widgets/*_test.dart"]),
)''', '''# Tests for widgets
[
    flutter_test(
        name = file.replace(".dart", "").replace("/", "_"),
        srcs = [file],
        embed = [":app_for_tests"],
        workspace_pubspec = "//:pubspec.yaml",
        test_files = [file],
    )
    for file in glob(["test/widgets/*_test.dart"])
]

test_suite(
    name = "widget_tests",
    tests = [":" + file.replace(".dart", "").replace("/", "_") for file in glob(["test/widgets/*_test.dart"])],
)
''')

with open('srcs/app/BUILD.bazel', 'w') as f:
    f.write(content)
