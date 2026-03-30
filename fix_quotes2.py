with open("bazel/rules/flutter/flutter/private/package_generation.bzl", "r") as f:
    lines = f.readlines()

with open("bazel/rules/flutter/flutter/private/package_generation.bzl", "w") as f:
    for line in lines:
        if 'repository_ctx.file(pub_deps_rel, "{\\"packages\\": []}")' in line:
            pass
        elif 'return False' in line and lines[lines.index(line)-1].strip() == 'repository_ctx.file(pub_deps_rel, "{\\"packages\\": []}")':
            pass
        else:
            f.write(line)
