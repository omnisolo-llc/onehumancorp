package pricing

import (
	"bytes"
	"compress/gzip"
	"encoding/base64"
	"io/ioutil"
	"strings"
)

// ReduceTokens performs a lossy reduction of text by removing stop words.
func ReduceTokens(text string) string {
	stopWords := []string{" the ", " a ", " an ", " in ", " on ", " at ", " to ", " for ", " with "}
	reduced := text
	for _, word := range stopWords {
		reduced = strings.ReplaceAll(reduced, word, " ")
	}
	return reduced
}

// CompressText performs lossless GZIP compression and returns a base64 encoded string.
func CompressText(text string) (string, error) {
	var b bytes.Buffer
	gz := gzip.NewWriter(&b)
	if _, err := gz.Write([]byte(text)); err != nil {
		return "", err
	}
	if err := gz.Close(); err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(b.Bytes()), nil
}

// DecompressText decompresses a base64 encoded GZIP string back to the original text.
func DecompressText(encoded string) (string, error) {
	decoded, err := base64.StdEncoding.DecodeString(encoded)
	if err != nil {
		return "", err
	}

	b := bytes.NewBuffer(decoded)
	gz, err := gzip.NewReader(b)
	if err != nil {
		return "", err
	}
	defer gz.Close()

	decompressed, err := ioutil.ReadAll(gz)
	if err != nil {
		return "", err
	}

	return string(decompressed), nil
}
