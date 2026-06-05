package translation

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"time"

	"github.com/google/uuid"
)

type TranslationCache struct {
	ID             string    `json:"id" db:"id"`
	TenantID       string    `json:"tenant_id" db:"tenant_id"`
	SourceHash     string    `json:"source_hash" db:"source_hash"`
	SourceText     string    `json:"source_text" db:"source_text"`
	TargetLocale   string    `json:"target_locale" db:"target_locale"`
	TranslatedText *string   `json:"translated_text" db:"translated_text"`
	Status         string    `json:"status" db:"status"`
	CreatedAt      time.Time `json:"created_at" db:"created_at"`
	UpdatedAt      time.Time `json:"updated_at" db:"updated_at"`
}

type TranslationRequest struct {
	TenantID     string
	SourceText   string
	TargetLocale string
}

type TranslationResponse struct {
	TranslatedText string
	Status         string // "COMPLETED" or "PENDING"
}

type DBInterface interface {
	GetTranslation(ctx context.Context, tenantID, sourceHash, targetLocale string) (*TranslationCache, error)
	InsertTranslation(ctx context.Context, t *TranslationCache) error
	EnqueueTranslationJob(ctx context.Context, tenantID, sourceText, targetLocale string) error
}

type TranslationMeshService struct {
	db DBInterface
}

func NewTranslationMeshService(db DBInterface) *TranslationMeshService {
	return &TranslationMeshService{
		db: db,
	}
}

func (s *TranslationMeshService) GetTranslation(ctx context.Context, req TranslationRequest) (*TranslationResponse, error) {
	hash := sha256.Sum256([]byte(req.SourceText))
	sourceHash := hex.EncodeToString(hash[:])

	cached, err := s.db.GetTranslation(ctx, req.TenantID, sourceHash, req.TargetLocale)
	if err == nil && cached != nil {
		if cached.Status == "COMPLETED" && cached.TranslatedText != nil {
			return &TranslationResponse{
				TranslatedText: *cached.TranslatedText,
				Status:         "COMPLETED",
			}, nil
		}
		// Return pending if it's already in the cache but not completed
		return &TranslationResponse{
			TranslatedText: "", // Or maybe return SourceText as fallback for now
			Status:         cached.Status,
		}, nil
	}

	// Not found, we need to create it and enqueue a job
	newCache := &TranslationCache{
		ID:           uuid.New().String(),
		TenantID:     req.TenantID,
		SourceHash:   sourceHash,
		SourceText:   req.SourceText,
		TargetLocale: req.TargetLocale,
		Status:       "PENDING",
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}

	err = s.db.InsertTranslation(ctx, newCache)
	if err != nil {
		return nil, err
	}

	// Enqueue background job to translate
	err = s.db.EnqueueTranslationJob(ctx, req.TenantID, req.SourceText, req.TargetLocale)
	if err != nil {
		return nil, err
	}

	return &TranslationResponse{
		TranslatedText: "",
		Status:         "PENDING",
	}, nil
}
