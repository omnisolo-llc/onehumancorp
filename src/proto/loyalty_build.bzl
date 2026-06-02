proto_library(
    name = "loyalty_proto",
    srcs = ["loyalty.proto"],
    visibility = ["//visibility:public"],
)

load("@rules_rust//rust:defs.bzl", "rust_library")
load("@rules_rust_prost//:defs.bzl", "rust_prost_library")

rust_prost_library(
    name = "loyalty_prost",
    proto = ":loyalty_proto",
    visibility = ["//visibility:public"],
)
