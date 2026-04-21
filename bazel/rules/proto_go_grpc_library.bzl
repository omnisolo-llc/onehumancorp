"""Rule for generating Go + gRPC code from Protobuf using modern protoc-gen-go.

Supports Protobuf Edition 2024. Uses protoc-gen-go and protoc-gen-go-grpc
binaries built from the go.mod dependencies, which support editions.
"""

load("@rules_proto//proto:defs.bzl", "ProtoInfo")
load("@rules_go//go:def.bzl", "go_library")

def _proto_go_grpc_library_impl(ctx):
    outs = []
    for src in ctx.files.protos:
        basename = src.basename.replace(".proto", "")
        outs.append(ctx.actions.declare_file(basename + ".pb.go"))
        outs.append(ctx.actions.declare_file(basename + "_grpc.pb.go"))

    protoc = ctx.executable._protoc
    gen_go = ctx.executable._gen_go
    gen_go_grpc = ctx.executable._gen_go_grpc
    gen_gotag = ctx.executable._gen_gotag

    out_dir = outs[0].dirname
    # Calculate the relative path from the output directory to the execroot
    proto_path_args = ["-I."]
    for p in ctx.files.protos:
        if p.root.path:
            proto_path_args.append("-I" + p.root.path)

    proto_files = [p.path for p in ctx.files.protos]

    wrapper = ctx.actions.declare_file(ctx.label.name + "_protoc_go_wrapper.sh")
    wrapper_content = "#!/bin/bash\n"
    wrapper_content += "set -e\n"
    wrapper_content += "{protoc} --plugin=protoc-gen-go={gen_go} --plugin=protoc-gen-go-grpc={gen_go_grpc} --plugin=protoc-gen-gotag={gen_gotag} --go_out={out_dir} --go_opt=paths=source_relative --go-grpc_out={out_dir} --go-grpc_opt=paths=source_relative --gotag_out={out_dir} {proto_path_args} {proto_files}\n".format(
        protoc = protoc.path,
        gen_go = gen_go.path,
        gen_go_grpc = gen_go_grpc.path,
        gen_gotag = gen_gotag.path,
        out_dir = out_dir,
        proto_path_args = " ".join(proto_path_args),
        proto_files = " ".join(proto_files),
    )
    ctx.actions.write(
        output = wrapper,
        content = wrapper_content,
        is_executable = True,
    )

    ctx.actions.run(
        outputs = outs,
        inputs = ctx.files.protos + [protoc, gen_go, gen_go_grpc, gen_gotag],
        executable = wrapper,
        mnemonic = "GoProtocGen",
    )

    return [DefaultInfo(files = depset(outs))]

proto_go_grpc_library = rule(
    implementation = _proto_go_grpc_library_impl,
    attrs = {
        "protos": attr.label_list(
            mandatory = True,
            allow_files = [".proto"],
        ),
        "importpath": attr.string(mandatory = True),
        "_protoc": attr.label(
            default = "@protobuf//:protoc",
            executable = True,
            cfg = "exec",
        ),
        "_gen_go": attr.label(
            default = "@gazelle++go_deps+org_golang_google_protobuf//cmd/protoc-gen-go:protoc-gen-go",
            executable = True,
            cfg = "exec",
        ),
        "_gen_go_grpc": attr.label(
            default = "@gazelle++go_deps+org_golang_google_grpc_cmd_protoc_gen_go_grpc//:protoc-gen-go-grpc",
            executable = True,
            cfg = "exec",
        ),
        "_gen_gotag": attr.label(
            default = "@gazelle++go_deps+com_github_srikrsna_protoc_gen_gotag//:protoc-gen-gotag",
            executable = True,
            cfg = "exec",
        ),
    },
)

def go_proto_library_with_tags(name, protos, importpath, visibility = None):
    """Generates a go_library from proto files with custom tags support."""
    proto_go_grpc_library(
        name = name + "_pb_srcs",
        protos = protos,
        importpath = importpath,
    )

    native.go_library(
        name = name,
        srcs = [":" + name + "_pb_srcs"],
        importpath = importpath,
        visibility = visibility,
        deps = [
            "@org_golang_google_grpc//:go_default_library",
            "@org_golang_google_grpc//codes:go_default_library",
            "@org_golang_google_grpc//status:go_default_library",
            "@org_golang_google_protobuf//reflect/protoreflect:go_default_library",
            "@org_golang_google_protobuf//runtime/protoimpl:go_default_library",
            "@org_golang_google_protobuf//types/known/timestamppb:timestamppb",
        ],
    )
