package orchestration

import (
	"bytes"
	"compress/gzip"
	"encoding/base64"
	"encoding/json"
	"io"
)

// compressDataStr compresses the given byte slice using gzip and encodes it to base64 string.
func compressDataStr(data []byte) (string, error) {
	var b bytes.Buffer
	w := gzip.NewWriter(&b)
	if _, err := w.Write(data); err != nil {
		return "", err
	}
	if err := w.Close(); err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(b.Bytes()), nil
}

// decompressDataStr decodes the base64 string and decompresses the underlying byte slice using gzip.
func decompressDataStr(base64Str string) ([]byte, error) {
	decodedBytes, err := base64.StdEncoding.DecodeString(base64Str)
	if err != nil {
		return nil, err
	}
	r, err := gzip.NewReader(bytes.NewReader(decodedBytes))
	if err != nil {
		return nil, err
	}
	defer r.Close()
	return io.ReadAll(r)
}

// compressJSON helper to compress the json payload
func compressJSON(dataJSON []byte) ([]byte, error) {
	compressedBase64, err := compressDataStr(dataJSON)
	if err != nil {
		return nil, err
	}
	wrapper := map[string]string{
		"_compressed_base64": compressedBase64,
	}
	return json.Marshal(wrapper)
}

// decompressJSON helper to optionally decompress the json payload
func decompressJSON(dataJSON []byte) ([]byte, error) {
	var wrapper struct {
		CompressedBase64 string `json:"_compressed_base64"`
	}
	if err := json.Unmarshal(dataJSON, &wrapper); err == nil && wrapper.CompressedBase64 != "" {
		decompressedBytes, err := decompressDataStr(wrapper.CompressedBase64)
		if err != nil {
			return nil, err
		}
		return decompressedBytes, nil
	}
	return dataJSON, nil
}
