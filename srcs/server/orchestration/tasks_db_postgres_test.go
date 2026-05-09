package orchestration

import (
	"context"
	"testing"
	"github.com/stretchr/testify/require"
	"github.com/DATA-DOG/go-sqlmock"
)

func TestPostgresTaskStore_ReportMissionHandover(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStore(db)

	mock.ExpectExec(`UPDATE agent_missions`).
		WithArgs("First blocker", "m_handoff").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = store.ReportMissionHandover(context.Background(), "m_handoff", "First blocker")
	require.NoError(t, err)

	err = mock.ExpectationsWereMet()
	require.NoError(t, err)
}
