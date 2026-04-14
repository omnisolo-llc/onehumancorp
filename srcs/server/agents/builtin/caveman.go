package builtin

import (
	"regexp"
	"strings"
)

// CavemanMode controls the verbosity of agent-to-agent communication.
// When communicating with users, Normal mode is always used.
// When agents communicate with each other (subagents), CavemanFull is the default,
// saving ~75% of output tokens while keeping full technical accuracy.
//
// Inspired by https://github.com/JuliusBrussee/caveman — "why use many token when few do trick".
type CavemanMode int

const (
	// CavemanOff — full, natural language.  Used for user-facing communication.
	CavemanOff CavemanMode = 0
	// CavemanLite — no filler/hedging.  Keeps articles and full sentences.
	CavemanLite CavemanMode = 1
	// CavemanFull — drops articles, uses fragments.  Default for agent↔agent.
	CavemanFull CavemanMode = 2
	// CavemanUltra — maximum compression.  Abbreviates, uses arrows for causality.
	CavemanUltra CavemanMode = 3
)

// cavemanSystemPrompt returns the system-prompt addendum that instructs the
// sub-agent to communicate using the requested caveman level.
// Empty string is returned for CavemanOff.
func cavemanSystemPrompt(mode CavemanMode) string {
	switch mode {
	case CavemanLite:
		return "\n\n[COMM-MODE: lite] No filler/hedging. Keep articles + full sentences. Professional but tight."
	case CavemanFull:
		return "\n\n[COMM-MODE: full] Terse caveman style. Drop articles. Fragments OK. Short synonyms. " +
			"Pattern: [thing] [action] [reason]. Technical terms exact. Code blocks unchanged."
	case CavemanUltra:
		return "\n\n[COMM-MODE: ultra] Max compression. Abbreviate (DB/auth/cfg/req/res/fn). " +
			"Strip conjunctions. Arrows for causality (X→Y). One word when one word enough."
	default:
		return ""
	}
}

// CavemanCompress applies caveman-style compression to a prompt or message.
// It is purely text-based (no ML), following the rules from the caveman SKILL:
//   - Drop: articles (a/an/the), filler (just/really/basically/actually/simply),
//     pleasantries (sure/certainly/of course/happy to), hedging
//   - Keep: technical terms, code blocks, error messages
//   - Fragments OK; short synonyms preferred
//
// For CavemanOff, the input is returned unchanged.
// For CavemanLite, only obvious filler/pleasantries are removed.
// For CavemanFull, full transformation including article removal.
// For CavemanUltra, additional abbreviations and causal arrows.
func CavemanCompress(text string, mode CavemanMode) string {
	if mode == CavemanOff || text == "" {
		return text
	}

	// Preserve code blocks — compress only prose.
	parts := splitCodeBlocks(text)
	for i, p := range parts {
		if p.isCode {
			continue
		}
		switch mode {
		case CavemanLite:
			p.text = applyLite(p.text)
		case CavemanFull:
			p.text = applyFull(p.text)
		case CavemanUltra:
			p.text = applyUltra(p.text)
		}
		parts[i] = p
	}
	return joinParts(parts)
}

// ─── lite rules ──────────────────────────────────────────────────────────────

var liteFillerRE = regexp.MustCompile(
	`(?i)\b(just|really|basically|actually|simply|certainly|of course|sure[,!]?|` +
		`happy to[^.]*?\.|I'd be happy to[^.]*?\.|Let me[^.]*?\.|` +
		`I'll help you[^.]*?\.|Great[,!]?|Absolutely[,!]?)\s*`,
)

func applyLite(s string) string {
	return strings.TrimSpace(liteFillerRE.ReplaceAllString(s, " "))
}

// ─── full rules ──────────────────────────────────────────────────────────────

var fullArticleRE = regexp.MustCompile(`(?i)\b(a|an|the)\s+`)
var fullFillerRE = regexp.MustCompile(
	`(?i)\b(just|really|basically|actually|simply|certainly|` +
		`of course|sure[,!]?|happy to|I would|I will|I'll|` +
		`you might want to|you should consider|` +
		`it is important to|please note that|` +
		`in order to|so that you can)\s*`,
)
var fullHedgeRE = regexp.MustCompile(
	`(?i)\b(I think|I believe|I suggest|I recommend|` +
		`it seems like|it appears that|it looks like|` +
		`might be|could be|probably|possibly|perhaps)\s+`,
)

func applyFull(s string) string {
	s = applyLite(s)
	s = fullFillerRE.ReplaceAllString(s, " ")
	s = fullHedgeRE.ReplaceAllString(s, "")
	s = fullArticleRE.ReplaceAllString(s, "")
	// Collapse multiple spaces.
	s = regexp.MustCompile(`\s{2,}`).ReplaceAllString(s, " ")
	return strings.TrimSpace(s)
}

// ─── ultra rules ─────────────────────────────────────────────────────────────

var ultraAbbrRE = regexp.MustCompile(
	`(?i)\b(database|authentication|configuration|` +
		`implementation|function|request|response|` +
		`because|therefore|which causes|which results in|` +
		`leading to|resulting in)\b`,
)

var ultraAbbrMap = map[string]string{
	"database":           "DB",
	"authentication":     "auth",
	"configuration":      "cfg",
	"implementation":     "impl",
	"function":           "fn",
	"request":            "req",
	"response":           "res",
	"because":            "→",
	"therefore":          "→",
	"which causes":       "→",
	"which results in":   "→",
	"leading to":         "→",
	"resulting in":       "→",
}

func applyUltra(s string) string {
	s = applyFull(s)
	s = ultraAbbrRE.ReplaceAllStringFunc(s, func(m string) string {
		if abbr, ok := ultraAbbrMap[strings.ToLower(m)]; ok {
			return abbr
		}
		return m
	})
	// Collapse multiple arrows.
	s = regexp.MustCompile(`(→\s*){2,}`).ReplaceAllString(s, "→")
	return strings.TrimSpace(s)
}

// ─── code-block splitter ─────────────────────────────────────────────────────

type textPart struct {
	text   string
	isCode bool
}

var codeBlockRE = regexp.MustCompile("(?s)```[a-zA-Z]*\n.*?```|`[^`\n]+`")

func splitCodeBlocks(text string) []textPart {
	var parts []textPart
	locs := codeBlockRE.FindAllStringIndex(text, -1)
	last := 0
	for _, loc := range locs {
		if loc[0] > last {
			parts = append(parts, textPart{text: text[last:loc[0]], isCode: false})
		}
		parts = append(parts, textPart{text: text[loc[0]:loc[1]], isCode: true})
		last = loc[1]
	}
	if last < len(text) {
		parts = append(parts, textPart{text: text[last:], isCode: false})
	}
	return parts
}

func joinParts(parts []textPart) string {
	var sb strings.Builder
	for _, p := range parts {
		sb.WriteString(p.text)
	}
	return sb.String()
}

// cavemanAgentSystemPrompt builds the complete system prompt for a sub-agent,
// injecting the caveman communication instructions.
func cavemanAgentSystemPrompt(basePrompt string, mode CavemanMode) string {
	return basePrompt + cavemanSystemPrompt(mode)
}
