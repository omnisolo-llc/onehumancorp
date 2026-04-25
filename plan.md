1. **Update API Service**: Add `Future<String> askHelpAgent(String query) async` to `src/app/lib/services/api_service.dart`.
2. **Update Help Chat Widget**: Update `HelpChatWidget` to use `askHelpAgent` to get actual AI responses instead of the mock.
3. **Implement Walkthrough Flows**: Add walkthrough flows ("Set up your store", "Accept your first payment", "Activate your AI Support Agent") in `src/app/lib/widgets/help/walkthrough_registry.dart`. Trigger "Set up your store" on initial login or dashboard load.
4. **Update E2E Tests**: Update `help_center_test.go` to test hovering over contextual tooltips and completing a walkthrough flow.
5. **Run Tests**: Execute `bazelisk test //...`
6. **Code Review**: Get feedback and finalize code.
7. **Submit**: Commit and push.
