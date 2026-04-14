package builtin

import (
	"strings"
	"testing"
)

func TestCavemanOff(t *testing.T) {
	input := "I would like to help you with that. The issue is basically that the database is failing."
	got := CavemanCompress(input, CavemanOff)
	if got != input {
		t.Errorf("CavemanOff should return input unchanged\ngot:  %q\nwant: %q", got, input)
	}
}

func TestCavemanOff_Empty(t *testing.T) {
	if got := CavemanCompress("", CavemanFull); got != "" {
		t.Errorf("empty string should return empty, got %q", got)
	}
}

func TestCavemanLite_RemovesFiller(t *testing.T) {
	cases := []struct {
		input    string
		mustDrop []string
	}{
		{
			input:    "Sure! I'll help you with that.",
			mustDrop: []string{"Sure"},
		},
		{
			input:    "Basically the problem is in the auth layer.",
			mustDrop: []string{"Basically"},
		},
		{
			input:    "Actually this is just a configuration issue.",
			mustDrop: []string{"Actually", "just"},
		},
	}
	for _, tc := range cases {
		got := CavemanCompress(tc.input, CavemanLite)
		for _, drop := range tc.mustDrop {
			if strings.Contains(got, drop) {
				t.Errorf("CavemanLite should drop %q:\n  input: %q\n  got:   %q", drop, tc.input, got)
			}
		}
	}
}

func TestCavemanFull_DropsArticles(t *testing.T) {
	input := "The function reads the database and returns the result."
	got := CavemanCompress(input, CavemanFull)
	// Articles should be removed.
	words := strings.Fields(got)
	for _, w := range words {
		lw := strings.ToLower(strings.Trim(w, ".,;:!?"))
		if lw == "the" || lw == "a" || lw == "an" {
			t.Errorf("CavemanFull should drop article %q in: %q", w, got)
		}
	}
	// Technical content preserved.
	if !strings.Contains(got, "function") && !strings.Contains(got, "fn") {
		t.Errorf("technical term 'function' should be preserved: %q", got)
	}
}

func TestCavemanFull_RemovesHedging(t *testing.T) {
	input := "I think the issue might be in the authentication middleware."
	got := CavemanCompress(input, CavemanFull)
	if strings.Contains(got, "I think") {
		t.Errorf("should drop 'I think': %q", got)
	}
	if strings.Contains(got, "might be") {
		t.Errorf("should drop 'might be': %q", got)
	}
	if !strings.Contains(got, "authentication") || !strings.Contains(got, "middleware") {
		t.Errorf("technical terms must be preserved: %q", got)
	}
}

func TestCavemanUltra_Abbreviations(t *testing.T) {
	input := "The authentication fails because the database connection times out."
	got := CavemanCompress(input, CavemanUltra)
	// Should use abbreviations.
	if !strings.Contains(got, "auth") {
		t.Errorf("'authentication' should become 'auth': %q", got)
	}
	if !strings.Contains(got, "DB") {
		t.Errorf("'database' should become 'DB': %q", got)
	}
	// "because" → "→"
	if !strings.Contains(got, "→") {
		t.Errorf("'because' should become '→': %q", got)
	}
}

func TestCavemanPreservesCodeBlocks(t *testing.T) {
	input := "The function should be rewritten. ```go\nfunc doIt(db *Database) error {\n    return nil\n}\n``` This is the fix."
	got := CavemanCompress(input, CavemanFull)

	// Code block must be preserved verbatim.
	if !strings.Contains(got, "func doIt(db *Database) error {") {
		t.Errorf("code block must be preserved verbatim:\n  input: %q\n  got:   %q", input, got)
	}
	// But prose "The function" should be compressed.
	if strings.HasPrefix(got, "The function") {
		t.Errorf("prose should be compressed:\n  got: %q", got)
	}
}

func TestCavemanPreservesInlineCode(t *testing.T) {
	input := "Set the `database_url` configuration to the correct value."
	got := CavemanCompress(input, CavemanFull)
	if !strings.Contains(got, "`database_url`") {
		t.Errorf("inline code must be preserved: %q", got)
	}
}

func TestCavemanSystemPrompt(t *testing.T) {
	for _, mode := range []CavemanMode{CavemanOff, CavemanLite, CavemanFull, CavemanUltra} {
		prompt := cavemanSystemPrompt(mode)
		if mode == CavemanOff && prompt != "" {
			t.Errorf("CavemanOff should return empty prompt, got %q", prompt)
		}
		if mode != CavemanOff && prompt == "" {
			t.Errorf("mode %d should return non-empty prompt", mode)
		}
	}
}

func TestCavemanAgentSystemPrompt(t *testing.T) {
	base := "You are a helpful agent."
	prompt := cavemanAgentSystemPrompt(base, CavemanFull)
	if !strings.HasPrefix(prompt, base) {
		t.Errorf("should start with base prompt: %q", prompt)
	}
	if !strings.Contains(prompt, "COMM-MODE") {
		t.Errorf("should contain COMM-MODE instruction: %q", prompt)
	}
}

func TestCavemanAgentSystemPrompt_NormalMode(t *testing.T) {
	base := "You are a helpful agent."
	prompt := cavemanAgentSystemPrompt(base, CavemanOff)
	if prompt != base {
		t.Errorf("CavemanOff should not modify base prompt: got %q", prompt)
	}
}

func TestCavemanUltra_CollapseArrows(t *testing.T) {
	input := "This fails because because the connection drops."
	got := CavemanCompress(input, CavemanUltra)
	// Double "because" should not produce "→→".
	if strings.Contains(got, "→→") {
		t.Errorf("double arrows should be collapsed: %q", got)
	}
}

func TestCavemanFull_TechnicalAccuracy(t *testing.T) {
	// Technical terms must never be mangled.
	technicalTerms := []string{
		"SELECT", "INSERT", "UPDATE", "DELETE",
		"nil", "error", "goroutine", "channel",
		"HTTP/2", "gRPC", "protobuf",
	}
	for _, term := range technicalTerms {
		input := "The " + term + " operation failed."
		got := CavemanCompress(input, CavemanFull)
		if !strings.Contains(got, term) {
			t.Errorf("technical term %q must be preserved in: %q", term, got)
		}
	}
}

func TestCavemanModes_Progression(t *testing.T) {
	// Ultra should be shorter than full, which should be shorter than lite, which should be
	// shorter than original.
	input := "I would like to help you with that. The issue is basically that the database authentication is failing because the configuration is incorrect."

	original := len(input)
	lite := len(CavemanCompress(input, CavemanLite))
	full := len(CavemanCompress(input, CavemanFull))
	ultra := len(CavemanCompress(input, CavemanUltra))

	if lite >= original {
		t.Errorf("lite (%d) should be shorter than original (%d)", lite, original)
	}
	if full >= lite {
		t.Errorf("full (%d) should be shorter than lite (%d)", full, lite)
	}
	if ultra >= full {
		t.Errorf("ultra (%d) should be shorter than full (%d)", ultra, full)
	}
}
