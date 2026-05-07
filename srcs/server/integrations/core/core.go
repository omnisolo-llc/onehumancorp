package core

type Metadata struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

type WizardStep struct {
	ID    string `json:"id"`
	Title string `json:"title"`
}

type Integration interface {
	Metadata() Metadata
	WizardSteps() []WizardStep
}

type TelemetryClient interface {
	BufferMetric(metricName string, metricType string, value float64, labels map[string]interface{}) error
}
