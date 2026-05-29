load(
    "//bazel/rules:tauri_mobile_versions.bzl",
    "ANDROID_BUILD_TOOLS_VERSION",
    "ANDROID_NDK_VERSION",
    "ANDROID_RUST_TARGET",
)

_GRADLE_TOOLCHAIN_TYPE = "//bazel/rules:gradle_toolchain_type"

def _copy_sources_commands(files):
    commands = []
    for f in files:
        dest = f.short_path
        if dest.startswith("../"):
            continue
        commands.append("mkdir -p \"$work/%s\"" % dest.rpartition("/")[0])
        commands.append("cp \"%s\" \"$work/%s\"" % (f.path, dest))
    return commands

def _tauri_mobile_package_impl(ctx):
    out = ctx.outputs.out
    is_android = ctx.attr.package_type in ["apk", "aab"]
    cargo = ctx.files.cargo[0]
    rustc = ctx.files.rustc[0]
    tauri_cli = ctx.executable.tauri_cli
    cargo_vendor_root = None
    if ctx.file.cargo_vendor_root:
        cargo_vendor_root = ctx.file.cargo_vendor_root.dirname
    android_sdk_root = None
    if ctx.file.android_sdk_root:
        android_sdk_root = ctx.file.android_sdk_root.dirname
    android_rustlib_dir = None
    if is_android:
        for f in ctx.files.android_rust_toolchain:
            if f.dirname.endswith("/lib/rustlib/%s/lib" % ANDROID_RUST_TARGET):
                android_rustlib_dir = f.dirname[:-4]
                break
        if not android_rustlib_dir:
            fail("android_rust_toolchain must include %s rust stdlib files" % ANDROID_RUST_TARGET)
    inputs = ctx.files.srcs + ctx.files.cargo + ctx.files.rustc + ctx.files.android_rust_toolchain + [tauri_cli] + ctx.files.android_sdk + ctx.files.cargo_vendor
    if ctx.file.cargo_vendor_root:
        inputs.append(ctx.file.cargo_vendor_root)
    gradle_distribution = None
    if is_android:
        gradle_toolchain = ctx.toolchains[_GRADLE_TOOLCHAIN_TYPE]
        gradle_distribution = gradle_toolchain.distribution
        inputs.append(gradle_distribution)
    app_dir = "src/ui/tauri"
    commands = [
        "set -euo pipefail",
        "work=\"$PWD/%s.work\"" % ctx.label.name,
        "tmp=\"$PWD/%s.tmp\"" % ctx.label.name,
        "rm -rf \"$work\"",
        "rm -rf \"$tmp\"",
        "mkdir -p \"$work\"",
        "mkdir -p \"$tmp\"",
    ]
    commands.extend(_copy_sources_commands(ctx.files.srcs))
    commands.extend([
        "execroot=\"$PWD\"",
        "cd \"$work/%s\"" % app_dir,
        "export RUSTC=\"$execroot/%s\"" % rustc.path,
        "export HOME=\"$tmp/home\"",
        "export CARGO_HOME=\"$tmp/cargo-home\"",
        "export CARGO_TARGET_DIR=\"$tmp/cargo-target\"",
        "export TMPDIR=\"$tmp\"",
        "mkdir -p \"$HOME\" \"$CARGO_HOME\" \"$CARGO_TARGET_DIR\" \"$tmp/bin\"",
        "ln -sf \"$execroot/%s\" \"$tmp/bin/cargo-tauri\"" % tauri_cli.path,
        "cargo=\"$execroot/%s\"" % cargo.path,
    ])
    if cargo_vendor_root:
        commands.extend([
            "cat > \"$CARGO_HOME/config.toml\" <<EOF",
            "[source.crates-io]",
            "replace-with = \"vendored-sources\"",
            "[source.vendored-sources]",
            "directory = \"$execroot/%s/vendor\"" % cargo_vendor_root,
            "[net]",
            "offline = true",
            "EOF",
            "export CARGO_NET_OFFLINE=true",
        ])

    if is_android:
        if not android_sdk_root:
            fail("android_sdk_root is required for Tauri Android packages")
        commands.extend([
            "mkdir -p icons",
            "base64 -d > icons/32x32.png <<'EOF'",
            "iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAAL0lEQVR42u3OIQEAAAgDMEoQkZ40gxg3E/Ornr2kEhAQEBAQEBAQEBAQEBAQSAceUTTkeYfW77IAAAAASUVORK5CYII=",
            "EOF",
            "base64 -d > icons/128x128.png <<'EOF'",
            "iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAADDPmHLAAAA8klEQVR42u3SMQ0AAAjAMEwgEZ84Axsk9JiBpZHVo7+FCQAYAYAAEAACQAAIAAEgAASAABAAAkAACAABIAAEgAAQAAJAAAgAASAABIAAEAACQAAIAAEgAASAABAAAkAACAABIAAEgAAQAAJAAAgAASAABIAAEAACQAAIAAEgAASAABAAAkAACAABIAAEgAAQAAJAAAgAASAABIAAEAACQAAIAAEgAASAABAAAgAAEwAwAgABIAAEgAAQAAJAAAgAASAABIAAEAACQAAIAAEgAASAABAAAkAACAABIAAEgAAQAAJAAAgAASAABIAAEAACQDdaDtVIU/IRL9oAAAAASUVORK5CYII=",
            "EOF",
            "base64 -d > icons/128x128@2x.png <<'EOF'",
            "iVBORw0KGgoAAAANSUhEUgAAAQAAAAEACAYAAABccqhmAAACYUlEQVR42u3UMQEAAAQAQSVE1FMzCmjghivww0dWD/BTiAAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAYABiAAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAYABiAAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAYABCAEGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAYABAAYAGABgAIABAAYAGABgAIABAAYAGABgAIABAAYAGABgAIABAAYAGABgAIABAAYAGABgAIABAAYAGABgAIABAAYAGABgAIABgAEABgAYAGAAgAEABgAYAGAAgAEABgAYAGAAgAEABgAYAGAAgAEABgAYAGAAgAEABgAYAGAAgAEABgAYAGAAgAEABgAYAGAAgAGAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQAGABgAYACAAQCXBf7XIVhY+iOkAAAAAElFTkSuQmCC",
            "EOF",
            "base64 -d > icons/icon.png <<'EOF'",
            "iVBORw0KGgoAAAANSUhEUgAAAgAAAAIACAYAAAD0eNT6AAAG40lEQVR42u3WMQEAAAQAQSVE1FMzStjccAV++sjqAQB+CREAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAAAyAEABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAAAYABEAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAAAyAEABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAAAYABEAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAAAyAEABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAAAYABEAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAAAyAEABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAAAYABEAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAwAAGAAAAADAAAYAADAAAAABgAAMAAAgAEAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAABgAAMAAAAAGAAAwAACAAQAADAAAYAAAAAMAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAAwAAAAAYAAAyAEABgAAAAAwAAGAAAwAAAAAYAADAAAIABAAAMAABgAAAAAwAAGAAA4M4CkI+FXXjiauAAAAAASUVORK5CYII=",
            "EOF",
            "real_rustc=\"$execroot/%s\"" % rustc.path,
            "real_sysroot=\"$($real_rustc --print sysroot)\"",
            "combined_sysroot=\"$tmp/rust-sysroot\"",
            "mkdir -p \"$combined_sysroot/lib/rustlib\"",
            "for entry in \"$real_sysroot\"/lib/*; do",
            "  name=\"$(basename \"$entry\")\"",
            "  if [[ \"$name\" != rustlib ]]; then ln -sf \"$entry\" \"$combined_sysroot/lib/$name\"; fi",
            "done",
            "for entry in \"$real_sysroot\"/lib/rustlib/*; do",
            "  ln -sf \"$entry\" \"$combined_sysroot/lib/rustlib/$(basename \"$entry\")\"",
            "done",
            "rm -f \"$combined_sysroot/lib/rustlib/%s\"" % ANDROID_RUST_TARGET,
            "ln -sf \"$execroot/%s\" \"$combined_sysroot/lib/rustlib/%s\"" % (android_rustlib_dir, ANDROID_RUST_TARGET),
            "cat > \"$tmp/bin/rustc\" <<EOF",
            "#!/usr/bin/env bash",
            "exec \"$real_rustc\" --sysroot \"$combined_sysroot\" \"\\$@\"",
            "EOF",
            "chmod +x \"$tmp/bin/rustc\"",
            "export RUSTC=\"$tmp/bin/rustc\"",
            "cat > \"$tmp/bin/rustup\" <<'EOF'",
            "#!/usr/bin/env bash",
            "if [[ \"${1:-}\" == target && \"${2:-}\" == add && \"${3:-}\" == %s ]]; then exit 0; fi" % ANDROID_RUST_TARGET,
            "if [[ \"${1:-}\" == target && \"${2:-}\" == list ]]; then echo '%s (installed)'; exit 0; fi" % ANDROID_RUST_TARGET,
            "echo 'Only rustup target add/list is supported in this Bazel action' >&2",
            "exit 1",
            "EOF",
            "chmod +x \"$tmp/bin/rustup\"",
            "if [[ ! -d \"$combined_sysroot/lib/rustlib/%s\" ]]; then" % ANDROID_RUST_TARGET,
            "  echo 'Bazel Android Rust stdlib was not linked into the action sysroot' >&2",
            "  ls -la \"$combined_sysroot/lib/rustlib\" >&2 || true",
            "  exit 1",
            "fi",
            "if ! \"$tmp/bin/rustc\" --print target-list | grep -qx '%s'; then" % ANDROID_RUST_TARGET,
            "  echo 'Bazel rustc does not recognize %s' >&2" % ANDROID_RUST_TARGET,
            "  exit 1",
            "fi",
            "export ANDROID_HOME=\"$execroot/%s\"" % android_sdk_root,
            "export ANDROID_SDK_ROOT=\"$ANDROID_HOME\"",
            "export JAVA_HOME=\"$ANDROID_HOME/jdk\"",
            "export ANDROID_NDK_HOME=\"$ANDROID_HOME/ndk/%s\"" % ctx.attr.android_ndk_version,
            "export ANDROID_NDK_ROOT=\"$ANDROID_NDK_HOME\"",
            "export NDK_HOME=\"$ANDROID_NDK_HOME\"",
            "export ANDROID_USER_HOME=\"$tmp/android-home\"",
            "export GRADLE_USER_HOME=\"$tmp/gradle-home\"",
            "mkdir -p \"$ANDROID_USER_HOME\" \"$GRADLE_USER_HOME\"",
            "export PATH=\"$JAVA_HOME/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/build-tools/%s:$tmp/bin:$execroot/%s:/usr/bin:/bin:/usr/sbin:/sbin\"" % (ctx.attr.android_build_tools_version, cargo.dirname),
            "if [[ ! -d gen/android ]]; then \"$cargo\" tauri android init --ci --skip-targets-install; fi",
            "app_build_gradle=\"gen/android/app/build.gradle.kts\"",
            "if [[ -f \"$app_build_gradle\" ]] && ! grep -q 'buildToolsVersion' \"$app_build_gradle\"; then",
            "  patched_gradle=\"$tmp/app-build.gradle.kts\"",
            "  awk -v version=\"%s\" '{ print; if ($0 ~ /^[[:space:]]*compileSdk = / && ! inserted) { print \"    buildToolsVersion = \\\"\" version \"\\\"\"; inserted=1 } }' \"$app_build_gradle\" > \"$patched_gradle\"" % ctx.attr.android_build_tools_version,
            "  mv \"$patched_gradle\" \"$app_build_gradle\"",
            "fi",
            "gradle_wrapper_properties=\"gen/android/gradle/wrapper/gradle-wrapper.properties\"",
            "gradle_distribution=\"$execroot/%s\"" % gradle_distribution.path,
            "if [[ -f \"$gradle_wrapper_properties\" && -f \"$gradle_distribution\" ]]; then",
            "  sed -i.bak \"s#^distributionUrl=.*#distributionUrl=file://$gradle_distribution#\" \"$gradle_wrapper_properties\"",
            "fi",
            "\"$cargo\" tauri android build --%s --target aarch64 --ci" % ctx.attr.package_type,
            "artifact=\"$(find gen/android -type f -name '*.%s' | head -n 1)\"" % ctx.attr.package_type,
        ])
    elif ctx.attr.package_type == "ipa":
        commands.extend([
            "export PATH=\"$tmp/bin:$execroot/%s:/usr/bin:/bin:/usr/sbin:/sbin\"" % cargo.dirname,
            "case \"$(uname -s)\" in Darwin) ;; *) echo 'Tauri iOS builds require a macOS host' >&2; exit 1 ;; esac",
            "if [[ ! -d gen/apple ]]; then \"$cargo\" tauri ios init --ci; fi",
            "ios_args=()",
            "if [[ -n \"${TAURI_IOS_EXPORT_METHOD:-}\" ]]; then ios_args+=(--export-method \"$TAURI_IOS_EXPORT_METHOD\"); fi",
            "\"$cargo\" tauri ios build --target aarch64 \"${ios_args[@]}\"",
            "artifact=\"$(find gen/apple target -type f -name '*.ipa' | head -n 1)\"",
        ])
    else:
        fail("unsupported Tauri mobile package type: %s" % ctx.attr.package_type)

    commands.extend([
        "if [[ -z \"${artifact:-}\" || ! -f \"$artifact\" ]]; then",
        "  echo 'Tauri build completed but no .%s artifact was found' >&2" % ctx.attr.package_type,
        "  find gen target -maxdepth 6 -type f 2>/dev/null | sort >&2 || true",
        "  exit 1",
        "fi",
        "mkdir -p \"$(dirname \"$execroot/%s\")\"" % out.path,
        "cp \"$artifact\" \"$execroot/%s\"" % out.path,
    ])

    ctx.actions.run_shell(
        inputs = depset(inputs),
        outputs = [out],
        command = "\n".join(commands),
        env = {
            "TAURI_IOS_EXPORT_METHOD": ctx.attr.ios_export_method,
        },
        mnemonic = "TauriMobilePackage",
        progress_message = "Building Tauri mobile package %{label}",
        use_default_shell_env = False,
    )

tauri_mobile_package = rule(
    implementation = _tauri_mobile_package_impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True, mandatory = True),
        "package_type": attr.string(mandatory = True, values = ["apk", "aab", "ipa"]),
        "cargo": attr.label(
            allow_files = True,
            default = Label("@rules_rust//rust/toolchain:current_cargo_files"),
            cfg = "exec",
        ),
        "cargo_vendor": attr.label(
            allow_files = True,
            default = Label("@tauri_cargo_vendor//:vendor"),
        ),
        "cargo_vendor_root": attr.label(
            allow_single_file = True,
            default = Label("@tauri_cargo_vendor//:.vendor_root"),
        ),
        "rustc": attr.label(
            allow_files = True,
            default = Label("@rules_rust//rust/toolchain:current_rustc_files"),
            cfg = "exec",
        ),
        "tauri_cli": attr.label(
            executable = True,
            cfg = "exec",
            default = Label("@crates//:tauri-cli__cargo-tauri"),
        ),
        "android_rust_toolchain": attr.label_list(
            allow_files = True,
            default = [
                Label("@@rules_rust++rust+rust_linux_x86_64__aarch64-linux-android__stable_tools//:rust_std-aarch64-linux-android"),
            ],
        ),
        "android_build_tools_version": attr.string(default = ANDROID_BUILD_TOOLS_VERSION),
        "android_ndk_version": attr.string(default = ANDROID_NDK_VERSION),
        "android_sdk_root": attr.label(
            allow_single_file = True,
            default = None,
        ),
        "android_sdk": attr.label(
            allow_files = True,
            default = None,
        ),
        "ios_export_method": attr.string(),
        "out": attr.output(mandatory = True),
    },
    toolchains = [_GRADLE_TOOLCHAIN_TYPE],
)
