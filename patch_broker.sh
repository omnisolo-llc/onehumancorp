cat << 'INNER_EOF' > srcs/server/orchestration/mesh/broker.go
package mesh

import "context"

type MeshBroker interface {
	Broadcast(ctx context.Context, channel string, payload []byte) error
}
INNER_EOF
