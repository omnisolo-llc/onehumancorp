package agents

import (
	"bytes"
	"context"
	"os"
	"testing"
)

func TestUniversalTransportBridge_StandaloneMode(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	buf := new(bytes.Buffer)
	bridge := NewUniversalTransportBridge(nil, "", "", nil, buf)

	// In Standalone mode, the underlying transport should be InProcessTransport
	if _, ok := bridge.transport.(*InProcessTransport); !ok {
		t.Errorf("Expected transport to be *InProcessTransport, got %T", bridge.transport)
	}

	err := bridge.Send(context.Background(), []byte("test message"))
	if err != nil {
		t.Fatalf("Unexpected error sending message: %v", err)
	}

	// Verify the buffer received the message with a newline
	if buf.String() != "test message\n" {
		t.Errorf("Expected 'test message\n', got %q", buf.String())
	}

	// Close down
	err = bridge.Close()
	if err != nil {
		t.Errorf("Unexpected error closing bridge: %v", err)
	}
}

func TestUniversalTransportBridge_Receive(t *testing.T) {
    os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	reader := bytes.NewBufferString("test message\n")
	bridge := NewUniversalTransportBridge(nil, "", "", reader, nil)

    msg, err := bridge.Receive(context.Background())
    if err != nil {
        t.Fatalf("Unexpected error receiving message: %v", err)
    }

    if string(msg) != "test message" {
        t.Errorf("Expected 'test message', got %q", string(msg))
    }

    bridge.Close()
}
