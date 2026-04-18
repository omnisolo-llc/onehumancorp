1. **Define `RoleAINewsCollector`**: Edit `srcs/server/domain/organization.go` to add `RoleAINewsCollector Role = "AI_NEWS_COLLECTOR"` in the `const` block for roles, probably near `RoleSoftwareEngineer` or marketing roles.
2. **Add to default role profiles**: Edit `srcs/server/domain/organization.go` in `defaultSoftwareCompanyRoleProfiles` to include the `RoleProfile` for `RoleAINewsCollector`.
   - `Role: RoleAINewsCollector`
   - `BasePrompt: "Scrape and summarize the latest AI industry news and trends."` (or similar)
   - `Capabilities: []string{"Scrape web sources", "Summarize articles", "Identify trends"}`
   - `ContextInputs: []string{"news feeds", "industry blogs", "social media"}`
   (Also check if `AI_NEWS_COLLECTOR` should be added to `NewSoftwareCompany`'s default `members` slice)
3. **Verify `srcs/server/dashboard/seeder.go` and `srcs/server/main.go`**: Make sure the seeders align with the role definition (which they seem to do, using string literal "AI_NEWS_COLLECTOR").
4. **Testing, verification, review, and reflection**: Run `bazelisk test //...` and `flutter test` in `srcs/app/` to ensure everything is working correctly.
