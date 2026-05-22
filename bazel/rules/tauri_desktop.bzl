def _copy_sources_commands(files):
    commands = []
    for f in files:
        dest = f.short_path
        if dest.startswith("../"):
            continue
        commands.append("mkdir -p \"$work/%s\"" % dest.rpartition("/")[0])
        commands.append("cp \"%s\" \"$work/%s\"" % (f.path, dest))
    return commands

def _tauri_desktop_package_impl(ctx):
    out = ctx.outputs.out
    cargo = ctx.files.cargo[0]
    rustc = ctx.files.rustc[0]
    tauri_cli = ctx.executable.tauri_cli
    cargo_vendor_root = None
    if ctx.file.cargo_vendor_root:
        cargo_vendor_root = ctx.file.cargo_vendor_root.dirname

    inputs = ctx.files.srcs + ctx.files.cargo + ctx.files.rustc + [tauri_cli] + ctx.files.cargo_vendor
    if ctx.file.cargo_vendor_root:
        inputs.append(ctx.file.cargo_vendor_root)

    extension = "exe" if ctx.attr.package_type == "nsis" else ctx.attr.package_type
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
        "cd \"$work/src/ui/tauri\"",
        "export RUSTC=\"$execroot/%s\"" % rustc.path,
        "export HOME=\"$tmp/home\"",
        "export CARGO_HOME=\"$tmp/cargo-home\"",
        "export CARGO_TARGET_DIR=\"$tmp/cargo-target\"",
        "export TMPDIR=\"$tmp\"",
        "mkdir -p \"$HOME\" \"$CARGO_HOME\" \"$CARGO_TARGET_DIR\" \"$tmp/bin\"",
        "case \"$(uname -s)\" in",
        "  MINGW*|MSYS*|CYGWIN*) cp \"$execroot/%s\" \"$tmp/bin/cargo-tauri.exe\" ;;" % tauri_cli.path,
        "  *) ln -sf \"$execroot/%s\" \"$tmp/bin/cargo-tauri\" ;;" % tauri_cli.path,
        "esac",
        "cargo=\"$execroot/%s\"" % cargo.path,
    ])
    if cargo_vendor_root:
        commands.extend([
            "vendor_dir=\"$execroot/%s/vendor\"" % cargo_vendor_root,
            "case \"$(uname -s)\" in MINGW*|MSYS*|CYGWIN*) vendor_dir=\"$(cygpath -m \"$vendor_dir\")\" ;; esac",
            "cat > \"$CARGO_HOME/config.toml\" <<EOF",
            "[source.crates-io]",
            "replace-with = \"vendored-sources\"",
            "[source.vendored-sources]",
            "directory = \"$vendor_dir\"",
            "[net]",
            "offline = true",
            "EOF",
            "export CARGO_NET_OFFLINE=true",
        ])

    commands.extend([
        "export PATH=\"$tmp/bin:$execroot/%s:/usr/bin:/bin:/usr/sbin:/sbin:$PATH\"" % cargo.dirname,
        "\"$cargo\" tauri build --bundles \"%s\"" % ctx.attr.package_type,
        "artifact=\"$(find \"$CARGO_TARGET_DIR\" target -type f -name '*.%s' 2>/dev/null | head -n 1)\"" % extension,
        "if [[ -z \"${artifact:-}\" || ! -f \"$artifact\" ]]; then",
        "  echo 'Tauri build completed but no .%s artifact was found' >&2" % extension,
        "  find \"$CARGO_TARGET_DIR\" target -maxdepth 8 -type f 2>/dev/null | sort >&2 || true",
        "  exit 1",
        "fi",
        "mkdir -p \"$(dirname \"$execroot/%s\")\"" % out.path,
        "cp \"$artifact\" \"$execroot/%s\"" % out.path,
    ])

    ctx.actions.run_shell(
        inputs = depset(inputs),
        outputs = [out],
        command = "\n".join(commands),
        execution_requirements = {
            "no-remote": "1",
        },
        mnemonic = "TauriDesktopPackage",
        progress_message = "Building Tauri desktop package %{label}",
        use_default_shell_env = True,
    )

tauri_desktop_package = rule(
    implementation = _tauri_desktop_package_impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True, mandatory = True),
        "package_type": attr.string(mandatory = True, values = ["deb", "rpm", "dmg", "msi", "nsis"]),
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
        "out": attr.output(mandatory = True),
    },
)
