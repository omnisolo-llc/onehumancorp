# OHC Hybrid Agentic OS User Guide

Welcome to the One Human Corp (OHC) Hybrid Agentic OS. This system is designed to empower a single human CEO to orchestrate AI agents seamlessly across platforms with extreme aesthetic excellence.

## Deployment Modes

OHC runs in three primary modes depending on your operational scale and isolation needs:

### 1. Cloud-Native Mode
Designed for high-scale, multi-tenant environments.
- **Orchestration**: Kubernetes
- **State & Caching**: PostgreSQL and Redis
- **Strengths**: Strict tenant isolation, horizontal scaling for high concurrency, and distributed tracing.

### 2. Standalone Desktop Mode
Designed for single-user sovereignty and local-first execution.
- **Orchestration**: Local Docker or direct execution
- **State**: SQLite (Zero config database)
- **Strengths**: Low latency, fully offline-capable, and degrades gracefully when heavy dependencies like Redis or Chatwoot are unavailable.

### 3. Thin Client Mode
A UI-only interface connecting to a remote OHC-HA cloud instance.
- **Interfaces**: Flutter Web, iOS, Android, macOS, Windows, Linux
- **Strengths**: Low device resource usage while controlling a powerful swarm in the cloud.

---

## The Flutter UI: Aesthetic Truth

Our interface prioritizes glassmorphism, readability (Outfit and Inter typography), and consistency. The hybrid OS layout adapts intuitively whether you are managing an enterprise from your phone or your multi-monitor command center.

Here is what the sign-in experience looks like across platforms:

### Web & Thin Client
![OHC Login on Web](app/web/login.png)

### macOS Desktop
![OHC Login on macOS](app/macos/login.png)

### Windows Desktop
![OHC Login on Windows](app/windows/login.png)

### Linux Desktop
![OHC Login on Linux](app/linux/login.png)

### iOS Mobile
![OHC Login on iOS](app/ios/login.png)

### Android Mobile
![OHC Login on Android](app/android/login.png)

## Operations & Mission Queue

As the CEO, you issue commands. The **Orchestration Hub** assigns these commands as missions to your workforce of AI agents.
Agents continuously report their memory and status to the Swarm Intelligence Protocol (OHC-SIP) database.
