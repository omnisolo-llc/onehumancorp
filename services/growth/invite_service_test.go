package growth

import (
	"testing"
)

func TestGenerateInviteLink(t *testing.T) {
	expected := "https://ohc.os/invite/user123-team"
	actual := GenerateInviteLink("user123")
	if actual != expected {
		t.Errorf("Expected %s, but got %s", expected, actual)
	}
}
