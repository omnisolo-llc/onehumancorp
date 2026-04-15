package docs

import (
	"fmt"
	"log/slog"
	"html"
	"strings"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter          = otel.Meter("github.com/onehumancorp/ohc/srcs/server/docs")
	docsGenerated  metric.Int64Counter
)

func init() {
	var err error
	docsGenerated, err = meter.Int64Counter("docs_generated_total", metric.WithDescription("Total number of premium design docs generated"))
	if err != nil {
		slog.Error("failed to initialize metrics", "err", err)
	}
}

const (
	premiumCardCSS = `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 16px; padding: 24px; font-family: 'Outfit', 'Inter', sans-serif; color: #e0e0e0;`
	premiumHeaderCSS = `font-family: 'Outfit', sans-serif; font-weight: 700; background: linear-gradient(90deg, #ffffff, #a0a0a0); -webkit-background-clip: text; -webkit-text-fill-color: transparent;`
)

type PremiumDocBuilder struct {
	title    string
	agentID  string
	date     string
	status   string
	sections []string
}

func NewPremiumDocBuilder() *PremiumDocBuilder {
	return &PremiumDocBuilder{
		date: time.Now().Format("2006-01-02"),
	}
}

func (b *PremiumDocBuilder) SetHeader(title, agentID, status string) {
	b.title = html.EscapeString(title)
	b.agentID = html.EscapeString(agentID)
	b.status = html.EscapeString(status)
}

func (b *PremiumDocBuilder) AddSection(title, content string) {
	safeTitle := html.EscapeString(title)
	safeContent := html.EscapeString(content)

	section := fmt.Sprintf(`<div style="%s">
<h2 style="%s">%s</h2>
%s
</div><br>`, premiumCardCSS, premiumHeaderCSS, safeTitle, safeContent)
	b.sections = append(b.sections, section)
}

func (b *PremiumDocBuilder) AddDiagram(mermaidCode string) {
	safeDiagram := html.EscapeString(mermaidCode)
	diagram := fmt.Sprintf(`<div style="%s">
<h2 style="%s">Architecture / Data Flow</h2>
<pre class="mermaid">
%s
</pre>
</div><br>`, premiumCardCSS, premiumHeaderCSS, safeDiagram)
	b.sections = append(b.sections, diagram)
}

func (b *PremiumDocBuilder) Render() string {
	var sb strings.Builder

	// Header
	sb.WriteString(fmt.Sprintf(`<div style="%s">
<h1 style="%s">%s</h1>
<p><strong>Author:</strong> %s | <strong>Date:</strong> %s | <strong>Status:</strong> %s</p>
</div><br>
`, premiumCardCSS, premiumHeaderCSS, html.EscapeString(b.title), html.EscapeString(b.agentID), html.EscapeString(b.date), html.EscapeString(b.status)))

	for _, section := range b.sections {
		sb.WriteString(section)
		sb.WriteString("\n")
	}

	if docsGenerated != nil {
		docsGenerated.Add(nil, 1)
	}

	return sb.String()
}

func NewArchitectureDocTemplate(title, agentID, summary, diagram, dataSchema, uiDetails string) string {
	b := NewPremiumDocBuilder()
	b.SetHeader(title, agentID, "DRAFT")
	b.AddSection("Executive Summary", summary)
	b.AddDiagram(diagram)
	b.AddSection("Database Schema / API Contracts", dataSchema)
	b.AddSection("Aesthetic / UI Considerations", uiDetails)
	return b.Render()
}
