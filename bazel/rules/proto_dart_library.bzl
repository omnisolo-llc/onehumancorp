"""Rule for generating Dart code from Protobuf using protoc-gen-dart.

Uses the hermetic protoc-gen-dart built from @google_protobuf_dart//:protoc_plugin_bin.

Generated outputs per proto source:
  • <name>.pb.dart      – message classes with getters/setters, toProto/fromProto
  • <name>.pbenum.dart  – enum classes
  • <name>.pbjson.dart  – JSON helpers (toJSON/fromJSON via toProto3Json /
                          mergeFromProto3Json)
  • <name>.pbgrpc.dart  – gRPC service stubs (when use_grpc = True)
  • <name>.pbyaml.dart  – YAML helpers (toYaml / fromYaml) backed by the
                          package:yaml library (when generate_yaml = True,
                          which is the default)
"""

load("@rules_proto//proto:defs.bzl", "ProtoInfo")

def _proto_dart_library_impl(ctx):
    proto_info = ctx.attr.protos[ProtoInfo]
    srcs = proto_info.direct_sources
    proto_root = proto_info.proto_source_root

    outs = []
    yaml_outs = []

    # protoc-gen-dart emits .pb.dart, .pbenum.dart, .pbjson.dart and,
    # when gRPC is enabled, .pbgrpc.dart.
    for src in srcs:
        basename = src.basename.replace(".proto", "")
        outs.append(ctx.actions.declare_file(basename + ".pb.dart"))
        outs.append(ctx.actions.declare_file(basename + ".pbenum.dart"))
        outs.append(ctx.actions.declare_file(basename + ".pbjson.dart"))
        if ctx.attr.use_grpc:
            outs.append(ctx.actions.declare_file(basename + ".pbgrpc.dart"))
        if ctx.attr.generate_yaml:
            yaml_outs.append(ctx.actions.declare_file(basename + ".pbyaml.dart"))

    protoc = ctx.executable._protoc
    plugin = ctx.executable._plugin

    # Build --proto_path arguments from all transitive sources.
    proto_paths = {}
    for src in proto_info.transitive_sources.to_list():
        proto_paths[src.dirname] = True

    if proto_root and proto_root != ".":
        proto_paths[proto_root] = True
    else:
        proto_paths["."] = True

    proto_path_args = " ".join(["--proto_path=" + p for p in sorted(proto_paths.keys())])
    proto_files = " ".join([src.path for src in srcs])
    out_dir = ctx.bin_dir.path
    dart_out_prefix = "grpc:" if ctx.attr.use_grpc else ""

    # ── protoc wrapper ──────────────────────────────────────────────────────
    wrapper = ctx.actions.declare_file(ctx.label.name + "_protoc_wrapper.sh")
    wrapper_content = "#!/bin/bash\nset -euo pipefail\n"
    wrapper_content += "{protoc} --plugin=protoc-gen-dart={plugin} --dart_out={dart_out_prefix}{out_dir} {proto_path_args} {proto_files}\n".format(
        protoc = protoc.path,
        plugin = plugin.path,
        dart_out_prefix = dart_out_prefix,
        out_dir = out_dir,
        proto_path_args = proto_path_args,
        proto_files = proto_files,
    )

    ctx.actions.write(
        output = wrapper,
        content = wrapper_content,
        is_executable = True,
    )

    ctx.actions.run(
        executable = wrapper,
        inputs = depset(
            srcs + [protoc, plugin],
            transitive = [proto_info.transitive_sources],
        ),
        outputs = outs,
        mnemonic = "ProtoDartGen",
        progress_message = "Generating Dart from %s" % ctx.label,
    )

    # ── YAML helper generation ───────────────────────────────────────────────
    # For each proto source file, run gen_dart_yaml.py to produce a
    # .pbyaml.dart file containing toYaml() / fromYaml() extension methods.
    if ctx.attr.generate_yaml and yaml_outs:
        yaml_gen = ctx.executable._yaml_gen
        for src, yaml_out in zip(srcs, yaml_outs):
            ctx.actions.run(
                executable = yaml_gen,
                arguments = [src.path, yaml_out.path],
                inputs = [src],
                outputs = [yaml_out],
                mnemonic = "ProtoDartYaml",
                progress_message = "Generating YAML helpers for %s" % src.basename,
            )

    return [
        DefaultInfo(files = depset(outs + yaml_outs)),
    ]

proto_dart_library = rule(
    implementation = _proto_dart_library_impl,
    attrs = {
        "protos": attr.label(
            providers = [ProtoInfo],
            mandatory = True,
            doc = "The proto_library target to generate Dart for.",
        ),
        "use_grpc": attr.bool(
            default = False,
            doc = "Whether to generate gRPC service stubs (.pbgrpc.dart).",
        ),
        "generate_yaml": attr.bool(
            default = True,
            doc = "Whether to generate YAML helpers (.pbyaml.dart) using gen_dart_yaml.py.",
        ),
        "_protoc": attr.label(
            default = "@protobuf//:protoc",
            executable = True,
            cfg = "exec",
        ),
        "_plugin": attr.label(
            default = "@google_protobuf_dart//:protoc_plugin_bin",
            executable = True,
            cfg = "exec",
        ),
        "_yaml_gen": attr.label(
            default = "//bazel/rules:gen_dart_yaml",
            executable = True,
            cfg = "exec",
        ),
    },
)
