package ml_resilience

import (
	"sync/atomic"
	"testing"
)

func TestDegradationHandlerSuccess(t *testing.T) {
	handler := NewDegradationHandler(3)
	result := handler.ExecuteWithFallback(func() (interface{}, error) {
		return "success", nil
	}, "fallback")
	if result != "success" {
		t.Errorf("Expected success, got %v", result)
	}
}

func TestDegradationHandlerFallback(t *testing.T) {
	handler := NewDegradationHandler(3)
	result := handler.ExecuteWithFallback(func() (interface{}, error) {
		return nil, ErrTimeout
	}, "fallback")
	if result != "fallback" {
		t.Errorf("Expected fallback, got %v", result)
	}
}

func TestDegradationHandlerRetry(t *testing.T) {
	handler := NewDegradationHandler(3)
	var attempts atomic.Uint32
	result := handler.ExecuteWithFallback(func() (interface{}, error) {
		count := attempts.Add(1)
		if count < 2 {
			return nil, ErrRateLimited
		}
		return "success after retry", nil
	}, "fallback")

	if result != "success after retry" {
		t.Errorf("Expected 'success after retry', got %v", result)
	}
	if attempts.Load() != 2 {
		t.Errorf("Expected 2 attempts, got %v", attempts.Load())
	}
}
