load("@gazelle//:deps.bzl", "go_repository")

def go_repositories():
    go_repository(
        name = "co_honnef_go_tools",
        importpath = "honnef.co/go/tools",
        sum = "h1:/hemPrYIhOhy8zYrNj+069zDB68us2sMGsfkFJO0iZs=",
        version = "v0.0.0-20190523083050-ea95bdfd59fc",
    )
    go_repository(
        name = "com_github_a2aproject_a2a_go",
        importpath = "github.com/a2aproject/a2a-go",
        sum = "h1:WpIcSHgCySIxD7OQEdV7U7WJc/HL/G2QQj0RJ0YhPi0=",
        version = "v0.3.13",
    )
    go_repository(
        name = "com_github_alecthomas_kingpin_v2",
        build_file_generation = "on",
        importpath = "github.com/alecthomas/kingpin/v2",
        sum = "h1:f48lwail6p8zpO1bC4TxtqACaGqHYA22qkHjHpqDjYY=",
        version = "v2.4.0",
    )
    go_repository(
        name = "com_github_alecthomas_units",
        build_file_generation = "on",
        importpath = "github.com/alecthomas/units",
        sum = "h1:mimo19zliBX/vSQ6PWWSL9lK8qwHozUj03+zLoEB8O0=",
        version = "v0.0.0-20240927000941-0f3dac36c52b",
    )
    go_repository(
        name = "com_github_alicebob_miniredis_v2",
        build_file_generation = "on",
        importpath = "github.com/alicebob/miniredis/v2",
        sum = "h1:RheObYW32G1aiJIj81XVt78ZHJpHonHLHW7OLIshq68=",
        version = "v2.37.0",
    )
    go_repository(
        name = "com_github_andybalholm_brotli",
        build_file_generation = "on",
        importpath = "github.com/andybalholm/brotli",
        sum = "h1:ukwgCxwYrmACq68yiUqwIWnGY0cTPox/M94sVwToPjQ=",
        version = "v1.2.0",
    )
    go_repository(
        name = "com_github_antlr4_go_antlr_v4",
        build_file_generation = "on",
        importpath = "github.com/antlr4-go/antlr/v4",
        sum = "h1:SqQKkuVZ+zWkMMNkjy5FZe5mr5WURWnlpmOuzYWrPrQ=",
        version = "v4.13.1",
    )
    go_repository(
        name = "com_github_awalterschulze_gographviz",
        importpath = "github.com/awalterschulze/gographviz",
        sum = "h1:9sVEXJBJLwGX7EQVhLm2elIKCm7P2YHFC8v6096G09E=",
        version = "v2.0.3+incompatible",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2",
        importpath = "github.com/aws/aws-sdk-go-v2",
        sum = "h1:DWpAJt66FmnnaRIOT/8ASTucrvuDPZASqhhLey6tLY8=",
        version = "v1.41.7",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_aws_protocol_eventstream",
        importpath = "github.com/aws/aws-sdk-go-v2/aws/protocol/eventstream",
        sum = "h1:gx1AwW1Iyk9Z9dD9F4akX5gnN3QZwUB20GGKH/I+Rho=",
        version = "v1.7.10",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_config",
        importpath = "github.com/aws/aws-sdk-go-v2/config",
        sum = "h1:FpL4/758/diKwqbytU0prpuiu60fgXKUWCpDJtApclU=",
        version = "v1.32.17",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_credentials",
        importpath = "github.com/aws/aws-sdk-go-v2/credentials",
        sum = "h1:r3RJBuU7X9ibt8RHbMjWE6y60QbKBiII6wSrXnapxSU=",
        version = "v1.19.16",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_feature_ec2_imds",
        importpath = "github.com/aws/aws-sdk-go-v2/feature/ec2/imds",
        sum = "h1:UuSfcORqNSz/ey3VPRS8TcVH2Ikf0/sC+Hdj400QI6U=",
        version = "v1.18.23",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_internal_configsources",
        importpath = "github.com/aws/aws-sdk-go-v2/internal/configsources",
        sum = "h1:GpT/TrnBYuE5gan2cZbTtvP+JlHsutdmlV2YfEyNde0=",
        version = "v1.4.23",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_internal_endpoints_v2",
        importpath = "github.com/aws/aws-sdk-go-v2/internal/endpoints/v2",
        sum = "h1:bpd8vxhlQi2r1hiueOw02f/duEPTMK59Q4QMAoTTtTo=",
        version = "v2.7.23",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_internal_v4a",
        importpath = "github.com/aws/aws-sdk-go-v2/internal/v4a",
        sum = "h1:OQqn11BtaYv1WLUowvcA30MpzIu8Ti4pcLPIIyoKZrA=",
        version = "v1.4.24",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_internal_accept_encoding",
        importpath = "github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding",
        sum = "h1:FLudkZLt5ci0ozzgkVo8BJGwvqNaZbTWb3UcucAateA=",
        version = "v1.13.9",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_internal_checksum",
        importpath = "github.com/aws/aws-sdk-go-v2/service/internal/checksum",
        sum = "h1:ieLCO1JxUWuxTZ1cRd0GAaeX7O6cIxnwk7tc1LsQhC4=",
        version = "v1.9.15",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_internal_presigned_url",
        importpath = "github.com/aws/aws-sdk-go-v2/service/internal/presigned-url",
        sum = "h1:pbrxO/kuIwgEsOPLkaHu0O+m4fNgLU8B3vxQ+72jTPw=",
        version = "v1.13.23",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_internal_s3shared",
        importpath = "github.com/aws/aws-sdk-go-v2/service/internal/s3shared",
        sum = "h1:03xatSQO4+AM1lTAbnRg5OK528EUg744nW7F73U8DKw=",
        version = "v1.19.23",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_s3",
        importpath = "github.com/aws/aws-sdk-go-v2/service/s3",
        sum = "h1:mxuT1xE+dI54NW3RkNjP8DUT5HXqbkiAFvfdyDFwE5c=",
        version = "v1.100.1",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_signin",
        importpath = "github.com/aws/aws-sdk-go-v2/service/signin",
        sum = "h1:TdJ+HdzOBhU8+iVAOGUTU63VXopcumCOF1paFulHWZc=",
        version = "v1.0.11",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_sso",
        importpath = "github.com/aws/aws-sdk-go-v2/service/sso",
        sum = "h1:7byT8HUWrgoRp6sXjxtZwgOKfhss5fW6SkLBtqzgRoE=",
        version = "v1.30.17",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_ssooidc",
        importpath = "github.com/aws/aws-sdk-go-v2/service/ssooidc",
        sum = "h1:+1Kl1zx6bWi4X7cKi3VYh29h8BvsCoHQEQ6ST9X8w7w=",
        version = "v1.35.21",
    )
    go_repository(
        name = "com_github_aws_aws_sdk_go_v2_service_sts",
        importpath = "github.com/aws/aws-sdk-go-v2/service/sts",
        sum = "h1:F/M5Y9I3nwr2IEpshZgh1GeHpOItExNM9L1euNuh/fk=",
        version = "v1.42.1",
    )
    go_repository(
        name = "com_github_aws_smithy_go",
        importpath = "github.com/aws/smithy-go",
        sum = "h1:J8ERsGSU7d+aCmdQur5Txg6bVoYelvQJgtZehD12GkI=",
        version = "v1.25.1",
    )

    go_repository(
        name = "com_github_beorn7_perks",
        build_file_generation = "on",
        importpath = "github.com/beorn7/perks",
        sum = "h1:VlbKKnNfV8bJzeqoa4cOKqO6bYr3WgKZxO8Z16+hsOM=",
        version = "v1.0.1",
    )
    go_repository(
        name = "com_github_burntsushi_toml",
        importpath = "github.com/BurntSushi/toml",
        sum = "h1:WXkYYl6Yr3qBf1K79EBnL4mak0OimBfB0XUf9Vl28OQ=",
        version = "v0.3.1",
    )
    go_repository(
        name = "com_github_cenkalti_backoff_v5",
        importpath = "github.com/cenkalti/backoff/v5",
        sum = "h1:ZN+IMa753KfX5hd8vVaMixjnqRZ3y8CuJKRKj1xcsSM=",
        version = "v5.0.3",
    )
    go_repository(
        name = "com_github_census_instrumentation_opencensus_proto",
        importpath = "github.com/census-instrumentation/opencensus-proto",
        sum = "h1:glEXhBS5PSLLv4IXzLA5yPRVX4bilULVyxxbrfOtDAk=",
        version = "v0.2.1",
    )
    go_repository(
        name = "com_github_centrifugal_centrifuge",
        build_file_generation = "on",
        importpath = "github.com/centrifugal/centrifuge",
        sum = "h1:UJTowwc5lSwnpvd3vbrTseODbU7osSggN67RTrJ8EfQ=",
        version = "v0.38.0",
    )
    go_repository(
        name = "com_github_centrifugal_protocol",
        build_file_generation = "on",
        importpath = "github.com/centrifugal/protocol",
        sum = "h1:hD0WczyiG7zrVJcgkQsd5/nhfFXt0Y04SJHV2Z7B1rg=",
        version = "v0.17.0",
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
        name = "com_github_clickhouse_ch_go",
        build_file_generation = "on",
        importpath = "github.com/ClickHouse/ch-go",
        sum = "h1:bUdZ/EZj/LcVHsMqaRUP2holqygrPWQKeMjc6nZoyRM=",
        version = "v0.71.0",
    )
    go_repository(
        name = "com_github_clickhouse_clickhouse_go_v2",
        build_file_generation = "on",
        importpath = "github.com/ClickHouse/clickhouse-go/v2",
        sum = "h1:fUR05TrF1GyvLDa/mAQjkx7KbgwdLRffs2n9O3WobtE=",
        version = "v2.43.0",
    )

    go_repository(
        name = "com_github_client9_misspell",
        importpath = "github.com/client9/misspell",
        sum = "h1:ta993UF76GwbvJcIo3Y68y/M3WxlpEHPWIGDkJYwzJI=",
        version = "v0.3.4",
    )
    go_repository(
        name = "com_github_cncf_xds_go",
        build_file_generation = "on",
        importpath = "github.com/cncf/xds/go",
        sum = "h1:6xNmx7iTtyBRev0+D/Tv1FZd4SCg8axKApyNyRsAt/w=",
        version = "v0.0.0-20251210132809-ee656c7534f5",
    )
    go_repository(
        name = "com_github_coder_websocket",
        build_file_generation = "on",
        importpath = "github.com/coder/websocket",
        sum = "h1:9L0p0iKiNOibykf283eHkKUHHrpG7f65OE3BhhO7v9g=",
        version = "v1.8.14",
    )
    go_repository(
        name = "com_github_containerd_errdefs",
        build_file_generation = "on",
        importpath = "github.com/containerd/errdefs",
        sum = "h1:tg5yIfIlQIrxYtu9ajqY42W3lpS19XqdxRQeEwYG8PI=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_containerd_errdefs_pkg",
        build_file_generation = "on",
        importpath = "github.com/containerd/errdefs/pkg",
        sum = "h1:9IKJ06FvyNlexW690DXuQNx2KA2cUJXx151Xdx3ZPPE=",
        version = "v0.3.0",
    )
    go_repository(
        name = "com_github_data_dog_go_sqlmock",
        importpath = "github.com/DATA-DOG/go-sqlmock",
        sum = "h1:OcvFkGmslmlZibjAjaHm3L//6LiuBgolP7OputlJIzU=",
        version = "v1.5.2",
    )
    go_repository(
        name = "com_github_davecgh_go_spew",
        build_file_generation = "on",
        importpath = "github.com/davecgh/go-spew",
        sum = "h1:vj9j/u1bqnvCEfJOwUhtlOARqs3+rkHYY13jYWTU97c=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_deckarep_golang_set_v2",
        build_file_generation = "on",
        importpath = "github.com/deckarep/golang-set/v2",
        sum = "h1:swm0rlPCmdWn9mESxKOjWk8hXSqoxOp+ZlfuyaAdFlQ=",
        version = "v2.8.0",
    )
    go_repository(
        name = "com_github_distribution_reference",
        build_file_generation = "on",
        importpath = "github.com/distribution/reference",
        sum = "h1:0IXCQ5g4/QMHHkarYzh5l+u8T3t73zM5QvfrDyIgxBk=",
        version = "v0.6.0",
    )
    go_repository(
        name = "com_github_docker_go_connections",
        build_file_generation = "on",
        importpath = "github.com/docker/go-connections",
        sum = "h1:LlMG9azAe1TqfR7sO+NJttz1gy6KO7VJBh+pMmjSD94=",
        version = "v0.6.0",
    )
    go_repository(
        name = "com_github_docker_go_units",
        build_file_generation = "on",
        importpath = "github.com/docker/go-units",
        sum = "h1:69rxXcBk27SvSaaxTtLh/8llcHD8vYHT7WSdRZ/jvr4=",
        version = "v0.5.0",
    )
    go_repository(
        name = "com_github_dolthub_maphash",
        build_file_generation = "on",
        importpath = "github.com/dolthub/maphash",
        sum = "h1:bsQ7JsF4FkkWyrP3oCnFJgrCUAFbFf3kOl4L/QxPDyQ=",
        version = "v0.1.0",
    )
    go_repository(
        name = "com_github_dustin_go_humanize",
        build_file_generation = "on",
        importpath = "github.com/dustin/go-humanize",
        sum = "h1:GzkhY7T5VNhEkwH0PVJgjz+fX1rhBrR7pRT3mDkpeCY=",
        version = "v1.0.1",
    )
    go_repository(
        name = "com_github_elastic_go_sysinfo",
        build_file_generation = "on",
        importpath = "github.com/elastic/go-sysinfo",
        sum = "h1:A3zQcunCxik14MgXu39cXFXcIw2sFXZ0zL886eyiv1Q=",
        version = "v1.15.4",
    )
    go_repository(
        name = "com_github_elastic_go_windows",
        build_file_generation = "on",
        importpath = "github.com/elastic/go-windows",
        sum = "h1:yoLLsAsV5cfg9FLhZ9EXZ2n2sQFKeDYrHenkcivY4vI=",
        version = "v1.0.2",
    )

    go_repository(
        name = "com_github_eliben_go_sentencepiece",
        importpath = "github.com/eliben/go-sentencepiece",
        sum = "h1:wbnefMCxYyVYmeTVtiMJet+mS9CVwq5klveLpfQLsnk=",
        version = "v0.6.0",
    )
    go_repository(
        name = "com_github_envoyproxy_go_control_plane",
        build_file_generation = "on",
        importpath = "github.com/envoyproxy/go-control-plane",
        sum = "h1:hbG2kr4RuFj222B6+7T83thSPqLjwBIfQawTkC++2HA=",
        version = "v0.14.0",
    )
    go_repository(
        name = "com_github_envoyproxy_go_control_plane_envoy",
        build_file_generation = "on",
        importpath = "github.com/envoyproxy/go-control-plane/envoy",
        sum = "h1:yg/JjO5E7ubRyKX3m07GF3reDNEnfOboJ0QySbH736g=",
        version = "v1.36.0",
    )
    go_repository(
        name = "com_github_envoyproxy_go_control_plane_ratelimit",
        build_file_generation = "on",
        importpath = "github.com/envoyproxy/go-control-plane/ratelimit",
        sum = "h1:/G9QYbddjL25KvtKTv3an9lx6VBE2cnb8wp1vEGNYGI=",
        version = "v0.1.0",
    )
    go_repository(
        name = "com_github_envoyproxy_protoc_gen_validate",
        build_file_generation = "on",
        importpath = "github.com/envoyproxy/protoc-gen-validate",
        sum = "h1:TvGH1wof4H33rezVKWSpqKz5NXWg5VPuZ0uONDT6eb4=",
        version = "v1.3.0",
    )
    go_repository(
        name = "com_github_ericlagergren_decimal",
        build_file_generation = "on",
        importpath = "github.com/ericlagergren/decimal",
        sum = "h1:R/ZjJpjQKsZ6L/+Gf9WHbt31GG8NMVcpRqUE+1mMIyo=",
        version = "v0.0.0-20240411145413-00de7ca16731",
    )
    go_repository(
        name = "com_github_felixge_httpsnoop",
        build_file_generation = "on",
        importpath = "github.com/felixge/httpsnoop",
        sum = "h1:NFTV2Zj1bL4mc9sqWACXbQFVBBg2W3GPvqp8/ESS2Wg=",
        version = "v1.0.4",
    )
    go_repository(
        name = "com_github_frankban_quicktest",
        build_file_generation = "on",
        importpath = "github.com/frankban/quicktest",
        sum = "h1:7Xjx+VpznH+oBnejlPUj8oUpdxnVs4f8XU8WnHkI4W8=",
        version = "v1.14.6",
    )
    go_repository(
        name = "com_github_fsnotify_fsnotify",
        build_file_generation = "on",
        importpath = "github.com/fsnotify/fsnotify",
        sum = "h1:dAwr6QBTBZIkG8roQaJjGof0pp0EeF+tNV7YBP3F/8M=",
        version = "v1.8.0",
    )
    go_repository(
        name = "com_github_fzambia_eagle",
        build_file_generation = "on",
        importpath = "github.com/FZambia/eagle",
        sum = "h1:1kQaZpJvbkvAXFRE/9K2ucBMuVqo+E29EMLYB74hIis=",
        version = "v0.2.0",
    )
    go_repository(
        name = "com_github_gammazero_deque",
        build_file_generation = "on",
        importpath = "github.com/gammazero/deque",
        sum = "h1:qSdsbG6pgp6nL7A0+K/B7s12mcCY/5l5SIUpMOl+dC0=",
        version = "v0.2.1",
    )
    go_repository(
        name = "com_github_glebarez_go_sqlite",
        importpath = "github.com/glebarez/go-sqlite",
        sum = "h1:7MZyUPh2XTrHS7xNEHQbrhfMZuPSzhkm2A1qgg0y5NY=",
        version = "v1.21.1",
    )
    go_repository(
        name = "com_github_glebarez_sqlite",
        importpath = "github.com/glebarez/sqlite",
        sum = "h1:02X12E2I/4C1n+v90yTqrjRa8yuo7c3KeHI3FRznCvc=",
        version = "v1.8.0",
    )
    go_repository(
        name = "com_github_go_faster_city",
        build_file_generation = "on",
        importpath = "github.com/go-faster/city",
        sum = "h1:4WAxSZ3V2Ws4QRDrscLEDcibJY8uf41H6AhXDrNDcGw=",
        version = "v1.0.1",
    )
    go_repository(
        name = "com_github_go_faster_errors",
        build_file_generation = "on",
        importpath = "github.com/go-faster/errors",
        sum = "h1:MkJTnDoEdi9pDabt1dpWf7AA8/BaSYZqibYyhZ20AYg=",
        version = "v0.7.1",
    )
    go_repository(
        name = "com_github_go_ini_ini",
        build_file_generation = "on",
        importpath = "github.com/go-ini/ini",
        sum = "h1:z6ZrTEZqSWOTyH2FlglNbNgARyHG8oLW9gMELqKr06A=",
        version = "v1.67.0",
    )
    go_repository(
        name = "com_github_go_jose_go_jose_v3",
        build_file_generation = "on",
        importpath = "github.com/go-jose/go-jose/v3",
        sum = "h1:Wp5HA7bLQcKnf6YYao/4kpRpVMp/yf6+pJKV8WFSaNY=",
        version = "v3.0.4",
    )
    go_repository(
        name = "com_github_go_jose_go_jose_v4",
        build_file_generation = "on",
        importpath = "github.com/go-jose/go-jose/v4",
        sum = "h1:CVLmWDhDVRa6Mi/IgCgaopNosCaHz7zrMeF9MlZRkrs=",
        version = "v4.1.3",
    )
    go_repository(
        name = "com_github_go_logr_logr",
        build_file_generation = "on",
        importpath = "github.com/go-logr/logr",
        sum = "h1:2y3SDp0ZXuc6/cjLSZ+Q3ir+QB9T/iG5yYRXqsagWSY=",
        version = "v1.3.0",
    )
    go_repository(
        name = "com_github_go_logr_stdr",
        build_file_generation = "on",
        importpath = "github.com/go-logr/stdr",
        sum = "h1:hSWxHoqTgW2S2qGc0LTAI563KZ5YKYRhT3MFKZMbjag=",
        version = "v1.2.2",
    )
    go_repository(
        name = "com_github_go_sql_driver_mysql",
        build_file_generation = "on",
        importpath = "github.com/go-sql-driver/mysql",
        sum = "h1:U/N249h2WzJ3Ukj8SowVFjdtZKfu9vlLZxjPXV1aweo=",
        version = "v1.9.3",
    )
    go_repository(
        name = "com_github_go_stack_stack",
        build_file_generation = "on",
        importpath = "github.com/go-stack/stack",
        sum = "h1:ntEHSVwIt7PNXNpgPmVfMrNhLtgjlmnZha2kOpuRiDw=",
        version = "v1.8.1",
    )
    go_repository(
        name = "com_github_go_test_deep",
        build_file_generation = "on",
        importpath = "github.com/go-test/deep",
        sum = "h1:0r/53hagsehfO4bzD2Pgr/+RgHqhmf+k1Bpse2cTu1U=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_go_viper_mapstructure_v2",
        build_file_generation = "on",
        importpath = "github.com/go-viper/mapstructure/v2",
        sum = "h1:ZAaOCxANMuZx5RCeg0mBdEZk7DZasvvZIxtHqx8aGss=",
        version = "v2.2.1",
    )
    go_repository(
        name = "com_github_golang_glog",
        build_file_generation = "on",
        importpath = "github.com/golang/glog",
        sum = "h1:DrW6hGnjIhtvhOIiAKT6Psh/Kd/ldepEa81DKeiRJ5I=",
        version = "v1.2.5",
    )
    go_repository(
        name = "com_github_golang_groupcache",
        importpath = "github.com/golang/groupcache",
        sum = "h1:oI5xCqsCo564l8iNU+DwB5epxmsaqB+rhGL0m5jtYqE=",
        version = "v0.0.0-20210331224755-41bb18bfe9da",
    )
    go_repository(
        name = "com_github_golang_jwt_jwt_v4",
        build_file_generation = "on",
        importpath = "github.com/golang-jwt/jwt/v4",
        sum = "h1:YtQM7lnr8iZ+j5q71MGKkNw9Mn7AjHM68uc9g5fXeUI=",
        version = "v4.5.2",
    )
    go_repository(
        name = "com_github_golang_jwt_jwt_v5",
        build_file_generation = "on",
        importpath = "github.com/golang-jwt/jwt/v5",
        sum = "h1:kYf81DTWFe7t+1VvL7eS+jKFVWaUnK9cB1qbwn63YCY=",
        version = "v5.3.1",
    )
    go_repository(
        name = "com_github_golang_mock",
        importpath = "github.com/golang/mock",
        sum = "h1:G5FRp8JnTd7RQH5kemVNlMeyXQAztQ3mOWV95KxsXH8=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_golang_protobuf",
        build_file_generation = "on",
        importpath = "github.com/golang/protobuf",
        sum = "h1:i7eJL8qZTpSEXOPTxNKhASYpMn+8e5Q6AdndVa1dWek=",
        version = "v1.5.4",
    )
    go_repository(
        name = "com_github_golang_snappy",
        importpath = "github.com/golang/snappy",
        sum = "h1:yAGX7huGHXlcLOEtBnF4w7FQwA26wojNCwOYAEhLjQM=",
        version = "v0.0.4",
    )
    go_repository(
        name = "com_github_golang_sql_civil",
        build_file_generation = "on",
        importpath = "github.com/golang-sql/civil",
        sum = "h1:au07oEsX2xN0ktxqI+Sida1w446QrXBRJ0nee3SNZlA=",
        version = "v0.0.0-20220223132316-b832511892a9",
    )
    go_repository(
        name = "com_github_golang_sql_sqlexp",
        build_file_generation = "on",
        importpath = "github.com/golang-sql/sqlexp",
        sum = "h1:ZCD6MBpcuOVfGVqsEmY5/4FtYiKz6tSyUv9LPEDei6A=",
        version = "v0.1.0",
    )
    go_repository(
        name = "com_github_google_cel_go",
        build_file_generation = "on",
        importpath = "github.com/google/cel-go",
        sum = "h1:iPbVVEdkhTX++hpe3lzSk7D3G3QSYqLGoHOcEio+UXQ=",
        version = "v0.26.1",
    )
    go_repository(
        name = "com_github_google_go_cmp",
        build_file_generation = "on",
        importpath = "github.com/google/go-cmp",
        sum = "h1:ofyhxvXcZhMsU5ulbFiLKl/XBFqE1GSq7atu8tAmTRI=",
        version = "v0.6.0",
    )
    go_repository(
        name = "com_github_google_go_pkcs11",
        importpath = "github.com/google/go-pkcs11",
        sum = "h1:PVRnTgtArZ3QQqTGtbtjtnIkzl2iY2kt24yqbrf7td8=",
        version = "v0.3.0",
    )
    go_repository(
        name = "com_github_google_jsonschema_go",
        build_file_generation = "on",
        importpath = "github.com/google/jsonschema-go",
        sum = "h1:tmrUohrwoLZZS/P3x7ex0WAVknEkBZM46iALbcqoRA8=",
        version = "v0.4.2",
    )
    go_repository(
        name = "com_github_google_martian_v3",
        importpath = "github.com/google/martian/v3",
        sum = "h1:DIhPTQrbPkgs2yJYdXU/eNACCG5DVQjySNRNlflZ9Fc=",
        version = "v3.3.3",
    )
    go_repository(
        name = "com_github_google_pprof",
        build_file_generation = "on",
        importpath = "github.com/google/pprof",
        sum = "h1:ijClszYn+mADRFY17kjQEVQ1XRhq2/JR1M3sGqeJoxs=",
        version = "v0.0.0-20250317173921-a4b03ec1a45e",
    )
    go_repository(
        name = "com_github_google_s2a_go",
        importpath = "github.com/google/s2a-go",
        sum = "h1:LGD7gtMgezd8a/Xak7mEWL0PjoTQFvpRudN895yqKW0=",
        version = "v0.1.9",
    )
    go_repository(
        name = "com_github_google_safehtml",
        importpath = "github.com/google/safehtml",
        sum = "h1:EwLKo8qawTKfsi0orxcQAZzu07cICaBeFMegAU9eaT8=",
        version = "v0.1.0",
    )
    go_repository(
        name = "com_github_google_uuid",
        build_file_generation = "on",
        importpath = "github.com/google/uuid",
        sum = "h1:NIvaJDMOsjHA8n1jAhLSgzrAzy1Hgr+hNrb57e+94F0=",
        version = "v1.6.0",
    )
    go_repository(
        name = "com_github_googleapis_enterprise_certificate_proxy",
        importpath = "github.com/googleapis/enterprise-certificate-proxy",
        sum = "h1:GW/XbdyBFQ8Qe+YAmFU9uHLo7OnF5tL52HFAgMmyrf4=",
        version = "v0.3.6",
    )
    go_repository(
        name = "com_github_googleapis_gax_go_v2",
        importpath = "github.com/googleapis/gax-go/v2",
        sum = "h1:SyjDc1mGgZU5LncH8gimWo9lW1DtIfPibOG81vgd/bo=",
        version = "v2.15.0",
    )
    go_repository(
        name = "com_github_googlecloudplatform_opentelemetry_operations_go_detectors_gcp",
        build_file_generation = "on",
        importpath = "github.com/GoogleCloudPlatform/opentelemetry-operations-go/detectors/gcp",
        sum = "h1:sBEjpZlNHzK1voKq9695PJSX2o5NEXl7/OL3coiIY0c=",
        version = "v1.30.0",
    )
    go_repository(
        name = "com_github_googlecloudplatform_opentelemetry_operations_go_exporter_metric",
        importpath = "github.com/GoogleCloudPlatform/opentelemetry-operations-go/exporter/metric",
        sum = "h1:owcC2UnmsZycprQ5RfRgjydWhuoxg71LUfyiQdijZuM=",
        version = "v0.53.0",
    )
    go_repository(
        name = "com_github_googlecloudplatform_opentelemetry_operations_go_internal_resourcemapping",
        importpath = "github.com/GoogleCloudPlatform/opentelemetry-operations-go/internal/resourcemapping",
        sum = "h1:Ron4zCA/yk6U7WOBXhTJcDpsUBG9npumK6xw2auFltQ=",
        version = "v0.53.0",
    )
    go_repository(
        name = "com_github_gorilla_mux",
        importpath = "github.com/gorilla/mux",
        sum = "h1:TuBL49tXwgrFYWhqrNgrUNEY92u81SPhu7sTdzQEiWY=",
        version = "v1.8.1",
    )
    go_repository(
        name = "com_github_grpc_ecosystem_grpc_gateway_v2",
        importpath = "github.com/grpc-ecosystem/grpc-gateway/v2",
        sum = "h1:X+2YciYSxvMQK0UZ7sg45ZVabVZBeBuvMkmuI2V3Fak=",
        version = "v2.27.7",
    )
    go_repository(
        name = "com_github_h2non_filetype",
        build_file_generation = "on",
        importpath = "github.com/h2non/filetype",
        sum = "h1:FKkx9QbD7HR/zjK1Ia5XiBsq9zdLi5Kf3zGyFTAFkGg=",
        version = "v1.1.3",
    )
    go_repository(
        name = "com_github_hashicorp_golang_lru_v2",
        build_file_generation = "on",
        importpath = "github.com/hashicorp/golang-lru/v2",
        sum = "h1:a+bsQ5rvGLjzHuww6tVxozPZFVghXaHOwFs4luLUK2k=",
        version = "v2.0.7",
    )
    go_repository(
        name = "com_github_inconshreveable_mousetrap",
        importpath = "github.com/inconshreveable/mousetrap",
        sum = "h1:wN+x4NVGpMsO7ErUn/mUI3vEoE6Jt13X2s0bqwp9tc8=",
        version = "v1.1.0",
    )
    go_repository(
        name = "com_github_jinzhu_inflection",
        importpath = "github.com/jinzhu/inflection",
        sum = "h1:K317FqzuhWc8YvSVlFMCCUb36O/S9MCKRDI7QkRKD/E=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_jinzhu_now",
        importpath = "github.com/jinzhu/now",
        sum = "h1:/o9tlHleP7gOFmsnYNz3RGnqzefHA47wQpKrrdTIwXQ=",
        version = "v1.1.5",
    )
    go_repository(
        name = "com_github_joho_godotenv",
        build_file_generation = "on",
        importpath = "github.com/joho/godotenv",
        sum = "h1:7eLL/+HRGLY0ldzfGMeQkb7vMd0as4CfYvUVzLqw0N0=",
        version = "v1.5.1",
    )
    go_repository(
        name = "com_github_jonboulle_clockwork",
        build_file_generation = "on",
        importpath = "github.com/jonboulle/clockwork",
        sum = "h1:Hyh9A8u51kptdkR+cqRpT1EebBwTn1oK9YfGYbdFz6I=",
        version = "v0.5.0",
    )
    go_repository(
        name = "com_github_josharian_intern",
        build_file_generation = "on",
        importpath = "github.com/josharian/intern",
        sum = "h1:vlS4z54oSdjm0bgjRigI+G1HpF+tI+9rE5LLzOg8HmY=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_jpillora_backoff",
        build_file_generation = "on",
        importpath = "github.com/jpillora/backoff",
        sum = "h1:uvFg412JmmHBHw7iwprIxkPMI+sGQ4kzOWsMeHnm2EA=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_json_iterator_go",
        build_file_generation = "on",
        importpath = "github.com/json-iterator/go",
        sum = "h1:PV8peI4a0ysnczrg+LtxykD8LfKY9ML6u2jnxaEnrnM=",
        version = "v1.1.12",
    )
    go_repository(
        name = "com_github_julienschmidt_httprouter",
        build_file_generation = "on",
        importpath = "github.com/julienschmidt/httprouter",
        sum = "h1:U0609e9tgbseu3rBINet9P48AI/D3oJs4dN7jwJOQ1U=",
        version = "v1.3.0",
    )
    go_repository(
        name = "com_github_kisielk_sqlstruct",
        importpath = "github.com/kisielk/sqlstruct",
        sum = "h1:veS9QfglfvqAw2e+eeNT/SbGySq8ajECXJ9e4fPoLhY=",
        version = "v0.0.0-20201105191214-5f3e10d3ab46",
    )

    go_repository(
        name = "com_github_klauspost_compress",
        build_file_generation = "on",
        importpath = "github.com/klauspost/compress",
        sum = "h1:/h1gH5Ce+VWNLSWqPzOVn6XBO+vJbCNGvjoaGBFW2IE=",
        version = "v1.18.5",
    )
    go_repository(
        name = "com_github_klauspost_crc32",
        build_file_generation = "on",
        importpath = "github.com/klauspost/crc32",
        sum = "h1:sSmTt3gUt81RP655XGZPElI0PelVTZ6YwCRnPSupoFM=",
        version = "v1.3.0",
    )
    go_repository(
        name = "com_github_kr_pretty",
        build_file_generation = "on",
        importpath = "github.com/kr/pretty",
        sum = "h1:flRD4NNwYAUpkphVc1HcthR4KEIFJ65n8Mw5qdRn3LE=",
        version = "v0.3.1",
    )
    go_repository(
        name = "com_github_kr_text",
        build_file_generation = "on",
        importpath = "github.com/kr/text",
        sum = "h1:5Nx0Ya0ZqY2ygV366QzturHI13Jq95ApcVaJBhpS+AY=",
        version = "v0.2.0",
    )
    go_repository(
        name = "com_github_kylelemons_godebug",
        build_file_generation = "on",
        importpath = "github.com/kylelemons/godebug",
        sum = "h1:RPNrshWIDI6G2gRW9EHilWtl7Z6Sb1BR0xunSBf0SNc=",
        version = "v1.1.0",
    )
    go_repository(
        name = "com_github_mailru_easyjson",
        build_file_generation = "on",
        importpath = "github.com/mailru/easyjson",
        sum = "h1:UGYAvKxe3sBsEDzO8ZeWOSlIQfWFlxbzLZe7hwFURr0=",
        version = "v0.7.7",
    )
    go_repository(
        name = "com_github_mattn_go_isatty",
        build_file_generation = "on",
        importpath = "github.com/mattn/go-isatty",
        sum = "h1:xfD0iDuEKnDkl03q4limB+vH+GxLEtL/jb4xVJSWWEY=",
        version = "v0.0.20",
    )
    go_repository(
        name = "com_github_mattn_go_sqlite3",
        build_file_generation = "on",
        importpath = "github.com/mattn/go-sqlite3",
        sum = "h1:3VSe+xafpbzsLbdr2AWlAZk9yRHiBhTBakioXaCKTF8=",
        version = "v1.14.44",
    )
    go_repository(
        name = "com_github_maypok86_otter",
        build_file_generation = "on",
        importpath = "github.com/maypok86/otter",
        sum = "h1:HhW1Pq6VdJkmWwcZZq19BlEQkHtI8xgsQzBVXJU0nfc=",
        version = "v1.2.4",
    )
    go_repository(
        name = "com_github_mfridman_interpolate",
        build_file_generation = "on",
        importpath = "github.com/mfridman/interpolate",
        sum = "h1:pnuTK7MQIxxFz1Gr+rjSIx9u7qVjf5VOoM/u6BbAxPY=",
        version = "v0.0.2",
    )
    go_repository(
        name = "com_github_mfridman_xflag",
        build_file_generation = "on",
        importpath = "github.com/mfridman/xflag",
        sum = "h1:TWZrZwG1QklFX5S4j1vxfF1sZbZeZSGofMwPMLAF29M=",
        version = "v0.1.0",
    )
    go_repository(
        name = "com_github_microsoft_go_mssqldb",
        build_file_generation = "on",
        importpath = "github.com/microsoft/go-mssqldb",
        sum = "h1:1MNQg5UiSsokiPz3++K2KPx4moKrwIqly1wv+RyCKTw=",
        version = "v1.9.6",
    )
    go_repository(
        name = "com_github_microsoft_go_winio",
        build_file_generation = "on",
        importpath = "github.com/Microsoft/go-winio",
        sum = "h1:F2VQgta7ecxGYO8k3ZZz3RS8fVIXVxONVUPlNERoyfY=",
        version = "v0.6.2",
    )
    go_repository(
        name = "com_github_minio_crc64nvme",
        build_file_generation = "on",
        importpath = "github.com/minio/crc64nvme",
        sum = "h1:8dwx/Pz49suywbO+auHCBpCtlW1OfpcLN7wYgVR6wAI=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_minio_md5_simd",
        build_file_generation = "on",
        importpath = "github.com/minio/md5-simd",
        sum = "h1:Gdi1DZK69+ZVMoNHRXJyNcxrMA4dSxoYHZSQbirFg34=",
        version = "v1.1.2",
    )
    go_repository(
        name = "com_github_minio_minio_go_v7",
        build_file_generation = "on",
        importpath = "github.com/minio/minio-go/v7",
        sum = "h1:ShkWi8Tyj9RtU57OQB2HIXKz4bFgtVib0bbT1sbtLI8=",
        version = "v7.0.100",
    )
    go_repository(
        name = "com_github_mitchellh_mapstructure",
        importpath = "github.com/mitchellh/mapstructure",
        sum = "h1:jeMsZIYE/09sWLaz43PL7Gy6RuMjD2eJVyuac5Z2hdY=",
        version = "v1.5.0",
    )
    go_repository(
        name = "com_github_mitchellh_protoc_gen_go_json",
        importpath = "github.com/mitchellh/protoc-gen-go-json",
        sum = "h1:lEi1xtXyYKDwA8EB5u27+UUZOTznC4JpqVOKZwCGJUo=",
        version = "v1.1.0",
    )
    go_repository(
        name = "com_github_moby_docker_image_spec",
        build_file_generation = "on",
        importpath = "github.com/moby/docker-image-spec",
        sum = "h1:jMKff3w6PgbfSa69GfNg+zN/XLhfXJGnEx3Nl2EsFP0=",
        version = "v1.3.1",
    )
    go_repository(
        name = "com_github_moby_moby_api",
        build_file_generation = "on",
        importpath = "github.com/moby/moby/api",
        sum = "h1:PihqG1ncw4W+8mZs69jlwGXdaYBeb5brF6BL7mPIS/w=",
        version = "v1.53.0",
    )
    go_repository(
        name = "com_github_moby_moby_client",
        build_file_generation = "on",
        importpath = "github.com/moby/moby/client",
        sum = "h1:Pt4hRMCAIlyjL3cr8M5TrXCwKzguebPAc2do2ur7dEM=",
        version = "v0.2.2",
    )
    go_repository(
        name = "com_github_modelcontextprotocol_go_sdk",
        build_file_generation = "on",
        importpath = "github.com/modelcontextprotocol/go-sdk",
        sum = "h1:CHU0FIX9kpueNkxuYtfYQn1Z0slhFzBZuq+x6IiblIU=",
        version = "v1.5.0",
    )
    go_repository(
        name = "com_github_modern_go_concurrent",
        build_file_generation = "on",
        importpath = "github.com/modern-go/concurrent",
        sum = "h1:TRLaZ9cD/w8PVh93nsPXa1VrQ6jlwL5oN8l14QlcNfg=",
        version = "v0.0.0-20180306012644-bacd9c7ef1dd",
    )
    go_repository(
        name = "com_github_modern_go_reflect2",
        build_file_generation = "on",
        importpath = "github.com/modern-go/reflect2",
        sum = "h1:xBagoLtFs94CBntxluKeaWgTMpvLxC4ur3nMaC9Gz0M=",
        version = "v1.0.2",
    )
    go_repository(
        name = "com_github_munnerz_goautoneg",
        build_file_generation = "on",
        importpath = "github.com/munnerz/goautoneg",
        sum = "h1:C3w9PqII01/Oq1c1nUAm88MOHcQC9l5mIlSMApZMrHA=",
        version = "v0.0.0-20191010083416-a7dc8b61c822",
    )
    go_repository(
        name = "com_github_mwitkow_go_conntrack",
        build_file_generation = "on",
        importpath = "github.com/mwitkow/go-conntrack",
        sum = "h1:KUppIJq7/+SVif2QVs3tOP0zanoHgBEVAwHxUSIzRqU=",
        version = "v0.0.0-20190716064945-2f068394615f",
    )
    go_repository(
        name = "com_github_nats_io_nats_go",
        build_file_generation = "on",
        importpath = "github.com/nats-io/nats.go",
        sum = "h1:ByW84XTz6W03GSSsygsZcA+xgKK8vPGaa/FCAAEHnAI=",
        version = "v1.51.0",
    )
    go_repository(
        name = "com_github_nats_io_nkeys",
        build_file_generation = "on",
        importpath = "github.com/nats-io/nkeys",
        sum = "h1:JACV5jRVO9V856KOapQ7x+EY8Jo3qw1vJt/9Jpwzkk4=",
        version = "v0.4.15",
    )
    go_repository(
        name = "com_github_nats_io_nuid",
        build_file_generation = "on",
        importpath = "github.com/nats-io/nuid",
        sum = "h1:5iA8DT8V7q8WK2EScv2padNa/rTESc1KdnPw4TC2paw=",
        version = "v1.0.1",
    )
    go_repository(
        name = "com_github_ncruces_go_strftime",
        build_file_generation = "on",
        importpath = "github.com/ncruces/go-strftime",
        sum = "h1:HMFp8mLCTPp341M/ZnA4qaf7ZlsbTc+miZjCLOFAw7w=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_onsi_gomega",
        build_file_generation = "on",
        importpath = "github.com/onsi/gomega",
        sum = "h1:koNYke6TVk6ZmnyHrCXba/T/MoLBXFjeC1PtvYgw0A8=",
        version = "v1.36.2",
    )
    go_repository(
        name = "com_github_opencontainers_go_digest",
        build_file_generation = "on",
        importpath = "github.com/opencontainers/go-digest",
        sum = "h1:apOUWs51W5PlhuyGyz9FCeeBIOUDA/6nW8Oi/yOhh5U=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_opencontainers_image_spec",
        build_file_generation = "on",
        importpath = "github.com/opencontainers/image-spec",
        sum = "h1:y0fUlFfIZhPF1W537XOLg0/fcx6zcHCJwooC2xJA040=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_orisano_pixelmatch",
        build_file_generation = "on",
        importpath = "github.com/orisano/pixelmatch",
        sum = "h1:J1//5K/6QF10cZ59zLcVNFGmBfiSrH8Cho/lNrViK9s=",
        version = "v0.0.0-20230914042517-fa304d1dc785",
    )
    go_repository(
        name = "com_github_paulmach_orb",
        build_file_generation = "on",
        importpath = "github.com/paulmach/orb",
        sum = "h1:z+zOwjmG3MyEEqzv92UN49Lg1JFYx0L9GpGKNVDKk1s=",
        version = "v0.12.0",
    )
    go_repository(
        name = "com_github_pelletier_go_toml_v2",
        build_file_generation = "on",
        importpath = "github.com/pelletier/go-toml/v2",
        sum = "h1:YmeHyLY8mFWbdkNWwpr+qIL2bEqT0o95WSdkNHvL12M=",
        version = "v2.2.3",
    )
    go_repository(
        name = "com_github_philhofer_fwd",
        build_file_generation = "on",
        importpath = "github.com/philhofer/fwd",
        sum = "h1:e6DnBTl7vGY+Gz322/ASL4Gyp1FspeMvx1RNDoToZuM=",
        version = "v1.2.0",
    )
    go_repository(
        name = "com_github_pierrec_lz4_v4",
        build_file_generation = "on",
        importpath = "github.com/pierrec/lz4/v4",
        sum = "h1:kocOqRffaIbU5djlIBr7Wh+cx82C0vtFb0fOurZHqD0=",
        version = "v4.1.25",
    )
    go_repository(
        name = "com_github_planetscale_vtprotobuf",
        build_file_generation = "on",
        importpath = "github.com/planetscale/vtprotobuf",
        sum = "h1:GFCKgmp0tecUJ0sJuv4pzYCqS9+RGSn52M3FUwPs+uo=",
        version = "v0.6.1-0.20240319094008-0393e58bdf10",
    )
    go_repository(
        name = "com_github_playwright_community_playwright_go",
        build_file_generation = "on",
        importpath = "github.com/playwright-community/playwright-go",
        sum = "h1:PNFb1byWqrTT720rEO0JL88C6Ju0EmUnR5deFLvtP/U=",
        version = "v0.5700.1",
    )
    go_repository(
        name = "com_github_pmezard_go_difflib",
        build_file_generation = "on",
        importpath = "github.com/pmezard/go-difflib",
        sum = "h1:4DBwDE0NGyQoBHbLQYPwSUPoCMWR5BEzIk/f1lZbAQM=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_pressly_goose_v3",
        build_file_generation = "on",
        importpath = "github.com/pressly/goose/v3",
        sum = "h1:/D30gVTuQhu0WsNZYbJi4DMOsx1lNq+6SkLe+Wp59BM=",
        version = "v3.27.0",
    )
    go_repository(
        name = "com_github_prometheus_client_golang",
        build_file_generation = "on",
        importpath = "github.com/prometheus/client_golang",
        sum = "h1:Je96obch5RDVy3FDMndoUsjAhG5Edi49h0RJWRi/o0o=",
        version = "v1.23.2",
    )
    go_repository(
        name = "com_github_prometheus_client_model",
        build_file_generation = "on",
        importpath = "github.com/prometheus/client_model",
        sum = "h1:oBsgwpGs7iVziMvrGhE53c/GrLUsZdHnqNwqPLxwZyk=",
        version = "v0.6.2",
    )
    go_repository(
        name = "com_github_prometheus_common",
        build_file_generation = "on",
        importpath = "github.com/prometheus/common",
        sum = "h1:pIgK94WWlQt1WLwAC5j2ynLaBRDiinoAb86HZHTUGI4=",
        version = "v0.67.5",
    )
    go_repository(
        name = "com_github_prometheus_procfs",
        build_file_generation = "on",
        importpath = "github.com/prometheus/procfs",
        sum = "h1:zUMhqEW66Ex7OXIiDkll3tl9a1ZdilUOd/F6ZXw4Vws=",
        version = "v0.19.2",
    )
    go_repository(
        name = "com_github_quagmt_udecimal",
        build_file_generation = "on",
        importpath = "github.com/quagmt/udecimal",
        sum = "h1:TLuZiFeg0HhS6X8VDa78Y6XTaitZZfh+z5q4SXMzpDQ=",
        version = "v1.9.0",
    )
    go_repository(
        name = "com_github_redis_rueidis",
        build_file_generation = "on",
        importpath = "github.com/redis/rueidis",
        sum = "h1:gept0E45JGxVigWb3zoWHvxEc4IOC7kc4V/4XvN8eG8=",
        version = "v1.0.68",
    )
    go_repository(
        name = "com_github_remyoudompheng_bigfft",
        build_file_generation = "on",
        importpath = "github.com/remyoudompheng/bigfft",
        sum = "h1:W09IVJc94icq4NjY3clb7Lk8O1qJ8BdBEF8z0ibU0rE=",
        version = "v0.0.0-20230129092748-24d4a6f8daec",
    )
    go_repository(
        name = "com_github_rogpeppe_go_internal",
        build_file_generation = "on",
        importpath = "github.com/rogpeppe/go-internal",
        sum = "h1:UQB4HGPB6osV0SQTLymcB4TgvyWu6ZyliaW0tI/otEQ=",
        version = "v1.14.1",
    )
    go_repository(
        name = "com_github_rs_xid",
        build_file_generation = "on",
        importpath = "github.com/rs/xid",
        sum = "h1:fV591PaemRlL6JfRxGDEPl69wICngIQ3shQtzfy2gxU=",
        version = "v1.6.0",
    )
    go_repository(
        name = "com_github_sagikazarmark_locafero",
        build_file_generation = "on",
        importpath = "github.com/sagikazarmark/locafero",
        sum = "h1:5MqpDsTGNDhY8sGp0Aowyf0qKsPrhewaLSsFaodPcyo=",
        version = "v0.7.0",
    )
    go_repository(
        name = "com_github_segmentio_asm",
        build_file_generation = "on",
        importpath = "github.com/segmentio/asm",
        sum = "h1:DTNbBqs57ioxAD4PrArqftgypG4/qNpXoJx8TVXxPR0=",
        version = "v1.2.1",
    )
    go_repository(
        name = "com_github_segmentio_encoding",
        build_file_generation = "on",
        importpath = "github.com/segmentio/encoding",
        sum = "h1:OW1VRern8Nw6ITAtwSZ7Idrl3MXCFwXHPgqESYfvNt0=",
        version = "v0.5.4",
    )
    go_repository(
        name = "com_github_sethvargo_go_retry",
        build_file_generation = "on",
        importpath = "github.com/sethvargo/go-retry",
        sum = "h1:EEt31A35QhrcRZtrYFDTBg91cqZVnFL2navjDrah2SE=",
        version = "v0.3.0",
    )
    go_repository(
        name = "com_github_shadowspore_fossil_delta",
        build_file_generation = "on",
        importpath = "github.com/shadowspore/fossil-delta",
        sum = "h1:/4/IJi5iyTdh6mqOUaASW148HQpujYiHl0Wl78dSOSc=",
        version = "v0.0.0-20241213113458-1d797d70cbe3",
    )
    go_repository(
        name = "com_github_shopspring_decimal",
        build_file_generation = "on",
        importpath = "github.com/shopspring/decimal",
        sum = "h1:bxl37RwXBklmTi0C79JfXCEBD1cqqHt0bbgBAGFp81k=",
        version = "v1.4.0",
    )
    go_repository(
        name = "com_github_slack_go_slack",
        build_file_generation = "on",
        importpath = "github.com/slack-go/slack",
        sum = "h1:jaUTiGoyhCl7xC/PuVh05BfM1ifVBsQQUKgsr5TLg5k=",
        version = "v0.22.0",
    )
    go_repository(
        name = "com_github_smacker_go_tree_sitter",
        build_file_generation = "on",
        importpath = "github.com/smacker/go-tree-sitter",
        sum = "h1:6C8qej6f1bStuePVkLSFxoU22XBS165D3klxlzRg8F4=",
        version = "v0.0.0-20240827094217-dd81d9e9be82",
    )
    go_repository(
        name = "com_github_sourcegraph_conc",
        build_file_generation = "on",
        importpath = "github.com/sourcegraph/conc",
        sum = "h1:OQTbbt6P72L20UqAkXXuLOj79LfEanQ+YQFNpLA9ySo=",
        version = "v0.3.0",
    )
    go_repository(
        name = "com_github_spf13_afero",
        build_file_generation = "on",
        importpath = "github.com/spf13/afero",
        sum = "h1:9tH6MapGnn/j0eb0yIXiLjERO8RB6xIVZRDCX7PtqWA=",
        version = "v1.14.0",
    )
    go_repository(
        name = "com_github_spf13_cast",
        build_file_generation = "on",
        importpath = "github.com/spf13/cast",
        sum = "h1:cuNEagBQEHWN1FnbGEjCXL2szYEXqfJPbP2HNUaca9Y=",
        version = "v1.7.1",
    )
    go_repository(
        name = "com_github_spf13_cobra",
        importpath = "github.com/spf13/cobra",
        sum = "h1:e5/vxKd/rZsfSJMUX1agtjeTDf+qv1/JdBF8gg5k9ZM=",
        version = "v1.8.1",
    )
    go_repository(
        name = "com_github_spf13_pflag",
        build_file_generation = "on",
        importpath = "github.com/spf13/pflag",
        sum = "h1:4EBh2KAYBwaONj6b2Ye1GiHfwjqyROoF4RwYO+vPwFk=",
        version = "v1.0.10",
    )
    go_repository(
        name = "com_github_spf13_viper",
        build_file_generation = "on",
        importpath = "github.com/spf13/viper",
        sum = "h1:ZMi+z/lvLyPSCoNtFCpqjy0S4kPbirhpTMwl8BkW9X4=",
        version = "v1.20.1",
    )
    go_repository(
        name = "com_github_spiffe_go_spiffe_v2",
        build_file_generation = "on",
        importpath = "github.com/spiffe/go-spiffe/v2",
        sum = "h1:l+DolpxNWYgruGQVV0xsfeya3CsC7m8iBzDnMpsbLuo=",
        version = "v2.6.0",
    )
    go_repository(
        name = "com_github_stoewer_go_strcase",
        build_file_generation = "on",
        importpath = "github.com/stoewer/go-strcase",
        sum = "h1:Z2iHWqGXH00XYgqDmNgQbIBxf3wrNq0F3feEy0ainaU=",
        version = "v1.2.0",
    )
    go_repository(
        name = "com_github_stretchr_testify",
        build_file_generation = "on",
        importpath = "github.com/stretchr/testify",
        sum = "h1:7s2iGBzp5EwR7/aIZr8ao5+dra3wiQyKjjFuvgVKu7U=",
        version = "v1.11.1",
    )
    go_repository(
        name = "com_github_subosito_gotenv",
        build_file_generation = "on",
        importpath = "github.com/subosito/gotenv",
        sum = "h1:9NlTDc1FTs4qu0DDq7AEtTPNw6SVm7uBMsUCUjABIf8=",
        version = "v1.6.0",
    )
    go_repository(
        name = "com_github_tidwall_gjson",
        build_file_generation = "on",
        importpath = "github.com/tidwall/gjson",
        sum = "h1:/Jocvlh98kcTfpN2+JzGQWQcqrPQwDrVEMApx/M5ZwM=",
        version = "v1.17.0",
    )
    go_repository(
        name = "com_github_tidwall_match",
        build_file_generation = "on",
        importpath = "github.com/tidwall/match",
        sum = "h1:+Ho715JplO36QYgwN9PGYNhgZvoUSc9X2c80KVTi+GA=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_tidwall_pretty",
        build_file_generation = "on",
        importpath = "github.com/tidwall/pretty",
        sum = "h1:qjsOFOWWQl+N3RsoF5/ssm1pHmJJwhjlSbZ51I6wMl4=",
        version = "v1.2.1",
    )
    go_repository(
        name = "com_github_tinylib_msgp",
        build_file_generation = "on",
        importpath = "github.com/tinylib/msgp",
        sum = "h1:ESRv8eL3u+DNHUoSAAQRE50Hm162zqAnBoGv9PzScPY=",
        version = "v1.6.1",
    )
    go_repository(
        name = "com_github_tursodatabase_libsql_client_go",
        build_file_generation = "on",
        importpath = "github.com/tursodatabase/libsql-client-go",
        sum = "h1:lzi/5fg2EfinRlh3v//YyIhnc4tY7BTqazQGwb1ar+0=",
        version = "v0.0.0-20251219100830-236aa1ff8acc",
    )
    go_repository(
        name = "com_github_valyala_bytebufferpool",
        build_file_generation = "on",
        importpath = "github.com/valyala/bytebufferpool",
        sum = "h1:GqA5TC/0021Y/b9FG4Oi9Mr3q7XYx6KllzawFIhcdPw=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_vertica_vertica_sql_go",
        build_file_generation = "on",
        importpath = "github.com/vertica/vertica-sql-go",
        sum = "h1:IrfH2WIgzZ45yDHyjVFrXU2LuKNIjF5Nwi90a6cfgUI=",
        version = "v1.3.5",
    )
    go_repository(
        name = "com_github_xhit_go_str2duration_v2",
        build_file_generation = "on",
        importpath = "github.com/xhit/go-str2duration/v2",
        sum = "h1:lxklc02Drh6ynqX+DdPyp5pCKLUQpRT8bp8Ydu2Bstc=",
        version = "v2.1.0",
    )
    go_repository(
        name = "com_github_ydb_platform_ydb_go_genproto",
        build_file_generation = "on",
        importpath = "github.com/ydb-platform/ydb-go-genproto",
        sum = "h1:kUXMT/fM/DpDT66WQgRUf3I8VOAWjypkMf52W5PChwA=",
        version = "v0.0.0-20260128080146-c4ed16b24b37",
    )
    go_repository(
        name = "com_github_ydb_platform_ydb_go_sdk_v3",
        build_file_generation = "on",
        importpath = "github.com/ydb-platform/ydb-go-sdk/v3",
        sum = "h1:OfHS9ZkZgCy6y/CJ9N8123DXrgaY2BPxWsQiQ8e3wC8=",
        version = "v3.127.0",
    )
    go_repository(
        name = "com_github_yosida95_uritemplate_v3",
        build_file_generation = "on",
        importpath = "github.com/yosida95/uritemplate/v3",
        sum = "h1:Ed3Oyj9yrmi9087+NczuL5BwkIc4wvTb5zIM+UJPGz4=",
        version = "v3.0.2",
    )
    go_repository(
        name = "com_github_yuin_goldmark",
        build_file_generation = "on",
        importpath = "github.com/yuin/goldmark",
        sum = "h1:fVcFKWvrslecOb/tg+Cc05dkeYx540o0FuFt3nUVDoE=",
        version = "v1.4.13",
    )
    go_repository(
        name = "com_github_yuin_gopher_lua",
        build_file_generation = "on",
        importpath = "github.com/yuin/gopher-lua",
        sum = "h1:kYKnWBjvbNP4XLT3+bPEwAXJx262OhaHDWDVOPjL46M=",
        version = "v1.1.1",
    )
    go_repository(
        name = "com_github_zeebo_errs",
        importpath = "github.com/zeebo/errs",
        sum = "h1:XNdoD/RRMKP7HD0UhJnIzUy74ISdGGxURlYG8HSWSfM=",
        version = "v1.4.0",
    )
    go_repository(
        name = "com_github_ziutek_mymysql",
        build_file_generation = "on",
        importpath = "github.com/ziutek/mymysql",
        sum = "h1:GB0qdRGsTwQSBVYuVShFBKaXSnSnYYC2d9knnE1LHFs=",
        version = "v1.5.4",
    )

    go_repository(
        name = "com_google_cloud_go",
        importpath = "cloud.google.com/go",
        sum = "h1:2NAUJwPR47q+E35uaJeYoNhuNEM9kM8SjgRgdeOJUSE=",
        version = "v0.123.0",
    )
    go_repository(
        name = "com_google_cloud_go_aiplatform",
        importpath = "cloud.google.com/go/aiplatform",
        sum = "h1:Tbc2iEp7vbzgk6Vs4QexfNo8/nl+E+Na+FEreRZdhcM=",
        version = "v1.105.0",
    )
    go_repository(
        name = "com_google_cloud_go_auth",
        importpath = "cloud.google.com/go/auth",
        sum = "h1:74yCm7hCj2rUyyAocqnFzsAYXgJhrG26XCFimrc/Kz4=",
        version = "v0.17.0",
    )
    go_repository(
        name = "com_google_cloud_go_auth_oauth2adapt",
        importpath = "cloud.google.com/go/auth/oauth2adapt",
        sum = "h1:keo8NaayQZ6wimpNSmW5OPc283g65QNIiLpZnkHRbnc=",
        version = "v0.2.8",
    )
    go_repository(
        name = "com_google_cloud_go_compute_metadata",
        build_file_generation = "on",
        importpath = "cloud.google.com/go/compute/metadata",
        sum = "h1:pDUj4QMoPejqq20dK0Pg2N4yG9zIkYGdBtwLoEkH9Zs=",
        version = "v0.9.0",
    )
    go_repository(
        name = "com_google_cloud_go_iam",
        importpath = "cloud.google.com/go/iam",
        sum = "h1:+vMINPiDF2ognBJ97ABAYYwRgsaqxPbQDlMnbHMjolc=",
        version = "v1.5.3",
    )
    go_repository(
        name = "com_google_cloud_go_longrunning",
        importpath = "cloud.google.com/go/longrunning",
        sum = "h1:FV0+SYF1RIj59gyoWDRi45GiYUMM3K1qO51qoboQT1E=",
        version = "v0.7.0",
    )
    go_repository(
        name = "com_google_cloud_go_monitoring",
        importpath = "cloud.google.com/go/monitoring",
        sum = "h1:dde+gMNc0UhPZD1Azu6at2e79bfdztVDS5lvhOdsgaE=",
        version = "v1.24.3",
    )
    go_repository(
        name = "com_google_cloud_go_storage",
        importpath = "cloud.google.com/go/storage",
        sum = "h1:n6gy+yLnHn0hTwBFzNn8zJ1kqWfR91wzdM8hjRF4wP0=",
        version = "v1.56.1",
    )
    go_repository(
        name = "com_google_cloud_go_translate",
        importpath = "cloud.google.com/go/translate",
        sum = "h1:g+B29z4gtRGsiKDoTF+bNeH25bLRokAaElygX2FcZkE=",
        version = "v1.10.3",
    )
    go_repository(
        name = "dev_cel_expr",
        build_file_generation = "on",
        importpath = "cel.dev/expr",
        sum = "h1:1KrZg61W6TWSxuNZ37Xy49ps13NUovb66QLprthtwi4=",
        version = "v0.25.1",
    )
    go_repository(
        name = "in_gopkg_check_v1",
        build_file_generation = "on",
        importpath = "gopkg.in/check.v1",
        sum = "h1:yhCVgyC4o1eVCa2tZl7eS0r+SDo693bJlVdllGtEeKM=",
        version = "v0.0.0-20161208181325-20d25e280405",
    )
    go_repository(
        name = "in_gopkg_yaml_v3",
        build_file_generation = "on",
        importpath = "gopkg.in/yaml.v3",
        sum = "h1:fxVm/GzAzEWqLHuvctI91KS9hhNmmWOoWu0XTYJS7CA=",
        version = "v3.0.1",
    )
    go_repository(
        name = "in_yaml_go_yaml_v2",
        build_file_generation = "on",
        importpath = "go.yaml.in/yaml/v2",
        sum = "h1:6gvOSjQoTB3vt1l+CU+tSyi/HOjfOjRLJ4YwYZGwRO0=",
        version = "v2.4.3",
    )
    go_repository(
        name = "in_yaml_go_yaml_v3",
        build_file_generation = "on",
        importpath = "go.yaml.in/yaml/v3",
        sum = "h1:tfq32ie2Jv2UxXFdLJdh3jXuOzWiL1fo0bu/FbuKpbc=",
        version = "v3.0.4",
    )
    go_repository(
        name = "io_filippo_edwards25519",
        build_file_generation = "on",
        importpath = "filippo.io/edwards25519",
        sum = "h1:crnVqOiS4jqYleHd9vaKZ+HKtHfllngJIiOpNpoJsjo=",
        version = "v1.2.0",
    )

    go_repository(
        name = "io_gorm_gorm",
        importpath = "gorm.io/gorm",
        sum = "h1:0VlycGreVhK7RF/Bwt51Fk8v0xLiiiFdbGDPIZQ7mJY=",
        version = "v1.31.0",
    )
    go_repository(
        name = "io_opencensus_go",
        importpath = "go.opencensus.io",
        sum = "h1:y73uSU6J157QMP2kn2r30vwW1A2W2WFwSCGnAVxeaD0=",
        version = "v0.24.0",
    )
    go_repository(
        name = "io_opentelemetry_go_auto_sdk",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/auto/sdk",
        sum = "h1:jXsnJ4Lmnqd11kwkBV2LgLoFMZKizbCi5fNZ/ipaZ64=",
        version = "v1.2.1",
    )
    go_repository(
        name = "io_opentelemetry_go_contrib_detectors_gcp",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/contrib/detectors/gcp",
        sum = "h1:kWRNZMsfBHZ+uHjiH4y7Etn2FK26LAGkNFw7RHv1DhE=",
        version = "v1.39.0",
    )
    go_repository(
        name = "io_opentelemetry_go_contrib_instrumentation_google_golang_org_grpc_otelgrpc",
        importpath = "go.opentelemetry.io/contrib/instrumentation/google.golang.org/grpc/otelgrpc",
        sum = "h1:YH4g8lQroajqUwWbq/tr2QX1JFmEXaDLgG+ew9bLMWo=",
        version = "v0.63.0",
    )
    go_repository(
        name = "io_opentelemetry_go_contrib_instrumentation_net_http_otelhttp",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp",
        sum = "h1:7iP2uCb7sGddAr30RRS6xjKy7AZ2JtTOPA3oolgVSw8=",
        version = "v0.65.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel",
        sum = "h1:vsb/ggIY+hUjD/zCAQHpzTmndPqv/ml2ArbsbfBYTAc=",
        version = "v1.20.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_exporters_otlp_otlplog_otlploghttp",
        importpath = "go.opentelemetry.io/otel/exporters/otlp/otlplog/otlploghttp",
        sum = "h1:djrxvDxAe44mJUrKataUbOhCKhR3F8QCyWucO16hTQs=",
        version = "v0.16.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_exporters_otlp_otlptrace",
        importpath = "go.opentelemetry.io/otel/exporters/otlp/otlptrace",
        sum = "h1:f0cb2XPmrqn4XMy9PNliTgRKJgS5WcL/u0/WRYGz4t0=",
        version = "v1.39.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_exporters_otlp_otlptrace_otlptracehttp",
        importpath = "go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracehttp",
        sum = "h1:Ckwye2FpXkYgiHX7fyVrN1uA/UYd9ounqqTuSNAv0k4=",
        version = "v1.39.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_exporters_prometheus",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/exporters/prometheus",
        sum = "h1:QXobPHrwiGLM4ufrY3EOmDPJpo2P90UuFau4CDPJA/I=",
        version = "v0.53.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_log",
        importpath = "go.opentelemetry.io/otel/log",
        sum = "h1:DeuBPqCi6pQwtCK0pO4fvMB5eBq6sNxEnuTs88pjsN4=",
        version = "v0.16.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_metric",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/metric",
        sum = "h1:ZlrO8Hu9+GAhnepmRGhSU7/VkpjrNowxRN9GyKR4wzA=",
        version = "v1.20.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_sdk",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/sdk",
        sum = "h1:KHW/jUzgo6wsPh9At46+h4upjtccTmuZCFAc9OJ71f8=",
        version = "v1.40.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_sdk_log",
        importpath = "go.opentelemetry.io/otel/sdk/log",
        sum = "h1:e/b4bdlQwC5fnGtG3dlXUrNOnP7c8YLVSpSfEBIkTnI=",
        version = "v0.16.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_sdk_metric",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/sdk/metric",
        sum = "h1:mtmdVqgQkeRxHgRv4qhyJduP3fYJRMX4AtAlbuWdCYw=",
        version = "v1.40.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_trace",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/trace",
        sum = "h1:+yxVAPZPbQhbC3OfAkeIVTky6iTFpcr4SiY9om7mXSQ=",
        version = "v1.20.0",
    )
    go_repository(
        name = "io_opentelemetry_go_proto_otlp",
        importpath = "go.opentelemetry.io/proto/otlp",
        sum = "h1:l706jCMITVouPOqEnii2fIAuO3IVGBRPV5ICjceRb/A=",
        version = "v1.9.0",
    )
    go_repository(
        name = "io_rsc_omap",
        importpath = "rsc.io/omap",
        sum = "h1:c1M8jchnHbzmJALzGLclfH3xDWXrPxSUHXzH5C+8Kdw=",
        version = "v1.2.0",
    )
    go_repository(
        name = "io_rsc_ordered",
        importpath = "rsc.io/ordered",
        sum = "h1:1kZM6RkTmceJgsFH/8DLQvkCVEYomVDJfBRLT595Uak=",
        version = "v1.1.1",
    )
    go_repository(
        name = "net_howett_plist",
        build_file_generation = "on",
        importpath = "howett.net/plist",
        sum = "h1:37GdZ8tP09Q35o9ych3ehygcsL+HqKSwzctveSlarvM=",
        version = "v1.0.1",
    )

    go_repository(
        name = "org_golang_google_adk",
        importpath = "google.golang.org/adk",
        sum = "h1:UNyIb604EWWJTCsmKg5tuo/oaESGiR9DHgFzTICN3zM=",
        version = "v1.1.0",
    )
    go_repository(
        name = "org_golang_google_api",
        importpath = "google.golang.org/api",
        sum = "h1:xfKJeAJaMwb8OC9fesr369rjciQ704AjU/psjkKURSI=",
        version = "v0.252.0",
    )
    go_repository(
        name = "org_golang_google_appengine",
        importpath = "google.golang.org/appengine",
        sum = "h1:IhEN5q69dyKagZPYMSdIjS2HqprW324FRQZJcGqPAsM=",
        version = "v1.6.8",
    )
    go_repository(
        name = "org_golang_google_genai",
        importpath = "google.golang.org/genai",
        sum = "h1:kYxyQSH+vsib8dvsgyLJzsVEIv5k3ZmHJyVqdvGncmc=",
        version = "v1.40.0",
    )
    go_repository(
        name = "org_golang_google_genproto",
        importpath = "google.golang.org/genproto",
        sum = "h1:vLd1CJuJOUgV6qijD7KT5Y2ZtC97ll4dxjTUappMnbo=",
        version = "v0.0.0-20251014184007-4626949a642f",
    )
    go_repository(
        name = "org_golang_google_genproto_googleapis_api",
        build_file_generation = "on",
        importpath = "google.golang.org/genproto/googleapis/api",
        sum = "h1:merA0rdPeUV3YIIfHHcH4qBkiQAc1nfCKSI7lB4cV2M=",
        version = "v0.0.0-20260128011058-8636f8732409",
    )
    go_repository(
        name = "org_golang_google_genproto_googleapis_rpc",
        build_file_generation = "on",
        importpath = "google.golang.org/genproto/googleapis/rpc",
        sum = "h1:t/LOSXPJ9R0B6fnZNyALBRfZBH0Uy0gT+uR+SJ6syqQ=",
        version = "v0.0.0-20260217215200-42d3e9bedb6d",
    )
    go_repository(
        name = "org_golang_google_grpc",
        build_file_generation = "on",
        importpath = "google.golang.org/grpc",
        sum = "h1:sybAEdRIEtvcD68Gx7dmnwjZKlyfuc61Dyo9pGXXkKE=",
        version = "v1.79.3",
    )
    go_repository(
        name = "org_golang_google_grpc_cmd_protoc_gen_go_grpc",
        build_file_generation = "on",
        importpath = "google.golang.org/grpc/cmd/protoc-gen-go-grpc",
        sum = "h1:dwpNT7iSgQjDx7JJW0csIQiQzV/x8M8sJYTa0aLfGyE=",
        version = "v1.6.2-0.20260327093101-b71c26202050",
    )
    go_repository(
        name = "org_golang_google_protobuf",
        build_file_generation = "on",
        importpath = "google.golang.org/protobuf",
        sum = "h1:fV6ZwhNocDyBLK0dj+fg8ektcVegBBuEolpbTQyBNVE=",
        version = "v1.36.11",
    )
    go_repository(
        name = "org_golang_x_crypto",
        build_file_generation = "on",
        importpath = "golang.org/x/crypto",
        sum = "h1:+Ng2ULVvLHnJ/ZFEq4KdcDd/cfjrrjjNSXNzxg0Y4U4=",
        version = "v0.49.0",
    )
    go_repository(
        name = "org_golang_x_exp",
        build_file_generation = "on",
        importpath = "golang.org/x/exp",
        sum = "h1:Zt3DZoOFFYkKhDT3v7Lm9FDMEV06GpzjG2jrqW+QTE0=",
        version = "v0.0.0-20260218203240-3dfff04db8fa",
    )
    go_repository(
        name = "org_golang_x_lint",
        importpath = "golang.org/x/lint",
        sum = "h1:XQyxROzUlZH+WIQwySDgnISgOivlhjIEwaQaJEJrrN0=",
        version = "v0.0.0-20190313153728-d0100b6bd8b3",
    )
    go_repository(
        name = "org_golang_x_mod",
        build_file_generation = "on",
        importpath = "golang.org/x/mod",
        sum = "h1:tHFzIWbBifEmbwtGz65eaWyGiGZatSrT9prnU8DbVL8=",
        version = "v0.33.0",
    )
    go_repository(
        name = "org_golang_x_net",
        build_file_generation = "on",
        importpath = "golang.org/x/net",
        sum = "h1:94R/GTO7mt3/4wIKpcR5gkGmRLOuE/2hNGeWq/GBIFo=",
        version = "v0.51.0",
    )
    go_repository(
        name = "org_golang_x_oauth2",
        build_file_generation = "on",
        importpath = "golang.org/x/oauth2",
        sum = "h1:Mv2mzuHuZuY2+bkyWXIHMfhNdJAdwW3FuWeCPYN5GVQ=",
        version = "v0.35.0",
    )
    go_repository(
        name = "org_golang_x_sync",
        build_file_generation = "on",
        importpath = "golang.org/x/sync",
        sum = "h1:e0PTpb7pjO8GAtTs2dQ6jYa5BWYlMuX047Dco/pItO4=",
        version = "v0.20.0",
    )
    go_repository(
        name = "org_golang_x_sys",
        build_file_generation = "on",
        importpath = "golang.org/x/sys",
        sum = "h1:omrd2nAlyT5ESRdCLYdm3+fMfNFE/+Rf4bDIQImRJeo=",
        version = "v0.42.0",
    )
    go_repository(
        name = "org_golang_x_term",
        build_file_generation = "on",
        importpath = "golang.org/x/term",
        sum = "h1:QCgPso/Q3RTJx2Th4bDLqML4W6iJiaXFq2/ftQF13YU=",
        version = "v0.41.0",
    )
    go_repository(
        name = "org_golang_x_text",
        build_file_generation = "on",
        importpath = "golang.org/x/text",
        sum = "h1:JOVx6vVDFokkpaq1AEptVzLTpDe9KGpj5tR4/X+ybL8=",
        version = "v0.35.0",
    )
    go_repository(
        name = "org_golang_x_time",
        importpath = "golang.org/x/time",
        sum = "h1:MRx4UaLrDotUKUdCIqzPC48t1Y9hANFKIRpNx+Te8PI=",
        version = "v0.14.0",
    )
    go_repository(
        name = "org_golang_x_tools",
        build_file_generation = "on",
        importpath = "golang.org/x/tools",
        sum = "h1:uNgphsn75Tdz5Ji2q36v/nsFSfR/9BRFvqhGBaJGd5k=",
        version = "v0.42.0",
    )
    go_repository(
        name = "org_golang_x_xerrors",
        build_file_generation = "on",
        importpath = "golang.org/x/xerrors",
        sum = "h1:9zdDQZ7Thm29KFXgAX/+yaf3eVbP7djjWp/dXAppNCc=",
        version = "v0.0.0-20190717185122-a985d3407aa7",
    )
    go_repository(
        name = "org_gonum_v1_gonum",
        build_file_generation = "on",
        importpath = "gonum.org/v1/gonum",
        sum = "h1:5+ul4Swaf3ESvrOnidPp4GZbzf0mxVQpDCYUQE7OJfk=",
        version = "v0.16.0",
    )
    go_repository(
        name = "org_modernc_cc_v4",
        build_file_generation = "on",
        importpath = "modernc.org/cc/v4",
        sum = "h1:9W30zRlYrefrDV2JE2O8VDtJ1yPGownxciz5rrbQZis=",
        version = "v4.27.1",
    )
    go_repository(
        name = "org_modernc_ccgo_v4",
        build_file_generation = "on",
        importpath = "modernc.org/ccgo/v4",
        sum = "h1:hjG66bI/kqIPX1b2yT6fr/jt+QedtP2fqojG2VrFuVw=",
        version = "v4.32.0",
    )
    go_repository(
        name = "org_modernc_fileutil",
        build_file_generation = "on",
        importpath = "modernc.org/fileutil",
        sum = "h1:j6ZzNTftVS054gi281TyLjHPp6CPHr2KCxEXjEbD6SM=",
        version = "v1.4.0",
    )
    go_repository(
        name = "org_modernc_gc_v2",
        build_file_generation = "on",
        importpath = "modernc.org/gc/v2",
        sum = "h1:nyqdV8q46KvTpZlsw66kWqwXRHdjIlJOhG6kxiV/9xI=",
        version = "v2.6.5",
    )
    go_repository(
        name = "org_modernc_gc_v3",
        build_file_generation = "on",
        importpath = "modernc.org/gc/v3",
        sum = "h1:ZtDCnhonXSZexk/AYsegNRV1lJGgaNZJuKjJSWKyEqo=",
        version = "v3.1.2",
    )
    go_repository(
        name = "org_modernc_goabi0",
        build_file_generation = "on",
        importpath = "modernc.org/goabi0",
        sum = "h1:HvEowk7LxcPd0eq6mVOAEMai46V+i7Jrj13t4AzuNks=",
        version = "v0.2.0",
    )
    go_repository(
        name = "org_modernc_libc",
        build_file_generation = "on",
        importpath = "modernc.org/libc",
        sum = "h1:U58NawXqXbgpZ/dcdS9kMshu08aiA6b7gusEusqzNkw=",
        version = "v1.70.0",
    )
    go_repository(
        name = "org_modernc_mathutil",
        build_file_generation = "on",
        importpath = "modernc.org/mathutil",
        sum = "h1:GCZVGXdaN8gTqB1Mf/usp1Y/hSqgI2vAGGP4jZMCxOU=",
        version = "v1.7.1",
    )
    go_repository(
        name = "org_modernc_memory",
        build_file_generation = "on",
        importpath = "modernc.org/memory",
        sum = "h1:o4QC8aMQzmcwCK3t3Ux/ZHmwFPzE6hf2Y5LbkRs+hbI=",
        version = "v1.11.0",
    )
    go_repository(
        name = "org_modernc_opt",
        build_file_generation = "on",
        importpath = "modernc.org/opt",
        sum = "h1:2kNGMRiUjrp4LcaPuLY2PzUfqM/w9N23quVwhKt5Qm8=",
        version = "v0.1.4",
    )
    go_repository(
        name = "org_modernc_sortutil",
        build_file_generation = "on",
        importpath = "modernc.org/sortutil",
        sum = "h1:+xyoGf15mM3NMlPDnFqrteY07klSFxLElE2PVuWIJ7w=",
        version = "v1.2.1",
    )
    go_repository(
        name = "org_modernc_sqlite",
        build_file_generation = "on",
        importpath = "modernc.org/sqlite",
        sum = "h1:ElZyLop3Q2mHYk5IFPPXADejZrlHu7APbpB0sF78bq4=",
        version = "v1.48.0",
    )
    go_repository(
        name = "org_modernc_strutil",
        build_file_generation = "on",
        importpath = "modernc.org/strutil",
        sum = "h1:UneZBkQA+DX2Rp35KcM69cSsNES9ly8mQWD71HKlOA0=",
        version = "v1.2.1",
    )
    go_repository(
        name = "org_modernc_token",
        build_file_generation = "on",
        importpath = "modernc.org/token",
        sum = "h1:Xl7Ap9dKaEs5kLoOQeQmPWevfnk/DM5qcLcYlA8ys6Y=",
        version = "v1.1.0",
    )
    go_repository(
        name = "org_uber_go_goleak",
        build_file_generation = "on",
        importpath = "go.uber.org/goleak",
        sum = "h1:2K3zAYmnTNqV73imy9J1T3WC+gmCePx2hEGkimedGto=",
        version = "v1.3.0",
    )
    go_repository(
        name = "org_uber_go_multierr",
        build_file_generation = "on",
        importpath = "go.uber.org/multierr",
        sum = "h1:blXXJkSxSSfBVBlC76pxqeO+LN3aDfLQo+309xJstO0=",
        version = "v1.11.0",
    )
