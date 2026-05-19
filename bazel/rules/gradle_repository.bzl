load(
    "//bazel/rules:tauri_mobile_versions.bzl",
    "ANDROID_GRADLE_DISTRIBUTION_SHA256",
    "ANDROID_GRADLE_DISTRIBUTION_URLS",
    "ANDROID_GRADLE_VERSION",
)

def _gradle_distribution_repository_impl(repo_ctx):
    repo_ctx.download(
        url = repo_ctx.attr.urls,
        output = "gradle-%s-bin.zip" % repo_ctx.attr.version,
        sha256 = repo_ctx.attr.sha256,
    )
    repo_ctx.file("defs.bzl", """\
def _gradle_toolchain_impl(ctx):
    return [
        platform_common.ToolchainInfo(
            distribution = ctx.file.distribution,
            version = ctx.attr.version,
        ),
    ]

gradle_toolchain = rule(
    implementation = _gradle_toolchain_impl,
    attrs = {
        "distribution": attr.label(allow_single_file = True, mandatory = True),
        "version": attr.string(mandatory = True),
    },
)
""")
    repo_ctx.file("BUILD.bazel", """\
load(":defs.bzl", "gradle_toolchain")

package(default_visibility = ["//visibility:public"])

exports_files(["gradle-%s-bin.zip"])

alias(
    name = "gradle_distribution",
    actual = "gradle-%s-bin.zip",
)

gradle_toolchain(
    name = "gradle_toolchain_impl",
    distribution = ":gradle-%s-bin.zip",
    version = "%s",
)

toolchain(
    name = "gradle_toolchain",
    toolchain = ":gradle_toolchain_impl",
    toolchain_type = "@//bazel/rules:gradle_toolchain_type",
)
""" % (repo_ctx.attr.version, repo_ctx.attr.version, repo_ctx.attr.version, repo_ctx.attr.version))

gradle_distribution_repository = repository_rule(
    implementation = _gradle_distribution_repository_impl,
    attrs = {
        "sha256": attr.string(default = ANDROID_GRADLE_DISTRIBUTION_SHA256),
        "urls": attr.string_list(default = ANDROID_GRADLE_DISTRIBUTION_URLS),
        "version": attr.string(default = ANDROID_GRADLE_VERSION),
    },
)
