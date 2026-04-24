package mesh

import (
    "testing"
)

func TestLocalMeshBroker(t *testing.T) {
    b := NewLocalMeshBroker()
    if b == nil {
        t.Fatal("expected broker")
    }
}
