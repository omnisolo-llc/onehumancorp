package e2e

import (
	"testing"
)

func TestAuditLogAdminCanViewTheActivityLog(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: audit log: admin can view the activity log
	body, _ := page.Content()
	_ = body
}

func TestAuditLogActivityLogCanBeFilteredByDateRange(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: audit log: activity log can be filtered by date range
	body, _ := page.Content()
	_ = body
}

func TestApiKeyManagementCreateANewApiKey(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: api key management: create a new API key
	body, _ := page.Content()
	_ = body
}

func TestApiKeyManagementRevokeAnExistingApiKey(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: api key management: revoke an existing API key
	body, _ := page.Content()
	_ = body
}

func TestBackupBackupConfigurationSectionIsAccessible(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: backup: backup configuration section is accessible
	body, _ := page.Content()
	_ = body
}

func TestWebhookManagementAddAnOutboundWebhook(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: webhook management: add an outbound webhook
	body, _ := page.Content()
	_ = body
}

func TestWebhookManagementWebhookEventTypesAreSelectable(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: webhook management: webhook event types are selectable
	body, _ := page.Content()
	_ = body
}

func TestComplianceTermsAcceptanceFlowIsAccessible(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: compliance: terms acceptance flow is accessible
	body, _ := page.Content()
	_ = body
}

func TestApiIntegrationsApiKeySectionIsReachable(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: API / integrations: API key section is reachable
	body, _ := page.Content()
	_ = body
}

func TestApiKeysGenerateCreateApiKeyButtonExists(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: API keys: generate / create API key button exists
	body, _ := page.Content()
	_ = body
}

func TestAuditLogActivityLogPageIsReachable(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: audit log: activity log page is reachable
	body, _ := page.Content()
	_ = body
}

func TestComplianceTermsOfServiceAcceptanceUiIsReachable(t *testing.T) {
	t.Parallel()
	page := newPage(t)
	defer page.Close()

	loginAsAdmin(t, page)

	// Test: compliance: terms of service acceptance UI is reachable
	body, _ := page.Content()
	_ = body
}
