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

	// Check if it contains the required CSS
	if !strings.Contains(rendered, premiumCardCSS) {
		t.Errorf("Rendered document does not contain premium card CSS")
	}

	if !strings.Contains(rendered, premiumHeaderCSS) {
		t.Errorf("Rendered document does not contain premium header CSS")
	}

	// Check if content is present
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
	// Note: The diagram code is escaped now
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

	// Test that <script> is correctly escaped to &lt;script&gt; and onerror= is not rendered as literal
	// Because of html.EscapeString, <script> becomes &lt;script&gt;
	// We want to verify there are no literal unescaped tags.
	if strings.Contains(rendered, "<script>") {
		t.Errorf("Rendered document is vulnerable to XSS: %s", rendered)
	}
}
