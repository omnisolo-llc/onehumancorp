package telemetry

import (
	"context"
	"log/slog"
)

type PIIRedactingHandler struct {
	Handler slog.Handler
}

func NewPIIRedactingHandler(h slog.Handler) *PIIRedactingHandler {
	return &PIIRedactingHandler{Handler: h}
}

func (h *PIIRedactingHandler) Enabled(ctx context.Context, level slog.Level) bool {
	return h.Handler.Enabled(ctx, level)
}

func (h *PIIRedactingHandler) Handle(ctx context.Context, r slog.Record) error {
	redactedMsg := RedactPII(r.Message)
	newRecord := slog.NewRecord(r.Time, r.Level, redactedMsg, r.PC)

	r.Attrs(func(a slog.Attr) bool {
		newRecord.AddAttrs(redactAttr(a))
		return true
	})

	return h.Handler.Handle(ctx, newRecord)
}

func (h *PIIRedactingHandler) WithAttrs(attrs []slog.Attr) slog.Handler {
	redactedAttrs := make([]slog.Attr, len(attrs))
	for i, a := range attrs {
		redactedAttrs[i] = redactAttr(a)
	}
	return NewPIIRedactingHandler(h.Handler.WithAttrs(redactedAttrs))
}

func (h *PIIRedactingHandler) WithGroup(name string) slog.Handler {
	return NewPIIRedactingHandler(h.Handler.WithGroup(name))
}

func redactAttr(a slog.Attr) slog.Attr {
	// If value is a group, recurse
	if a.Value.Kind() == slog.KindGroup {
		attrs := a.Value.Group()
		redactedAttrs := make([]slog.Attr, len(attrs))
		for i, attr := range attrs {
			redactedAttrs[i] = redactAttr(attr)
		}
		return slog.Group(a.Key, anyToAnyArgs(redactedAttrs)...)
	}

	val := a.Value.Any()
	if str, ok := val.(string); ok {
		return slog.String(a.Key, RedactPII(str))
	}

	redactedVal := RedactInterfacePII(val)
	return slog.Any(a.Key, redactedVal)
}

func anyToAnyArgs(attrs []slog.Attr) []any {
	args := make([]any, len(attrs))
	for i, a := range attrs {
		args[i] = a
	}
	return args
}
