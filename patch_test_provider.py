with open("srcs/server/db/test_provider_test.go", "r") as f:
    content = f.read()

content = content.replace("""// FetchPendingSyncs retrieves records from the local DB that need syncing
func (m *MockProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return nil, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (m *MockProvider) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (m *MockProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}""", "")

with open("srcs/server/db/test_provider_test.go", "w") as f:
    f.write(content)
