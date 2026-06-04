def _rust_sharded_test_impl(ctx):
    ctx.actions.symlink(
        is_executable = True,
        output = ctx.outputs.executable,
        target_file = ctx.executable._runner,
    )

    runfiles = ctx.runfiles(files = [ctx.file.binary])
    runfiles = runfiles.merge(ctx.attr.binary[DefaultInfo].default_runfiles)
    runfiles = runfiles.merge(ctx.attr._runner[DefaultInfo].default_runfiles)

    return [
        DefaultInfo(
            executable = ctx.outputs.executable,
            runfiles = runfiles,
        ),
        RunEnvironmentInfo(
            environment = {
                "RUST_SHARDED_TEST_BINARY": ctx.file.binary.short_path,
                "RUST_SHARDED_TEST_FILTERS": "\n".join(ctx.attr.filters),
            },
        ),
    ]

rust_sharded_test = rule(
    implementation = _rust_sharded_test_impl,
    attrs = {
        "binary": attr.label(
            allow_single_file = True,
            cfg = "target",
            executable = True,
            mandatory = True,
        ),
        "filters": attr.string_list(),
        "_runner": attr.label(
            cfg = "target",
            default = Label("//bazel/rules:rust_sharded_test_runner"),
            executable = True,
        ),
    },
    test = True,
)
