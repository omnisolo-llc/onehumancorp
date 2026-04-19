module github.com/onehumancorp/mono

go 1.26

require (
	github.com/alicebob/miniredis/v2 v2.37.0
	github.com/centrifugal/centrifuge v0.38.0
	github.com/google/uuid v1.6.0
	github.com/gorilla/websocket v1.5.3
	github.com/jackc/pgx/v5 v5.9.1
	github.com/minio/minio-go/v7 v7.0.100
	github.com/modelcontextprotocol/go-sdk v1.5.0
	github.com/nats-io/nats.go v1.51.0
	github.com/prometheus/client_golang v1.23.2
	github.com/redis/go-redis/v9 v9.18.0
	github.com/redis/rueidis v1.0.68
	github.com/slack-go/slack v0.22.0
	github.com/spf13/afero v1.14.0
	github.com/spf13/viper v1.20.1
	github.com/stretchr/testify v1.11.1
	go.opentelemetry.io/otel v1.42.0
	go.opentelemetry.io/otel/exporters/prometheus v0.53.0
	go.opentelemetry.io/otel/metric v1.42.0
	go.opentelemetry.io/otel/sdk/metric v1.40.0
	golang.org/x/crypto v0.49.0
	google.golang.org/adk v1.1.0
	google.golang.org/genai v1.40.0
	google.golang.org/grpc v1.79.3
	google.golang.org/grpc/cmd/protoc-gen-go-grpc v1.6.2-0.20260327093101-b71c26202050
	google.golang.org/protobuf v1.36.11
	gopkg.in/yaml.v3 v3.0.1
	modernc.org/sqlite v1.48.0
)

require (
	cloud.google.com/go v0.123.0 //
	cloud.google.com/go/auth v0.17.0 //
	cloud.google.com/go/compute/metadata v0.9.0 //
	github.com/FZambia/eagle v0.2.0 //
	github.com/beorn7/perks v1.0.1 //
	github.com/centrifugal/protocol v0.17.0 //
	github.com/cespare/xxhash/v2 v2.3.0 //
	github.com/davecgh/go-spew v1.1.1 //
	github.com/deckarep/golang-set/v2 v2.8.0 //
	github.com/dgryski/go-rendezvous v0.0.0-20200823014737-9f7001d12a5f //
	github.com/dolthub/maphash v0.1.0 //
	github.com/dustin/go-humanize v1.0.1 //
	github.com/felixge/httpsnoop v1.0.4 //
	github.com/fsnotify/fsnotify v1.8.0 //
	github.com/gammazero/deque v0.2.1 //
	github.com/go-ini/ini v1.67.0 //
	github.com/go-jose/go-jose/v3 v3.0.4 //
	github.com/go-logr/logr v1.4.3 //
	github.com/go-logr/stdr v1.2.2 //
	github.com/go-stack/stack v1.8.1 //
	github.com/go-viper/mapstructure/v2 v2.2.1 //
	github.com/google/go-cmp v0.7.0 //
	github.com/google/jsonschema-go v0.4.2 //
	github.com/google/s2a-go v0.1.9 //
	github.com/google/safehtml v0.1.0 //
	github.com/googleapis/enterprise-certificate-proxy v0.3.6 //
	github.com/googleapis/gax-go/v2 v2.15.0 //
	github.com/jackc/pgpassfile v1.0.0 //
	github.com/jackc/pgservicefile v0.0.0-20240606120523-5a60cdf6a761 //
	github.com/jackc/puddle/v2 v2.2.2 //
	github.com/josharian/intern v1.0.0 //
	github.com/klauspost/compress v1.18.5 //
	github.com/klauspost/cpuid/v2 v2.2.11 //
	github.com/klauspost/crc32 v1.3.0 //
	github.com/mailru/easyjson v0.7.7 //
	github.com/mattn/go-isatty v0.0.20 //
	github.com/maypok86/otter v1.2.4 //
	github.com/minio/crc64nvme v1.1.1 //
	github.com/minio/md5-simd v1.1.2 //
	github.com/munnerz/goautoneg v0.0.0-20191010083416-a7dc8b61c822 //
	github.com/nats-io/nkeys v0.4.15 //
	github.com/nats-io/nuid v1.0.1 //
	github.com/ncruces/go-strftime v1.0.0 //
	github.com/pelletier/go-toml/v2 v2.2.3 //
	github.com/philhofer/fwd v1.2.0 //
	github.com/planetscale/vtprotobuf v0.6.1-0.20240319094008-0393e58bdf10 //
	github.com/playwright-community/playwright-go v0.5700.1 //
	github.com/pmezard/go-difflib v1.0.0 //
	github.com/prometheus/client_model v0.6.2 //
	github.com/prometheus/common v0.67.5 //
	github.com/prometheus/procfs v0.19.2 //
	github.com/quagmt/udecimal v1.9.0 //
	github.com/remyoudompheng/bigfft v0.0.0-20230129092748-24d4a6f8daec //
	github.com/rs/xid v1.6.0 //
	github.com/sagikazarmark/locafero v0.7.0 //
	github.com/segmentio/asm v1.2.0 //
	github.com/segmentio/encoding v0.5.4 //
	github.com/shadowspore/fossil-delta v0.0.0-20241213113458-1d797d70cbe3 //
	github.com/sourcegraph/conc v0.3.0 //
	github.com/spf13/cast v1.7.1 //
	github.com/spf13/pflag v1.0.10 //
	github.com/subosito/gotenv v1.6.0 //
	github.com/tinylib/msgp v1.6.1 //
	github.com/valyala/bytebufferpool v1.0.0 //
	github.com/yosida95/uritemplate/v3 v3.0.2 //
	github.com/yuin/gopher-lua v1.1.1 //
	go.opentelemetry.io/auto/sdk v1.2.1 //
	go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp v0.63.0 //
	go.opentelemetry.io/otel/log v0.16.0 //
	go.opentelemetry.io/otel/sdk v1.40.0 //
	go.opentelemetry.io/otel/trace v1.42.0 //
	go.uber.org/atomic v1.11.0 //
	go.uber.org/multierr v1.9.0 //
	go.yaml.in/yaml/v2 v2.4.3 //
	go.yaml.in/yaml/v3 v3.0.4 //
	golang.org/x/net v0.51.0 //
	golang.org/x/oauth2 v0.35.0 //
	golang.org/x/sync v0.20.0 //
	golang.org/x/sys v0.42.0 //
	golang.org/x/text v0.35.0 //
	google.golang.org/genproto/googleapis/rpc v0.0.0-20260128011058-8636f8732409 //
	modernc.org/libc v1.70.0 //
	modernc.org/mathutil v1.7.1 //
	modernc.org/memory v1.11.0 //
	rsc.io/omap v1.2.0 //
	rsc.io/ordered v1.1.1 //
)
