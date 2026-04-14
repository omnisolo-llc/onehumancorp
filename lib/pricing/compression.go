package pricing

import (
	"bytes"
	"compress/gzip"
	"encoding/base64"
	"io"
	"strings"
)

const compressionPrefix = "gz_b64:"

// CompressLossless losslessly compresses a string using gzip and encodes to base64
func CompressLossless(data string) (string, error) {
	var b bytes.Buffer
	gz := gzip.NewWriter(&b)
	if _, err := gz.Write([]byte(data)); err != nil {
		return "", err
	}
	if err := gz.Close(); err != nil {
		return "", err
	}
	return compressionPrefix + base64.StdEncoding.EncodeToString(b.Bytes()), nil
}

// DecompressLossless losslessly decompresses a base64 encoded gzip string
func DecompressLossless(data string) (string, error) {
	if len(data) < len(compressionPrefix) || data[:len(compressionPrefix)] != compressionPrefix {
		// Not compressed with our prefix
		return data, nil
	}

	compressedData, err := base64.StdEncoding.DecodeString(data[len(compressionPrefix):])
	if err != nil {
		return "", err
	}
	b := bytes.NewReader(compressedData)
	gz, err := gzip.NewReader(b)
	if err != nil {
		return "", err
	}
	defer gz.Close()
	decompressed, err := io.ReadAll(gz)
	if err != nil {
		return "", err
	}
	return string(decompressed), nil
}

// ReduceTokens removes common English stop words to reduce the overall token size.
// This is a lossy operation and should not be combined with lossless compression
// if exact recovery is required.
func ReduceTokens(data string) string {
	stopWords := map[string]bool{
		"a": true, "an": true, "the": true, "is": true, "are": true,
		"and": true, "or": true, "but": true, "in": true, "on": true,
		"at": true, "to": true, "for": true, "with": true, "by": true,
		"about": true, "as": true, "of": true,
	}

	words := strings.Fields(data)
	var reduced []string

	for _, word := range words {
		// Clean the word for comparison (lowercase, keep punctuation separate if needed,
		// but simple lowercase works for basic stop words)
		cleanWord := strings.ToLower(word)
		if !stopWords[cleanWord] {
			reduced = append(reduced, word)
		}
	}

	return strings.Join(reduced, " ")
}

// TruncateByWordCount limits the data to a maximum number of words.
// This is a lossy operation intended for cost safety boundaries.
func TruncateByWordCount(data string, maxWords int) string {
	if maxWords <= 0 {
		return ""
	}
	words := strings.Fields(data)
	if len(words) <= maxWords {
		return data
	}
	return strings.Join(words[:maxWords], " ")
}
