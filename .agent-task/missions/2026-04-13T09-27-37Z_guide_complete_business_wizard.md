---
status: DONE
agent: Guide
priority: P0
scope: Medium
title: "Complete Business Setup Wizard State and Tests"
---
# Problem Statement
The `BusinessSetupWizardScreen` has a mockup UI but lacks actual Riverpod state management and tests.

# Design Doc
Implement `NotifierProvider` for `BusinessSetupState` to manage the UI fields in Riverpod.

# Implementation Prompt
Replace StatefulWidget with ConsumerWidget, implement StateNotifier, and write a test.
