<<<<<<< SEARCH
func TestSIPThrottling_Timeout(t *testing.T) {
	ctx := context.Background()
	// Disable environment throttling specifically for this timeout test
	// The mock setup handles timeouts at the test execution level
	os.Unsetenv("OHC_STANDALONE")
=======
func TestSIPThrottling_Timeout(t *testing.T) {
	// Disable environment throttling specifically for this timeout test
	// The mock setup handles timeouts at the test execution level
	os.Unsetenv("OHC_STANDALONE")
>>>>>>> REPLACE
<<<<<<< SEARCH
func TestSIPThrottling_NoTimeout(t *testing.T) {
	ctx := context.Background()

	// Enable environment throttling
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")
=======
func TestSIPThrottling_NoTimeout(t *testing.T) {

	// Enable environment throttling
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")
>>>>>>> REPLACE
