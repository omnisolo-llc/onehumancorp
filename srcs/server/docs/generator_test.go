package docs

import (
	"strings"
	"testing"
)

func TestPremiumDocBuilder(t *testing.T) {
	b := NewPremiumDocBuilder()
	b.SetHeader("Test Title", "Agent-1", "DRAFT")
	b.AddSection("Test Section", "This is a test section.")
	b.AddDiagram("graph TD;\n    A-->B;")

	rendered := b.Render()

	if !strings.Contains(rendered, "Test Title") {
		t.Errorf("Rendered document does not contain title")
	}
	if !strings.Contains(rendered, "Agent-1") {
		t.Errorf("Rendered document does not contain agent ID")
	}
	if !strings.Contains(rendered, "Test Section") {
		t.Errorf("Rendered document does not contain section title")
	}
	if !strings.Contains(rendered, "This is a test section.") {
		t.Errorf("Rendered document does not contain section content")
	}
	if !strings.Contains(rendered, "```mermaid") {
		t.Errorf("Rendered document does not contain mermaid fence")
	}
	if !strings.Contains(rendered, "graph TD;\n    A--&gt;B;") {
		t.Errorf("Rendered document does not contain diagram code")
	}
}

func TestNewArchitectureDocTemplate(t *testing.T) {
	rendered := NewArchitectureDocTemplate(
		"Arch Title",
		"Agent-X",
		"Exec summary here",
		"graph TD;\n    X-->Y;",
		"Schema here",
		"UI details here",
	)

	// Check required sections are present
	if !strings.Contains(rendered, "Arch Title") {
		t.Errorf("Rendered template missing title")
	}
	if !strings.Contains(rendered, "Executive Summary") {
		t.Errorf("Rendered template missing Executive Summary header")
	}
	if !strings.Contains(rendered, "Exec summary here") {
		t.Errorf("Rendered template missing summary content")
	}
	if !strings.Contains(rendered, "Database Schema / API Contracts") {
		t.Errorf("Rendered template missing Database Schema header")
	}
}

func TestXSSPrevention(t *testing.T) {
	b := NewPremiumDocBuilder()
	b.SetHeader("<script>alert('title')</script>", "Agent-<script>", "DRAFT")
	b.AddSection("<script>alert('section')</script>", "<img src=x onerror=alert('content')>")
	b.AddDiagram("<script>alert('diagram')</script>")

	rendered := b.Render()

	if strings.Contains(rendered, "<script>") {
		t.Errorf("Rendered document is vulnerable to XSS: %s", rendered)
	}
	if strings.Contains(rendered, "<div") {
		t.Errorf("Rendered document should be markdown-only: %s", rendered)
	}
}
