import glob
import os

flutter_actions_files = glob.glob(os.path.expanduser("~/.cache/bazel/*/*/external/rules_flutter+/flutter/private/flutter_actions.bzl"))
for f in flutter_actions_files:
    with open(f, "r") as file:
        content = file.read()

    # Once again, Python format string `{}` error inside flutter_actions.bzl.
    # Because flutter_actions.bzl has `echo '{"packages":[]}' > pub_deps.json`, and it's inside a bash script that is then passed to `format()`.
    content = content.replace('echo \'{"packages":[]}\' > pub_deps.json', 'echo \'{{"packages":[]}}\' > pub_deps.json')

    with open(f, "w") as file:
        file.write(content)
