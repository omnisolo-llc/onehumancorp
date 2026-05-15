module github.com/onehumancorp/mono/src/server/orchestration

go 1.24.3

require (
	github.com/mattn/go-sqlite3 v1.14.44
	github.com/redis/rueidis v1.0.1
)

require go.opentelemetry.io/otel v1.21.0
require go.opentelemetry.io/otel/metric v1.21.0

require go.opentelemetry.io/otel/trace v1.21.0
require github.com/go-logr/logr v1.3.0
require github.com/go-logr/stdr v1.2.2

require google.golang.org/grpc v1.59.0
