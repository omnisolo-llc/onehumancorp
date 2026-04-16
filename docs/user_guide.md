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

![Landing Page – Web](app/web/landing-page.png)

![Landing Page – Linux](app/linux/landing-page.png)

![Landing Page – macOS](app/macos/landing-page.png)

![Landing Page – Windows](app/windows/landing-page.png)

![Landing Page – Android](app/android/landing-page.png)

![Landing Page – iOS](app/ios/landing-page.png)

![Landing Page – Linux](../public/assets/screenshots/app/landing-page/linux.png)

![Landing Page – macOS](../public/assets/screenshots/app/landing-page/macos.png)

![Landing Page – Windows](../public/assets/screenshots/app/landing-page/windows.png)

![Landing Page – Android](../public/assets/screenshots/app/landing-page/android.png)

![Landing Page – iOS](../public/assets/screenshots/app/landing-page/ios.png)

![Landing Page – Linux](../public/assets/screenshots/app/landing-page/linux.png)

![Landing Page – macOS](../public/assets/screenshots/app/landing-page/macos.png)

![Landing Page – Windows](../public/assets/screenshots/app/landing-page/windows.png)

![Landing Page – Android](../public/assets/screenshots/app/landing-page/android.png)

![Landing Page – iOS](../public/assets/screenshots/app/landing-page/ios.png)

### Login

![Login – Web](app/web/login.png)

![Login – Linux](app/linux/login.png)

![Login – macOS](app/macos/login.png)

![Login – Windows](app/windows/login.png)

![Login – Android](app/android/login.png)

![Login – iOS](app/ios/login.png)

![Login – Linux](../public/assets/screenshots/app/login/linux.png)

![Login – macOS](../public/assets/screenshots/app/login/macos.png)

![Login – Windows](../public/assets/screenshots/app/login/windows.png)

![Login – Android](../public/assets/screenshots/app/login/android.png)

![Login – iOS](../public/assets/screenshots/app/login/ios.png)

![Login – Linux](../public/assets/screenshots/app/login/linux.png)

![Login – macOS](../public/assets/screenshots/app/login/macos.png)

![Login – Windows](../public/assets/screenshots/app/login/windows.png)

![Login – Android](../public/assets/screenshots/app/login/android.png)

![Login – iOS](../public/assets/screenshots/app/login/ios.png)

### Dashboard

![Dashboard – Web](app/web/dashboard.png)

![Dashboard – Linux](app/linux/dashboard.png)

![Dashboard – macOS](app/macos/dashboard.png)

![Dashboard – Windows](app/windows/dashboard.png)

![Dashboard – Android](app/android/dashboard.png)

![Dashboard – iOS](app/ios/dashboard.png)

![Dashboard – Linux](../public/assets/screenshots/app/dashboard/linux.png)

![Dashboard – macOS](../public/assets/screenshots/app/dashboard/macos.png)

![Dashboard – Windows](../public/assets/screenshots/app/dashboard/windows.png)

![Dashboard – Android](../public/assets/screenshots/app/dashboard/android.png)

![Dashboard – iOS](../public/assets/screenshots/app/dashboard/ios.png)

![Dashboard – Linux](../public/assets/screenshots/app/dashboard/linux.png)

![Dashboard – macOS](../public/assets/screenshots/app/dashboard/macos.png)

![Dashboard – Windows](../public/assets/screenshots/app/dashboard/windows.png)

![Dashboard – Android](../public/assets/screenshots/app/dashboard/android.png)

![Dashboard – iOS](../public/assets/screenshots/app/dashboard/ios.png)

### Agents

![Agents – Web](app/web/agents.png)

![Agents – Linux](app/linux/agents.png)

![Agents – macOS](app/macos/agents.png)

![Agents – Windows](app/windows/agents.png)

![Agents – Android](app/android/agents.png)

![Agents – iOS](app/ios/agents.png)

![Agents – Linux](../public/assets/screenshots/app/agents/linux.png)

![Agents – macOS](../public/assets/screenshots/app/agents/macos.png)

![Agents – Windows](../public/assets/screenshots/app/agents/windows.png)

![Agents – Android](../public/assets/screenshots/app/agents/android.png)

![Agents – iOS](../public/assets/screenshots/app/agents/ios.png)

![Agents – Linux](../public/assets/screenshots/app/agents/linux.png)

![Agents – macOS](../public/assets/screenshots/app/agents/macos.png)

![Agents – Windows](../public/assets/screenshots/app/agents/windows.png)

![Agents – Android](../public/assets/screenshots/app/agents/android.png)

![Agents – iOS](../public/assets/screenshots/app/agents/ios.png)

### Agent Hire Wizard

![Agent Hire Wizard – Web](app/web/agent-hire-wizard.png)

![Agent Hire Wizard – Linux](app/linux/agent-hire-wizard.png)

![Agent Hire Wizard – macOS](app/macos/agent-hire-wizard.png)

![Agent Hire Wizard – Windows](app/windows/agent-hire-wizard.png)

![Agent Hire Wizard – Android](app/android/agent-hire-wizard.png)

![Agent Hire Wizard – iOS](app/ios/agent-hire-wizard.png)

![Agent Hire Wizard – Linux](../public/assets/screenshots/app/agent-hire-wizard/linux.png)

![Agent Hire Wizard – macOS](../public/assets/screenshots/app/agent-hire-wizard/macos.png)

![Agent Hire Wizard – Windows](../public/assets/screenshots/app/agent-hire-wizard/windows.png)

![Agent Hire Wizard – Android](../public/assets/screenshots/app/agent-hire-wizard/android.png)

![Agent Hire Wizard – iOS](../public/assets/screenshots/app/agent-hire-wizard/ios.png)

![Agent Hire Wizard – Linux](../public/assets/screenshots/app/agent-hire-wizard/linux.png)

![Agent Hire Wizard – macOS](../public/assets/screenshots/app/agent-hire-wizard/macos.png)

![Agent Hire Wizard – Windows](../public/assets/screenshots/app/agent-hire-wizard/windows.png)

![Agent Hire Wizard – Android](../public/assets/screenshots/app/agent-hire-wizard/android.png)

![Agent Hire Wizard – iOS](../public/assets/screenshots/app/agent-hire-wizard/ios.png)

### Prompt Tuning Wizard

![Prompt Tuning Wizard – Web](app/web/prompt-tuning-wizard.png)

![Prompt Tuning Wizard – Linux](app/linux/prompt-tuning-wizard.png)

![Prompt Tuning Wizard – macOS](app/macos/prompt-tuning-wizard.png)

![Prompt Tuning Wizard – Windows](app/windows/prompt-tuning-wizard.png)

![Prompt Tuning Wizard – Android](app/android/prompt-tuning-wizard.png)

![Prompt Tuning Wizard – iOS](app/ios/prompt-tuning-wizard.png)

![Prompt Tuning Wizard – Linux](../public/assets/screenshots/app/prompt-tuning-wizard/linux.png)

![Prompt Tuning Wizard – macOS](../public/assets/screenshots/app/prompt-tuning-wizard/macos.png)

![Prompt Tuning Wizard – Windows](../public/assets/screenshots/app/prompt-tuning-wizard/windows.png)

![Prompt Tuning Wizard – Android](../public/assets/screenshots/app/prompt-tuning-wizard/android.png)

![Prompt Tuning Wizard – iOS](../public/assets/screenshots/app/prompt-tuning-wizard/ios.png)

![Prompt Tuning Wizard – Linux](../public/assets/screenshots/app/prompt-tuning-wizard/linux.png)

![Prompt Tuning Wizard – macOS](../public/assets/screenshots/app/prompt-tuning-wizard/macos.png)

![Prompt Tuning Wizard – Windows](../public/assets/screenshots/app/prompt-tuning-wizard/windows.png)

![Prompt Tuning Wizard – Android](../public/assets/screenshots/app/prompt-tuning-wizard/android.png)

![Prompt Tuning Wizard – iOS](../public/assets/screenshots/app/prompt-tuning-wizard/ios.png)

### Meetings

![Meetings – Web](app/web/meetings.png)

![Meetings – Linux](app/linux/meetings.png)

![Meetings – macOS](app/macos/meetings.png)

![Meetings – Windows](app/windows/meetings.png)

![Meetings – Android](app/android/meetings.png)

![Meetings – iOS](app/ios/meetings.png)

![Meetings – Linux](../public/assets/screenshots/app/meetings/linux.png)

![Meetings – macOS](../public/assets/screenshots/app/meetings/macos.png)

![Meetings – Windows](../public/assets/screenshots/app/meetings/windows.png)

![Meetings – Android](../public/assets/screenshots/app/meetings/android.png)

![Meetings – iOS](../public/assets/screenshots/app/meetings/ios.png)

![Meetings – Linux](../public/assets/screenshots/app/meetings/linux.png)

![Meetings – macOS](../public/assets/screenshots/app/meetings/macos.png)

![Meetings – Windows](../public/assets/screenshots/app/meetings/windows.png)

![Meetings – Android](../public/assets/screenshots/app/meetings/android.png)

![Meetings – iOS](../public/assets/screenshots/app/meetings/ios.png)

### Chat

![Chat – Web](app/web/chat.png)

![Chat – Linux](app/linux/chat.png)

![Chat – macOS](app/macos/chat.png)

![Chat – Windows](app/windows/chat.png)

![Chat – Android](app/android/chat.png)

![Chat – iOS](app/ios/chat.png)

![Chat – Linux](../public/assets/screenshots/app/chat/linux.png)

![Chat – macOS](../public/assets/screenshots/app/chat/macos.png)

![Chat – Windows](../public/assets/screenshots/app/chat/windows.png)

![Chat – Android](../public/assets/screenshots/app/chat/android.png)

![Chat – iOS](../public/assets/screenshots/app/chat/ios.png)

![Chat – Linux](../public/assets/screenshots/app/chat/linux.png)

![Chat – macOS](../public/assets/screenshots/app/chat/macos.png)

![Chat – Windows](../public/assets/screenshots/app/chat/windows.png)

![Chat – Android](../public/assets/screenshots/app/chat/android.png)

![Chat – iOS](../public/assets/screenshots/app/chat/ios.png)

### Channels

![Channels – Web](app/web/channels.png)

![Channels – Linux](app/linux/channels.png)

![Channels – macOS](app/macos/channels.png)

![Channels – Windows](app/windows/channels.png)

![Channels – Android](app/android/channels.png)

![Channels – iOS](app/ios/channels.png)

![Channels – Linux](../public/assets/screenshots/app/channels/linux.png)

![Channels – macOS](../public/assets/screenshots/app/channels/macos.png)

![Channels – Windows](../public/assets/screenshots/app/channels/windows.png)

![Channels – Android](../public/assets/screenshots/app/channels/android.png)

![Channels – iOS](../public/assets/screenshots/app/channels/ios.png)

![Channels – Linux](../public/assets/screenshots/app/channels/linux.png)

![Channels – macOS](../public/assets/screenshots/app/channels/macos.png)

![Channels – Windows](../public/assets/screenshots/app/channels/windows.png)

![Channels – Android](../public/assets/screenshots/app/channels/android.png)

![Channels – iOS](../public/assets/screenshots/app/channels/ios.png)

### AI Providers

![AI Providers – Web](app/web/ai-providers.png)

![AI Providers – Linux](app/linux/ai-providers.png)

![AI Providers – macOS](app/macos/ai-providers.png)

![AI Providers – Windows](app/windows/ai-providers.png)

![AI Providers – Android](app/android/ai-providers.png)

![AI Providers – iOS](app/ios/ai-providers.png)

![AI Providers – Linux](../public/assets/screenshots/app/ai-providers/linux.png)

![AI Providers – macOS](../public/assets/screenshots/app/ai-providers/macos.png)

![AI Providers – Windows](../public/assets/screenshots/app/ai-providers/windows.png)

![AI Providers – Android](../public/assets/screenshots/app/ai-providers/android.png)

![AI Providers – iOS](../public/assets/screenshots/app/ai-providers/ios.png)

![AI Providers – Linux](../public/assets/screenshots/app/ai-providers/linux.png)

![AI Providers – macOS](../public/assets/screenshots/app/ai-providers/macos.png)

![AI Providers – Windows](../public/assets/screenshots/app/ai-providers/windows.png)

![AI Providers – Android](../public/assets/screenshots/app/ai-providers/android.png)

![AI Providers – iOS](../public/assets/screenshots/app/ai-providers/ios.png)

### Skills

![Skills – Web](app/web/skills.png)

![Skills – Linux](app/linux/skills.png)

![Skills – macOS](app/macos/skills.png)

![Skills – Windows](app/windows/skills.png)

![Skills – Android](app/android/skills.png)

![Skills – iOS](app/ios/skills.png)

![Skills – Linux](../public/assets/screenshots/app/skills/linux.png)

![Skills – macOS](../public/assets/screenshots/app/skills/macos.png)

![Skills – Windows](../public/assets/screenshots/app/skills/windows.png)

![Skills – Android](../public/assets/screenshots/app/skills/android.png)

![Skills – iOS](../public/assets/screenshots/app/skills/ios.png)

![Skills – Linux](../public/assets/screenshots/app/skills/linux.png)

![Skills – macOS](../public/assets/screenshots/app/skills/macos.png)

![Skills – Windows](../public/assets/screenshots/app/skills/windows.png)

![Skills – Android](../public/assets/screenshots/app/skills/android.png)

![Skills – iOS](../public/assets/screenshots/app/skills/ios.png)

### Logs

![Logs – Web](app/web/logs.png)

![Logs – Linux](app/linux/logs.png)

![Logs – macOS](app/macos/logs.png)

![Logs – Windows](app/windows/logs.png)

![Logs – Android](app/android/logs.png)

![Logs – iOS](app/ios/logs.png)

![Logs – Linux](../public/assets/screenshots/app/logs/linux.png)

![Logs – macOS](../public/assets/screenshots/app/logs/macos.png)

![Logs – Windows](../public/assets/screenshots/app/logs/windows.png)

![Logs – Android](../public/assets/screenshots/app/logs/android.png)

![Logs – iOS](../public/assets/screenshots/app/logs/ios.png)

![Logs – Linux](../public/assets/screenshots/app/logs/linux.png)

![Logs – macOS](../public/assets/screenshots/app/logs/macos.png)

![Logs – Windows](../public/assets/screenshots/app/logs/windows.png)

![Logs – Android](../public/assets/screenshots/app/logs/android.png)

![Logs – iOS](../public/assets/screenshots/app/logs/ios.png)

### Security

![Security – Web](app/web/security.png)

![Security – Linux](app/linux/security.png)

![Security – macOS](app/macos/security.png)

![Security – Windows](app/windows/security.png)

![Security – Android](app/android/security.png)

![Security – iOS](app/ios/security.png)

![Security – Linux](../public/assets/screenshots/app/security/linux.png)

![Security – macOS](../public/assets/screenshots/app/security/macos.png)

![Security – Windows](../public/assets/screenshots/app/security/windows.png)

![Security – Android](../public/assets/screenshots/app/security/android.png)

![Security – iOS](../public/assets/screenshots/app/security/ios.png)

![Security – Linux](../public/assets/screenshots/app/security/linux.png)

![Security – macOS](../public/assets/screenshots/app/security/macos.png)

![Security – Windows](../public/assets/screenshots/app/security/windows.png)

![Security – Android](../public/assets/screenshots/app/security/android.png)

![Security – iOS](../public/assets/screenshots/app/security/ios.png)

### Settings

![Settings – Web](app/web/settings.png)

![Settings – Linux](app/linux/settings.png)

![Settings – macOS](app/macos/settings.png)

![Settings – Windows](app/windows/settings.png)

![Settings – Android](app/android/settings.png)

![Settings – iOS](app/ios/settings.png)

![Settings – Linux](../public/assets/screenshots/app/settings/linux.png)

![Settings – macOS](../public/assets/screenshots/app/settings/macos.png)

![Settings – Windows](../public/assets/screenshots/app/settings/windows.png)

![Settings – Android](../public/assets/screenshots/app/settings/android.png)

![Settings – iOS](../public/assets/screenshots/app/settings/ios.png)

![Settings – Linux](../public/assets/screenshots/app/settings/linux.png)

![Settings – macOS](../public/assets/screenshots/app/settings/macos.png)

![Settings – Windows](../public/assets/screenshots/app/settings/windows.png)

![Settings – Android](../public/assets/screenshots/app/settings/android.png)

![Settings – iOS](../public/assets/screenshots/app/settings/ios.png)

### Service Management

![Service Management – Web](app/web/service-management.png)

![Service Management – Linux](app/linux/service-management.png)

![Service Management – macOS](app/macos/service-management.png)

![Service Management – Windows](app/windows/service-management.png)

![Service Management – Android](app/android/service-management.png)

![Service Management – iOS](app/ios/service-management.png)

![Service Management – Linux](../public/assets/screenshots/app/service-management/linux.png)

![Service Management – macOS](../public/assets/screenshots/app/service-management/macos.png)

![Service Management – Windows](../public/assets/screenshots/app/service-management/windows.png)

![Service Management – Android](../public/assets/screenshots/app/service-management/android.png)

![Service Management – iOS](../public/assets/screenshots/app/service-management/ios.png)

![Service Management – Linux](../public/assets/screenshots/app/service-management/linux.png)

![Service Management – macOS](../public/assets/screenshots/app/service-management/macos.png)

![Service Management – Windows](../public/assets/screenshots/app/service-management/windows.png)

![Service Management – Android](../public/assets/screenshots/app/service-management/android.png)

![Service Management – iOS](../public/assets/screenshots/app/service-management/ios.png)

### Setup Wizard

![Setup Wizard – Web](app/web/setup-wizard.png)

![Setup Wizard – Linux](app/linux/setup-wizard.png)

![Setup Wizard – macOS](app/macos/setup-wizard.png)

![Setup Wizard – Windows](app/windows/setup-wizard.png)

![Setup Wizard – Android](app/android/setup-wizard.png)

![Setup Wizard – iOS](app/ios/setup-wizard.png)

![Setup Wizard – Linux](../public/assets/screenshots/app/setup-wizard/linux.png)

![Setup Wizard – macOS](../public/assets/screenshots/app/setup-wizard/macos.png)

![Setup Wizard – Windows](../public/assets/screenshots/app/setup-wizard/windows.png)

![Setup Wizard – Android](../public/assets/screenshots/app/setup-wizard/android.png)

![Setup Wizard – iOS](../public/assets/screenshots/app/setup-wizard/ios.png)

![Setup Wizard – Linux](../public/assets/screenshots/app/setup-wizard/linux.png)

![Setup Wizard – macOS](../public/assets/screenshots/app/setup-wizard/macos.png)

![Setup Wizard – Windows](../public/assets/screenshots/app/setup-wizard/windows.png)

![Setup Wizard – Android](../public/assets/screenshots/app/setup-wizard/android.png)

![Setup Wizard – iOS](../public/assets/screenshots/app/setup-wizard/ios.png)

### Diagnostics

![Diagnostics – Web](app/web/diagnostics.png)

![Diagnostics – Linux](app/linux/diagnostics.png)

![Diagnostics – macOS](app/macos/diagnostics.png)

![Diagnostics – Windows](app/windows/diagnostics.png)

![Diagnostics – Android](app/android/diagnostics.png)

![Diagnostics – iOS](app/ios/diagnostics.png)

![Diagnostics – Linux](../public/assets/screenshots/app/diagnostics/linux.png)

![Diagnostics – macOS](../public/assets/screenshots/app/diagnostics/macos.png)

![Diagnostics – Windows](../public/assets/screenshots/app/diagnostics/windows.png)

![Diagnostics – Android](../public/assets/screenshots/app/diagnostics/android.png)

![Diagnostics – iOS](../public/assets/screenshots/app/diagnostics/ios.png)

![Diagnostics – Linux](../public/assets/screenshots/app/diagnostics/linux.png)

![Diagnostics – macOS](../public/assets/screenshots/app/diagnostics/macos.png)

![Diagnostics – Windows](../public/assets/screenshots/app/diagnostics/windows.png)

![Diagnostics – Android](../public/assets/screenshots/app/diagnostics/android.png)

![Diagnostics – iOS](../public/assets/screenshots/app/diagnostics/ios.png)

### Business Setup Wizard

![Business Setup Wizard – Web](app/web/business-setup-wizard.png)

![Business Setup Wizard – Linux](app/linux/business-setup-wizard.png)

![Business Setup Wizard – macOS](app/macos/business-setup-wizard.png)

![Business Setup Wizard – Windows](app/windows/business-setup-wizard.png)

![Business Setup Wizard – Android](app/android/business-setup-wizard.png)

![Business Setup Wizard – iOS](app/ios/business-setup-wizard.png)

![Business Setup Wizard – Linux](../public/assets/screenshots/app/business-setup-wizard/linux.png)

![Business Setup Wizard – macOS](../public/assets/screenshots/app/business-setup-wizard/macos.png)

![Business Setup Wizard – Windows](../public/assets/screenshots/app/business-setup-wizard/windows.png)

![Business Setup Wizard – Android](../public/assets/screenshots/app/business-setup-wizard/android.png)

![Business Setup Wizard – iOS](../public/assets/screenshots/app/business-setup-wizard/ios.png)

![Business Setup Wizard – Linux](../public/assets/screenshots/app/business-setup-wizard/linux.png)

![Business Setup Wizard – macOS](../public/assets/screenshots/app/business-setup-wizard/macos.png)

![Business Setup Wizard – Windows](../public/assets/screenshots/app/business-setup-wizard/windows.png)

![Business Setup Wizard – Android](../public/assets/screenshots/app/business-setup-wizard/android.png)

![Business Setup Wizard – iOS](../public/assets/screenshots/app/business-setup-wizard/ios.png)

### Handoffs

![Handoffs – Web](app/web/handoffs.png)

![Handoffs – Linux](app/linux/handoffs.png)

![Handoffs – macOS](app/macos/handoffs.png)

![Handoffs – Windows](app/windows/handoffs.png)

![Handoffs – Android](app/android/handoffs.png)

![Handoffs – iOS](app/ios/handoffs.png)

![Handoffs – Linux](../public/assets/screenshots/app/handoffs/linux.png)

![Handoffs – macOS](../public/assets/screenshots/app/handoffs/macos.png)

![Handoffs – Windows](../public/assets/screenshots/app/handoffs/windows.png)

![Handoffs – Android](../public/assets/screenshots/app/handoffs/android.png)

![Handoffs – iOS](../public/assets/screenshots/app/handoffs/ios.png)

![Handoffs – Linux](../public/assets/screenshots/app/handoffs/linux.png)

![Handoffs – macOS](../public/assets/screenshots/app/handoffs/macos.png)

![Handoffs – Windows](../public/assets/screenshots/app/handoffs/windows.png)

![Handoffs – Android](../public/assets/screenshots/app/handoffs/android.png)

![Handoffs – iOS](../public/assets/screenshots/app/handoffs/ios.png)

### Cost Dashboard

![Cost Dashboard – Web](app/web/cost-dashboard.png)

![Cost Dashboard – Linux](app/linux/cost-dashboard.png)

![Cost Dashboard – macOS](app/macos/cost-dashboard.png)

![Cost Dashboard – Windows](app/windows/cost-dashboard.png)

![Cost Dashboard – Android](app/android/cost-dashboard.png)

![Cost Dashboard – iOS](app/ios/cost-dashboard.png)

![Cost Dashboard – Linux](../public/assets/screenshots/app/cost-dashboard/linux.png)

![Cost Dashboard – macOS](../public/assets/screenshots/app/cost-dashboard/macos.png)

![Cost Dashboard – Windows](../public/assets/screenshots/app/cost-dashboard/windows.png)

![Cost Dashboard – Android](../public/assets/screenshots/app/cost-dashboard/android.png)

![Cost Dashboard – iOS](../public/assets/screenshots/app/cost-dashboard/ios.png)

![Cost Dashboard – Linux](../public/assets/screenshots/app/cost-dashboard/linux.png)

![Cost Dashboard – macOS](../public/assets/screenshots/app/cost-dashboard/macos.png)

![Cost Dashboard – Windows](../public/assets/screenshots/app/cost-dashboard/windows.png)

![Cost Dashboard – Android](../public/assets/screenshots/app/cost-dashboard/android.png)

![Cost Dashboard – iOS](../public/assets/screenshots/app/cost-dashboard/ios.png)

### Dynamic Scaling

![Dynamic Scaling – Web](app/web/dynamic-scaling.png)

![Dynamic Scaling – Linux](app/linux/dynamic-scaling.png)

![Dynamic Scaling – macOS](app/macos/dynamic-scaling.png)

![Dynamic Scaling – Windows](app/windows/dynamic-scaling.png)

![Dynamic Scaling – Android](app/android/dynamic-scaling.png)

![Dynamic Scaling – iOS](app/ios/dynamic-scaling.png)

![Dynamic Scaling – Linux](../public/assets/screenshots/app/dynamic-scaling/linux.png)

![Dynamic Scaling – macOS](../public/assets/screenshots/app/dynamic-scaling/macos.png)

![Dynamic Scaling – Windows](../public/assets/screenshots/app/dynamic-scaling/windows.png)

![Dynamic Scaling – Android](../public/assets/screenshots/app/dynamic-scaling/android.png)

![Dynamic Scaling – iOS](../public/assets/screenshots/app/dynamic-scaling/ios.png)

![Dynamic Scaling – Linux](../public/assets/screenshots/app/dynamic-scaling/linux.png)

![Dynamic Scaling – macOS](../public/assets/screenshots/app/dynamic-scaling/macos.png)

![Dynamic Scaling – Windows](../public/assets/screenshots/app/dynamic-scaling/windows.png)

![Dynamic Scaling – Android](../public/assets/screenshots/app/dynamic-scaling/android.png)

![Dynamic Scaling – iOS](../public/assets/screenshots/app/dynamic-scaling/ios.png)

### Pipelines

![Pipelines – Web](app/web/pipelines.png)

![Pipelines – Linux](app/linux/pipelines.png)

![Pipelines – macOS](app/macos/pipelines.png)

![Pipelines – Windows](app/windows/pipelines.png)

![Pipelines – Android](app/android/pipelines.png)

![Pipelines – iOS](app/ios/pipelines.png)

![Pipelines – Linux](../public/assets/screenshots/app/pipelines/linux.png)

![Pipelines – macOS](../public/assets/screenshots/app/pipelines/macos.png)

![Pipelines – Windows](../public/assets/screenshots/app/pipelines/windows.png)

![Pipelines – Android](../public/assets/screenshots/app/pipelines/android.png)

![Pipelines – iOS](../public/assets/screenshots/app/pipelines/ios.png)

![Pipelines – Linux](../public/assets/screenshots/app/pipelines/linux.png)

![Pipelines – macOS](../public/assets/screenshots/app/pipelines/macos.png)

![Pipelines – Windows](../public/assets/screenshots/app/pipelines/windows.png)

![Pipelines – Android](../public/assets/screenshots/app/pipelines/android.png)

![Pipelines – iOS](../public/assets/screenshots/app/pipelines/ios.png)

### Integrations

![Integrations – Web](app/web/integrations.png)

![Integrations – Linux](app/linux/integrations.png)

![Integrations – macOS](app/macos/integrations.png)

![Integrations – Windows](app/windows/integrations.png)

![Integrations – Android](app/android/integrations.png)

![Integrations – iOS](app/ios/integrations.png)

![Integrations – Linux](../public/assets/screenshots/app/integrations/linux.png)

![Integrations – macOS](../public/assets/screenshots/app/integrations/macos.png)

![Integrations – Windows](../public/assets/screenshots/app/integrations/windows.png)

![Integrations – Android](../public/assets/screenshots/app/integrations/android.png)

![Integrations – iOS](../public/assets/screenshots/app/integrations/ios.png)

![Integrations – Linux](../public/assets/screenshots/app/integrations/linux.png)

![Integrations – macOS](../public/assets/screenshots/app/integrations/macos.png)

![Integrations – Windows](../public/assets/screenshots/app/integrations/windows.png)

![Integrations – Android](../public/assets/screenshots/app/integrations/android.png)

![Integrations – iOS](../public/assets/screenshots/app/integrations/ios.png)

### User Management

![User Management – Web](app/web/user-management.png)

![User Management – Linux](app/linux/user-management.png)

![User Management – macOS](app/macos/user-management.png)

![User Management – Windows](app/windows/user-management.png)

![User Management – Android](app/android/user-management.png)

![User Management – iOS](app/ios/user-management.png)

![User Management – Linux](../public/assets/screenshots/app/user-management/linux.png)

![User Management – macOS](../public/assets/screenshots/app/user-management/macos.png)

![User Management – Windows](../public/assets/screenshots/app/user-management/windows.png)

![User Management – Android](../public/assets/screenshots/app/user-management/android.png)

![User Management – iOS](../public/assets/screenshots/app/user-management/ios.png)

![User Management – Linux](../public/assets/screenshots/app/user-management/linux.png)

![User Management – macOS](../public/assets/screenshots/app/user-management/macos.png)

![User Management – Windows](../public/assets/screenshots/app/user-management/windows.png)

![User Management – Android](../public/assets/screenshots/app/user-management/android.png)

![User Management – iOS](../public/assets/screenshots/app/user-management/ios.png)

### Fix-This Wizard

![Fix-This Wizard – Web](app/web/fix-wizard.png)

![Fix-This Wizard – Linux](app/linux/fix-wizard.png)

![Fix-This Wizard – macOS](app/macos/fix-wizard.png)

![Fix-This Wizard – Windows](app/windows/fix-wizard.png)

![Fix-This Wizard – Android](app/android/fix-wizard.png)

![Fix-This Wizard – iOS](app/ios/fix-wizard.png)

![Fix-This Wizard – Linux](../public/assets/screenshots/app/fix-wizard/linux.png)

![Fix-This Wizard – macOS](../public/assets/screenshots/app/fix-wizard/macos.png)

![Fix-This Wizard – Windows](../public/assets/screenshots/app/fix-wizard/windows.png)

![Fix-This Wizard – Android](../public/assets/screenshots/app/fix-wizard/android.png)

![Fix-This Wizard – iOS](../public/assets/screenshots/app/fix-wizard/ios.png)

![Fix-This Wizard – Linux](../public/assets/screenshots/app/fix-wizard/linux.png)

![Fix-This Wizard – macOS](../public/assets/screenshots/app/fix-wizard/macos.png)

![Fix-This Wizard – Windows](../public/assets/screenshots/app/fix-wizard/windows.png)

![Fix-This Wizard – Android](../public/assets/screenshots/app/fix-wizard/android.png)

![Fix-This Wizard – iOS](../public/assets/screenshots/app/fix-wizard/ios.png)

### Upgrade Wizard

![Upgrade Wizard – Web](app/web/upgrade-wizard.png)

![Upgrade Wizard – Linux](app/linux/upgrade-wizard.png)

![Upgrade Wizard – macOS](app/macos/upgrade-wizard.png)

![Upgrade Wizard – Windows](app/windows/upgrade-wizard.png)

![Upgrade Wizard – Android](app/android/upgrade-wizard.png)

![Upgrade Wizard – iOS](app/ios/upgrade-wizard.png)

![Upgrade Wizard – Linux](../public/assets/screenshots/app/upgrade-wizard/linux.png)

![Upgrade Wizard – macOS](../public/assets/screenshots/app/upgrade-wizard/macos.png)

![Upgrade Wizard – Windows](../public/assets/screenshots/app/upgrade-wizard/windows.png)

![Upgrade Wizard – Android](../public/assets/screenshots/app/upgrade-wizard/android.png)

![Upgrade Wizard – iOS](../public/assets/screenshots/app/upgrade-wizard/ios.png)

![Upgrade Wizard – Linux](../public/assets/screenshots/app/upgrade-wizard/linux.png)

![Upgrade Wizard – macOS](../public/assets/screenshots/app/upgrade-wizard/macos.png)

![Upgrade Wizard – Windows](../public/assets/screenshots/app/upgrade-wizard/windows.png)

![Upgrade Wizard – Android](../public/assets/screenshots/app/upgrade-wizard/android.png)

![Upgrade Wizard – iOS](../public/assets/screenshots/app/upgrade-wizard/ios.png)

### Billing Wizard

![Billing Wizard – Web](app/web/billing-wizard.png)

![Billing Wizard – Linux](app/linux/billing-wizard.png)

![Billing Wizard – macOS](app/macos/billing-wizard.png)

![Billing Wizard – Windows](app/windows/billing-wizard.png)

![Billing Wizard – Android](app/android/billing-wizard.png)

![Billing Wizard – iOS](app/ios/billing-wizard.png)

![Billing Wizard – Linux](../public/assets/screenshots/app/billing-wizard/linux.png)

![Billing Wizard – macOS](../public/assets/screenshots/app/billing-wizard/macos.png)

![Billing Wizard – Windows](../public/assets/screenshots/app/billing-wizard/windows.png)

![Billing Wizard – Android](../public/assets/screenshots/app/billing-wizard/android.png)

![Billing Wizard – iOS](../public/assets/screenshots/app/billing-wizard/ios.png)

![Billing Wizard – Linux](../public/assets/screenshots/app/billing-wizard/linux.png)

![Billing Wizard – macOS](../public/assets/screenshots/app/billing-wizard/macos.png)

![Billing Wizard – Windows](../public/assets/screenshots/app/billing-wizard/windows.png)

![Billing Wizard – Android](../public/assets/screenshots/app/billing-wizard/android.png)

![Billing Wizard – iOS](../public/assets/screenshots/app/billing-wizard/ios.png)

### Task List (Orchestration)

![Task List (Orchestration) – Web](app/web/task-list.png)

![Task List (Orchestration) – Linux](app/linux/task-list.png)

![Task List (Orchestration) – macOS](app/macos/task-list.png)

![Task List (Orchestration) – Windows](app/windows/task-list.png)

![Task List (Orchestration) – Android](app/android/task-list.png)

![Task List (Orchestration) – iOS](app/ios/task-list.png)

![Task List (Orchestration) – Linux](../public/assets/screenshots/app/task-list/linux.png)

![Task List (Orchestration) – macOS](../public/assets/screenshots/app/task-list/macos.png)

![Task List (Orchestration) – Windows](../public/assets/screenshots/app/task-list/windows.png)

![Task List (Orchestration) – Android](../public/assets/screenshots/app/task-list/android.png)

![Task List (Orchestration) – iOS](../public/assets/screenshots/app/task-list/ios.png)

![Task List (Orchestration) – Linux](../public/assets/screenshots/app/task-list/linux.png)

![Task List (Orchestration) – macOS](../public/assets/screenshots/app/task-list/macos.png)

![Task List (Orchestration) – Windows](../public/assets/screenshots/app/task-list/windows.png)

![Task List (Orchestration) – Android](../public/assets/screenshots/app/task-list/android.png)

![Task List (Orchestration) – iOS](../public/assets/screenshots/app/task-list/ios.png)

### Swarm Memory

![Swarm Memory – Web](app/web/swarm-memory.png)

![Swarm Memory – Linux](app/linux/swarm-memory.png)

![Swarm Memory – macOS](app/macos/swarm-memory.png)

![Swarm Memory – Windows](app/windows/swarm-memory.png)

![Swarm Memory – Android](app/android/swarm-memory.png)

![Swarm Memory – iOS](app/ios/swarm-memory.png)

![Swarm Memory – Linux](../public/assets/screenshots/app/swarm-memory/linux.png)

![Swarm Memory – macOS](../public/assets/screenshots/app/swarm-memory/macos.png)

![Swarm Memory – Windows](../public/assets/screenshots/app/swarm-memory/windows.png)

![Swarm Memory – Android](../public/assets/screenshots/app/swarm-memory/android.png)

![Swarm Memory – iOS](../public/assets/screenshots/app/swarm-memory/ios.png)

![Swarm Memory – Linux](../public/assets/screenshots/app/swarm-memory/linux.png)

![Swarm Memory – macOS](../public/assets/screenshots/app/swarm-memory/macos.png)

![Swarm Memory – Windows](../public/assets/screenshots/app/swarm-memory/windows.png)

![Swarm Memory – Android](../public/assets/screenshots/app/swarm-memory/android.png)

![Swarm Memory – iOS](../public/assets/screenshots/app/swarm-memory/ios.png)

### Growth Experiments

![Growth Experiments – Web](app/web/growth-experiments.png)

![Growth Experiments – Linux](app/linux/growth-experiments.png)

![Growth Experiments – macOS](app/macos/growth-experiments.png)

![Growth Experiments – Windows](app/windows/growth-experiments.png)

![Growth Experiments – Android](app/android/growth-experiments.png)

![Growth Experiments – iOS](app/ios/growth-experiments.png)

![Growth Experiments – Linux](../public/assets/screenshots/app/growth-experiments/linux.png)

![Growth Experiments – macOS](../public/assets/screenshots/app/growth-experiments/macos.png)

![Growth Experiments – Windows](../public/assets/screenshots/app/growth-experiments/windows.png)

![Growth Experiments – Android](../public/assets/screenshots/app/growth-experiments/android.png)

![Growth Experiments – iOS](../public/assets/screenshots/app/growth-experiments/ios.png)

![Growth Experiments – Linux](../public/assets/screenshots/app/growth-experiments/linux.png)

![Growth Experiments – macOS](../public/assets/screenshots/app/growth-experiments/macos.png)

![Growth Experiments – Windows](../public/assets/screenshots/app/growth-experiments/windows.png)

![Growth Experiments – Android](../public/assets/screenshots/app/growth-experiments/android.png)

![Growth Experiments – iOS](../public/assets/screenshots/app/growth-experiments/ios.png)

### Referrals

![Referrals – Web](app/web/referrals.png)

![Referrals – Linux](app/linux/referrals.png)

![Referrals – macOS](app/macos/referrals.png)

![Referrals – Windows](app/windows/referrals.png)

![Referrals – Android](app/android/referrals.png)

![Referrals – iOS](app/ios/referrals.png)

![Referrals – Linux](../public/assets/screenshots/app/referrals/linux.png)

![Referrals – macOS](../public/assets/screenshots/app/referrals/macos.png)

![Referrals – Windows](../public/assets/screenshots/app/referrals/windows.png)

![Referrals – Android](../public/assets/screenshots/app/referrals/android.png)

![Referrals – iOS](../public/assets/screenshots/app/referrals/ios.png)

![Referrals – Linux](../public/assets/screenshots/app/referrals/linux.png)

![Referrals – macOS](../public/assets/screenshots/app/referrals/macos.png)

![Referrals – Windows](../public/assets/screenshots/app/referrals/windows.png)

![Referrals – Android](../public/assets/screenshots/app/referrals/android.png)

![Referrals – iOS](../public/assets/screenshots/app/referrals/ios.png)

</div>
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

## 5. Documentation

Please refer to the detailed architecture documents in the `docs/` folder:
- [KAIROS Orchestration Design Phase 4](./kairos_orchestration_phase4.md)

</div>
