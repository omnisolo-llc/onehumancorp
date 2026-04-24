package nats

import (
	"context"
	"sync"
	"testing"

	"github.com/onehumancorp/mono/src/server/msgbus"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNatsIntegration_Embedded(t *testing.T) {
	ni, err := NewNatsIntegration(Config{IsLocal: true})
	require.NoError(t, err)
	require.NotNil(t, ni)
	defer ni.Close()

	var wg sync.WaitGroup
	wg.Add(1)

	var receivedMsg *msgbus.Message
	err = ni.Subscribe("test.topic", func(msg *msgbus.Message) {
		receivedMsg = msg
		wg.Done()
	})
	require.NoError(t, err)

	testData := []byte("hello event mesh")
	err = ni.Publish(context.Background(), "test.topic", testData)
	require.NoError(t, err)

	wg.Wait()

	assert.NotNil(t, receivedMsg)
	assert.Equal(t, "test.topic", receivedMsg.Topic)
	assert.Equal(t, testData, receivedMsg.Payload)
}

func TestNatsIntegration_Metadata(t *testing.T) {
	ni := &NatsIntegration{}
	meta := ni.Metadata()
	assert.NotNil(t, meta)

	steps := ni.WizardSteps()
	assert.Len(t, steps, 1)
	assert.NotNil(t, steps[0])
}
