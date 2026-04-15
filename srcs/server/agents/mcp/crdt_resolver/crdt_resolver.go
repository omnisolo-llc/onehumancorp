package crdt_resolver

import (
	"context"
	"encoding/json"
	"fmt"
)

type CRDTResolver struct{}

func NewCRDTResolver() *CRDTResolver {
	return &CRDTResolver{}
}

func (r *CRDTResolver) Name() string {
	return "resolve_conflict"
}

// CRDTMergeInput defines the input structure for conflict resolution.
type CRDTMergeInput struct {
	ObjectA map[string]interface{} `json:"object_a"`
	ObjectB map[string]interface{} `json:"object_b"`
}

func (r *CRDTResolver) Execute(ctx context.Context, input []byte) ([]byte, error) {
	var mergeInput CRDTMergeInput
	if err := json.Unmarshal(input, &mergeInput); err != nil {
		return nil, fmt.Errorf("failed to parse input: %w", err)
	}

	merged := r.mergeObjects(mergeInput.ObjectA, mergeInput.ObjectB)

	return json.Marshal(merged)
}

func (r *CRDTResolver) mergeObjects(a, b map[string]interface{}) map[string]interface{} {
	merged := make(map[string]interface{})
	for k, v := range a {
		merged[k] = v
	}

	for k, v := range b {
		if existing, exists := merged[k]; exists {
			merged[k] = r.mergeValues(existing, v)
		} else {
			merged[k] = v
		}
	}
	return merged
}

func (r *CRDTResolver) mergeValues(a, b interface{}) interface{} {
	// If both are maps, recursively merge them.
	mapA, isMapA := a.(map[string]interface{})
	mapB, isMapB := b.(map[string]interface{})
	if isMapA && isMapB {
		return r.mergeObjects(mapA, mapB)
	}

	// If both are slices, merge them using array union.
	sliceA, isSliceA := a.([]interface{})
	sliceB, isSliceB := b.([]interface{})
	if isSliceA && isSliceB {
		return r.mergeSlices(sliceA, sliceB)
	}

	// Last-writer-wins for simple types, prioritizing 'b' over 'a' for simplicity in this example.
	// In a real CRDT, this would involve timestamps or version vectors.
	return b
}

func (r *CRDTResolver) mergeSlices(a, b []interface{}) []interface{} {
	merged := make([]interface{}, 0, len(a)+len(b))
	seen := make(map[string]bool)

	for _, v := range a {
		// Use JSON encoding for consistent map keys for arbitrary structures
		keyBytes, _ := json.Marshal(v)
		key := string(keyBytes)
		if !seen[key] {
			merged = append(merged, v)
			seen[key] = true
		}
	}

	for _, v := range b {
		keyBytes, _ := json.Marshal(v)
		key := string(keyBytes)
		if !seen[key] {
			merged = append(merged, v)
			seen[key] = true
		}
	}
	return merged
}
