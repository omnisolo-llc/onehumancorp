package orchestration

import (
	"testing"
    "github.com/stretchr/testify/assert"
)

func TestMockHarness_CompactAndReset(t *testing.T) {
	mock := &MockHarness{}
    err1 := mock.Compact()
    err2 := mock.Reset()
    assert.NoError(t, err1)
    assert.NoError(t, err2)
}
