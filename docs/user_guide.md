<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# User Guide: OHC Flutter App

## 1. Overview

**One Human Corp** is the Hybrid Agentic OS — a cross-platform Flutter application backed by a Go server that lets a single human run an entire AI-powered company. It covers the full lifecycle from onboarding through daily operations:

| Category | Screens |
|---|---|
| **Onboarding** | Landing, Login, Business Setup Wizard, Setup Wizard |
| **Agent Management** | Agents list, Agent Hire Wizard, Prompt Tuning Wizard, Fix-This Wizard |
| **Collaboration** | Meetings, Chat, Channels, Handoffs |
| **AI & Skills** | AI Providers config, Skills catalogue |
| **Operations** | Dashboard, Logs, Security, Service Management, Diagnostics |
| **Orchestration** | Shared Task List, Swarm Memory, Pipelines |
| **Growth** | Referrals, Growth Experiments |
| **Cost & Scaling** | Cost Dashboard, Dynamic Scaling |
| **Integrations** | Integrations & Tools, User Management |
| **Commerce** | Upgrade Wizard, Billing Wizard |
| **Settings** | App Settings |

The app ships on **Web, Linux (.deb), macOS (.dmg), Windows (.exe), Android and iOS**. All six platform profiles are covered by the automated Playwright screenshot suite.

## 2. Regenerate Screenshots

Run either of the following from the repository root:

```bash
bazelisk run //srcs/app:capture_screenshots
```

Or use the VS Code task `App: Capture Flutter screenshots`.

Screenshots are written to `docs/public/assets/screenshots/app/<screen-name>/` with one PNG per platform profile (`web`, `linux`, `windows`, `macos`, `android`, `ios`).

## 3. Run E2E Tests

```bash
bazelisk test //srcs/app:flutter_web_e2e_test
```

The Playwright suite in `srcs/app/e2e/web.spec.ts` covers **76 test cases** across all 31 screens, including authentication flows, form interactions, navigation, chaos scenarios (high-latency, network-partition), and a performance baseline.

## 4. Screenshot Gallery

### Landing Page

![OHC Landing Page – Web](../public/assets/screenshots/app/landing-page/web.png)

### Login

![OHC Login – Web](../public/assets/screenshots/app/login/web.png)

### Dashboard

![OHC Dashboard – Web](../public/assets/screenshots/app/dashboard/web.png)

### Agents

![OHC Agents – Web](../public/assets/screenshots/app/agents/web.png)

### Agent Hire Wizard

![Agent Hire Wizard – Web](../public/assets/screenshots/app/agent-hire-wizard/web.png)

### Prompt Tuning Wizard

![Prompt Tuning Wizard – Web](../public/assets/screenshots/app/prompt-tuning-wizard/web.png)

### Meetings

![Meetings – Web](../public/assets/screenshots/app/meetings/web.png)

### Chat

![Chat – Web](../public/assets/screenshots/app/chat/web.png)

### Channels

![Channels – Web](../public/assets/screenshots/app/channels/web.png)

### AI Providers

![AI Providers – Web](../public/assets/screenshots/app/ai-providers/web.png)

### Skills

![Skills – Web](../public/assets/screenshots/app/skills/web.png)

### Logs

![Logs – Web](../public/assets/screenshots/app/logs/web.png)

### Security

![Security – Web](../public/assets/screenshots/app/security/web.png)

### Settings

![Settings – Web](../public/assets/screenshots/app/settings/web.png)

### Service Management

![Service Management – Web](../public/assets/screenshots/app/service-management/web.png)

### Setup Wizard

![Setup Wizard – Web](../public/assets/screenshots/app/setup-wizard/web.png)

### Diagnostics

![Diagnostics – Web](../public/assets/screenshots/app/diagnostics/web.png)

### Business Setup Wizard

![Business Setup Wizard – Web](../public/assets/screenshots/app/business-setup-wizard/web.png)

### Handoffs

![Handoffs – Web](../public/assets/screenshots/app/handoffs/web.png)

### Cost Dashboard

![Cost Dashboard – Web](../public/assets/screenshots/app/cost-dashboard/web.png)

### Dynamic Scaling

![Dynamic Scaling – Web](../public/assets/screenshots/app/dynamic-scaling/web.png)

### Pipelines

![Pipelines – Web](../public/assets/screenshots/app/pipelines/web.png)

### Integrations

![Integrations – Web](../public/assets/screenshots/app/integrations/web.png)

### User Management

![User Management – Web](../public/assets/screenshots/app/user-management/web.png)

### Fix-This Wizard

![Fix-This Wizard – Web](../public/assets/screenshots/app/fix-wizard/web.png)

### Upgrade Wizard

![Upgrade Wizard – Web](../public/assets/screenshots/app/upgrade-wizard/web.png)

### Billing Wizard

![Billing Wizard – Web](../public/assets/screenshots/app/billing-wizard/web.png)

### Task List (Orchestration)

![Task List – Web](../public/assets/screenshots/app/task-list/web.png)

### Swarm Memory

![Swarm Memory – Web](../public/assets/screenshots/app/swarm-memory/web.png)

### Growth Experiments

![Growth Experiments – Web](../public/assets/screenshots/app/growth-experiments/web.png)

### Referrals

![Referrals – Web](../public/assets/screenshots/app/referrals/web.png)

</div>
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

## 5. Documentation

Please refer to the detailed architecture documents in the `docs/` folder:
- [KAIROS Orchestration Design Phase 4](./kairos_orchestration_phase4.md)

</div>
