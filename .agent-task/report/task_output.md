issue_title: "F11"
issue_description: |
  F11: Named full-journey tests only delegate to a smoke helper.

  Attempted to run `make test-e2e` and `make test-backend` but encountered execution timeouts (400+ seconds) indicating a blocked no-work finding due to trace limitations. The task requires database/full-journey validations which are currently blocked by the environment limitations.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
