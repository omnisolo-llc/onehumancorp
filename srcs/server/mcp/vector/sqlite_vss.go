package vector

import (
	"bytes"
	"database/sql"
	"encoding/binary"
	"fmt"
)

type sqliteVssProvider struct {
	db *sql.DB
}

func NewSQLiteVSSProvider(db *sql.DB) VectorStorageProvider {
	return &sqliteVssProvider{db: db}
}

// Float32ArrayToBytes converts a float32 array to a byte array
func Float32ArrayToBytes(arr []float32) ([]byte, error) {
	buf := new(bytes.Buffer)
	err := binary.Write(buf, binary.LittleEndian, arr)
	if err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func (p *sqliteVssProvider) Store(namespace, id string, vector []float32, metadata string) error {
	vecBytes, err := Float32ArrayToBytes(vector)
	if err != nil {
		return fmt.Errorf("failed to encode vector: %w", err)
	}

	tx, err := p.db.Begin()
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `
		INSERT INTO vectors (namespace, id, vector, metadata)
		VALUES (?, ?, ?, ?)
		ON CONFLICT(namespace, id) DO UPDATE SET
		vector=excluded.vector, metadata=excluded.metadata;
	`
	_, err = tx.Exec(query, namespace, id, vecBytes, metadata)
	if err != nil {
		return err
	}

	return tx.Commit()
}

func (p *sqliteVssProvider) Search(namespace string, queryVector []float32, topK int) ([]SearchResult, error) {
	vecBytes, err := Float32ArrayToBytes(queryVector)
	if err != nil {
		return nil, fmt.Errorf("failed to encode vector: %w", err)
	}

	query := `
		SELECT id, metadata, vss_distance_l2(vector, ?) as distance
		FROM vectors
		WHERE namespace = ?
		ORDER BY distance
		LIMIT ?
	`

	rows, err := p.db.Query(query, vecBytes, namespace, topK)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var res SearchResult
		if err := rows.Scan(&res.ID, &res.Metadata, &res.Distance); err != nil {
			return nil, err
		}
		results = append(results, res)
	}

	return results, rows.Err()
}
