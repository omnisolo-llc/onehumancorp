1. **Create `GrowMyBusinessWizardScreen`**:
   - Create `src/app/lib/screens/grow_my_business_wizard_screen.dart` with a multi-step stepper or guided flow for the "Grow my business" wizard.
   - Steps should reflect actions to grow a business: e.g. "Add Products", "Connect Social Media", "Launch Email Campaign".

2. **Create `WebsiteBuilderWizardScreen`**:
   - Create `src/app/lib/screens/website_builder_wizard_screen.dart` to implement the 4-step website builder.
   - Steps: "Template Gallery", "Brand Colors & Logo", "Add your first product/service", "Connect Domain & Publish".

3. **Create `AI Agent Configuration Wizard`** (`agent_config_wizard_screen.dart` or expand `agent_hire_wizard_screen.dart`):
   - We need to implement the AI agent configuration wizard which allows users to configure an agent's capabilities without technical knowledge.

4. **Update `router.dart`**:
   - Add routes for `/wizards/grow`, `/wizards/website`, `/wizards/agent_config`, etc.

5. **Update Dashboard / Navigation**:
   - Add triggers to open these wizards from the `DashboardScreen` or side navigation as outlined in the issue. "Grow my business" from home dashboard, "Build My Website" to open the website builder wizard.

6. **Add Tests**:
   - Add E2E and widget tests for the new wizards ensuring 100% test coverage. E2E tests must follow the exact flow starting from the dashboard.

7. **Pre Commit Steps**:
   - Run `pre_commit_instructions` tool to complete pre-commit checks.
