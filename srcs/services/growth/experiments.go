package growth

import (
	"context"
	"crypto/sha256"
	"encoding/binary"
	"fmt"
	"sync"
)

type Experiment struct {
	Name     string
	Variants []string
	Weights  []int // Should sum to 100
}

type ExperimentManager struct {
	mu          sync.RWMutex
	experiments map[string]Experiment
}

func NewExperimentManager() *ExperimentManager {
	return &ExperimentManager{
		experiments: make(map[string]Experiment),
	}
}

func (m *ExperimentManager) AddExperiment(exp Experiment) error {
	sum := 0
	for _, w := range exp.Weights {
		sum += w
	}
	if sum != 100 {
		return fmt.Errorf("weights must sum to 100, got %d", sum)
	}
	if len(exp.Variants) != len(exp.Weights) {
		return fmt.Errorf("variants and weights length mismatch")
	}

	m.mu.Lock()
	defer m.mu.Unlock()
	m.experiments[exp.Name] = exp
	return nil
}

func (m *ExperimentManager) GetVariant(ctx context.Context, experimentName string, userID string) (string, error) {
	m.mu.RLock()
	exp, ok := m.experiments[experimentName]
	m.mu.RUnlock()

	if !ok {
		return "", fmt.Errorf("experiment %q not found", experimentName)
	}

	hash := sha256.Sum256([]byte(experimentName + userID))
	val := int(binary.BigEndian.Uint64(hash[:8]) % 100)

	sum := 0
	for i, w := range exp.Weights {
		sum += w
		if val < sum {
			return exp.Variants[i], nil
		}
	}

	// This should be unreachable given the validation in AddExperiment
	return exp.Variants[0], nil
}
