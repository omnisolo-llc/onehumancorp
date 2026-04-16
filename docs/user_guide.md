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
![OHC Landing Page – Android](app/android/landing-page.png)
![OHC Landing Page – iOS](app/ios/landing-page.png)
![OHC Landing Page – Windows](app/windows/landing-page.png)
![OHC Landing Page – Linux](app/linux/landing-page.png)
![OHC Landing Page – macOS](app/macos/landing-page.png)

### Login

![OHC Login – Web](../public/assets/screenshots/app/login/web.png)
![OHC Login – Android](app/android/login.png)
![OHC Login – iOS](app/ios/login.png)
![OHC Login – Windows](app/windows/login.png)
![OHC Login – Linux](app/linux/login.png)
![OHC Login – macOS](app/macos/login.png)

### Dashboard

![OHC Dashboard – Web](../public/assets/screenshots/app/dashboard/web.png)
![OHC Dashboard – Android](app/android/dashboard.png)
![OHC Dashboard – iOS](app/ios/dashboard.png)
![OHC Dashboard – Windows](app/windows/dashboard.png)
![OHC Dashboard – Linux](app/linux/dashboard.png)
![OHC Dashboard – macOS](app/macos/dashboard.png)

### Agents

![OHC Agents – Web](../public/assets/screenshots/app/agents/web.png)
![OHC Agents – Android](app/android/agents.png)
![OHC Agents – iOS](app/ios/agents.png)
![OHC Agents – Windows](app/windows/agents.png)
![OHC Agents – Linux](app/linux/agents.png)
![OHC Agents – macOS](app/macos/agents.png)

### Agent Hire Wizard

![Agent Hire Wizard – Web](../public/assets/screenshots/app/agent-hire-wizard/web.png)
![Agent Hire Wizard – Android](app/android/agent-hire-wizard.png)
![Agent Hire Wizard – iOS](app/ios/agent-hire-wizard.png)
![Agent Hire Wizard – Windows](app/windows/agent-hire-wizard.png)
![Agent Hire Wizard – Linux](app/linux/agent-hire-wizard.png)
![Agent Hire Wizard – macOS](app/macos/agent-hire-wizard.png)

### Prompt Tuning Wizard

![Prompt Tuning Wizard – Web](../public/assets/screenshots/app/prompt-tuning-wizard/web.png)
![Prompt Tuning Wizard – Android](app/android/prompt-tuning-wizard.png)
![Prompt Tuning Wizard – iOS](app/ios/prompt-tuning-wizard.png)
![Prompt Tuning Wizard – Windows](app/windows/prompt-tuning-wizard.png)
![Prompt Tuning Wizard – Linux](app/linux/prompt-tuning-wizard.png)
![Prompt Tuning Wizard – macOS](app/macos/prompt-tuning-wizard.png)

### Meetings

![Meetings – Web](../public/assets/screenshots/app/meetings/web.png)
![Meetings – Android](app/android/meetings.png)
![Meetings – iOS](app/ios/meetings.png)
![Meetings – Windows](app/windows/meetings.png)
![Meetings – Linux](app/linux/meetings.png)
![Meetings – macOS](app/macos/meetings.png)

### Chat

![Chat – Web](../public/assets/screenshots/app/chat/web.png)
![Chat – Android](app/android/chat.png)
![Chat – iOS](app/ios/chat.png)
![Chat – Windows](app/windows/chat.png)
![Chat – Linux](app/linux/chat.png)
![Chat – macOS](app/macos/chat.png)

### Channels

![Channels – Web](../public/assets/screenshots/app/channels/web.png)
![Channels – Android](app/android/channels.png)
![Channels – iOS](app/ios/channels.png)
![Channels – Windows](app/windows/channels.png)
![Channels – Linux](app/linux/channels.png)
![Channels – macOS](app/macos/channels.png)

### AI Providers

![AI Providers – Web](../public/assets/screenshots/app/ai-providers/web.png)
![AI Providers – Android](app/android/ai-providers.png)
![AI Providers – iOS](app/ios/ai-providers.png)
![AI Providers – Windows](app/windows/ai-providers.png)
![AI Providers – Linux](app/linux/ai-providers.png)
![AI Providers – macOS](app/macos/ai-providers.png)

### Skills

![Skills – Web](../public/assets/screenshots/app/skills/web.png)
![Skills – Android](app/android/skills.png)
![Skills – iOS](app/ios/skills.png)
![Skills – Windows](app/windows/skills.png)
![Skills – Linux](app/linux/skills.png)
![Skills – macOS](app/macos/skills.png)

### Logs

![Logs – Web](../public/assets/screenshots/app/logs/web.png)
![Logs – Android](app/android/logs.png)
![Logs – iOS](app/ios/logs.png)
![Logs – Windows](app/windows/logs.png)
![Logs – Linux](app/linux/logs.png)
![Logs – macOS](app/macos/logs.png)

### Security

![Security – Web](../public/assets/screenshots/app/security/web.png)
![Security – Android](app/android/security.png)
![Security – iOS](app/ios/security.png)
![Security – Windows](app/windows/security.png)
![Security – Linux](app/linux/security.png)
![Security – macOS](app/macos/security.png)

### Settings

![Settings – Web](../public/assets/screenshots/app/settings/web.png)
![Settings – Android](app/android/settings.png)
![Settings – iOS](app/ios/settings.png)
![Settings – Windows](app/windows/settings.png)
![Settings – Linux](app/linux/settings.png)
![Settings – macOS](app/macos/settings.png)

### Service Management

![Service Management – Web](../public/assets/screenshots/app/service-management/web.png)
![Service Management – Android](app/android/service-management.png)
![Service Management – iOS](app/ios/service-management.png)
![Service Management – Windows](app/windows/service-management.png)
![Service Management – Linux](app/linux/service-management.png)
![Service Management – macOS](app/macos/service-management.png)

### Setup Wizard

![Setup Wizard – Web](../public/assets/screenshots/app/setup-wizard/web.png)
![Setup Wizard – Android](app/android/setup-wizard.png)
![Setup Wizard – iOS](app/ios/setup-wizard.png)
![Setup Wizard – Windows](app/windows/setup-wizard.png)
![Setup Wizard – Linux](app/linux/setup-wizard.png)
![Setup Wizard – macOS](app/macos/setup-wizard.png)

### Diagnostics

![Diagnostics – Web](../public/assets/screenshots/app/diagnostics/web.png)
![Diagnostics – Android](app/android/diagnostics.png)
![Diagnostics – iOS](app/ios/diagnostics.png)
![Diagnostics – Windows](app/windows/diagnostics.png)
![Diagnostics – Linux](app/linux/diagnostics.png)
![Diagnostics – macOS](app/macos/diagnostics.png)

### Business Setup Wizard

![Business Setup Wizard – Web](../public/assets/screenshots/app/business-setup-wizard/web.png)
![Business Setup Wizard – Android](app/android/business-setup-wizard.png)
![Business Setup Wizard – iOS](app/ios/business-setup-wizard.png)
![Business Setup Wizard – Windows](app/windows/business-setup-wizard.png)
![Business Setup Wizard – Linux](app/linux/business-setup-wizard.png)
![Business Setup Wizard – macOS](app/macos/business-setup-wizard.png)

### Handoffs

![Handoffs – Web](../public/assets/screenshots/app/handoffs/web.png)
![Handoffs – Android](app/android/handoffs.png)
![Handoffs – iOS](app/ios/handoffs.png)
![Handoffs – Windows](app/windows/handoffs.png)
![Handoffs – Linux](app/linux/handoffs.png)
![Handoffs – macOS](app/macos/handoffs.png)

### Cost Dashboard

![Cost Dashboard – Web](../public/assets/screenshots/app/cost-dashboard/web.png)
![Cost Dashboard – Android](app/android/cost-dashboard.png)
![Cost Dashboard – iOS](app/ios/cost-dashboard.png)
![Cost Dashboard – Windows](app/windows/cost-dashboard.png)
![Cost Dashboard – Linux](app/linux/cost-dashboard.png)
![Cost Dashboard – macOS](app/macos/cost-dashboard.png)

### Dynamic Scaling

![Dynamic Scaling – Web](../public/assets/screenshots/app/dynamic-scaling/web.png)
![Dynamic Scaling – Android](app/android/dynamic-scaling.png)
![Dynamic Scaling – iOS](app/ios/dynamic-scaling.png)
![Dynamic Scaling – Windows](app/windows/dynamic-scaling.png)
![Dynamic Scaling – Linux](app/linux/dynamic-scaling.png)
![Dynamic Scaling – macOS](app/macos/dynamic-scaling.png)

### Pipelines

![Pipelines – Web](../public/assets/screenshots/app/pipelines/web.png)
![Pipelines – Android](app/android/pipelines.png)
![Pipelines – iOS](app/ios/pipelines.png)
![Pipelines – Windows](app/windows/pipelines.png)
![Pipelines – Linux](app/linux/pipelines.png)
![Pipelines – macOS](app/macos/pipelines.png)

### Integrations

![Integrations – Web](../public/assets/screenshots/app/integrations/web.png)
![Integrations – Android](app/android/integrations.png)
![Integrations – iOS](app/ios/integrations.png)
![Integrations – Windows](app/windows/integrations.png)
![Integrations – Linux](app/linux/integrations.png)
![Integrations – macOS](app/macos/integrations.png)

### User Management

![User Management – Web](../public/assets/screenshots/app/user-management/web.png)
![User Management – Android](app/android/user-management.png)
![User Management – iOS](app/ios/user-management.png)
![User Management – Windows](app/windows/user-management.png)
![User Management – Linux](app/linux/user-management.png)
![User Management – macOS](app/macos/user-management.png)

### Fix-This Wizard

![Fix-This Wizard – Web](../public/assets/screenshots/app/fix-wizard/web.png)
![Fix-This Wizard – Android](app/android/fix-wizard.png)
![Fix-This Wizard – iOS](app/ios/fix-wizard.png)
![Fix-This Wizard – Windows](app/windows/fix-wizard.png)
![Fix-This Wizard – Linux](app/linux/fix-wizard.png)
![Fix-This Wizard – macOS](app/macos/fix-wizard.png)

### Upgrade Wizard

![Upgrade Wizard – Web](../public/assets/screenshots/app/upgrade-wizard/web.png)
![Upgrade Wizard – Android](app/android/upgrade-wizard.png)
![Upgrade Wizard – iOS](app/ios/upgrade-wizard.png)
![Upgrade Wizard – Windows](app/windows/upgrade-wizard.png)
![Upgrade Wizard – Linux](app/linux/upgrade-wizard.png)
![Upgrade Wizard – macOS](app/macos/upgrade-wizard.png)

### Billing Wizard

![Billing Wizard – Web](../public/assets/screenshots/app/billing-wizard/web.png)
![Billing Wizard – Android](app/android/billing-wizard.png)
![Billing Wizard – iOS](app/ios/billing-wizard.png)
![Billing Wizard – Windows](app/windows/billing-wizard.png)
![Billing Wizard – Linux](app/linux/billing-wizard.png)
![Billing Wizard – macOS](app/macos/billing-wizard.png)

### Task List (Orchestration)

![Task List – Web](../public/assets/screenshots/app/task-list/web.png)
![Task List – Android](app/android/task-list.png)
![Task List – iOS](app/ios/task-list.png)
![Task List – Windows](app/windows/task-list.png)
![Task List – Linux](app/linux/task-list.png)
![Task List – macOS](app/macos/task-list.png)

### Swarm Memory

![Swarm Memory – Web](../public/assets/screenshots/app/swarm-memory/web.png)
![Swarm Memory – Android](app/android/swarm-memory.png)
![Swarm Memory – iOS](app/ios/swarm-memory.png)
![Swarm Memory – Windows](app/windows/swarm-memory.png)
![Swarm Memory – Linux](app/linux/swarm-memory.png)
![Swarm Memory – macOS](app/macos/swarm-memory.png)

### Growth Experiments

![Growth Experiments – Web](../public/assets/screenshots/app/growth-experiments/web.png)
![Growth Experiments – Android](app/android/growth-experiments.png)
![Growth Experiments – iOS](app/ios/growth-experiments.png)
![Growth Experiments – Windows](app/windows/growth-experiments.png)
![Growth Experiments – Linux](app/linux/growth-experiments.png)
![Growth Experiments – macOS](app/macos/growth-experiments.png)

### Referrals

![Referrals – Web](../public/assets/screenshots/app/referrals/web.png)
![Referrals – Android](app/android/referrals.png)
![Referrals – iOS](app/ios/referrals.png)
![Referrals – Windows](app/windows/referrals.png)
![Referrals – Linux](app/linux/referrals.png)
![Referrals – macOS](app/macos/referrals.png)

</div>
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

## 5. Documentation

Please refer to the detailed architecture documents in the `docs/` folder:
- [KAIROS Orchestration Design Phase 4](./kairos_orchestration_phase4.md)

</div>
