load("@gazelle//:deps.bzl", "go_repository")

def go_repositories():
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
        name = "com_github_antlr4_go_antlr_v4",
        build_file_generation = "on",
        importpath = "github.com/antlr4-go/antlr/v4",
        sum = "h1:lxCg3LAv+EUK6t1i0y1V6/SLeUi0eKEKdhQAlS8TVTI=",
        version = "v4.13.0",
    )
    go_repository(
        name = "com_github_beorn7_perks",
        build_file_generation = "on",
        importpath = "github.com/beorn7/perks",
        sum = "h1:VlbKKnNfV8bJzeqoa4cOKqO6bYr3WgKZxO8Z16+hsOM=",
        version = "v1.0.1",
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
        name = "com_github_cncf_xds_go",
        build_file_generation = "on",
        importpath = "github.com/cncf/xds/go",
        sum = "h1:6xNmx7iTtyBRev0+D/Tv1FZd4SCg8axKApyNyRsAt/w=",
        version = "v0.0.0-20251210132809-ee656c7534f5",
    )
    go_repository(
        name = "com_github_davecgh_go_spew",
        build_file_generation = "on",
        importpath = "github.com/davecgh/go-spew",
        sum = "h1:vj9j/u1bqnvCEfJOwUhtlOARqs3+rkHYY13jYWTU97c=",
        version = "v1.1.1",
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
        sum = "h1:CjnDlHq8ikf6E492q6eKboGOC0T8CDaOvkHCIg8idEI=",
        version = "v1.4.3",
    )
    go_repository(
        name = "com_github_go_logr_stdr",
        build_file_generation = "on",
        importpath = "github.com/go-logr/stdr",
        sum = "h1:hSWxHoqTgW2S2qGc0LTAI563KZ5YKYRhT3MFKZMbjag=",
        version = "v1.2.2",
    )
    go_repository(
        name = "com_github_golang_glog",
        build_file_generation = "on",
        importpath = "github.com/golang/glog",
        sum = "h1:DrW6hGnjIhtvhOIiAKT6Psh/Kd/ldepEa81DKeiRJ5I=",
        version = "v1.2.5",
    )
    go_repository(
        name = "com_github_golang_jwt_jwt_v5",
        build_file_generation = "on",
        importpath = "github.com/golang-jwt/jwt/v5",
        sum = "h1:pv4AsKCKKZuqlgs5sUmn4x8UlGa0kEVt/puTpKx9vvo=",
        version = "v5.3.0",
    )
    go_repository(
        name = "com_github_golang_protobuf",
        build_file_generation = "on",
        importpath = "github.com/golang/protobuf",
        sum = "h1:i7eJL8qZTpSEXOPTxNKhASYpMn+8e5Q6AdndVa1dWek=",
        version = "v1.5.4",
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
        sum = "h1:wk8382ETsv4JYUZwIsn6YpYiWiBsYLSJiTsyBybVuN8=",
        version = "v0.7.0",
    )
    go_repository(
        name = "com_github_google_pprof",
        build_file_generation = "on",
        importpath = "github.com/google/pprof",
        sum = "h1:gbpYu9NMq8jhDVbvlGkMFWCjLFlqqEZjEmObmhUy6Vo=",
        version = "v0.0.0-20240409012703-83162a5b38cd",
    )
    go_repository(
        name = "com_github_google_uuid",
        build_file_generation = "on",
        importpath = "github.com/google/uuid",
        sum = "h1:NIvaJDMOsjHA8n1jAhLSgzrAzy1Hgr+hNrb57e+94F0=",
        version = "v1.6.0",
    )
    go_repository(
        name = "com_github_googlecloudplatform_opentelemetry_operations_go_detectors_gcp",
        build_file_generation = "on",
        importpath = "github.com/GoogleCloudPlatform/opentelemetry-operations-go/detectors/gcp",
        sum = "h1:sBEjpZlNHzK1voKq9695PJSX2o5NEXl7/OL3coiIY0c=",
        version = "v1.30.0",
    )
    go_repository(
        name = "com_github_hashicorp_golang_lru_v2",
        build_file_generation = "on",
        importpath = "github.com/hashicorp/golang-lru/v2",
        sum = "h1:a+bsQ5rvGLjzHuww6tVxozPZFVghXaHOwFs4luLUK2k=",
        version = "v2.0.7",
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
        name = "com_github_klauspost_compress",
        build_file_generation = "on",
        importpath = "github.com/klauspost/compress",
        sum = "h1:c/Cqfb0r+Yi+JtIEq73FWXVkRonBlf0CRNYc8Zttxdo=",
        version = "v1.18.0",
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
        name = "com_github_maypok86_otter",
        build_file_generation = "on",
        importpath = "github.com/maypok86/otter",
        sum = "h1:HhW1Pq6VdJkmWwcZZq19BlEQkHtI8xgsQzBVXJU0nfc=",
        version = "v1.2.4",
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
        name = "com_github_planetscale_vtprotobuf",
        build_file_generation = "on",
        importpath = "github.com/planetscale/vtprotobuf",
        sum = "h1:GFCKgmp0tecUJ0sJuv4pzYCqS9+RGSn52M3FUwPs+uo=",
        version = "v0.6.1-0.20240319094008-0393e58bdf10",
    )
    go_repository(
        name = "com_github_pmezard_go_difflib",
        build_file_generation = "on",
        importpath = "github.com/pmezard/go-difflib",
        sum = "h1:4DBwDE0NGyQoBHbLQYPwSUPoCMWR5BEzIk/f1lZbAQM=",
        version = "v1.0.0",
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
        name = "com_github_segmentio_asm",
        build_file_generation = "on",
        importpath = "github.com/segmentio/asm",
        sum = "h1:9BQrFxC+YOHJlTlHGkTrFWf59nbL3XnCoFLTwDCI7ys=",
        version = "v1.2.0",
    )
    go_repository(
        name = "com_github_segmentio_encoding",
        build_file_generation = "on",
        importpath = "github.com/segmentio/encoding",
        sum = "h1:OjMgICtcSFuNvQCdwqMCv9Tg7lEOXGwm1J5RPQccx6w=",
        version = "v0.5.3",
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
        name = "com_github_sony_gobreaker",
        build_file_generation = "on",
        importpath = "github.com/sony/gobreaker",
        sum = "h1:feX5fGGXSl3dYd4aHZItw+FpHLvvoaqkawKjVNiFMNQ=",
        version = "v1.0.0",
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
        name = "com_github_stretchr_objx",
        build_file_generation = "on",
        importpath = "github.com/stretchr/objx",
        sum = "h1:4G4v2dO3VZwixGIRoQ5Lfboy6nUhCyYzaqnIAPPhYs4=",
        version = "v0.1.0",
    )
    go_repository(
        name = "com_github_stretchr_testify",
        build_file_generation = "on",
        importpath = "github.com/stretchr/testify",
        sum = "h1:7s2iGBzp5EwR7/aIZr8ao5+dra3wiQyKjjFuvgVKu7U=",
        version = "v1.11.1",
    )
    go_repository(
        name = "com_github_valyala_bytebufferpool",
        build_file_generation = "on",
        importpath = "github.com/valyala/bytebufferpool",
        sum = "h1:GqA5TC/0021Y/b9FG4Oi9Mr3q7XYx6KllzawFIhcdPw=",
        version = "v1.0.0",
    )
    go_repository(
        name = "com_github_xhit_go_str2duration_v2",
        build_file_generation = "on",
        importpath = "github.com/xhit/go-str2duration/v2",
        sum = "h1:lxklc02Drh6ynqX+DdPyp5pCKLUQpRT8bp8Ydu2Bstc=",
        version = "v2.1.0",
    )
    go_repository(
        name = "com_google_cloud_go_compute_metadata",
        build_file_generation = "on",
        importpath = "cloud.google.com/go/compute/metadata",
        sum = "h1:pDUj4QMoPejqq20dK0Pg2N4yG9zIkYGdBtwLoEkH9Zs=",
        version = "v0.9.0",
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
        sum = "h1:Hei/4ADfdWqJk1ZMxUNpqntNwaWcugrBjAiHlqqRiVk=",
        version = "v1.0.0-20201130134442-10cb98267c6c",
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
        name = "io_opentelemetry_go_otel",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel",
        sum = "h1:lSQGzTgVR3+sgJDAU/7/ZMjN9Z+vUip7leaqBKy4sho=",
        version = "v1.42.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_exporters_prometheus",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/exporters/prometheus",
        sum = "h1:QXobPHrwiGLM4ufrY3EOmDPJpo2P90UuFau4CDPJA/I=",
        version = "v0.53.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_metric",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/metric",
        sum = "h1:2jXG+3oZLNXEPfNmnpxKDeZsFI5o4J+nz6xUlaFdF/4=",
        version = "v1.42.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_sdk",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/sdk",
        sum = "h1:nMLYcjVsvdui1B/4FRkwjzoRVsMK8uL/cj0OyhKzt18=",
        version = "v1.39.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_sdk_metric",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/sdk/metric",
        sum = "h1:cXMVVFVgsIf2YL6QkRF4Urbr/aMInf+2WKg+sEJTtB8=",
        version = "v1.39.0",
    )
    go_repository(
        name = "io_opentelemetry_go_otel_trace",
        build_file_generation = "on",
        importpath = "go.opentelemetry.io/otel/trace",
        sum = "h1:OUCgIPt+mzOnaUTpOQcBiM/PLQ/Op7oq6g4LenLmOYY=",
        version = "v1.42.0",
    )
    go_repository(
        name = "org_golang_google_genproto_googleapis_api",
        build_file_generation = "on",
        importpath = "google.golang.org/genproto/googleapis/api",
        sum = "h1:fCvbg86sFXwdrl5LgVcTEvNC+2txB5mgROGmRL5mrls=",
        version = "v0.0.0-20251202230838-ff82c1b0f217",
    )
    go_repository(
        name = "org_golang_google_genproto_googleapis_rpc",
        build_file_generation = "on",
        importpath = "google.golang.org/genproto/googleapis/rpc",
        sum = "h1:gRkg/vSppuSQoDjxyiGfN4Upv/h/DQmIR10ZU8dh4Ww=",
        version = "v0.0.0-20251202230838-ff82c1b0f217",
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
        sum = "h1:cKRW/pmt1pKAfetfu+RCEvjvZkA9RimPbh7bhFjGVBU=",
        version = "v0.46.0",
    )
    go_repository(
        name = "org_golang_x_exp",
        build_file_generation = "on",
        importpath = "golang.org/x/exp",
        sum = "h1:nDVHiLt8aIbd/VzvPWN6kSOPE7+F/fNFDSXLVYkE/Iw=",
        version = "v0.0.0-20250305212735-054e65f0b394",
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
        sum = "h1:zyQRTTrjc33Lhh0fBgT/H3oZq9WuvRR5gPC70xpDiQU=",
        version = "v0.48.0",
    )
    go_repository(
        name = "org_golang_x_oauth2",
        build_file_generation = "on",
        importpath = "golang.org/x/oauth2",
        sum = "h1:hqK/t4AKgbqWkdkcAeI8XLmbK+4m4G5YeQRrmiotGlw=",
        version = "v0.34.0",
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
        sum = "h1:Ivj+2Cp/ylzLiEU89QhWblYnOE9zerudt9Ftecq2C6k=",
        version = "v0.41.0",
    )
    go_repository(
        name = "org_golang_x_term",
        build_file_generation = "on",
        importpath = "golang.org/x/term",
        sum = "h1:PQ5pkm/rLO6HnxFR7N2lJHOZX6Kez5Y1gDSJla6jo7Q=",
        version = "v0.38.0",
    )
    go_repository(
        name = "org_golang_x_text",
        build_file_generation = "on",
        importpath = "golang.org/x/text",
        sum = "h1:ZD01bjUt1FQ9WJ0ClOL5vxgxOI/sVCNgX1YtKwcY0mU=",
        version = "v0.32.0",
    )
    go_repository(
        name = "org_golang_x_tools",
        build_file_generation = "on",
        importpath = "golang.org/x/tools",
        sum = "h1:uNgphsn75Tdz5Ji2q36v/nsFSfR/9BRFvqhGBaJGd5k=",
        version = "v0.42.0",
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
        name = "org_modernc_ebnf",
        build_file_generation = "on",
        importpath = "modernc.org/ebnf",
        sum = "h1:ilLq2kO1xGezeg75RyKffLsCLdamQHEmjv0CVq1QEQU=",
        version = "v1.1.0",
    )
    go_repository(
        name = "org_modernc_ebnfutil",
        build_file_generation = "on",
        importpath = "modernc.org/ebnfutil",
        sum = "h1:8AZ7iHDSIV6lrlgtexrIgmsey6wuSnB8s642ASDaTkc=",
        version = "v1.1.0",
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
        sum = "h1:wnIcc4XIGoWVkM9qGKn2PARAmpXsQWGebuOVOBYZZVY=",
        version = "v1.34.0",
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
