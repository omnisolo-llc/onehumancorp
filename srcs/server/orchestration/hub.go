package orchestration

import (
	"context"

	pb "github.com/onehumancorp/ohc/srcs/proto"
)

// MeshTransport defines the high-performance Pub/Sub layer for agent communication.
type MeshTransport interface {
	Publish(ctx context.Context, channel string, event *pb.MeshEvent) error
	Subscribe(ctx context.Context, channel string, handler func(*pb.MeshEvent)) error
	Close() error
}
