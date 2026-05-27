load(
    "//bazel/rules:tauri_mobile_versions.bzl",
    "ANDROID_API_LEVEL",
    "ANDROID_BUILD_TOOLS_VERSION",
    "ANDROID_CMDLINE_TOOLS_SHA256",
    "ANDROID_CMDLINE_TOOLS_URL",
    "ANDROID_COMPAT_BUILD_TOOLS_VERSIONS",
    "ANDROID_JDK_SHA256",
    "ANDROID_JDK_STRIP_PREFIX",
    "ANDROID_JDK_URLS",
    "ANDROID_NDK_VERSION",
)

def _tauri_android_sdk_repository_impl(repo_ctx):
    bash = repo_ctx.which("bash")
    if not bash:
        fail("bash is required to provision the Android SDK repository")

    repo_ctx.file(".sdk_root", "")
    repo_ctx.download_and_extract(
        url = repo_ctx.attr.jdk_urls,
        output = "jdk",
        sha256 = repo_ctx.attr.jdk_sha256,
        stripPrefix = repo_ctx.attr.jdk_strip_prefix,
    )
    repo_ctx.download_and_extract(
        url = repo_ctx.attr.cmdline_tools_url,
        output = "cmdline-tools/latest",
        sha256 = repo_ctx.attr.cmdline_tools_sha256,
        stripPrefix = "cmdline-tools",
    )

    sdkmanager = repo_ctx.path("cmdline-tools/latest/bin/sdkmanager")
    repo_ctx.execute(["chmod", "+x", str(sdkmanager)])
    packages = [
        "platform-tools",
        "platforms;android-%s" % repo_ctx.attr.api_level,
        "build-tools;%s" % repo_ctx.attr.build_tools_version,
        "ndk;%s" % repo_ctx.attr.ndk_version,
    ]
    for version in repo_ctx.attr.compat_build_tools_versions:
        if version != repo_ctx.attr.build_tools_version:
            packages.append("build-tools;%s" % version)
    jdk = repo_ctx.path("jdk")
    install = "export JAVA_HOME=\"%s\"\nexport PATH=\"%s/bin:$PATH\"\nwhile true; do printf 'y\\n'; done | \"%s\" --sdk_root=\"%s\" --licenses >/dev/null || true\nwhile true; do printf 'y\\n'; done | \"%s\" --sdk_root=\"%s\" %s" % (
        jdk,
        jdk,
        sdkmanager,
        repo_ctx.path("."),
        sdkmanager,
        repo_ctx.path("."),
        " ".join(["\"%s\"" % package for package in packages]),
    )
    result = repo_ctx.execute([bash, "-c", install], quiet = False)
    if result.return_code != 0:
        fail("failed to install Android SDK packages (exit %s):\n%s\n%s" % (
            result.return_code,
            result.stdout,
            result.stderr,
        ))

    repo_ctx.file("BUILD.bazel", """\
package(default_visibility = ["//visibility:public"])

exports_files([".sdk_root"])

filegroup(
    name = "sdk",
    srcs = glob(["**"], exclude = ["BUILD.bazel"]),
)
""")

tauri_android_sdk_repository = repository_rule(
    implementation = _tauri_android_sdk_repository_impl,
    attrs = {
        "api_level": attr.int(default = ANDROID_API_LEVEL),
        "build_tools_version": attr.string(default = ANDROID_BUILD_TOOLS_VERSION),
        "cmdline_tools_sha256": attr.string(
            default = ANDROID_CMDLINE_TOOLS_SHA256,
        ),
        "cmdline_tools_url": attr.string(
            default = ANDROID_CMDLINE_TOOLS_URL,
        ),
        "compat_build_tools_versions": attr.string_list(
            default = ANDROID_COMPAT_BUILD_TOOLS_VERSIONS,
        ),
        "jdk_sha256": attr.string(
            default = ANDROID_JDK_SHA256,
        ),
        "jdk_strip_prefix": attr.string(
            default = ANDROID_JDK_STRIP_PREFIX,
        ),
        "jdk_urls": attr.string_list(
            default = ANDROID_JDK_URLS,
        ),
        "ndk_version": attr.string(default = ANDROID_NDK_VERSION),
    },
)
