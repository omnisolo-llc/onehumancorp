load("@gazelle//:deps.bzl", "go_repository")

workspace(name = "mono")

load("//:repositories.bzl", "go_repositories")

go_repository(
    name = "com_github_bsm_ginkgo_v2",
    build_file_generation = "on",
    importpath = "github.com/bsm/ginkgo/v2",
    sum = "h1:Ny8MWAHyOepLGlLKYmXG4IEkioBysk6GpaRTLC8zwWs=",
    version = "v2.12.0",
)

go_repository(
    name = "com_github_bsm_gomega",
    build_file_generation = "on",
    importpath = "github.com/bsm/gomega",
    sum = "h1:yeMWxP2pV2fG3FgAODIY8EiRE3dy0aeFYt4l7wh6yKA=",
    version = "v1.27.10",
)

go_repository(
    name = "com_github_dgryski_go_rendezvous",
    build_file_generation = "on",
    importpath = "github.com/dgryski/go-rendezvous",
    sum = "h1:lO4WD4F/rVNCu3HqELle0jiPLLBs70cWOduZpkS1E78=",
    version = "v0.0.0-20200823014737-9f7001d12a5f",
)

go_repository(
    name = "com_github_klauspost_cpuid_v2",
    build_file_generation = "on",
    importpath = "github.com/klauspost/cpuid/v2",
    sum = "h1:tBs3QSyvjDyFTq3uoc/9xFpCuOsJQFNPiAhYdw2skhE=",
    version = "v2.2.10",
)

go_repository(
    name = "com_github_redis_go_redis_v9",
    build_file_generation = "on",
    importpath = "github.com/redis/go-redis/v9",
    sum = "h1:XPVaaPSnG6RhYf7p+rmSa9zZfeVAnWsH5h3lxthOm/k=",
    version = "v9.19.0",
)

go_repository(
    name = "com_github_zeebo_xxh3",
    build_file_generation = "on",
    importpath = "github.com/zeebo/xxh3",
    sum = "h1:s7DLGDK45Dyfg7++yxI0khrfwq9661w9EN78eP/UZVs=",
    version = "v1.1.0",
)

go_repository(
    name = "org_uber_go_atomic",
    build_file_generation = "on",
    importpath = "go.uber.org/atomic",
    sum = "h1:ZvwS0R+56ePWxUNi+Atn9dWONBPp/AUETXlHW0DxSjE=",
    version = "v1.11.0",
)

load("//:bazel/rules/go_deps.bzl", "go_deps")

# gazelle:repository_macro bazel/rules/go_deps.bzl%go_deps
go_deps()

# gazelle:repository_macro repositories.bzl%go_repositories
go_repositories()
