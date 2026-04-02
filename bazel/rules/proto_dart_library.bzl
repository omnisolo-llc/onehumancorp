"""Rule for generating Dart code from Protobuf using protoc-gen-dart.

Uses the hermetic protoc-gen-dart built from @google_protobuf_dart//:protoc_plugin_bin.
"""

load("@rules_proto//proto:defs.bzl", "ProtoInfo")

def _proto_dart_library_impl(ctx):
    proto_info = ctx.attr.protos[ProtoInfo]
    srcs = proto_info.direct_sources
    proto_root = proto_info.proto_source_root

    outs = []
    # Modern protoc-gen-dart emits .pb.dart, .pbenum.dart, .pbjson.dart and,
    # when gRPC is enabled, .pbgrpc.dart.
    for src in srcs:
        basename = src.basename.replace(".proto", "")
        outs.append(ctx.actions.declare_file(basename + ".pb.dart"))
        outs.append(ctx.actions.declare_file(basename + ".pbenum.dart"))
        outs.append(ctx.actions.declare_file(basename + ".pbjson.dart"))
        if ctx.attr.use_grpc:
            outs.append(ctx.actions.declare_file(basename + ".pbgrpc.dart"))

    protoc = ctx.executable._protoc
    plugin = ctx.executable._plugin
    
    # Build proto_path args
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

    # Create wrapper script
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

    return [
        DefaultInfo(files = depset(outs)),
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
            doc = "Whether to generate gRPC service files.",
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
    },
)
