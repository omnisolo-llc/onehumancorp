package translation

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
)

type TranslationJobPayload struct {
	TenantID     string `json:"tenant_id"`
	SourceText   string `json:"source_text"`
	TargetLocale string `json:"target_locale"`
}

type TranslationDBInterface interface {
	MarkTranslationCompleted(ctx context.Context, tenantID, sourceHash, targetLocale, translatedText string) error
	MarkTranslationFailed(ctx context.Context, tenantID, sourceHash, targetLocale string) error
}

type LLMInterface interface {
	Translate(ctx context.Context, text, targetLocale string) (string, error)
}

type TranslationWorker struct {
	db  TranslationDBInterface
	llm LLMInterface
}

func NewTranslationWorker(db TranslationDBInterface, llm LLMInterface) *TranslationWorker {
	return &TranslationWorker{
		db:  db,
		llm: llm,
	}
}

func (w *TranslationWorker) ProcessJob(ctx context.Context, payload []byte) error {
	var job TranslationJobPayload
	if err := json.Unmarshal(payload, &job); err != nil {
		return fmt.Errorf("failed to unmarshal job payload: %w", err)
	}

	hash := sha256.Sum256([]byte(job.SourceText))
	sourceHash := hex.EncodeToString(hash[:])

	translatedText, err := w.llm.Translate(ctx, job.SourceText, job.TargetLocale)
	if err != nil {
		log.Printf("translation failed for %s to %s: %v", job.SourceText, job.TargetLocale, err)
		if markErr := w.db.MarkTranslationFailed(ctx, job.TenantID, sourceHash, job.TargetLocale); markErr != nil {
			log.Printf("failed to mark translation as failed in DB: %v", markErr)
		}
		return err
	}

	if err := w.db.MarkTranslationCompleted(ctx, job.TenantID, sourceHash, job.TargetLocale, translatedText); err != nil {
		return fmt.Errorf("failed to mark translation as completed in DB: %w", err)
	}

	log.Printf("successfully processed translation for tenant %s, locale %s", job.TenantID, job.TargetLocale)
	return nil
}
