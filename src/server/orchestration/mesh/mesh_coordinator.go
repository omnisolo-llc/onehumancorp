package mesh

import (
	"context"
	"encoding/json"
	"errors"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
)

type AgentMeshMessage struct {
	ID        string    `json:"id"`
	TenantID  string    `json:"tenant_id"`
	Sender    string    `json:"sender"`
	Recipient *string   `json:"recipient"`
	Channel   string    `json:"channel"`
	Content   []byte    `json:"content"`
	CreatedAt time.Time `json:"created_at"`
}

type MeshCoordinatorService struct {
	mesh TeammateMesh
	db   db.Provider
}

func NewMeshCoordinatorService(mesh TeammateMesh, dbProvider db.Provider) *MeshCoordinatorService {
	return &MeshCoordinatorService{
		mesh: mesh,
		db:   dbProvider,
	}
}

// Publish stores the message in DB and then publishes it via TeammateMesh
func (s *MeshCoordinatorService) Publish(ctx context.Context, msg *AgentMeshMessage) error {
	if msg.ID == "" {
		msg.ID = uuid.NewString()
	}
	if msg.TenantID == "" {
		return errors.New("tenant_id is required")
	}

	query := `INSERT INTO agent_mesh_messages (id, tenant_id, sender, recipient, channel, content) VALUES ($1, $2, $3, $4, $5, $6)`
	var recipient string
	if msg.Recipient != nil {
		recipient = *msg.Recipient
	}

	if s.db.IsSQLite() {
		query = `INSERT INTO agent_mesh_messages (id, tenant_id, sender, recipient, channel, content) VALUES (?, ?, ?, ?, ?, ?)`
		_, err := s.db.Exec(ctx, query, msg.ID, msg.TenantID, msg.Sender, recipient, msg.Channel, string(msg.Content))
		if err != nil {
			return err
		}
	} else {
		_, err := s.db.Exec(ctx, query, msg.ID, msg.TenantID, msg.Sender, recipient, msg.Channel, string(msg.Content))
		if err != nil {
			return err
		}
	}

	payload, err := json.Marshal(msg)
	if err != nil {
		return err
	}

	return s.mesh.Publish(ctx, msg.Channel, payload)
}

func (s *MeshCoordinatorService) Subscribe(ctx context.Context, channel string, handler func(msg *AgentMeshMessage)) (Subscription, error) {
	return s.mesh.Subscribe(ctx, channel, func(payload []byte) {
		var msg AgentMeshMessage
		if err := json.Unmarshal(payload, &msg); err == nil {
			handler(&msg)
		}
	})
}
