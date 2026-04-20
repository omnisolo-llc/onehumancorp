package nats

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/stretchr/testify/assert"
)

func TestMCPTool_PublishSubscribe(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	ns, err := StartEmbeddedServerIfNeeded()
	assert.NoError(t, err)
	assert.NotNil(t, ns)
	defer ns.Shutdown()

	clientURL := ns.ClientURL()

	nc, err := nats.Connect(clientURL)
	assert.NoError(t, err)
	js, err := nc.JetStream()
	assert.NoError(t, err)

	// Create a stream for testing JetStream
	_, err = js.AddStream(&nats.StreamConfig{
		Name:     "TEST_STREAM",
		Subjects: []string{"swarm.>"},
	})
	assert.NoError(t, err)
	nc.Close()


	tool, err := NewMCPTool(clientURL)
	assert.NoError(t, err)
	defer tool.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	subject := "swarm.test"
	sub, err := tool.SubscribeSync(ctx, subject)
	assert.NoError(t, err)

	payload := map[string]string{"message": "hello swarm"}
	err = tool.PublishJSON(ctx, subject, payload)
	assert.NoError(t, err)

	msg, err := sub.NextMsg(2 * time.Second)
	assert.NoError(t, err)
	assert.Contains(t, string(msg.Data), "hello swarm")
}
