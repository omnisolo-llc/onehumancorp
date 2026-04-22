package monitoring

import (
	"testing"
	"github.com/stretchr/testify/assert"
	"go.opentelemetry.io/otel/attribute"
	"github.com/prometheus/client_golang/prometheus"
)

func TestObservabilityTelemetryLayer_AnnotateAttributes(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	layer := NewObservabilityTelemetryLayer()
	assert.Equal(t, "cloud_native", layer.deploymentMode)

	attrs := layer.AnnotateAttributes([]attribute.KeyValue{
		attribute.String("existing", "val"),
	})
	assert.Len(t, attrs, 2)
	assert.Equal(t, attribute.String("deployment_mode", "cloud_native"), attrs[1])
}

func TestObservabilityTelemetryLayer_WrapCounter(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	layer := NewObservabilityTelemetryLayer()

	counter := prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "test_counter",
		},
		[]string{"deployment_mode"},
	)

	wrapped := layer.WrapCounter(counter)
	wrapped.Inc()
}

func TestObservabilityTelemetryLayer_WrapHistogram(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	layer := NewObservabilityTelemetryLayer()

	histogram := prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name: "test_histogram",
		},
		[]string{"deployment_mode"},
	)

	wrapped := layer.WrapHistogram(histogram)
	wrapped.Observe(1.0)
}
