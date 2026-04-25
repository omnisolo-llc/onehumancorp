package mcp_audit_sync

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
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

func TestAuditTool_ListTools(t *testing.T) {
	tool := NewAuditTool(nil)
	tools := tool.ListTools()

	assert.Len(t, tools, 1)
	assert.Equal(t, "sync_audit_logs_to_cloud", tools[0].Name)
}

func TestAuditTool_CallTool_Success(t *testing.T) {
	provider := &mockProvider{}
	tool := NewAuditTool(provider)

	ctx := contextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org1"})
	args := map[string]interface{}{
		"tenant_id": "org1",
		"agent_id":  "agent1",
		"action":    "test_action",
		"resource":  "test_resource",
		"status":    "success",
		"metadata":  "{}",
		"timestamp": float64(time.Now().Unix()),
	}

	res, err := tool.CallTool(ctx, "sync_audit_logs_to_cloud", args)
	require.NoError(t, err)

	resMap, ok := res.(map[string]interface{})
	require.True(t, ok)
	assert.Equal(t, "success", resMap["status"])
}

func TestAuditTool_CallTool_MissingArgs(t *testing.T) {
	tool := NewAuditTool(nil)

	args := map[string]interface{}{
		"tenant_id": "org1",
		// Missing other required args
	}

	_, err := tool.CallTool(context.Background(), "sync_audit_logs_to_cloud", args)
	require.Error(t, err)
	assert.Contains(t, err.Error(), "missing or invalid arguments")
}

func TestAuditTool_CallTool_UnknownTool(t *testing.T) {
	tool := NewAuditTool(nil)

	_, err := tool.CallTool(context.Background(), "unknown_tool", map[string]interface{}{})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "unknown tool: unknown_tool")
}
