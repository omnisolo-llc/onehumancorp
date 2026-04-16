# Standalone Onboarding CUJ (Critical User Journey)

## Persona
- A Developer setting up OHC for the first time on a Mac/Linux machine (Desktop Mode).
- A Platform Engineer deploying the Helm chart on K8s (Cloud Mode).

## Journey: Developer (Desktop Mode)
1. **Goal**: Start the application immediately without providing external DB credentials.
2. **Action**: The developer starts the server with `OHC_MULTITENANT=false`.
3. **Result**: The wizard verifies local storage paths, automatically provisions the `~/.ohc` configuration directory, and allows the standalone process to boot successfully.

## Journey: Platform Engineer (Cloud Mode)
1. **Goal**: Ensure the deployment runs safely inside a K8s pod with expected environment injection.
2. **Action**: The engineer deploys the application with `OHC_MULTITENANT=true`.
3. **Result**: The wizard verifies the presence of K8s environment variables. If they are missing, it logs an actionable warning.
