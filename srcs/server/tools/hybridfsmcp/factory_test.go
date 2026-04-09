package hybridfsmcp

import (
	"os"
	"testing"
)

func TestFactory(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_WORKSPACE_DIR", "/tmp/testworkspace")

	server := NewFileSystemServer()
	if server == nil {
		t.Fatal("expected server")
	}

	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_TENANT_PV_DIR", "/tmp/testtenant")

	serverCloud := NewFileSystemServer()
	if serverCloud == nil {
		t.Fatal("expected cloud server")
	}
}

func TestFactoryFallback(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_WORKSPACE_DIR", "")

	server := NewFileSystemServer()
	if server == nil {
		t.Fatal("expected server")
	}

	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_TENANT_PV_DIR", "")

	serverCloud := NewFileSystemServer()
	if serverCloud == nil {
		t.Fatal("expected cloud server")
	}
}
