load("@gazelle//:deps.bzl", "go_repository")

def go_repositories():
    go_repository(
        name = "com_github_alicebob_miniredis_v2",
        build_file_generation = "on",
        importpath = "github.com/alicebob/miniredis/v2",
        sum = "h1:nZAzCR+Lj+Vxk4ZXzm2NuKq2O33RXj1XxJ2e2uP9jiw=",
        version = "v2.38.0",
    )
    go_repository(
        name = "com_github_cespare_xxhash_v2",
        build_file_generation = "on",
        importpath = "github.com/cespare/xxhash/v2",
        sum = "h1:UL815xU9SqsFlibzuggzjXhog7bL6oX9BbNZnL2UFvs=",
        version = "v2.3.0",
    )
    go_repository(
        name = "com_github_chzyer_logex",
        build_file_generation = "on",
        importpath = "github.com/chzyer/logex",
        sum = "h1:Swpa1K6QvQznwJRcfTfQJmTE72DqScAa40E+fbHEXEE=",
        version = "v1.1.10",
    )
    go_repository(
        name = "com_github_chzyer_readline",
        build_file_generation = "on",
        importpath = "github.com/chzyer/readline",
        sum = "h1:fY5BOSpyZCqRo5OhCuC+XN+r/bBCmeuuJtjz+bCNIf8=",
        version = "v0.0.0-20180603132655-2972be24d48e",
    )
    go_repository(
        name = "com_github_chzyer_test",
        build_file_generation = "on",
        importpath = "github.com/chzyer/test",
        sum = "h1:q763qf9huN11kDQavWsoZXJNW3xEE4JJyHa5Q25/sd8=",
        version = "v0.0.0-20180213035817-a1ea475d72b1",
    )
    go_repository(
        name = "com_github_davecgh_go_spew",
        build_file_generation = "on",
        importpath = "github.com/davecgh/go-spew",
        sum = "h1:vj9j/u1bqnvCEfJOwUhtlOARqs3+rkHYY13jYWTU97c=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_go_redis_redis",
        importpath = "github.com/go-redis/redis",
        sum = "h1:K0pv1D7EQUjfyoMql+r/jZqCLizCGKFlFgcHWWmHQjg=",
        version = "v6.15.9+incompatible",
    )
    go_repository(
        name = "com_github_go_redis_redis_v7",
        importpath = "github.com/go-redis/redis/v7",
        sum = "h1:PASvf36gyUpr2zdOUS/9Zqc80GbM+9BDyiJSJDDOrTI=",
        version = "v7.4.1",
    )
    go_repository(
        name = "com_github_go_redis_redis_v8",
        importpath = "github.com/go-redis/redis/v8",
        sum = "h1:AcZZR7igkdvfVmQTPnu9WE37LRrO/YrBH5zWyjDC0oI=",
        version = "v8.11.5",
    )
    go_repository(
        name = "com_github_go_redsync_redsync_v4",
        importpath = "github.com/go-redsync/redsync/v4",
        sum = "h1:bNcOzeHH9d3s6pghU9NJFMPrQa41f5Nx3L4YKr3BdEU=",
        version = "v4.16.0",
    )
    go_repository(
        name = "com_github_gomodule_redigo",
        importpath = "github.com/gomodule/redigo",
        sum = "h1:dNPSXeXv6HCq2jdyWfjgmhBdqnR6PRO3m/G05nvpPC8=",
        version = "v1.9.3",
    )
    go_repository(
        name = "com_github_hashicorp_errwrap",
        importpath = "github.com/hashicorp/errwrap",
        sum = "h1:OxrOeh75EUXMY8TBjag2fzXGZ40LB6IKw45YeGUDY2I=",
        version = "v1.1.0",
    )
    go_repository(
        name = "com_github_hashicorp_go_multierror",
        importpath = "github.com/hashicorp/go-multierror",
        sum = "h1:H5DkEtf6CXdFp0N0Em5UCwQpXMWke8IA0+lD48awMYo=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_pmezard_go_difflib",
        build_file_generation = "on",
        importpath = "github.com/pmezard/go-difflib",
        sum = "h1:4DBwDE0NGyQoBHbLQYPwSUPoCMWR5BEzIk/f1lZbAQM=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_redis_rueidis",
        build_file_generation = "on",
        importpath = "github.com/redis/rueidis",
        sum = "h1:pODtnAR5GAB7j4ekhldZ29HKOxe4Hph0GTDGk1ayEQY=",
        version = "v1.0.71",
    )
    go_repository(
        name = "com_github_redis_rueidis_rueidiscompat",
        importpath = "github.com/redis/rueidis/rueidiscompat",
        sum = "h1:wNZ//kEjMZgBM0KCk7ncOX8KmAgROU2kDdDNpwheG4w=",
        version = "v1.0.71",
    )
    go_repository(
        name = "com_github_stretchr_testify",
        build_file_generation = "on",
        importpath = "github.com/stretchr/testify",
        sum = "h1:TivCn/peBQ7UY8ooIcPgZFpTNSz0Q2U6UrFlUfqbe0Q=",
        version = "v1.3.0",
    )
    go_repository(
        name = "com_github_stvp_tempredis",
        importpath = "github.com/stvp/tempredis",
        sum = "h1:QVqDTf3h2WHt08YuiTGPZLls0Wq99X9bWd0Q5ZSBesM=",
        version = "v0.0.0-20181119212430-b82af8480203",
    )
    go_repository(
        name = "com_github_valkey_io_valkey_go",
        importpath = "github.com/valkey-io/valkey-go",
        sum = "h1:iRWt1hJyOchcEgbHSkRY3aKkcBudxvMaVMsmxuYxuxE=",
        version = "v1.0.72",
    )
    go_repository(
        name = "com_github_valkey_io_valkey_go_valkeycompat",
        importpath = "github.com/valkey-io/valkey-go/valkeycompat",
        sum = "h1:Lyyr0kJ4QWwU8thgc2fqlCH49jG7hjR9ldhLRf4dE84=",
        version = "v1.0.72",
    )
    go_repository(
        name = "com_github_yuin_gopher_lua",
        build_file_generation = "on",
        importpath = "github.com/yuin/gopher-lua",
        sum = "h1:kYKnWBjvbNP4XLT3+bPEwAXJx262OhaHDWDVOPjL46M=",
        version = "v1.1.1",
    )
    go_repository(
        name = "org_golang_x_sync",
        build_file_generation = "on",
        importpath = "golang.org/x/sync",
        sum = "h1:vV+1eWNmZ5geRlYjzm2adRgW2/mcpevXNg50YZtPCE4=",
        version = "v0.19.0",
    )
    go_repository(
        name = "org_golang_x_sys",
        build_file_generation = "on",
        importpath = "golang.org/x/sys",
        sum = "h1:CvCKL8MeisomCi6qNZ+wbb0DN9E5AATixKsvNtMoMFk=",
        version = "v0.39.0",
    )
