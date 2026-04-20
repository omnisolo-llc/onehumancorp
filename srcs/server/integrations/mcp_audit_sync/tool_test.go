package mcp_audit_sync

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockProvider struct {
	db.Provider
	execErr error
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 1, m.execErr
}

func contextWithClaims(ctx context.Context, claims *auth.Claims) context.Context {
	return context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)
}

func TestAuditTool_SyncAuditLogsToCloud_NoAuth(t *testing.T) {
	tool := NewAuditTool(nil)
	err := tool.SyncAuditLogsToCloud(context.Background(), AuditSyncPayload{})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "unauthorized")
}

func TestAuditTool_SyncAuditLogsToCloud_MismatchTenant(t *testing.T) {
	tool := NewAuditTool(nil)
	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	err := tool.SyncAuditLogsToCloud(ctx, AuditSyncPayload{TenantID: "org2"})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "tenant ID mismatch")
}

func TestAuditTool_SyncAuditLogsToCloud_Success(t *testing.T) {
	provider := &mockProvider{}
	tool := NewAuditTool(provider)

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	err := tool.SyncAuditLogsToCloud(ctx, AuditSyncPayload{
		TenantID:  "org1",
		AgentID:   "agent1",
		Action:    "login",
		Resource:  "system",
		Status:    "success",
		Metadata:  "{}",
		Timestamp: time.Now().Unix(),
	})
	require.NoError(t, err)
}

func TestAuditTool_SyncAuditLogsToCloud_ExecError(t *testing.T) {
	provider := &mockProvider{execErr: os.ErrPermission}
	tool := NewAuditTool(provider)

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	err := tool.SyncAuditLogsToCloud(ctx, AuditSyncPayload{
		TenantID: "org1",
	})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to sync audit logs to cloud")
}
