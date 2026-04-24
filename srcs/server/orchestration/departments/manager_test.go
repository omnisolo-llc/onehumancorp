package departments

import (
	"testing"
)

func TestManager(t *testing.T) {
	manager := NewManager()

	mock := &mockDepartment{}
	manager.RegisterDepartment("Sales", mock)

	d, err := manager.GetDepartment("Sales")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if d == nil {
		t.Fatalf("expected department to not be nil")
	}

	_, err = manager.GetDepartment("Unknown")
	if err == nil {
		t.Fatalf("expected error for unknown department")
	}
}
