"""Repository rule for downloading Dart packages from pub.dev.

Features:
- If `version` is omitted, resolves the latest stable version from pub.dev API.
- Generates BUILD targets for the package by analysing pubspec metadata.
"""

load("//flutter/private:package_generation.bzl", "generate_package_build")

_DOC = """Download and setup a Dart package from pub.dev"""

_ATTRS = {
    "package": attr.string(
        mandatory = True,
        doc = "Name of the package on pub.dev",
    ),
    # Version is optional; when omitted we resolve the latest stable version
    "version": attr.string(
        doc = "Version of the package. If omitted, the latest stable is used.",
    ),
    "pub_dev_url": attr.string(
        default = "https://pub.dev",
        doc = "Base URL for pub.dev API",
    ),
    "sdk_repo": attr.string(
        doc = "Repository label providing Flutter SDK packages (e.g. @flutter_sdk)",
    ),
}

def _pub_dev_repository_impl(repository_ctx):
    """Implementation of pub_dev_repository rule."""
    package_name = repository_ctx.attr.package
    requested_version = repository_ctx.attr.version
    pub_dev_url = repository_ctx.attr.pub_dev_url

    version = requested_version
    if not version or version.strip() == "":
        api_url = "{}/api/packages/{}".format(pub_dev_url, package_name)
        result = repository_ctx.download(
            url = api_url,
            output = "package_info.json",
            allow_fail = True,
        )

        if not result.success:
            fail("Failed to download package information for {}: {}".format(package_name, result))

        content = repository_ctx.read("package_info.json")

        latest_idx = content.find('"latest"')
        if latest_idx != -1:
            ver_key_idx = content.find('"version"', latest_idx)
            if ver_key_idx != -1:
                colon_idx = content.find(":", ver_key_idx)
                if colon_idx != -1:
                    first_quote = content.find('"', colon_idx + 1)
                    second_quote = content.find('"', first_quote + 1) if first_quote != -1 else -1
                    if first_quote != -1 and second_quote != -1:
                        version = content[first_quote + 1:second_quote]
        if not version:
            fail("Could not determine latest version for {} from pub.dev metadata".format(package_name))

    archive_url = "{}/packages/{}/versions/{}.tar.gz".format(pub_dev_url, package_name, version)

    repository_ctx.download_and_extract(
        url = archive_url,
        stripPrefix = "",  # pub.dev packages typically have no prefix
    )

    generate_package_build(
        repository_ctx,
        package_name,
        sdk_repo = repository_ctx.attr.sdk_repo,
    )

    repository_ctx.file(
        "PUB_PACKAGE_INFO",
        "Package: {}\nVersion: {}\nDownloaded from: {}\n".format(package_name, version, archive_url),
    )

pub_dev_repository = repository_rule(
    implementation = _pub_dev_repository_impl,
    attrs = _ATTRS,
    doc = _DOC,
)
