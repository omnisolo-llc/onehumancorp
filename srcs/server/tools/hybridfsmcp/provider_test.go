package hybridfsmcp

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tmpDir := t.TempDir()
	t.Setenv("OHC_WORKSPACE_DIR", tmpDir)

	p := NewLocalFSProvider()

	// Create a sibling directory to test sibling access
	siblingDir := tmpDir + "sibling"
	os.MkdirAll(siblingDir, 0700)
	defer os.RemoveAll(siblingDir)

	_, err := p.resolvePath("../escaped")
	if err == nil {
		t.Errorf("expected error escaping base dir")
	}

	_, err = p.resolvePath(filepath.Join("..", filepath.Base(tmpDir)+"sibling"))
	if err == nil {
		t.Errorf("expected error escaping base dir (sibling test)")
	}

	validPath, err := p.resolvePath("valid.txt")
	if err != nil {
		t.Errorf("expected no error for valid path, got %v", err)
	}
	if filepath.Base(validPath) != "valid.txt" {
		t.Errorf("expected valid.txt, got %s", validPath)
	}
}

func TestCloudFSProvider_PathTraversal(t *testing.T) {
	tmpDir := t.TempDir()
	t.Setenv("OHC_TENANT_PV_DIR", tmpDir)

	p := NewCloudFSProvider()

	claims := &auth.Claims{OrganizationID: "tenant1"}

	// Create a sibling directory
	siblingDir := filepath.Join(tmpDir, "tenant1sibling")
	os.MkdirAll(siblingDir, 0700)
	defer os.RemoveAll(siblingDir)

	_, err := p.resolvePath("../tenant2", claims)
	if err == nil {
		t.Errorf("expected error escaping tenant dir")
	}

	_, err = p.resolvePath("../tenant1sibling/file.txt", claims)
	if err == nil {
		t.Errorf("expected error escaping tenant dir (sibling test)")
	}

	validPath, err := p.resolvePath("valid.txt", claims)
	if err != nil {
		t.Errorf("expected no error for valid path, got %v", err)
	}
	if filepath.Base(validPath) != "valid.txt" {
		t.Errorf("expected valid.txt, got %s", validPath)
	}
}

func TestCloudFSProvider_MissingClaims(t *testing.T) {
	tmpDir := t.TempDir()
	t.Setenv("OHC_TENANT_PV_DIR", tmpDir)

	p := NewCloudFSProvider()

	_, err := p.resolvePath("valid.txt", nil)
	if err == nil {
		t.Errorf("expected error for missing claims")
	}

	_, err = p.resolvePath("valid.txt", &auth.Claims{OrganizationID: ""})
	if err == nil {
		t.Errorf("expected error for empty OrganizationID")
	}
}
