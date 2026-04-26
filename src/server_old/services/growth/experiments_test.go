package growth

import "testing"

func TestExperimentManager(t *testing.T) {
	em := NewExperimentManager()
	em.AddExperiment("exp1", "Test", 1.0)

	variant := em.GetVariant("exp1", "user1")
	if variant != "treatment" {
		t.Errorf("expected treatment, got %s", variant)
	}

	em.AddExperiment("exp2", "Test2", 0.0)
	variant = em.GetVariant("exp2", "user1")
	if variant != "control" {
		t.Errorf("expected control, got %s", variant)
	}

	em.AddExperiment("exp3", "Test3", 0.5)

	// "exp3" + "user1" hashed will be deterministic
	var1 := em.GetVariant("exp3", "user1")
	var2 := em.GetVariant("exp3", "user1")

	if var1 != var2 {
		t.Errorf("expected deterministic variant, got %s and %s", var1, var2)
	}
}
