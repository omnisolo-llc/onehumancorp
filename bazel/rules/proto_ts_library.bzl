load("@rules_proto//proto:defs.bzl", "ProtoInfo")

def _proto_ts_library_impl(ctx):
    proto_info = ctx.attr.protos[ProtoInfo]
    srcs = proto_info.direct_sources
    proto_root = proto_info.proto_source_root

    outs = []
    for src in srcs:
        basename = src.basename[:-6] if src.basename.endswith(".proto") else src.basename
        out = ctx.actions.declare_file(basename + ".ts")
        outs.append(out)

    # Our tool is the js_binary wrapper that executes protoc.
    # It takes protoc_path as the FIRST argument.
    wrapper = ctx.executable._tool
    protoc = ctx.executable._protoc
    
    # Build proto_path args. 
    # We generally want the workspace root as a proto_path.
    proto_path_args = ["--proto_path=."]
    
    # If there's a proto_root (e.g. from an external repo), add it.
    if proto_root and proto_root != ".":
        proto_path_args.append("--proto_path=" + proto_root)

    proto_files = [src.path for src in srcs]
    
    ts_proto_opts = "esModuleInterop=true,forceLong=string,outputServices=generic-definitions,useOptionals=all"

    # We run the wrapper with protoc and its arguments.
    # The wrapper internally handles --plugin=... resolving to the correct ts-proto.
    # BAZEL_BINDIR is required by the js_binary launcher in aspect_rules_js.
    ctx.actions.run(
        executable = wrapper,
        arguments = [
            protoc.path,
            "--ts_proto_out=" + ctx.bin_dir.path,
            "--ts_proto_opt=" + ts_proto_opts,
        ] + proto_path_args + proto_files,
        inputs = depset(
            srcs + [protoc],
            transitive = [
                proto_info.transitive_sources,
            ],
        ),
        outputs = outs,
        env = {
            "BAZEL_BINDIR": ctx.bin_dir.path,
        },
        mnemonic = "ProtoTsGen",
        progress_message = "Generating ts-proto v2 from %s" % ctx.label,
    )

    return [
        DefaultInfo(files = depset(outs)),
    ]

proto_ts_library = rule(
    implementation = _proto_ts_library_impl,
    attrs = {
        "protos": attr.label(
            providers = [ProtoInfo],
            mandatory = True,
            doc = "The proto_library target to generate TypeScript for.",
        ),
        "_protoc": attr.label(
            default = "@protobuf//:protoc",
            executable = True,
            cfg = "exec",
        ),
        "_tool": attr.label(
            default = "//bazel/rules:ts_proto_wrapper",
            executable = True,
            cfg = "exec",
        ),
    },
    doc = "Generates TypeScript files from a proto_library using ts-proto v2. Supports Edition 2024.",
)
