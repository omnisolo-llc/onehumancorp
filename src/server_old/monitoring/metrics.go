package monitoring

import (
    "go.opentelemetry.io/otel/attribute"
)

type ObservabilityTelemetryLayer struct {
    deploymentMode string
}

func NewObservabilityTelemetryLayer(mode string) *ObservabilityTelemetryLayer {
    return &ObservabilityTelemetryLayer{deploymentMode: mode}
}

func (l *ObservabilityTelemetryLayer) AnnotateAttributes(attrs []attribute.KeyValue) []attribute.KeyValue {
    return append(attrs, attribute.String("deployment_mode", l.deploymentMode))
}
