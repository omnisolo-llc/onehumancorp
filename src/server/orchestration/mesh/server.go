package mesh

import (
	"context"

	hub "github.com/onehumancorp/mono/src/proto/hub"
)

type MeshServer struct {
	hub.UnimplementedHubServiceServer
	node *CentrifugeNode
}

func NewMeshServer(node *CentrifugeNode) *MeshServer {
	return &MeshServer{node: node}
}

func (s *MeshServer) StreamMeshEvents(req *hub.EventStreamRequest, srv hub.HubService_StreamMeshEventsServer) error {
	ctx := srv.Context()
	ch := make(chan *hub.MeshEvent, 100)

	s.node.SubscribeNode(ctx, req.Topic, func(e *hub.MeshEvent) {
		ch <- e
	})

	for {
		select {
		case <-ctx.Done():
			return ctx.Err()
		case e := <-ch:
			if err := srv.Send(e); err != nil {
				return err
			}
		}
	}
}

func (s *MeshServer) PublishMeshEvent(ctx context.Context, req *hub.PublishMeshEventRequest) (*hub.PublishMessageResponse, error) {
	err := s.node.Broadcast(ctx, req.Event.Topic, req.Event)
	return &hub.PublishMessageResponse{Success: err == nil}, err
}
