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
    name = "com_github_gorilla_websocket",
    build_file_generation = "on",
    importpath = "github.com/gorilla/websocket",
    sum = "h1:saDtZ6Pbx/0u+bgYQ3q96pZgCzfhKXGPqt7kZ72aNNg=",
    version = "v1.5.3",
)

go_repository(
    name = "com_github_jackc_pgpassfile",
    build_file_generation = "on",
    importpath = "github.com/jackc/pgpassfile",
    sum = "h1:/6Hmqy13Ss2zCq62VdNG8tM1wchn8zjSGOBJ6icpsIM=",
    version = "v1.0.0",
)

go_repository(
    name = "com_github_jackc_pgservicefile",
    build_file_generation = "on",
    importpath = "github.com/jackc/pgservicefile",
    sum = "h1:iCEnooe7UlwOQYpKFhBabPMi4aNAfoODPEFNiAnClxo=",
    version = "v0.0.0-20240606120523-5a60cdf6a761",
)

go_repository(
    name = "com_github_jackc_pgx_v5",
    build_file_generation = "on",
    importpath = "github.com/jackc/pgx/v5",
    sum = "h1:uwrxJXBnx76nyISkhr33kQLlUqjv7et7b9FjCen/tdc=",
    version = "v5.9.1",
)

go_repository(
    name = "com_github_jackc_puddle_v2",
    build_file_generation = "on",
    importpath = "github.com/jackc/puddle/v2",
    sum = "h1:PR8nw+E/1w0GLuRFSmiioY6UooMp6KJv0/61nB7icHo=",
    version = "v2.2.2",
)

go_repository(
    name = "com_github_klauspost_cpuid_v2",
    build_file_generation = "on",
    importpath = "github.com/klauspost/cpuid/v2",
    sum = "h1:0OwqZRYI2rFrjS4kvkDnqJkKHdHaRnCm68/DY4OxRzU=",
    version = "v2.2.11",
)

go_repository(
    name = "com_github_redis_go_redis_v9",
    build_file_generation = "on",
    importpath = "github.com/redis/go-redis/v9",
    sum = "h1:pMkxYPkEbMPwRdenAzUNyFNrDgHx9U+DrBabWNfSRQs=",
    version = "v9.18.0",
)

go_repository(
    name = "com_github_stretchr_objx",
    build_file_generation = "on",
    importpath = "github.com/stretchr/objx",
    sum = "h1:xuMeJ0Sdp5ZMRXx/aWO6RZxdr3beISkG5/G/aIRr3pY=",
    version = "v0.5.2",
)

go_repository(
    name = "com_github_zeebo_xxh3",
    build_file_generation = "on",
    importpath = "github.com/zeebo/xxh3",
    sum = "h1:xZmwmqxHZA8AI603jOQ0tMqmBr9lPeFwGg6d+xy9DC0=",
    version = "v1.0.2",
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

go_repository(
    name = "com_github_stretchr_testify",
    build_file_generation = "on",
    importpath = "github.com/stretchr/testify",
    sum = "h1:w7B6lhMri9wdJUVmEZPGGhZzrYTPvgJArz7wNPgYK4Q=",
    version = "v1.8.4",
)

go_repository(
    name = "com_github_go_redsync_redsync_v4",
    build_file_generation = "on",
    importpath = "github.com/go-redsync/redsync/v4",
    sum = "h1:09PjW+uA6T+xN7C57/w9uG67dO3j7oO5lSThY1O0E/c=",
    version = "v4.16.0",
)

go_repository(
    name = "com_github_data_dog_go_sqlmock",
    build_file_generation = "on",
    importpath = "github.com/DATA-DOG/go-sqlmock",
    sum = "h1:0D0w5o3dM7m9Gj1nJ5u4G/w3w3Yw3m0tZ5gX1/aN+rA=",
    version = "v1.5.2",
)
