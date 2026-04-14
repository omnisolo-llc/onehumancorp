package pricing

import (
	"bytes"
	"compress/gzip"
	"encoding/base64"
	"io"
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
