1. **Refactor `AgentHireWizardScreen`**: Use `run_in_bash_session` to write a python script that will overwrite `srcs/app/lib/screens/agent_hire_wizard_screen.dart` with a new 4-step wizard implementing the Agent Gallery (Card grid), Capability selection, Schedule / frequency, and Review & Activate steps.
2. **Verify `AgentHireWizardScreen` changes**: Use `run_in_bash_session` with `cat srcs/app/lib/screens/agent_hire_wizard_screen.dart` to verify that the file was correctly refactored.
3. **Update `AgentHireWizardScreen` Tests**: Use `run_in_bash_session` to write a python script that will overwrite `srcs/app/test/screens/agent_hire_wizard_screen_test.dart` to match the new 4-step wizard implementation and achieve 100% test coverage.
4. **Verify Test changes**: Use `run_in_bash_session` with `cat srcs/app/test/screens/agent_hire_wizard_screen_test.dart` to verify that the test file was correctly overwritten.
5. **Run tests**: Execute `bazelisk test //srcs/app/...` to verify that all frontend tests pass and cover the modified code.
6. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
7. **Submit the code**: Use the `submit` tool to finalize the task.
