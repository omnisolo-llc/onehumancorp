"""Rule for generating Dart code from Protobuf using protoc-gen-dart."""

load("@rules_proto//proto:defs.bzl", "ProtoInfo")
load("@bazel_skylib//lib:paths.bzl", "paths")

def _compute_repo_relative_path(ctx, artifact, repo_name):
    """Compute relative path from generated artifact to an external repository root."""
    bin_segments = [segment for segment in ctx.bin_dir.path.split("/") if segment]
    short_dir = paths.dirname(artifact.short_path)
    short_segments = []
    if short_dir and short_dir != ".":
        short_segments = [segment for segment in short_dir.split("/") if segment]
    up_count = len(bin_segments) + len(short_segments)
    components = [".."] * up_count
    components.extend(["external", repo_name])
    return "/".join(components).replace("+", "%2B")

def _proto_dart_library_impl(ctx):
    proto_info = ctx.attr.protos[ProtoInfo]
    srcs = proto_info.direct_sources
    outs = []

    # Standard outputs
    for src in srcs:
        basename = src.basename.replace(".proto", "")
        outs.append(ctx.actions.declare_file(basename + ".pb.dart"))
        outs.append(ctx.actions.declare_file(basename + ".pbenum.dart"))
        outs.append(ctx.actions.declare_file(basename + ".pbjson.dart"))
        if ctx.attr.use_grpc:
            outs.append(ctx.actions.declare_file(basename + ".pbgrpc.dart"))

    # Optional Domain Model outputs
    domain_outs = []
    if ctx.attr.generate_domain_models:
        for src in srcs:
            basename = src.basename.replace(".proto", "")
            domain_outs.append(ctx.actions.declare_file(basename + ".domain.dart"))

    protoc = ctx.executable._protoc
    dart = ctx.executable._dart_bin

    # Resolve plugin paths
    plugin_srcs = ctx.attr._plugin_srcs.files.to_list()
    plugin_entry = None
    for f in plugin_srcs:
        if f.path.endswith("bin/protoc_plugin.dart"):
            plugin_entry = f
            break
    if not plugin_entry:
        fail("protoc_plugin.dart not found in plugin sources")

    package_config = ctx.actions.declare_file(ctx.label.name + "_package_config.json")
    plugin_repo = ctx.attr._plugin_srcs.label.workspace_name
    package_entries = [
        """    {{
      "name": "protoc_plugin",
      "rootUri": "{plugin_root}",
      "packageUri": "lib/",
      "languageVersion": "3.7"
    }}""".format(plugin_root = _compute_repo_relative_path(ctx, package_config, plugin_repo)),
    ]

    for pkg_name, pkg_label in [
        ("collection", ctx.attr._collection_pkg),
        ("fixnum", ctx.attr._fixnum_pkg),
        ("meta", ctx.attr._meta_pkg),
        ("path", ctx.attr._path_pkg),
        ("protobuf", ctx.attr._protobuf_pkg),
        ("dart_style", ctx.attr._dart_style_pkg),
        ("pub_semver", ctx.attr._pub_semver_pkg),
    ]:
        pkg_root = _compute_repo_relative_path(ctx, package_config, pkg_label.label.workspace_name)
        package_entries.append(
            """    {{
      "name": "{name}",
      "rootUri": "{root}",
      "packageUri": "lib/",
      "languageVersion": "3.7"
    }}""".format(name = pkg_name, root = pkg_root),
        )

    package_config_content = "{\n  \"configVersion\": 2,\n  \"packages\": [\n" + ",\n".join(package_entries) + "\n  ]\n}\n"
    ctx.actions.write(output = package_config, content = package_config_content)

    plugin_wrapper = ctx.actions.declare_file(ctx.label.name + "_protoc_gen_dart.sh")
    plugin_wrapper_content = "#!/bin/bash\nset -euo pipefail\nSCRIPT_DIR=\"$(cd \"$(dirname \"$0\")\" && pwd)\"\nexec {dart} --packages=\"$SCRIPT_DIR/{package_config}\" \"{plugin_entry}\" \"$@\"\n".format(
        dart = dart.path,
        package_config = package_config.basename,
        plugin_entry = plugin_entry.path,
    )
    ctx.actions.write(output = plugin_wrapper, content = plugin_wrapper_content, is_executable = True)

    proto_paths = {}
    for p in proto_info.transitive_proto_path.to_list():
        proto_paths[p] = True
    for src in proto_info.transitive_sources.to_list():
        if "google/protobuf/" in src.path:
             base = src.path.split("google/protobuf/")[0]
             proto_paths[base] = True
    if proto_info.proto_source_root and proto_info.proto_source_root != ".":
        proto_paths[proto_info.proto_source_root] = True
    else:
        proto_paths["."] = True

    proto_path_args = " ".join(["--proto_path=" + p for p in sorted(proto_paths.keys())])
    proto_files = " ".join([src.path for src in srcs])
    out_dir = ctx.bin_dir.path
    dart_out_prefix = "grpc:" if ctx.attr.use_grpc else ""

    wrapper = ctx.actions.declare_file(ctx.label.name + "_protoc_wrapper.sh")
    wrapper_content = "#!/bin/bash\nset -euo pipefail\n"
    wrapper_content += "{protoc} --plugin=protoc-gen-dart={plugin_wrapper} --dart_out={dart_out_prefix}{out_dir} {proto_path_args} {proto_files}\n".format(
        protoc = protoc.path,
        plugin_wrapper = plugin_wrapper.path,
        dart_out_prefix = dart_out_prefix,
        out_dir = out_dir,
        proto_path_args = proto_path_args,
        proto_files = proto_files,
    )
    for out in outs:
        if any([out.path.endswith(suffix) for suffix in [".pbenum.dart", ".pbjson.dart", ".pbgrpc.dart"]]):
             wrapper_content += 'if [ ! -f "{path}" ]; then echo "// No content" > "{path}"; fi\n'.format(path = out.path)
    ctx.actions.write(output = wrapper, content = wrapper_content, is_executable = True)

    ctx.actions.run(
        executable = wrapper,
        inputs = depset(
            srcs + [protoc, dart, package_config, plugin_wrapper],
            transitive = [
                proto_info.transitive_sources,
                ctx.attr._plugin_srcs.files,
                ctx.attr._protobuf_pkg.files,
                ctx.attr._fixnum_pkg.files,
                ctx.attr._path_pkg.files,
                ctx.attr._meta_pkg.files,
            ],
        ),
        outputs = outs,
        mnemonic = "ProtoDartGen",
        progress_message = "Generating Dart from %s" % ctx.label,
    )

    if ctx.attr.generate_domain_models:
        for i in range(len(srcs)):
            proto_src = srcs[i]
            domain_dart = domain_outs[i]
            ctx.actions.run(
                executable = ctx.executable._model_gen_tool,
                arguments = [proto_src.path, domain_dart.path],
                inputs = depset(
                    [proto_src],
                    transitive = [proto_info.transitive_sources],
                ),
                outputs = [domain_dart],
                mnemonic = "DomainModelGen",
                progress_message = "Generating Domain Model for %s" % proto_src.basename,
            )

    return [DefaultInfo(files = depset(outs + domain_outs))]

proto_dart_library = rule(
    implementation = _proto_dart_library_impl,
    attrs = {
        "protos": attr.label(providers = [ProtoInfo], mandatory = True),
        "use_grpc": attr.bool(default = False),
        "generate_domain_models": attr.bool(default = False),
        "_protoc": attr.label(default = "@protobuf//:protoc", executable = True, cfg = "exec"),
        "_dart_bin": attr.label(default = "@flutter_sdk//:dart_vm", executable = True, cfg = "exec"),
        "_model_gen_tool": attr.label(default = "//bazel/rules/dart/tools/model_gen:generate_domain_models", executable = True, cfg = "exec"),
        "_plugin_srcs": attr.label(default = "@google_protobuf_dart//:protoc_plugin_srcs"),
        "_protobuf_pkg": attr.label(default = "@pub_protobuf//:protobuf"),
        "_fixnum_pkg": attr.label(default = "@pub_fixnum//:fixnum"),
        "_path_pkg": attr.label(default = "@pub_path//:path"),
        "_meta_pkg": attr.label(default = "@pub_meta//:meta"),
        "_collection_pkg": attr.label(default = "@pub_collection//:collection"),
        "_dart_style_pkg": attr.label(default = "@pub_dart_style//:dart_style"),
        "_pub_semver_pkg": attr.label(default = "@pub_pub_semver//:pub_semver"),
    },
)
