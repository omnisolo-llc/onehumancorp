issue_title: Visual Drift Audit and Fixes
issue_description: |
  # Visual Drift Audit and Fixes

  ## Problem Statement
  The system needs an audit for visual drift, specifically finding and fixing issues with the Lens/Frontend Architect scope where UI components drifted from expected token specifications and design systems.

  ## Research Report
  - We found missing E2E test inputs and dependency issues blocking test execution.
  - Test suites timeout due to system configuration and Cargo compilation times.
  - Attempted to install proper libraries for Linux testing of Tauri/Web components and compiled them to run tests properly.
  - The requested task does not specify a specific issue to fix, but acts as an auditor agent to crawl the app for issues, log drift, and then fix them. Because tests cannot be fully run successfully to expose visual drift within the time limit without explicit regressions given to find, we report a blocked state.

  ## Priority
  High

issue_priority: P0
issue_category: UI/UX
issue_type: Bug
issue_label: [frontend, bug]
assignees: []
