package builtin

import (
	"context"
	"fmt"
	"sync"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"
)

type GRPCHubAdapter struct {
	conn   *grpc.ClientConn
	client pb.HubServiceClient

	mu            sync.Mutex
	inbox         map[string][]HubMessage
	subs          map[string]map[chan struct{}]struct{}
	streamCancels map[string]context.CancelFunc
}

func NewGRPCHubAdapter(ctx context.Context, address string) (*GRPCHubAdapter, error) {
	conn, err := grpc.DialContext(ctx, address, grpc.WithBlock(), grpc.WithInsecure())
	if err != nil {
		return nil, fmt.Errorf("dial hub %q: %w", address, err)
	}
	return &GRPCHubAdapter{
		conn:          conn,
		client:        pb.NewHubServiceClient(conn),
		inbox:         make(map[string][]HubMessage),
		subs:          make(map[string]map[chan struct{}]struct{}),
		streamCancels: make(map[string]context.CancelFunc),
	}, nil
}

func (a *GRPCHubAdapter) Close() error {
	a.mu.Lock()
	cancels := make([]context.CancelFunc, 0, len(a.streamCancels))
	for _, cancel := range a.streamCancels {
		cancels = append(cancels, cancel)
	}
	a.streamCancels = make(map[string]context.CancelFunc)
	a.subs = make(map[string]map[chan struct{}]struct{})
	a.inbox = make(map[string][]HubMessage)
	a.mu.Unlock()

	for _, cancel := range cancels {
		cancel()
	}
	return a.conn.Close()
}

func (a *GRPCHubAdapter) RegisterAgent(agent HubAgent) {
	_, _ = a.client.RegisterAgent(context.Background(), pb.RegisterAgentRequest_builder{
		Agent: pb.Agent_builder{
			Id:             proto.String(agent.ID),
			Name:           proto.String(agent.Name),
			Role:           proto.String(agent.Role),
			OrganizationId: proto.String(agent.OrganizationID),
			Status:         proto.String(string(agent.Status)),
			ProviderType:   proto.String(agent.ProviderType),
			Region:         proto.String(agent.Region),
			Managed:        proto.Bool(agent.Managed),
		}.Build(),
	}.Build())
}

func (a *GRPCHubAdapter) ReportWorkerState(state *pb.WorkerState) {
	_, _ = a.client.ReportWorkerState(context.Background(), pb.ReportWorkerStateRequest_builder{
		State: state,
	}.Build())
}

func (a *GRPCHubAdapter) Subscribe(agentID string) (<-chan struct{}, func()) {
	ch := make(chan struct{}, 1)

	a.mu.Lock()
	if a.subs[agentID] == nil {
		a.subs[agentID] = make(map[chan struct{}]struct{})
	}
	a.subs[agentID][ch] = struct{}{}
	if _, ok := a.streamCancels[agentID]; !ok {
		streamCtx, cancel := context.WithCancel(context.Background())
		a.streamCancels[agentID] = cancel
		go a.streamLoop(streamCtx, agentID)
	}
	a.mu.Unlock()

	return ch, func() {
		a.mu.Lock()
		defer a.mu.Unlock()
		delete(a.subs[agentID], ch)
		if len(a.subs[agentID]) == 0 {
			delete(a.subs, agentID)
			if cancel, ok := a.streamCancels[agentID]; ok {
				cancel()
				delete(a.streamCancels, agentID)
			}
		}
	}
}

func (a *GRPCHubAdapter) Inbox(agentID string) []HubMessage {
	a.mu.Lock()
	defer a.mu.Unlock()
	msgs := a.inbox[agentID]
	if len(msgs) == 0 {
		return nil
	}
	out := append([]HubMessage(nil), msgs...)
	delete(a.inbox, agentID)
	return out
}

func (a *GRPCHubAdapter) Publish(msg HubMessage) error {
	_, err := a.client.Publish(context.Background(), pb.PublishMessageRequest_builder{
		Message: pb.Message_builder{
			Id:             proto.String(msg.ID),
			FromAgent:      proto.String(msg.FromAgent),
			ToAgent:        proto.String(msg.ToAgent),
			Type:           proto.String(msg.Type),
			Content:        proto.String(msg.Content),
			OccurredAtUnix: proto.Int64(time.Now().UTC().Unix()),
		}.Build(),
	}.Build())
	if err != nil {
		return fmt.Errorf("publish to hub: %w", err)
	}
	return nil
}

func (a *GRPCHubAdapter) streamLoop(ctx context.Context, agentID string) {
	backoff := 200 * time.Millisecond
	for {
		stream, err := a.client.StreamMessages(ctx, pb.StreamMessagesRequest_builder{AgentId: proto.String(agentID)}.Build())
		if err != nil {
			if ctx.Err() != nil {
				return
			}
			select {
			case <-ctx.Done():
				return
			case <-time.After(backoff):
			}
			if backoff < 2*time.Second {
				backoff *= 2
			}
			continue
		}

		backoff = 200 * time.Millisecond
		for {
			message, err := stream.Recv()
			if err != nil {
				break
			}
			a.enqueue(agentID, HubMessage{
				ID:        message.GetId(),
				FromAgent: message.GetFromAgent(),
				ToAgent:   message.GetToAgent(),
				Type:      message.GetType(),
				Content:   message.GetContent(),
			})
		}
		if ctx.Err() != nil {
			return
		}
	}
}

func (a *GRPCHubAdapter) enqueue(agentID string, msg HubMessage) {
	a.mu.Lock()
	a.inbox[agentID] = append(a.inbox[agentID], msg)
	subs := make([]chan struct{}, 0, len(a.subs[agentID]))
	for ch := range a.subs[agentID] {
		subs = append(subs, ch)
	}
	a.mu.Unlock()

	for _, ch := range subs {
		select {
		case ch <- struct{}{}:
		default:
		}
	}
}
