load("@gazelle//:deps.bzl", "go_repository")

def go_deps():
    go_repository(
        name = "com_github_fatih_structtag",
        importpath = "github.com/fatih/structtag",
        sum = "h1:/OdNE99OxoI/PqaW/SuSK9uxxT3f/tcSZgon/ssNSx4=",
        version = "v1.2.0",
    )
    go_repository(
        name = "com_github_kr_fs",
        importpath = "github.com/kr/fs",
        sum = "h1:Jskdu9ieNAYnjxsi0LbQp1ulIKZV1LAFgK1tWhpZgl8=",
        version = "v0.1.0",
    )
    go_repository(
        name = "com_github_lyft_protoc_gen_star_v2",
        importpath = "github.com/lyft/protoc-gen-star/v2",
        sum = "h1:/3+/2sWyXeMLzKd1bX+ixWKgEMsULrIivpDsuaF441o=",
        version = "v2.0.3",
    )
    go_repository(
        name = "com_github_pkg_errors",
        importpath = "github.com/pkg/errors",
        sum = "h1:iURUrRGxPUNPdy5/HRSm+Yj6okJ6UtLINN0Q9M4+h3I=",
        version = "v0.8.1",
    )
    go_repository(
        name = "com_github_pkg_sftp",
        importpath = "github.com/pkg/sftp",
        sum = "h1:VasscCm72135zRysgrJDKsntdmPN+OuU3+nnHYA9wyc=",
        version = "v1.10.1",
    )
    go_repository(
        name = "com_github_srikrsna_protoc_gen_gotag",
        importpath = "github.com/srikrsna/protoc-gen-gotag",
        sum = "h1:4okv8GlbVbvmL678VX0AobxaMkERlBbHvgWhUnbcrPM=",
        version = "v1.0.2",
    )
    go_repository(
        name = "in_gopkg_yaml_v2",
        importpath = "gopkg.in/yaml.v2",
        sum = "h1:ZCJp+EgiOT7lHqUV2J862kp8Qj64Jo6az82+3Td9dZw=",
        version = "v2.2.2",
    )
    go_repository(
        name = "org_golang_x_telemetry",
        importpath = "golang.org/x/telemetry",
        sum = "h1:bTLqdHv7xrGlFbvf5/TXNxy/iUwwdkjhqQTJDjW7aj0=",
        version = "v0.0.0-20260209163413-e7419c687ee4",
    )
    go_repository(
        name = "com_github_redis_rueidis",
        importpath = "github.com/redis/rueidis",
        sum = "h1:J5ZNyxMqX+sDQxQztRI928W6TrERpo+pHSwhftnX7NA=",
        version = "v1.0.35",
    )
    go_repository(
        name = "com_github_alicebob_miniredis_v2",
        importpath = "github.com/alicebob/miniredis/v2",
        sum = "h1:1wKzOa0D1J7o2Xp6wR3zNl2LgD3P81Q3w0o2w9X0QxI=",
        version = "v2.37.0",
    )
    go_repository(
        name = "com_github_yuin_gopher_lua",
        importpath = "github.com/yuin/gopher-lua",
        sum = "h1:R5QAZKx+H3B9U9nUv302uC/uR1dOEQyY2859G5D/53A=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_alicebob_gopher_json",
        importpath = "github.com/alicebob/gopher-json",
        sum = "h1:1M+pA47l6J4Tngc+9r5tI7l01z7mI1e0n+A5wL4cOQw=",
        version = "v0.0.0-20230218143504-906a9b012302",
    )
