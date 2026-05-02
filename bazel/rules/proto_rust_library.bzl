load("@rules_rust_prost//:defs.bzl", "rust_prost_library")

def proto_rust_library(name, proto, use_grpc = False, visibility = None):
    """
    A macro to generate rust_prost_library targets for a given proto_library.
    Currently, both _prost and _tonic targets are generated as they are identical
    when the toolchain is configured with Tonic.
    """
    rust_prost_library(
        name = name + "_prost",
        proto = proto,
        visibility = visibility,
    )

    if use_grpc:
        rust_prost_library(
            name = name + "_tonic",
            proto = proto,
            visibility = visibility,
        )
