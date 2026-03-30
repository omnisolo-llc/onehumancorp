sed -i 's/repository_ctx.file(pub_deps_rel, "{"packages": \[\]}")/repository_ctx.file(pub_deps_rel, "{\\"packages\\": []}")/' bazel/rules/flutter/flutter/private/package_generation.bzl
