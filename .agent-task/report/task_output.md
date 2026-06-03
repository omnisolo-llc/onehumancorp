issue_title: "Integrate Gusto Embedded Payroll for Zero-Touch SMB HR & Team Management"
issue_description: |
  As small businesses grow from solopreneurs (like Carlos the Handyman) to employing a small team, payroll becomes a significant, high-stress barrier. Managing W-2s, 1099 contractors, tax withholdings, and compliance across state lines is terrifying for non-technical, non-accountant owners. Existing platforms force the user to leave the ecosystem, purchase expensive standalone HR software (like ADP or standalone Gusto), and manually sync hours from their booking/scheduling tools. OHC users need an invisible, zero-touch payroll engine where employee hours logged in the OHC Operations Agent automatically trigger compliant payouts and tax filings without manual spreadsheets.

  **Research Report**
  - **Strategy**: Deep, native API integration with Gusto Embedded Payroll.
  - **Target Persona**: Carlos (The Freelance Handyman expanding his crew), Fatima (Food Cart Operator hiring part-time help), Priya (The Boutique Owner paying shop assistants).
  - **Advantages**: Zero Compliance Risk, Seamless Workflow, Contractor Support.
  - **Competitor Landscape**: Shopify does not natively offer payroll; it relies on apps. Square offers payroll, but it is heavily tied to their POS hardware.
  - **Evaluation of Tool (Gusto Embedded API)**: Offers an embedded payroll API specifically designed for SaaS platforms. Supports both W-2 employees and 1099 contractors.

  **Design Doc**
  - **Integration with OHC**:
      - **Onboarding Flow**: The AI Agent guides the owner through connecting a bank account and setting up company tax details.
      - **Time Tracking & Sync**: Operations Agent logs billable hours and Finance Agent pushes them to the Gusto API.
      - **Payroll Execution**: Owner approves a plain-language summary to process direct deposits and handle tax withholdings.

  **Implementation Prompt**
  Build a native integration with the Gusto Embedded Payroll API. Implement the OAuth and company onboarding flow using Gusto's pre-built embedded components. Create data models to sync OHC Team Members with Gusto Employees/Contractors. Develop a background job to aggregate hours tracked in OHC's operational tools and sync them to Gusto payrolls. Build the "Run Payroll" UI flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
