package vector

import (
	"testing"
)

func TestNewVectorStorageTransport(t *testing.T) {
	// Simple test for factory since we use sqlmock for db level tests
	providerCloud := NewVectorStorageTransport("cloud", nil)
	if _, ok := providerCloud.(*pgvectorProvider); !ok {
		t.Errorf("Expected *pgvectorProvider, got %T", providerCloud)
	}

	providerStandalone := NewVectorStorageTransport("standalone", nil)
	if _, ok := providerStandalone.(*sqliteVssProvider); !ok {
		t.Errorf("Expected *sqliteVssProvider, got %T", providerStandalone)
	}
}

func TestFloat32ArrayToBytes(t *testing.T) {
	arr := []float32{1.0, 2.0, 3.0}
	b, err := Float32ArrayToBytes(arr)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	// 3 floats * 4 bytes each = 12 bytes
	if len(b) != 12 {
		t.Errorf("Expected 12 bytes, got %d", len(b))
	}
}
