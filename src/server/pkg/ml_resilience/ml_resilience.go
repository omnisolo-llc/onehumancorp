package ml_resilience

import (
	"errors"
	"time"
)

var (
	ErrTimeout           = errors.New("timeout")
	ErrRateLimited       = errors.New("rate limited")
	ErrMalformedResponse = errors.New("malformed response")
)

type DegradationHandler struct {
	MaxRetries uint32
}

func NewDegradationHandler(maxRetries uint32) *DegradationHandler {
	return &DegradationHandler{
		MaxRetries: maxRetries,
	}
}

func (h *DegradationHandler) ExecuteWithFallback(operation func() (interface{}, error), defaultVal interface{}) interface{} {
	var retries uint32 = 0
	for retries < h.MaxRetries {
		val, err := operation()
		if err == nil {
			return val
		}
		if err == ErrTimeout || err == ErrRateLimited {
			// Exponential backoff
			time.Sleep(time.Duration(100*(1<<retries)) * time.Millisecond)
			retries++
		} else if err == ErrMalformedResponse {
			retries++
		} else {
			break
		}
	}
	return defaultVal
}
