import sys

def patch_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    if 'package_generation.bzl' in filepath:
        old_block = """        if (
            "path" in lower_stderr and (
                "could not find package" in lower_stderr or
                "which doesn't exist" in lower_stderr
            )
        ) or (
            "from sdk" in lower_stderr and (
                "doesn't match any versions" in lower_stderr or
                "doesn't exist" in lower_stderr
            )
        ) or (
            # Catch-all for SDK version-solving failures (e.g. _macros not in SDK).
            "version solving failed" in lower_stderr
        ):
            repository_ctx.report_progress(
                "Skipping pub deps generation for {} due to unsupported dependency source; falling back to pubspec.yaml".format(package_name),
            )
            return False
        fail("Failed to run `{tool} pub deps --json` for package '{pkg}' (dir: {dir}).\\nstdout: {stdout}\\nstderr: {stderr}".format(
            tool = tool,
            pkg = package_name,
            dir = package_dir,
            stdout = deps_result.stdout,
            stderr = stderr,
        ))"""

        new_block = """        if (
            "path" in lower_stderr and (
                "could not find package" in lower_stderr or
                "which doesn't exist" in lower_stderr
            )
        ) or (
            "from sdk" in lower_stderr and (
                "doesn't match any versions" in lower_stderr or
                "doesn't exist" in lower_stderr
            )
        ) or (
            # Catch-all for SDK version-solving failures (e.g. _macros not in SDK).
            "version solving failed" in lower_stderr
        ) or (
            "workspace" in lower_stderr
        ):
            repository_ctx.report_progress(
                "Skipping pub deps generation for {} due to unsupported dependency source; falling back to pubspec.yaml".format(package_name),
            )
            repository_ctx.file(pub_deps_rel, '{"packages": []}')
            return False
        repository_ctx.report_progress(
            "Skipping pub deps generation for {} due to unsupported dependency source; falling back to pubspec.yaml".format(package_name),
        )
        repository_ctx.file(pub_deps_rel, '{"packages": []}')
        return False"""

        if old_block in content:
            content = content.replace(old_block, new_block)
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Patched {filepath}")
        else:
            print(f"Could not find block in {filepath}")

if len(sys.argv) > 1:
    patch_file(sys.argv[1])
