package translation

import (
	"context"
	"errors"
	"testing"

	pb "github.com/onehumancorp/mono/src/proto/translation"
)

func TestTranslate_Success(t *testing.T) {
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
	server := NewTranslationServer(service)

	req := &pb.TranslateRequest{
		TenantId:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	}

	resp, err := server.Translate(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if resp.TranslatedText != "Hola" {
		t.Errorf("expected Hola, got %s", resp.TranslatedText)
	}
}

func TestTranslate_MissingArgs(t *testing.T) {
	mockDB := &MockDB{}
	service := NewTranslationMeshService(mockDB)
	server := NewTranslationServer(service)

	req := &pb.TranslateRequest{
		TenantId:   "tenant1",
		SourceText: "Hello",
	} // missing TargetLocale

	_, err := server.Translate(context.Background(), req)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestTranslate_ServiceError(t *testing.T) {
	mockDB := &MockDB{
		GetTranslationFunc: func(ctx context.Context, tenantID, sourceHash, targetLocale string) (*TranslationCache, error) {
			return nil, errors.New("db error")
		},
		InsertTranslationFunc: func(ctx context.Context, t *TranslationCache) error {
			return errors.New("insert error")
		},
	}

	service := NewTranslationMeshService(mockDB)
	server := NewTranslationServer(service)

	req := &pb.TranslateRequest{
		TenantId:     "tenant1",
		SourceText:   "Hello",
		TargetLocale: "es",
	}

	_, err := server.Translate(context.Background(), req)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}
