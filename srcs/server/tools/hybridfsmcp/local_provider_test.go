package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestLocalProviderErrors(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfsmcp_err2")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)

	// test write error (make dir fail)
	err = os.Mkdir(tempDir+"/readonly", 0555)
	if err != nil {
		t.Fatal(err)
	}

	// Can't write to readonly dir
	err = provider.WriteFile(context.Background(), "readonly/test.txt", []byte("hi"))
	if err == nil {
		t.Fatal("expected error")
	}
}
