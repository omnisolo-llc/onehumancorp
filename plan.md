1. **Analyze Failure**: The CI checks failed in `bazel-test`. The specific test failure was in `//src/tests/e2e:e2e_wizard_test`.
```
--- FAIL: TestBusinessSetupWizardEndToEnd (41.36s)
    wizard_test.go:398: Did not find 'Start Business Setup', assuming wizard loaded directly
    wizard_test.go:404:
		Error Trace:	src/tests/e2e/wizard_test.go:404
		Error:      	Received unexpected error:
				playwright: timeout: Timeout 10000ms exceeded.
		Test:       	TestBusinessSetupWizardEndToEnd
		Messages:   	Wizard welcome text not found
```
Wait, `TestBusinessSetupWizardEndToEnd` failed. Let's examine `src/tests/e2e/wizard_test.go` line 398 and 404 to see what's wrong.
