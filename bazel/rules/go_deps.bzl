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
        name = "com_github_go_redsync_redsync_v4",
        importpath = "github.com/go-redsync/redsync/v4",
        sum = "h1:09PjW+uA6T+xN7C57/w9uG67dO3j7oO5lSThY1O0E/c=",
        version = "v4.16.0",
    )
