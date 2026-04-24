1. **Analyze Environment Configuration Usage**
   - The user requires replacing all occurrences of `os.Getenv("OHC_STANDALONE")` checks with `!envBoolDefault("OHC_MULTITENANT", true)`.
   - Wait, looking closely at how `OHC_STANDALONE` was used previously, it was generally checked if it equals `"true"`. Since Standalone mode implies *not* multitenant, and the system architecture dictates using `OHC_MULTITENANT`, `os.Getenv("OHC_STANDALONE") == "true"` correctly becomes `!envBoolDefault("OHC_MULTITENANT", true)`.
   - Also, `os.Getenv("OHC_STANDALONE") != "true"` becomes `envBoolDefault("OHC_MULTITENANT", true)`.

2. **Modify the AppConfig and Enforce Use of Config**
   - We should ideally modify `src/server/config/config.go` so `StandaloneMode` is initialized as `!v.GetBool("ohc_multitenant")`. However, this might break tests that set `OHC_STANDALONE`.
   - I already tried the automated approach and fixed all test failures. The main issue was missing imports for `strings` or `os` after `envBoolDefault` was injected.
   - Wait, since `envBoolDefault` is a function that checks the environment, I should ensure it's defined and imported correctly, or simply use `config.Get().StandaloneMode` wherever possible. But using `config.Get()` might create import cycles in some packages. This is why injecting `envBoolDefault` is safer and avoids cyclic dependencies.

3. **Apply the Changes Across the Repository**
   - Rerun the python script to apply the changes to the Go backend (`src/server/...`).
   - Run the python script to apply changes to the Dart frontend (`src/app/lib/services/settings_service.dart`) and launcher scripts (`src/app/standalone_linux_launcher.sh`, `src/app/test/standalone_e2e_test.sh`).

4. **Fix Compilation Errors**
   - Ensure `envBoolDefault` doesn't conflict across files in the same package (e.g. `blob_provider.go` and `secret_provider.go` in `mcp` package).
   - Ensure `strings` and `os` packages are correctly imported.

5. **Test and Verify**
   - Run `bazelisk test //...` to ensure all tests pass.

6. **Pre Commit Steps**
   - Ensure proper testing, verification, review, and reflection are done by calling `pre_commit_instructions`.

7. **Submit the PR**
   - Commit and submit the code.