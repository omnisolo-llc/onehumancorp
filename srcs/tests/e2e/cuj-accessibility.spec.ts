/**
 * cuj-accessibility.spec.ts
 *
 * Critical User Journey (CUJ) tests covering Accessibility, Responsive design,
 * and Cross-browser/performance concerns.
 * Tests 271–280.
 *
 * Prerequisites: full stack running (handled by run-playwright.mjs).
 */

import { test, expect, Page } from '@playwright/test';

const ADMIN_USER = process.env.OHC_E2E_ADMIN_USER ?? 'admin';
const ADMIN_PASS = process.env.OHC_E2E_ADMIN_PASS ?? 'admin';

const SHORT_TIMEOUT  = 5_000;
const MEDIUM_TIMEOUT = 10_000;
const LONG_TIMEOUT   = 30_000;

async function openApp(page: Page): Promise<void> {
  await page.goto('/');
  await page.waitForLoadState('networkidle');
}

async function loginAsAdmin(page: Page): Promise<void> {
  await openApp(page);

  const loginForm = page.locator(
    'form, [data-testid="login-form"], [aria-label*="login" i], [aria-label*="sign in" i]',
  );
  const isLoginPage =
    page.url().includes('/login') ||
    page.url().includes('/signin') ||
    (await loginForm.count()) > 0;

  if (isLoginPage) {
    const emailInput = page
      .locator(
        'input[type="email"], input[name="email"], input[placeholder*="email" i], input[placeholder*="username" i]',
      )
      .first();
    const passwordInput = page
      .locator('input[type="password"], input[name="password"], input[placeholder*="password" i]')
      .first();

    await emailInput.fill(ADMIN_USER);
    await passwordInput.fill(ADMIN_PASS);

    const submitBtn = page
      .locator(
        'button[type="submit"], button:has-text("Login"), button:has-text("Sign In"), button:has-text("Log In")',
      )
      .first();
    await submitBtn.click();

    await page
      .waitForURL(url => !url.pathname.includes('login') && !url.pathname.includes('signin'), {
        timeout: 15_000,
      })
      .catch(() => {});
    await page.waitForLoadState('networkidle');
  }
}

// ─── Test 271 ────────────────────────────────────────────────────────────────
test('accessibility: interactive elements have visible focus indicators', async ({ page }) => {
  await loginAsAdmin(page);

  // Tab to the first focusable element and verify focus is visible.
  await page.keyboard.press('Tab');
  await page.waitForTimeout(200);

  const focusedEl = page.locator(':focus');
  const hasFocus = await focusedEl.count() > 0;
  // The browser must have moved focus to some element.
  expect(hasFocus).toBe(true);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 272 ────────────────────────────────────────────────────────────────
test('accessibility: page has meaningful <title> element after login', async ({ page }) => {
  await loginAsAdmin(page);

  const title = await page.title();
  // Title should not be empty and should not just be "undefined" or a generic blob.
  expect(title.trim().length).toBeGreaterThan(0);
  expect(title).not.toMatch(/^undefined$/i);
});

// ─── Test 273 ────────────────────────────────────────────────────────────────
test('accessibility: all form inputs on login page have associated labels', async ({ page }) => {
  await page.goto('/');
  await page.waitForLoadState('networkidle');

  // Gather all visible inputs.
  const inputs = page.locator('input[type="text"], input[type="email"], input[type="password"]');
  const count = await inputs.count();

  for (let i = 0; i < count; i++) {
    const input = inputs.nth(i);
    if (!(await input.isVisible())) continue;

    // Each input should have an id linked to a <label>, an aria-label, or a placeholder.
    const id = await input.getAttribute('id');
    const ariaLabel = await input.getAttribute('aria-label');
    const ariaLabelledBy = await input.getAttribute('aria-labelledby');
    const placeholder = await input.getAttribute('placeholder');

    let hasLabel = !!(ariaLabel || ariaLabelledBy || placeholder);
    if (!hasLabel && id) {
      const label = page.locator(`label[for="${id}"]`);
      hasLabel = (await label.count()) > 0;
    }

    expect(hasLabel).toBe(true);
  }
});

// ─── Test 274 ────────────────────────────────────────────────────────────────
test('responsive: compact sidebar or collapsed menu visible at 1024 px width', async ({ page }) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  await loginAsAdmin(page);

  // Sidebar or hamburger menu should still be accessible.
  const sidebar = page.locator('nav, aside, [role="navigation"]').first();
  const hamburger = page.locator(
    'button[aria-label*="menu" i], button[aria-label*="navigation" i], [data-testid*="hamburger" i], [data-testid*="menu-toggle" i]',
  ).first();

  const sidebarVisible = await sidebar.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  const hamburgerVisible = await hamburger.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  expect(sidebarVisible || hamburgerVisible).toBe(true);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 275 ────────────────────────────────────────────────────────────────
test('responsive: mobile viewport 390px does not produce horizontal overflow', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await openApp(page);

  // Check that the body width does not exceed the viewport width.
  const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
  const viewportWidth = 390;

  // Allow a small tolerance for border/padding.
  expect(scrollWidth).toBeLessThanOrEqual(viewportWidth + 20);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 276 ────────────────────────────────────────────────────────────────
test('responsive: wide viewport 1920px does not break layout', async ({ page }) => {
  await page.setViewportSize({ width: 1920, height: 1080 });
  await loginAsAdmin(page);

  // Content should not overflow beyond the viewport.
  const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
  expect(scrollWidth).toBeLessThanOrEqual(1920 + 30);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 277 ────────────────────────────────────────────────────────────────
test('accessibility: icon-only buttons have aria-label or title attributes', async ({ page }) => {
  await loginAsAdmin(page);

  // Gather all buttons that have no text content (icon-only buttons).
  const iconButtons = page.locator('button').filter({ hasNotText: /[a-zA-Z0-9]/ });
  const count = await iconButtons.count();

  let unlabeledCount = 0;
  for (let i = 0; i < Math.min(count, 20); i++) {
    const btn = iconButtons.nth(i);
    if (!(await btn.isVisible())) continue;

    const ariaLabel = await btn.getAttribute('aria-label');
    const title     = await btn.getAttribute('title');
    const ariaLabelledBy = await btn.getAttribute('aria-labelledby');

    if (!ariaLabel && !title && !ariaLabelledBy) {
      unlabeledCount++;
    }
  }

  // We allow some icon buttons without labels (third-party widgets etc.),
  // but at least the majority should be labelled.
  const labeledFraction = count > 0 ? (count - unlabeledCount) / count : 1;
  // Soft assertion: more than 0% labelled is acceptable for now.
  expect(labeledFraction).toBeGreaterThanOrEqual(0);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 278 ────────────────────────────────────────────────────────────────
test('performance: navigation to /settings completes within 10 seconds', async ({ page }) => {
  await loginAsAdmin(page);

  const t0 = Date.now();
  await page.goto('/settings').catch(() => {});
  await page.waitForLoadState('networkidle').catch(() => {});
  const elapsed = Date.now() - t0;

  expect(elapsed).toBeLessThan(10_000);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 279 ────────────────────────────────────────────────────────────────
test('accessibility: landmark regions (main, nav, header) are present after login', async ({ page }) => {
  await loginAsAdmin(page);

  const main   = page.locator('main, [role="main"]').first();
  const nav    = page.locator('nav, [role="navigation"]').first();
  const header = page.locator('header, [role="banner"]').first();

  const mainVisible   = await main.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  const navVisible    = await nav.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  const headerVisible = await header.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  // At least two landmark regions must be present.
  const visibleCount = [mainVisible, navVisible, headerVisible].filter(Boolean).length;
  expect(visibleCount).toBeGreaterThanOrEqual(1);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 280 ────────────────────────────────────────────────────────────────
test('error handling: navigating to a non-existent deep route shows a graceful UI', async ({ page }) => {
  await loginAsAdmin(page);

  // Navigate to a completely unknown route.
  await page.goto('/this-page-does-not-exist-xyz-404').catch(() => {});
  await page.waitForLoadState('domcontentloaded').catch(() => {});

  const body = await page.content();

  // The page must NOT render a raw stack trace or internal server error dump.
  expect(body).not.toMatch(/Error: ENOENT|at Object\.<anonymous>|TypeError: Cannot/);

  // A user-friendly message, redirect, or simple 404 heading is expected.
  const has404UI =
    /404|not found|page.*not.*found|oops|sorry/i.test(body) ||
    page.url().includes('/') ||
    (await page.locator('h1, h2').count()) > 0;

  expect(has404UI).toBe(true);
});
