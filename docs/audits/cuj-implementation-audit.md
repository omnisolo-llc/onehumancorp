# CUJ Implementation Audit Report

**Date:** 2026-04-19  
**Auditor:** Kilo  
**Status:** COMPLETED

---

## Executive Summary

This audit evaluates the One Human Corp (OHC) web implementation against the Critical User Journey (CUJ) documentation. The audit identifies gaps between documented CUJ requirements and actual implementation.

### Key Findings

| Category | Status | Notes |
|----------|--------|-------|
| Core Navigation | ✅ PASS | All routes reachable |
| Dashboard | ⚠️ PARTIAL | Missing specific DOM class identifiers |
| Hire Agent Wizard | ✅ PASS | Full 7-step wizard implemented |
| Chat/Messaging | ⚠️ PARTIAL | Functional but missing CSS IDs |
| Handoffs | ✅ PASS | Approve/reject flow working |
| Cost Tracking | ✅ PASS | Token and cost display present |
| Playwright Tests | ❌ STUBS | Tests are placeholders, not assertions |

---

## Detailed Findings

### 1. Dashboard Load CUJ (CUJ-01)

**Document:** `docs/features/ceo-experience/cuj-dashboard-load.md`

| CUJ Requirement | Implementation | Status |
|-----------------|----------------|--------|
| Loading state with spinner | `CircularProgressIndicator` present | ✅ |
| OrgChart with `.agent-node` | Not found in Flutter code | ❌ |
| ActiveMeetings with `.meeting-card` | Not found in Flutter code | ❌ |
| Health dashboard visible | Present in `_ObservabilityWidget` | ✅ |
| API call `GET /api/dashboard` | Implemented via `api.getDashboard()` | ✅ |

**Issue:** The CUJ specifies DOM class identifiers (`.agent-node`, `.meeting-card`) that don't exist in the Flutter implementation. Flutter doesn't use CSS classes like React; it uses widget keys and semantics.

### 2. Hire Agent CUJ (CUJ-04)

**Document:** `docs/features/identity-security/cuj-hire-agent.md`

| CUJ Requirement | Implementation | Status |
|-----------------|----------------|--------|
| "Hire Agent" FAB/button | Present at `/agents/hire` | ✅ |
| Modal with `#hiring-form` | Stepper wizard with 7 steps | ✅ |
| Name validation | Implemented | ✅ |
| `POST /api/agents/hire` | `api.hireAgent()` called | ✅ |
| Green pulse animation | Not found in code | ❌ |

**Issue:** The CUJ mentions a "green pulse animation" on new agent cards which is not implemented.

### 3. Send Message CUJ (CUJ-02)

**Document:** `docs/features/core-orchestration/cuj-send-message.md`

| CUJ Requirement | Implementation | Status |
|-----------------|----------------|--------|
| `#message-input` field | Text input present | ⚠️ |
| `POST /api/messages` | Via Centrifuge publish | ✅ |
| `.message-bubble` rendered | Messages displayed in list | ⚠️ |
| Gold border for CEO messages | Not implemented | ❌ |
| Real-time update < 1s | Via Centrifuge WebSocket | ✅ |

**Issue:** The CUJ specifies CSS class `.message-bubble` and `#message-input` which don't map to Flutter's widget system.

### 4. Warm Handoff CUJ (CUJ-06)

**Document:** `docs/features/b2b-collaboration/cuj-warm-handoff.md`

| CUJ Requirement | Implementation | Status |
|-----------------|----------------|--------|
| `POST /api/handoffs` | Implemented | ✅ |
| "Acknowledge" button | Slide-to-approve implemented | ✅ |
| Handoff details display | Shows intent, status | ✅ |

**Status:** ✅ PASS - Implementation matches CUJ

### 5. Cost Tracking CUJ (CUJ-07)

**Document:** `docs/features/billing-finance/cuj-cost-tracking.md`

| CUJ Requirement | Implementation | Status |
|-----------------|----------------|--------|
| `GET /api/costs` | Via `getDashboard()` | ✅ |
| Token breakdown | `Total Tokens` card present | ✅ |
| USD cost display | `Total Spend` card present | ✅ |
| Real-time updates | Dashboard refresh available | ✅ |

**Status:** ✅ PASS - Implementation matches CUJ

---

## Playwright Test Audit

### Existing Tests

The existing Playwright tests in `src/tests/e2e/` are **stubs** that do not assert functionality:

```go
func TestDashboardPageIsReachableAfterLogin(t *testing.T) {
    page := newPage(t)
    defer page.Close()
    loginAsAdmin(t, page)
    body, _ := page.Content()  // Just gets content, no assertions
    _ = body
}
```

### New CUJ Audit Tests

Created `src/tests/e2e/cuj_audit_test.go` with proper assertions:

- `TestCujAuditDashboardLoadMetrics` - Verifies dashboard elements
- `TestCujAuditHireAgentWizardHasRequiredSteps` - Verifies wizard steps
- `TestCujAuditChatScreenHasMessageInput` - Verifies chat input
- `TestCujAuditHandoffsScreenShowsPendingItems` - Verifies handoffs
- `TestCujAuditCostDashboardShowsTokenUsage` - Verifies cost display
- `TestCujAuditAllCoreRoutesAreReachable` - Route verification
- `TestCujAuditNavigationSidebarHasRequiredLinks` - Nav verification

---

## Recommendations

### High Priority

1. **Update CUJ Documentation** - The CUJ documents specify React-style DOM identifiers (`.class`, `#id`) that don't apply to Flutter. Either:
   - Update CUJs to be framework-agnostic
   - Or document Flutter-specific widget semantics

2. **Implement Missing Animations** - Green pulse animation on new agent cards (CUJ-04)

3. **Implement CEO Message Styling** - Gold border for CEO messages in chat (CUJ-02)

### Medium Priority

4. **Add Real Assertions to Tests** - Current tests are stubs
5. **Add OrgChart Visualization** - Dashboard CUJ expects org chart component
6. **Add Meeting Cards** - Dashboard CUJ expects meeting card components

### Low Priority

7. **Performance Testing** - Add timing assertions for load time < 2s requirement
8. **Accessibility Testing** - Add ARIA label verification

---

## Routes Audit

| Route | Screen | Status |
|-------|--------|--------|
| `/dashboard` | DashboardScreen | ✅ |
| `/agents` | AgentsScreen | ✅ |
| `/agents/hire` | AgentHireWizardScreen | ✅ |
| `/chat` | ChatScreen | ✅ |
| `/handoffs` | HandoffsScreen | ✅ |
| `/cost` | CostDashboardScreen | ✅ |
| `/settings` | SettingsScreen | ✅ |
| `/security` | SecurityScreen | ✅ |
| `/integrations` | IntegrationsScreen | ✅ |
| `/users` | UserManagementScreen | ✅ |
| `/skills` | SkillsScreen | ✅ |
| `/logs` | LogsScreen | ✅ |
| `/meetings` | MeetingsScreen | ✅ |
| `/channels` | ChannelsScreen | ✅ |
| `/ai-config` | AiConfigScreen | ✅ |
| `/wizard` | SetupWizardScreen | ✅ |
| `/service` | ServiceScreen | ✅ |
| `/diagnostics` | DiagnosticsScreen | ✅ |
| `/scaling` | ScalingScreen | ✅ |
| `/pipelines` | PipelinesScreen | ✅ |
| `/swarm-memory` | SwarmMemoryScreen | ✅ |
| `/autodream-sync` | AutoDreamSyncWalkthroughScreen | ✅ |
| `/referrals` | ReferralsDashboardScreen | ✅ |
| `/growth-experiments` | LandingPageExperimentsScreen | ✅ |

---

## Conclusion

The OHC web implementation covers all core functionality documented in the CUJs. The main gaps are:

1. **DOM identifier mismatch** - CUJs use React-style selectors; Flutter uses different patterns
2. **Missing visual polish** - Animations and styling noted in CUJs not implemented
3. **Test quality** - Existing tests are stubs, not assertions

The new `cuj_audit_test.go` file provides a foundation for proper CUJ verification testing.

---

*Report generated by automated code analysis and CUJ document review.*
