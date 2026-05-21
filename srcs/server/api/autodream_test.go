package api

import (
    "bytes"
    "encoding/json"
    "net/http"
    "net/http/httptest"
    "testing"
    "time"

    "github.com/DATA-DOG/go-sqlmock"
    "github.com/google/uuid"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestHandleQuery(t *testing.T) {
    mockDB, mock, err := sqlmock.New()
    if err != nil {
        t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
    }
    defer mockDB.Close()

    store := db.NewAutoDreamStore(mockDB)
    api := NewAutoDreamAPI(store)

    reqBody := QueryRequest{
        Limit:     5,
        Embedding: []float32{1.0, 2.0, 3.0},
    }
    b, _ := json.Marshal(reqBody)
    req, err := http.NewRequest("POST", "/query", bytes.NewBuffer(b))
    if err != nil {
        t.Fatal(err)
    }

    id := uuid.New()
    now := time.Now()

    rows := sqlmock.NewRows([]string{"id", "timestamp", "content", "embedding"}).
        AddRow(id, now, "content 1", "[1,2,3]")

    mock.ExpectQuery(`SELECT id, timestamp, content, embedding FROM autodream_findings ORDER BY embedding <=> \$1 LIMIT \$2`).
        WithArgs(sqlmock.AnyArg(), 5).
        WillReturnRows(rows)

    rr := httptest.NewRecorder()
    handler := http.HandlerFunc(api.HandleQuery)

    handler.ServeHTTP(rr, req)

    if status := rr.Code; status != http.StatusOK {
        t.Errorf("handler returned wrong status code: got %v want %v\nResponse: %s",
            status, http.StatusOK, rr.Body.String())
    }

    var findings []db.AutoDreamFinding
    if err := json.Unmarshal(rr.Body.Bytes(), &findings); err != nil {
        t.Errorf("error unmarshaling response body: %v", err)
    }

    if err := mock.ExpectationsWereMet(); err != nil {
        t.Errorf("there were unfulfilled expectations: %s", err)
    }
}
