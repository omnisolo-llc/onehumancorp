import re

with open("srcs/server/dashboard/server_test.go", "r") as f:
    content = f.read()

bad_test_search = """func TestMeshEndpoints(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider := setupTestDB(t)
	defer provider.Close()

	hubService := orchestration.NewHubService(provider)

	store := store.NewStore()
	svr := NewServer(provider, store, hubService)
	ts := httptest.NewServer(svr.Handler())
	defer ts.Close()"""

fixed_test = """func TestMeshEndpoints(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider := db.NewTestProvider(t)
	defer provider.Close()

	hubService := orchestration.NewHubService(provider)
	tracker := billing.NewTracker(provider)
	store := store.NewStore()
	svr := NewServer(provider, tracker, store, hubService)
	ts := httptest.NewServer(svr.Handler())
	defer ts.Close()"""

content = content.replace(bad_test_search, fixed_test)

with open("srcs/server/dashboard/server_test.go", "w") as f:
    f.write(content)
