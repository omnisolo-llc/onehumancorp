_CRATES_IO_SOURCE = "registry+https://github.com/rust-lang/crates.io-index"
_CRATES_IO_URL = "https://static.crates.io/crates/{name}/{name}-{version}.crate"

def _quoted_value(line):
    first = line.find("\"")
    last = line.rfind("\"")
    if first == -1 or last <= first:
        return None
    return line[first + 1:last]

def _parse_cargo_lock(lockfile):
    packages = []
    current = {}

    def finish_package():
        if not current:
            return
        if current.get("source") == _CRATES_IO_SOURCE:
            name = current.get("name")
            version = current.get("version")
            checksum = current.get("checksum")
            if not name or not version or not checksum:
                fail("crates.io package in Cargo.lock is missing name, version, or checksum: %s" % current)
            packages.append({
                "name": name,
                "version": version,
                "checksum": checksum,
            })

    for raw_line in lockfile.splitlines():
        line = raw_line.strip()
        if line == "[[package]]":
            finish_package()
            current = {}
            continue
        if line.startswith("name = "):
            current["name"] = _quoted_value(line)
        elif line.startswith("version = "):
            current["version"] = _quoted_value(line)
        elif line.startswith("source = "):
            current["source"] = _quoted_value(line)
        elif line.startswith("checksum = "):
            current["checksum"] = _quoted_value(line)

    finish_package()
    return packages

def _cargo_vendor_repository_impl(repo_ctx):
    packages = _parse_cargo_lock(repo_ctx.read(repo_ctx.attr.cargo_lock))
    seen = {}
    for package in packages:
        key = "{name}-{version}".format(**package)
        if key in seen:
            continue
        seen[key] = True
        repo_ctx.download_and_extract(
            url = [_CRATES_IO_URL.format(
                name = package["name"],
                version = package["version"],
            )],
            output = "vendor/%s" % key,
            sha256 = package["checksum"],
            stripPrefix = key,
            type = "tar.gz",
        )
        repo_ctx.file(
            "vendor/%s/.cargo-checksum.json" % key,
            "{\"files\":{},\"package\":\"%s\"}\n" % package["checksum"],
        )

    repo_ctx.file(".vendor_root", "")
    repo_ctx.file("BUILD.bazel", """\
package(default_visibility = ["//visibility:public"])

exports_files([".vendor_root"])

filegroup(
    name = "vendor",
    srcs = glob(["vendor/**"], exclude = ["BUILD.bazel"]),
)
""")

cargo_vendor_repository = repository_rule(
    implementation = _cargo_vendor_repository_impl,
    attrs = {
        "cargo_lock": attr.label(
            allow_single_file = True,
            mandatory = True,
        ),
    },
)
