import re
path = 'bazel/rules/flutter/flutter/private/package_generation.bzl'
with open(path, 'r') as f:
    text = f.read()

# Make it output valid JSON:
old_block = '''        print("Failed to run flutter pub deps --json for package %s" % package_name)
        repository_ctx.file(package_dir + "/pub_deps.json", '{"packages": []}')
        return False'''

new_block = '''        print("Failed to run flutter pub deps --json for package %s" % package_name)
        repository_ctx.file(package_dir + "/pub_deps.json", '{"packages": []}')
        return False'''
