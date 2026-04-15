package docs

import (
	"fmt"
	"html"
	"log/slog"
	"strings"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter         = otel.Meter("github.com/onehumancorp/mono/srcs/server/docs")
	docsGenerated metric.Int64Counter
)

func init() {
	var err error
	docsGenerated, err = meter.Int64Counter("docs_generated_total", metric.WithDescription("Total number of generated markdown docs"))
	if err != nil {
		slog.Error("failed to initialize metrics", "err", err)
	}
}

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

	section := fmt.Sprintf("## %s\n\n%s\n", safeTitle, safeContent)
	b.sections = append(b.sections, section)
}

func (b *PremiumDocBuilder) AddDiagram(mermaidCode string) {
	safeDiagram := html.EscapeString(mermaidCode)
	diagram := fmt.Sprintf("## Architecture / Data Flow\n\n```mermaid\n%s\n```\n", safeDiagram)
	b.sections = append(b.sections, diagram)
}

func (b *PremiumDocBuilder) Render() string {
	var sb strings.Builder

	sb.WriteString(fmt.Sprintf("# %s\n\n", html.EscapeString(b.title)))
	sb.WriteString(fmt.Sprintf("- Author: %s\n", html.EscapeString(b.agentID)))
	sb.WriteString(fmt.Sprintf("- Date: %s\n", html.EscapeString(b.date)))
	sb.WriteString(fmt.Sprintf("- Status: %s\n\n", html.EscapeString(b.status)))

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
