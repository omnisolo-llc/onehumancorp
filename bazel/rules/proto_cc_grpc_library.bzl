"""Rule for generating C++ + gRPC code from a proto_library.

Uses the protoc binary from @protobuf and the grpc_cpp_plugin from @grpc.
Supports Protobuf Edition 2024 (requires protoc >= 26.x, which protobuf 34.x
provides).
"""

load("@rules_proto//proto:defs.bzl", "ProtoInfo")
load("@rules_cc//cc:defs.bzl", "cc_library")

def _proto_cc_grpc_srcs_impl(ctx):
    proto_info = ctx.attr.protos[ProtoInfo]
    srcs = proto_info.direct_sources
    proto_root = proto_info.proto_source_root

    outs = []
    for src in srcs:
        basename = src.basename.replace(".proto", "")
        outs.append(ctx.actions.declare_file(basename + ".pb.h"))
        outs.append(ctx.actions.declare_file(basename + ".pb.cc"))
        outs.append(ctx.actions.declare_file(basename + ".grpc.pb.h"))
        outs.append(ctx.actions.declare_file(basename + ".grpc.pb.cc"))

    protoc = ctx.executable._protoc
    grpc_plugin = ctx.executable._grpc_cpp_plugin

    proto_paths = {}
    for src in srcs:
        if proto_root and proto_root != ".":
            proto_paths[proto_root] = True
        else:
            proto_paths[src.dirname] = True

    proto_path_args = " ".join(
        ["--proto_path=" + p for p in sorted(proto_paths.keys())]
    )
    out_dir = outs[0].dirname

    wrapper = ctx.actions.declare_file(ctx.label.name + "_protoc_cc_wrapper.sh")
    wrapper_content = "#!/bin/bash\nset -euo pipefail\n"
    wrapper_content += "tmpdir=$(mktemp -d)\n"
    wrapper_content += "trap 'rm -rf \"$tmpdir\"' EXIT\n"
    wrapper_content += "sanitized_files=()\n"
    for src in srcs:
        wrapper_content += "cp {src} \"$tmpdir/{basename}\"\n".format(
            src = src.path,
            basename = src.basename,
        )
        # grpc_cpp_plugin does not yet understand Edition 2024. Generate the
        # C++ stubs from a temporary proto3-compatible copy while keeping the
        # checked-in proto on Edition 2024 for Go and other toolchains.
        wrapper_content += (
            "sed -E -i 's/^edition = \"[0-9]+\";$/syntax = \"proto3\";/' " +
            "\"$tmpdir/{basename}\"\n"
        ).format(basename = src.basename)
        wrapper_content += "sanitized_files+=(\"$tmpdir/{basename}\")\n".format(
            basename = src.basename,
        )
    wrapper_content += (
        "{protoc} --plugin=protoc-gen-grpc={grpc_plugin}" +
        " --cpp_out={out_dir}" +
        " --grpc_out={out_dir}" +
        " --proto_path=\"$tmpdir\"" +
        " {proto_path_args} \"${{sanitized_files[@]}}\"\n"
    ).format(
        protoc = protoc.path,
        grpc_plugin = grpc_plugin.path,
        out_dir = out_dir,
        proto_path_args = proto_path_args,
    )

    ctx.actions.write(
        output = wrapper,
        content = wrapper_content,
        is_executable = True,
    )

    ctx.actions.run(
        executable = wrapper,
        inputs = depset(
            srcs + [protoc, grpc_plugin],
            transitive = [proto_info.transitive_sources],
        ),
        outputs = outs,
        mnemonic = "ProtoCCGrpcGen",
        progress_message = "Generating C++ + gRPC from %s" % ctx.label,
    )

    return [DefaultInfo(files = depset(outs))]

_proto_cc_grpc_srcs = rule(
    implementation = _proto_cc_grpc_srcs_impl,
    attrs = {
        "protos": attr.label(
            providers = [ProtoInfo],
            mandatory = True,
        ),
        "_protoc": attr.label(
            default = "@protobuf//:protoc",
            executable = True,
            cfg = "exec",
        ),
        "_grpc_cpp_plugin": attr.label(
            default = "@grpc//src/compiler:grpc_cpp_plugin",
            executable = True,
            cfg = "exec",
        ),
    },
)

def proto_cc_grpc_library(name, protos, visibility = None):
    """Generates a C++ cc_library from a proto_library with gRPC support.

    Produces both the protobuf message types (.pb.h/.pb.cc) and the gRPC
    service stubs (.grpc.pb.h/.grpc.pb.cc).

    Args:
        name:       Name of the resulting cc_library target.
        protos:     Label of the proto_library target.
        visibility: Visibility of the generated library.
    """
    srcs_name = name + "_cc_grpc_srcs"

    _proto_cc_grpc_srcs(
        name = srcs_name,
        protos = protos,
    )

    cc_library(
        name = name,
        srcs = [":" + srcs_name],
        copts = ["-std=c++20"],
        visibility = visibility,
        deps = [
            "@com_google_absl//absl/status",
            "@grpc//:grpc++",
            "@protobuf//:protobuf",
        ],
    )
