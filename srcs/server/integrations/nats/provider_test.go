package nats

import (
	"testing"
	"os"
	"github.com/stretchr/testify/assert"
)

func TestNATSIntegration_Metadata(t *testing.T) {
	integration := &NATSIntegration{}
	metadata := integration.Metadata()
	assert.Equal(t, "nats", metadata.GetId())
	assert.Equal(t, "NATS JetStream", metadata.GetName())
}

func TestNATSIntegration_WizardSteps(t *testing.T) {
	integration := &NATSIntegration{}
	steps := integration.WizardSteps()
	assert.Len(t, steps, 1)
	assert.Equal(t, "Connection Data", steps[0].GetTitle())
}

func TestStartEmbeddedServerIfNeeded_Cloud(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	ns, err := StartEmbeddedServerIfNeeded()
	assert.NoError(t, err)
	assert.Nil(t, ns)
}

func TestStartEmbeddedServerIfNeeded_Standalone(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	ns, err := StartEmbeddedServerIfNeeded()
	assert.NoError(t, err)
	assert.NotNil(t, ns)
	if ns != nil {
		ns.Shutdown()
	}
}
