package crdt_resolver

import (
	"context"
	"encoding/json"
	"reflect"
	"testing"
)

func TestCRDTResolver_Name(t *testing.T) {
	resolver := NewCRDTResolver()
	if name := resolver.Name(); name != "resolve_conflict" {
		t.Errorf("Expected name resolve_conflict, got %s", name)
	}
}

func TestCRDTResolver_Execute(t *testing.T) {
	resolver := NewCRDTResolver()

	tests := []struct {
		name    string
		input   CRDTMergeInput
		want    map[string]interface{}
		wantErr bool
	}{
		{
			name: "Merge separate keys",
			input: CRDTMergeInput{
				ObjectA: map[string]interface{}{"a": 1},
				ObjectB: map[string]interface{}{"b": 2},
			},
			want: map[string]interface{}{"a": float64(1), "b": float64(2)},
		},
		{
			name: "Last writer wins for scalar",
			input: CRDTMergeInput{
				ObjectA: map[string]interface{}{"a": 1},
				ObjectB: map[string]interface{}{"a": 2},
			},
			want: map[string]interface{}{"a": float64(2)},
		},
		{
			name: "Merge nested objects",
			input: CRDTMergeInput{
				ObjectA: map[string]interface{}{"nested": map[string]interface{}{"x": 1}},
				ObjectB: map[string]interface{}{"nested": map[string]interface{}{"y": 2}},
			},
			want: map[string]interface{}{"nested": map[string]interface{}{"x": float64(1), "y": float64(2)}},
		},
		{
			name: "Merge slices union",
			input: CRDTMergeInput{
				ObjectA: map[string]interface{}{"list": []interface{}{1, 2}},
				ObjectB: map[string]interface{}{"list": []interface{}{2, 3}},
			},
			want: map[string]interface{}{"list": []interface{}{float64(1), float64(2), float64(3)}},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			inputBytes, err := json.Marshal(tt.input)
			if err != nil {
				t.Fatalf("Failed to marshal input: %v", err)
			}

			gotBytes, err := resolver.Execute(context.Background(), inputBytes)
			if (err != nil) != tt.wantErr {
				t.Errorf("CRDTResolver.Execute() error = %v, wantErr %v", err, tt.wantErr)
				return
			}

			if !tt.wantErr {
				var got map[string]interface{}
				if err := json.Unmarshal(gotBytes, &got); err != nil {
					t.Fatalf("Failed to unmarshal output: %v", err)
				}
				if !reflect.DeepEqual(got, tt.want) {
					t.Errorf("CRDTResolver.Execute() = %v, want %v", got, tt.want)
				}
			}
		})
	}
}

func TestCRDTResolver_Execute_InvalidInput(t *testing.T) {
	resolver := NewCRDTResolver()
	_, err := resolver.Execute(context.Background(), []byte("invalid json"))
	if err == nil {
		t.Error("Expected error for invalid input, got nil")
	}
}
