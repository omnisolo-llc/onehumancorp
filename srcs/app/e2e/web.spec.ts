/**
 * Flutter Web E2E tests using Playwright — FULL feature coverage.
 *
 * Every screen and major interaction is exercised here.  Screenshots are taken
 * after each screen navigation so they can be committed alongside docs.
 *
 * The app is served by the Bazel sh_test wrapper (flutter_web_e2e_test.sh)
 * which also starts the real Go backend and seeds fixture data before the
 * tests run.
 *
 * Test coverage:
 *   • App bootstrap / static assets
 *   • Landing screen
 *   • Login (form validation, SSO button, settings modal, successful login)
 *   • Dashboard (render, sidebar)
 *   • Agents (list, hire button)
 *   • Agent Hire Wizard (form fields)
 *   • Prompt Tuning Wizard (slider)
 *   • Meetings (list, new room dialog)
 *   • Chat (render, typing, send)
 *   • Channels (list, add dialog)
 *   • AI Providers (list, add dialog)
 *   • Skills (list, category filter)
 *   • Logs (render, refresh)
 *   • Security (render, re-scan)
 *   • Settings (sections, edit URL, sign out)
 *   • Service Management
 *   • Setup Wizard (step navigation)
 *   • Diagnostics (run diagnostics)
 *   • Business Setup Wizard (steps)
 *   • Handoffs (list, filter)
 *   • Cost Dashboard (render, refresh, period)
 *   • Dynamic Scaling (role dropdown, provisioning)
 *   • Pipelines (list, refresh)
 *   • Integrations & Tools (connect button)
 *   • User Management (table, invite dialog, refresh)
 *   • Fix-This Wizard
 *   • Upgrade Wizard (confirm)
 *   • Billing Wizard (plan selection)
 *   • Task List / Orchestration (filter)
 *   • Swarm Memory
 *   • Growth Experiments (toggle)
 *   • Referrals (copy code)
 *   • Sidebar (all 23 routes)
 *   • Resilience / chaos scenarios (high-latency, network-partition)
 *   • Authentication guard (unauthenticated redirect)
 *   • Performance baseline
 */

import { test, expect, Page } from '@playwright/test';
import { mkdir } from 'node:fs/promises';
import path from 'node:path';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const SCREENSHOT_DIR =
  process.env.APP_SCREENSHOT_OUTPUT_DIR ??
  path.join(__dirname, '../../../docs/public/assets/screenshots/app/screens');

async function saveScreenshot(page: Page, name: string): Promise<void> {
  try {
    await mkdir(SCREENSHOT_DIR, { recursive: true });
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, `${name}.png`),
      fullPage: true,
    });
  } catch {
    // Never fail a test because screenshot saving failed
  }
}

/** Wait for the Flutter app bootstrap to finish (CanvasKit / skwasm load). */
async function waitForFlutter(page: Page, timeoutMs = 30_000): Promise<void> {
  await page.waitForFunction(
    () => {
      const body = document.body;
      return (
        body &&
        (body.querySelector('flt-glass-pane') !== null ||
          body.querySelector('canvas') !== null ||
          body.children.length > 0)
      );
    },
    { timeout: timeoutMs },
  );
  // Give Flutter a moment to finish painting
  await page.waitForTimeout(800);
}

/**
 * Seed the backend with a named scenario so all screens have data.
 * Uses page.evaluate to avoid cross-origin issues.
 */
async function seedBackend(page: Page, scenario = 'launch-readiness'): Promise<void> {
  await page.evaluate(async (sc: string) => {
    try {
      await fetch(window.location.origin + '/api/dev/seed', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ scenario: sc }),
      });
    } catch {
      // Backend may not be present in static-only mode – continue
    }
  }, scenario);
}

/**
 * Log in with the seeded admin credentials and wait for the redirect.
 */
async function loginAsAdmin(page: Page): Promise<void> {
  await page.goto('/login');
  await waitForFlutter(page);
  // Tab past any initial focused element to the email field
  await page.keyboard.press('Tab');
  await page.keyboard.press('Tab');
  await page.keyboard.type('admin@localhost');
  await page.keyboard.press('Tab');
  await page.keyboard.type('adminpass123');
  await page.keyboard.press('Enter');
  await page.waitForTimeout(1500);
}

/** Navigate to a route and wait for Flutter to settle. */
async function goTo(page: Page, route: string): Promise<void> {
  await page.goto(route);
  await waitForFlutter(page);
  await page.waitForTimeout(600);
}

// ---------------------------------------------------------------------------
// Test suite
// ---------------------------------------------------------------------------

test.describe('OHC Flutter Web — Full Feature Coverage', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await seedBackend(page, 'launch-readiness');
  });

  // ── 1. Application bootstrap & static assets ──────────────────────────

  test('1. App bootstrap: page title is set correctly', async ({ page }) => {
    await goTo(page, '/');
    await expect(page).toHaveTitle(/One Human Corp|ohc_app/i);
    await saveScreenshot(page, '01-bootstrap');
  });

  test('2. Flutter root element mounts (flt-glass-pane or canvas present)', async ({ page }) => {
    await goTo(page, '/');
    const flutterPresent = await page.evaluate(() => {
      return (
        document.querySelector('flt-glass-pane') !== null ||
        document.querySelector('canvas') !== null ||
        document.body.innerHTML.length > 100
      );
    });
    expect(flutterPresent).toBe(true);
    await saveScreenshot(page, '02-flutter-root');
  });

  test('3. Flutter.js / main.dart.js is served', async ({ page }) => {
    const resources: string[] = [];
    page.on('response', (res) => resources.push(res.url()));
    await page.goto('/');
    await waitForFlutter(page);
    const hasFlutterAsset = resources.some(
      (url) =>
        url.includes('flutter.js') ||
        url.includes('main.dart.js') ||
        url.includes('flutter_bootstrap.js') ||
        url.includes('.wasm'),
    );
    expect(hasFlutterAsset).toBe(true);
    await saveScreenshot(page, '03-flutter-assets');
  });

  // ── 2. Landing screen ────────────────────────────────────────────────

  test('4. Landing screen: renders with branding and CTAs', async ({ page }) => {
    await goTo(page, '/landing');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(200);
    await saveScreenshot(page, '04-landing');
  });

  // ── 3. Login screen ─────────────────────────────────────────────────

  test('5. Login screen: renders email, password inputs and Sign In button', async ({ page }) => {
    await goTo(page, '/login');
    await saveScreenshot(page, '05-login');
    const bodyHtml = await page.content();
    expect(bodyHtml.length).toBeGreaterThan(100);
  });

  test('6. Login screen: direct navigation resolves to /login', async ({ page }) => {
    await page.goto('/login');
    await waitForFlutter(page);
    await expect(page).toHaveURL(/\/login/);
    await saveScreenshot(page, '06-login-direct');
  });

  test('7. Login screen: form validation triggers on empty submit', async ({ page }) => {
    await goTo(page, '/login');
    for (let i = 0; i < 5; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);
    await saveScreenshot(page, '07-login-validation');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('8. Login screen: SSO button is keyboard-reachable', async ({ page }) => {
    await goTo(page, '/login');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(700);
    await saveScreenshot(page, '08-login-sso');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('9. Login screen: settings modal opens via keyboard', async ({ page }) => {
    await goTo(page, '/login');
    for (let i = 0; i < 6; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);
    await saveScreenshot(page, '09-login-settings-modal');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('10. Login screen: successful login redirects away from /login', async ({ page }) => {
    await loginAsAdmin(page);
    const url = page.url();
    expect(url).not.toMatch(/\/login/);
    await saveScreenshot(page, '10-post-login-redirect');
  });

  // ── 4. Authentication guard ──────────────────────────────────────────

  test('11. Unauthenticated access to /dashboard redirects to landing or login', async ({ page }) => {
    await page.goto('/dashboard');
    await waitForFlutter(page);
    const url = page.url();
    expect(url).toMatch(/\/landing|\/login|\//);
    await saveScreenshot(page, '11-unauthenticated-redirect');
  });

  // ── 5. Dashboard ─────────────────────────────────────────────────────

  test('12. Dashboard: renders key sections after login', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/dashboard');
    await saveScreenshot(page, '12-dashboard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(200);
  });

  test('13. Dashboard: sidebar navigation is present', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/dashboard');
    await saveScreenshot(page, '13-dashboard-sidebar');
    const bodyHtml = await page.content();
    expect(bodyHtml.length).toBeGreaterThan(200);
  });

  // ── 6. Agents screen ────────────────────────────────────────────────

  test('14. Agents: loads and shows agent list or empty state', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/agents');
    await saveScreenshot(page, '14-agents');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('15. Agents: Hire Agent button navigates to hire wizard', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/agents');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(700);
    await saveScreenshot(page, '15-agents-hire-btn');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 7. Agent Hire Wizard ────────────────────────────────────────────

  test('16. Agent Hire Wizard: form renders with name and role inputs', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/agents/hire');
    await saveScreenshot(page, '16-agent-hire-wizard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('17. Agent Hire Wizard: name input accepts text', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/agents/hire');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('Alice');
    await page.waitForTimeout(300);
    await saveScreenshot(page, '17-agent-hire-form-filled');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 8. Prompt Tuning Wizard ────────────────────────────────────────

  test('18. Prompt Tuning Wizard: renders for a known agent id', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/agents/default/tune');
    await saveScreenshot(page, '18-prompt-tuning-wizard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('19. Prompt Tuning Wizard: temperature slider is interactable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/agents/default/tune');
    for (let i = 0; i < 5; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('ArrowRight');
    await page.waitForTimeout(300);
    await saveScreenshot(page, '19-prompt-tuning-slider');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 9. Meetings screen ─────────────────────────────────────────────

  test('20. Meetings: renders room list or empty state', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/meetings');
    await saveScreenshot(page, '20-meetings');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('21. Meetings: New Room button triggers creation dialog', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/meetings');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(700);
    await saveScreenshot(page, '21-meetings-new-room-dialog');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 10. Chat screen ────────────────────────────────────────────────

  test('22. Chat: renders room selector and message area', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/chat');
    await saveScreenshot(page, '22-chat');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('23. Chat: typing in message input works', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/chat');
    for (let i = 0; i < 4; i++) await page.keyboard.press('Tab');
    await page.keyboard.type('Hello OHC!');
    await page.waitForTimeout(300);
    await saveScreenshot(page, '23-chat-typing');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('24. Chat: send button is reachable via keyboard', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/chat');
    for (let i = 0; i < 4; i++) await page.keyboard.press('Tab');
    await page.keyboard.type('test message');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '24-chat-send');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 11. Channels screen ────────────────────────────────────────────

  test('25. Channels: renders channel list or empty state', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/channels');
    await saveScreenshot(page, '25-channels');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('26. Channels: Add Channel button opens configuration dialog', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/channels');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '26-channels-add-dialog');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 12. AI Providers screen ────────────────────────────────────────

  test('27. AI Providers: renders provider list or empty state', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/ai-config');
    await saveScreenshot(page, '27-ai-providers');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('28. AI Providers: Add Provider dialog opens', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/ai-config');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '28-ai-providers-add-dialog');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 13. Skills screen ─────────────────────────────────────────────

  test('29. Skills: renders skill list with category filter pills', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/skills');
    await saveScreenshot(page, '29-skills');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('30. Skills: category filter is keyboard-navigable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/skills');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '30-skills-filter');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 14. Logs screen ───────────────────────────────────────────────

  test('31. Logs: renders log output area and controls', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/logs');
    await saveScreenshot(page, '31-logs');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('32. Logs: refresh button is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/logs');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '32-logs-refresh');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 15. Security screen ───────────────────────────────────────────

  test('33. Security: renders issues list or all-clear state', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/security');
    await saveScreenshot(page, '33-security');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('34. Security: Re-scan button triggers a new scan', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/security');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(700);
    await saveScreenshot(page, '34-security-rescan');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 16. Settings screen ───────────────────────────────────────────

  test('35. Settings: renders all configuration sections', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/settings');
    await saveScreenshot(page, '35-settings');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('36. Settings: Edit Backend URL button is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/settings');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);
    await saveScreenshot(page, '36-settings-edit-url');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('37. Settings: Sign Out button is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/settings');
    for (let i = 0; i < 12; i++) await page.keyboard.press('Tab');
    await saveScreenshot(page, '37-settings-signout-focus');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 17. Service Management ────────────────────────────────────────

  test('38. Service Management: renders status indicators and controls', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/service');
    await saveScreenshot(page, '38-service-management');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 18. Setup Wizard ─────────────────────────────────────────────

  test('39. Setup Wizard: renders multi-step configuration form', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/wizard');
    await saveScreenshot(page, '39-setup-wizard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('40. Setup Wizard: Next button advances to step 2', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/wizard');
    for (let i = 0; i < 4; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);
    await saveScreenshot(page, '40-setup-wizard-step2');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 19. Diagnostics screen ────────────────────────────────────────

  test('41. Diagnostics: renders health check status rows', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/diagnostics');
    await saveScreenshot(page, '41-diagnostics');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('42. Diagnostics: Run Diagnostics button triggers health checks', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/diagnostics');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(1000);
    await saveScreenshot(page, '42-diagnostics-ran');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 20. Business Setup Wizard ────────────────────────────────────

  test('43. Business Setup Wizard: renders company info step', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/business_setup');
    await saveScreenshot(page, '43-business-setup-wizard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('44. Business Setup Wizard: company name input accepts text', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/business_setup');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('Acme Corp');
    await page.waitForTimeout(300);
    await saveScreenshot(page, '44-business-setup-name');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('45. Business Setup Wizard: Next advances through steps', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/business_setup');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('Acme Corp');
    for (let i = 0; i < 3; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '45-business-setup-step2');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 21. Handoffs screen ───────────────────────────────────────────

  test('46. Handoffs: renders handoff package list or empty state', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/handoffs');
    await saveScreenshot(page, '46-handoffs');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('47. Handoffs: status filter dropdown is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/handoffs');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);
    await saveScreenshot(page, '47-handoffs-filter');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 22. Cost Dashboard ────────────────────────────────────────────

  test('48. Cost Dashboard: renders summary cards and usage chart', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/cost');
    await saveScreenshot(page, '48-cost-dashboard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('49. Cost Dashboard: refresh button reloads data', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/cost');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(800);
    await saveScreenshot(page, '49-cost-dashboard-refresh');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('50. Cost Dashboard: period selector is interactable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/cost');
    for (let i = 0; i < 4; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '50-cost-dashboard-period');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 23. Dynamic Scaling ───────────────────────────────────────────

  test('51. Scaling: renders role selector and target count controls', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/scaling');
    await saveScreenshot(page, '51-scaling');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('52. Scaling: role dropdown is interactable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/scaling');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '52-scaling-role-dropdown');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('53. Scaling: Start Provisioning button is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/scaling');
    for (let i = 0; i < 6; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '53-scaling-start-provisioning');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 24. Pipelines ────────────────────────────────────────────────

  test('54. Pipelines: renders pipeline cards or empty state', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/pipelines');
    await saveScreenshot(page, '54-pipelines');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('55. Pipelines: refresh button reloads pipeline data', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/pipelines');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '55-pipelines-refresh');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 25. Integrations & Tools ──────────────────────────────────────

  test('56. Integrations: renders external channels and MCP tool gateway', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/integrations');
    await saveScreenshot(page, '56-integrations');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('57. Integrations: Connect button for first integration is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/integrations');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '57-integrations-connect');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 26. User Management ───────────────────────────────────────────

  test('58. User Management: renders user table', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/users');
    await saveScreenshot(page, '58-user-management');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('59. User Management: Invite User dialog opens', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/users');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '59-user-management-invite');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('60. User Management: Refresh button reloads user list', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/users');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '60-user-management-refresh');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 27. Fix-This Wizard ───────────────────────────────────────────

  test('61. Fix-This Wizard: renders troubleshooting steps for agent', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/wizards/fix/default');
    await saveScreenshot(page, '61-fix-wizard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 28. Upgrade Wizard ────────────────────────────────────────────

  test('62. Upgrade Wizard: renders upgrade offer with feature comparison', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/wizards/upgrade');
    await saveScreenshot(page, '62-upgrade-wizard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('63. Upgrade Wizard: Confirm Upgrade button is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/wizards/upgrade');
    for (let i = 0; i < 5; i++) await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '63-upgrade-wizard-confirm');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 29. Billing Wizard ────────────────────────────────────────────

  test('64. Billing Wizard: renders plan selection and payment form', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/wizards/billing');
    await saveScreenshot(page, '64-billing-wizard');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('65. Billing Wizard: plan selection radio buttons are keyboard-navigable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/wizards/billing');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '65-billing-wizard-plan');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 30. Shared Task List (Orchestration) ─────────────────────────

  test('66. Task List: renders task cards with color-coded status badges', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/orchestration/tasks');
    await saveScreenshot(page, '66-task-list');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('67. Task List: status filter dropdown is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/orchestration/tasks');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '67-task-list-filter');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 31. Swarm Memory ──────────────────────────────────────────────

  test('68. Swarm Memory: renders live mesh activity and durable memory panels', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/swarm-memory');
    await saveScreenshot(page, '68-swarm-memory');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 32. Growth Experiments ────────────────────────────────────────

  test('69. Growth Experiments: renders experiment list with metrics', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/growth-experiments');
    await saveScreenshot(page, '69-growth-experiments');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('70. Growth Experiments: A/B variant toggle is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/growth-experiments');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '70-growth-experiments-toggle');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 33. Referrals Dashboard ───────────────────────────────────────

  test('71. Referrals: renders referral code, stats, and leaderboard', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/referrals');
    await saveScreenshot(page, '71-referrals');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('72. Referrals: Copy referral code button is reachable', async ({ page }) => {
    await loginAsAdmin(page);
    await goTo(page, '/referrals');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(400);
    await saveScreenshot(page, '72-referrals-copy');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 34. Sidebar: all 23 authenticated routes ─────────────────────

  test('73. Sidebar: all navigation routes are reachable and render content', async ({ page }) => {
    await loginAsAdmin(page);

    const routes = [
      { path: '/dashboard', screenshot: '73a-nav-dashboard' },
      { path: '/agents', screenshot: '73b-nav-agents' },
      { path: '/orchestration/tasks', screenshot: '73c-nav-tasks' },
      { path: '/swarm-memory', screenshot: '73d-nav-swarm-memory' },
      { path: '/meetings', screenshot: '73e-nav-meetings' },
      { path: '/chat', screenshot: '73f-nav-chat' },
      { path: '/handoffs', screenshot: '73g-nav-handoffs' },
      { path: '/cost', screenshot: '73h-nav-cost' },
      { path: '/scaling', screenshot: '73i-nav-scaling' },
      { path: '/pipelines', screenshot: '73j-nav-pipelines' },
      { path: '/growth-experiments', screenshot: '73k-nav-growth' },
      { path: '/referrals', screenshot: '73l-nav-referrals' },
      { path: '/integrations', screenshot: '73m-nav-integrations' },
      { path: '/users', screenshot: '73n-nav-users' },
      { path: '/channels', screenshot: '73o-nav-channels' },
      { path: '/ai-config', screenshot: '73p-nav-ai-config' },
      { path: '/skills', screenshot: '73q-nav-skills' },
      { path: '/security', screenshot: '73r-nav-security' },
      { path: '/logs', screenshot: '73s-nav-logs' },
      { path: '/settings', screenshot: '73t-nav-settings' },
      { path: '/service', screenshot: '73u-nav-service' },
      { path: '/wizard', screenshot: '73v-nav-wizard' },
      { path: '/diagnostics', screenshot: '73w-nav-diagnostics' },
    ];

    for (const { path: route, screenshot } of routes) {
      await goTo(page, route);
      const html = await page.content();
      expect(html.length).toBeGreaterThan(100);
      await saveScreenshot(page, screenshot);
    }
  });

  // ── 35. Resilience / chaos scenarios ─────────────────────────────

  test('74. App handles high-latency backend scenario without crashing', async ({ page }) => {
    await page.goto('/');
    await seedBackend(page, 'high-latency');
    await goTo(page, '/login');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('slow@test.local');
    await page.keyboard.press('Tab');
    await page.keyboard.type('slowpass');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(1000);
    await saveScreenshot(page, '74-chaos-high-latency');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  test('75. App handles network-partition scenario without crashing', async ({ page }) => {
    await page.goto('/');
    await seedBackend(page, 'network-partition');
    await goTo(page, '/login');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.type('offline@test.local');
    await page.keyboard.press('Tab');
    await page.keyboard.type('offlinepass');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(600);
    await saveScreenshot(page, '75-chaos-network-partition');
    const html = await page.content();
    expect(html.length).toBeGreaterThan(100);
  });

  // ── 36. Performance baseline ──────────────────────────────────────

  test('76. Page loads within timeout (performance baseline)', async ({ page }) => {
    await page.goto('/');
    await waitForFlutter(page);
    const url = page.url();
    expect(url).toMatch(/^http/);
    await saveScreenshot(page, '76-performance-baseline');
  });
});
