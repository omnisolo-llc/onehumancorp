package monitoring

import (
	"os"

	"go.opentelemetry.io/otel/attribute"
	"github.com/prometheus/client_golang/prometheus"
)

type ObservabilityTelemetryLayer struct {
	deploymentMode string
}

func NewObservabilityTelemetryLayer() *ObservabilityTelemetryLayer {
	mode := "standalone"
	if os.Getenv("OHC_HEADLESS") == "true" {
		mode = "headless"
	} else if os.Getenv("OHC_MULTITENANT") == "true" {
		mode = "cloud_native"
	}
	return &ObservabilityTelemetryLayer{deploymentMode: mode}
}

func (l *ObservabilityTelemetryLayer) AnnotateAttributes(attrs []attribute.KeyValue) []attribute.KeyValue {
	return append(attrs, attribute.String("deployment_mode", l.deploymentMode))
}

func (l *ObservabilityTelemetryLayer) WrapCounter(counter *prometheus.CounterVec) prometheus.Counter {
	return counter.WithLabelValues(l.deploymentMode)
}

func (l *ObservabilityTelemetryLayer) WrapHistogram(histogram *prometheus.HistogramVec) prometheus.Observer {
	return histogram.WithLabelValues(l.deploymentMode)
}
