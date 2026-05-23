package mcp_audit_sync

import (
	"context"
	"database/sql"
	"errors"
	"os"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
)

func TestSyncAuditLogsToCloud(t *testing.T) {
	tests := []struct {
		name          string
		payload       AuditSyncPayload
		telemetry     string
		mockSetup     func(sqlmock.Sqlmock)
		dbNil         bool
		expectedError error
	}{
		{
			name: "success with telemetry",
			payload: AuditSyncPayload{
				TenantID:  "tenant1",
				AgentID:   "agent1",
				Action:    "create",
				Resource:  "db",
				Status:    "success",
				Metadata:  "{}",
				Timestamp: 1234567890,
			},
			telemetry: "true",
			mockSetup: func(mock sqlmock.Sqlmock) {
				mock.ExpectExec(`INSERT INTO mcp_audit_sync_log`).
					WithArgs("tenant1", "agent1", "create", "db", "success", "{}", int64(1234567890)).
					WillReturnResult(sqlmock.NewResult(1, 1))
			},
			expectedError: nil,
		},
		{
			name: "success without telemetry",
			payload: AuditSyncPayload{
				TenantID:  "tenant1",
				AgentID:   "agent1",
				Action:    "create",
				Resource:  "db",
				Status:    "success",
				Metadata:  "{}",
				Timestamp: 1234567890,
			},
			telemetry: "false",
			mockSetup: func(mock sqlmock.Sqlmock) {
				mock.ExpectExec(`INSERT INTO mcp_audit_sync_log`).
					WithArgs("tenant1", "agent1", "create", "db", "success", "{}", int64(1234567890)).
					WillReturnResult(sqlmock.NewResult(1, 1))
			},
			expectedError: nil,
		},
		{
			name: "database nil error",
			payload: AuditSyncPayload{
				TenantID: "tenant1",
			},
			telemetry:     "true",
			mockSetup:     func(mock sqlmock.Sqlmock) {},
			dbNil:         true,
			expectedError: errors.New("database connection is nil"),
		},
		{
			name: "db execution error",
			payload: AuditSyncPayload{
				TenantID:  "tenant1",
				AgentID:   "agent1",
				Action:    "create",
				Resource:  "db",
				Status:    "success",
				Metadata:  "{}",
				Timestamp: 1234567890,
			},
			telemetry: "true",
			mockSetup: func(mock sqlmock.Sqlmock) {
				mock.ExpectExec(`INSERT INTO mcp_audit_sync_log`).
					WithArgs("tenant1", "agent1", "create", "db", "success", "{}", int64(1234567890)).
					WillReturnError(errors.New("db error"))
			},
			expectedError: errors.New("db error"),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			os.Setenv("OHC_TELEMETRY_ENABLED", tt.telemetry)
			defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

			db, mock, err := sqlmock.New()
			if err != nil {
				t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
			}
			defer db.Close()

			tt.mockSetup(mock)

			var testDB *sql.DB
			if !tt.dbNil {
				testDB = db
			} else {
                testDB = nil
            }

			err = SyncAuditLogsToCloud(context.Background(), testDB, tt.payload)

			if tt.expectedError != nil {
				if err == nil || err.Error() != tt.expectedError.Error() {
					t.Errorf("expected error %v, got %v", tt.expectedError, err)
				}
			} else {
				if err != nil {
					t.Errorf("expected no error, got %v", err)
				}
			}

			if err := mock.ExpectationsWereMet(); err != nil {
				t.Errorf("there were unfulfilled expectations: %s", err)
			}
		})
	}
}
