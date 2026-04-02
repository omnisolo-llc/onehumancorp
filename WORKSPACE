load("@gazelle//:deps.bzl", "go_repository")

workspace(name = "mono")

load("//:repositories.bzl", "go_repositories")

go_repository(
    name = "com_github_jackc_pgpassfile",
    importpath = "github.com/jackc/pgpassfile",
    sum = "h1:/6Hmqy13Ss2zCq62VdNG8tM1wchn8zjSGOBJ6icpsIM=",
    version = "v1.0.0",
)

go_repository(
    name = "com_github_jackc_pgservicefile",
    importpath = "github.com/jackc/pgservicefile",
    sum = "h1:iCEnooe7UlwOQYpKFhBabPMi4aNAfoODPEFNiAnClxo=",
    version = "v0.0.0-20240606120523-5a60cdf6a761",
)

go_repository(
    name = "com_github_jackc_pgx_v5",
    importpath = "github.com/jackc/pgx/v5",
    sum = "h1:uwrxJXBnx76nyISkhr33kQLlUqjv7et7b9FjCen/tdc=",
    version = "v5.9.1",
)

go_repository(
    name = "com_github_jackc_puddle_v2",
    importpath = "github.com/jackc/puddle/v2",
    sum = "h1:PR8nw+E/1w0GLuRFSmiioY6UooMp6KJv0/61nB7icHo=",
    version = "v2.2.2",
)

go_repository(
    name = "com_github_stretchr_objx",
    importpath = "github.com/stretchr/objx",
    sum = "h1:xuMeJ0Sdp5ZMRXx/aWO6RZxdr3beISkG5/G/aIRr3pY=",
    version = "v0.5.2",
)

# gazelle:repository_macro repositories.bzl%go_repositories
go_repositories()
