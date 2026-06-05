package translation

import (
	"context"
	"encoding/json"
	"errors"
	"testing"
)

type MockDB struct {
	MarkCompletedFunc func(ctx context.Context, tenantID, sourceHash, targetLocale, translatedText string) error
	MarkFailedFunc    func(ctx context.Context, tenantID, sourceHash, targetLocale string) error
}

func (m *MockDB) MarkTranslationCompleted(ctx context.Context, tenantID, sourceHash, targetLocale, translatedText string) error {
	if m.MarkCompletedFunc != nil {
		return m.MarkCompletedFunc(ctx, tenantID, sourceHash, targetLocale, translatedText)
	}
	return nil
}

func (m *MockDB) MarkTranslationFailed(ctx context.Context, tenantID, sourceHash, targetLocale string) error {
	if m.MarkFailedFunc != nil {
		return m.MarkFailedFunc(ctx, tenantID, sourceHash, targetLocale)
	}
	return nil
}

type MockLLM struct {
	TranslateFunc func(ctx context.Context, text, targetLocale string) (string, error)
}

func (m *MockLLM) Translate(ctx context.Context, text, targetLocale string) (string, error) {
	if m.TranslateFunc != nil {
		return m.TranslateFunc(ctx, text, targetLocale)
	}
	return "Translated", nil
}

func TestProcessJob_Success(t *testing.T) {
	completedCalled := false
	mockDB := &MockDB{
		MarkCompletedFunc: func(ctx context.Context, tenantID, sourceHash, targetLocale, translatedText string) error {
			completedCalled = true
			if translatedText != "Hola" {
				t.Errorf("expected Hola, got %s", translatedText)
			}
			return nil
		},
	}
	mockLLM := &MockLLM{
		TranslateFunc: func(ctx context.Context, text, targetLocale string) (string, error) {
			return "Hola", nil
		},
	}

	worker := NewTranslationWorker(mockDB, mockLLM)
	job := TranslationJobPayload{
		TenantID:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	}
	payload, _ := json.Marshal(job)

	err := worker.ProcessJob(context.Background(), payload)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !completedCalled {
		t.Errorf("expected MarkTranslationCompleted to be called")
	}
}

func TestProcessJob_UnmarshalError(t *testing.T) {
	mockDB := &MockDB{}
	mockLLM := &MockLLM{}
	worker := NewTranslationWorker(mockDB, mockLLM)

	err := worker.ProcessJob(context.Background(), []byte("invalid json"))
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestProcessJob_LLMError(t *testing.T) {
	failedCalled := false
	mockDB := &MockDB{
		MarkFailedFunc: func(ctx context.Context, tenantID, sourceHash, targetLocale string) error {
			failedCalled = true
			return nil
		},
	}
	mockLLM := &MockLLM{
		TranslateFunc: func(ctx context.Context, text, targetLocale string) (string, error) {
			return "", errors.New("llm error")
		},
	}

	worker := NewTranslationWorker(mockDB, mockLLM)
	job := TranslationJobPayload{
		TenantID:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	}
	payload, _ := json.Marshal(job)

	err := worker.ProcessJob(context.Background(), payload)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if !failedCalled {
		t.Errorf("expected MarkTranslationFailed to be called")
	}
}

func TestProcessJob_MarkFailedError(t *testing.T) {
	mockDB := &MockDB{
		MarkFailedFunc: func(ctx context.Context, tenantID, sourceHash, targetLocale string) error {
			return errors.New("db error")
		},
	}
	mockLLM := &MockLLM{
		TranslateFunc: func(ctx context.Context, text, targetLocale string) (string, error) {
			return "", errors.New("llm error")
		},
	}

	worker := NewTranslationWorker(mockDB, mockLLM)
	job := TranslationJobPayload{
		TenantID:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	}
	payload, _ := json.Marshal(job)

	err := worker.ProcessJob(context.Background(), payload)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestProcessJob_MarkCompletedError(t *testing.T) {
	mockDB := &MockDB{
		MarkCompletedFunc: func(ctx context.Context, tenantID, sourceHash, targetLocale, translatedText string) error {
			return errors.New("db error")
		},
	}
	mockLLM := &MockLLM{
		TranslateFunc: func(ctx context.Context, text, targetLocale string) (string, error) {
			return "Hola", nil
		},
	}

	worker := NewTranslationWorker(mockDB, mockLLM)
	job := TranslationJobPayload{
		TenantID:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	}
	payload, _ := json.Marshal(job)

	err := worker.ProcessJob(context.Background(), payload)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}
