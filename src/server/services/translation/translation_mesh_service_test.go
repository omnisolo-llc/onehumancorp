package translation

import (
	"context"
	"errors"
	"testing"
)

type MockDB struct {
	GetTranslationFunc        func(ctx context.Context, tenantID, sourceHash, targetLocale string) (*TranslationCache, error)
	InsertTranslationFunc     func(ctx context.Context, t *TranslationCache) error
	EnqueueTranslationJobFunc func(ctx context.Context, tenantID, sourceText, targetLocale string) error
}

func (m *MockDB) GetTranslation(ctx context.Context, tenantID, sourceHash, targetLocale string) (*TranslationCache, error) {
	if m.GetTranslationFunc != nil {
		return m.GetTranslationFunc(ctx, tenantID, sourceHash, targetLocale)
	}
	return nil, errors.New("not found")
}

func (m *MockDB) InsertTranslation(ctx context.Context, t *TranslationCache) error {
	if m.InsertTranslationFunc != nil {
		return m.InsertTranslationFunc(ctx, t)
	}
	return nil
}

func (m *MockDB) EnqueueTranslationJob(ctx context.Context, tenantID, sourceText, targetLocale string) error {
	if m.EnqueueTranslationJobFunc != nil {
		return m.EnqueueTranslationJobFunc(ctx, tenantID, sourceText, targetLocale)
	}
	return nil
}

func TestGetTranslation_Cached(t *testing.T) {
	translated := "Hola"
	mockDB := &MockDB{
		GetTranslationFunc: func(ctx context.Context, tenantID, sourceHash, targetLocale string) (*TranslationCache, error) {
			return &TranslationCache{
				Status:         "COMPLETED",
				TranslatedText: &translated,
			}, nil
		},
	}

	service := NewTranslationMeshService(mockDB)
	resp, err := service.GetTranslation(context.Background(), TranslationRequest{
		TenantID:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	})

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if resp.Status != "COMPLETED" {
		t.Errorf("expected COMPLETED, got %s", resp.Status)
	}
	if resp.TranslatedText != "Hola" {
		t.Errorf("expected Hola, got %s", resp.TranslatedText)
	}
}

func TestGetTranslation_NotCached(t *testing.T) {
	insertCalled := false
	enqueueCalled := false

	mockDB := &MockDB{
		GetTranslationFunc: func(ctx context.Context, tenantID, sourceHash, targetLocale string) (*TranslationCache, error) {
			return nil, errors.New("not found")
		},
		InsertTranslationFunc: func(ctx context.Context, cache *TranslationCache) error {
			insertCalled = true
			return nil
		},
		EnqueueTranslationJobFunc: func(ctx context.Context, tenantID, sourceText, targetLocale string) error {
			enqueueCalled = true
			return nil
		},
	}

	service := NewTranslationMeshService(mockDB)
	resp, err := service.GetTranslation(context.Background(), TranslationRequest{
		TenantID:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	})

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if resp.Status != "PENDING" {
		t.Errorf("expected PENDING, got %s", resp.Status)
	}
	if !insertCalled {
		t.Errorf("expected InsertTranslation to be called")
	}
	if !enqueueCalled {
		t.Errorf("expected EnqueueTranslationJob to be called")
	}
}
