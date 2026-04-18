package telemetry

import (
	"context"
	"log/slog"
)

// PIIRedactingHandler wraps an slog.Handler to redact PII from messages and attributes in multi-tenant environments.
type PIIRedactingHandler struct {
	handler slog.Handler
}

// NewPIIRedactingHandler creates a new PIIRedactingHandler.
func NewPIIRedactingHandler(h slog.Handler) *PIIRedactingHandler {
	return &PIIRedactingHandler{handler: h}
}

// Enabled delegates to the underlying handler.
func (h *PIIRedactingHandler) Enabled(ctx context.Context, level slog.Level) bool {
	return h.handler.Enabled(ctx, level)
}

// Handle redacts the message and attributes before passing to the underlying handler.
func (h *PIIRedactingHandler) Handle(ctx context.Context, r slog.Record) error {
	// Redact the message
	redactedMessage := RedactPII(r.Message)

	// Create a new record with the redacted message
	newRecord := slog.NewRecord(r.Time, r.Level, redactedMessage, r.PC)

	// Redact attributes
	r.Attrs(func(a slog.Attr) bool {
		newRecord.AddAttrs(redactAttr(a))
		return true
	})

	return h.handler.Handle(ctx, newRecord)
}

// WithAttrs delegates to the underlying handler after redacting the attributes.
func (h *PIIRedactingHandler) WithAttrs(attrs []slog.Attr) slog.Handler {
	redactedAttrs := make([]slog.Attr, len(attrs))
	for i, a := range attrs {
		redactedAttrs[i] = redactAttr(a)
	}
	return NewPIIRedactingHandler(h.handler.WithAttrs(redactedAttrs))
}

// WithGroup delegates to the underlying handler.
func (h *PIIRedactingHandler) WithGroup(name string) slog.Handler {
	return NewPIIRedactingHandler(h.handler.WithGroup(name))
}

func redactAttr(a slog.Attr) slog.Attr {
	// Let slog.Attr handle empty attributes
	if a.Equal(slog.Attr{}) {
		return a
	}

	if a.Value.Kind() == slog.KindGroup {
		// Handle empty groups
		if len(a.Value.Group()) == 0 {
			return a
		}

		attrs := a.Value.Group()
		var redactedAttrs []any
		for _, attr := range attrs {
			redactedAttrs = append(redactedAttrs, redactAttr(attr))
		}

		return slog.Group(a.Key, redactedAttrs...)
	}

	val := a.Value.Any()
	if val == nil {
		return a
	}
	redactedVal := RedactInterfacePII(val)
	return slog.Any(a.Key, redactedVal)
}
