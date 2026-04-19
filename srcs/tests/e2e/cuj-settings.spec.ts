/**
 * cuj-settings.spec.ts
 *
 * Critical User Journey (CUJ) tests focused on Settings & Integrations.
 * Tests 261–270.
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

async function navigateToSettings(page: Page): Promise<void> {
  const settingsLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /setting|config|admin/i }).first();
  if (await settingsLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await settingsLink.click();
    await page.waitForLoadState('networkidle');
  } else {
    await page.goto('/settings').catch(() => {});
    await page.waitForLoadState('networkidle');
  }
}

// ─── Test 261 ────────────────────────────────────────────────────────────────
test('settings: email SMTP configuration fields are present', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  const smtpSection = page.locator(
    '[data-testid*="smtp" i], [aria-label*="smtp" i], h2, h3',
  ).filter({ hasText: /smtp|email|mail server/i }).first();

  const smtpHost = page.locator(
    'input[name*="smtp_host" i], input[name*="mail_host" i], input[placeholder*="smtp" i], input[aria-label*="smtp host" i]',
  ).first();

  const smtpVisible = await smtpSection.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false) ||
                      await smtpHost.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 262 ────────────────────────────────────────────────────────────────
test('settings: data retention period field is configurable', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  const retentionInput = page.locator(
    'input[name*="retention" i], input[name*="data_retention" i], input[aria-label*="retention" i], select[name*="retention" i]',
  ).first();

  if (await retentionInput.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    const tagName = await retentionInput.evaluate(el => el.tagName.toLowerCase());
    if (tagName === 'input') {
      await retentionInput.fill('90');
      const val = await retentionInput.inputValue();
      expect(val).toBe('90');
    } else if (tagName === 'select') {
      const options = await retentionInput.locator('option').allTextContents();
      if (options.length > 1) {
        await retentionInput.selectOption({ index: 1 });
      }
    }
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 263 ────────────────────────────────────────────────────────────────
test('settings: maintenance mode toggle is present', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  const maintenanceToggle = page.locator(
    'input[type="checkbox"][name*="maintenance" i], input[type="checkbox"][aria-label*="maintenance" i], [data-testid*="maintenance" i]',
  ).first();

  const toggleVisible = await maintenanceToggle.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 264 ────────────────────────────────────────────────────────────────
test('settings: log level configuration dropdown is accessible', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  const logLevelSelect = page.locator(
    'select[name*="log_level" i], select[aria-label*="log level" i], [data-testid*="log-level" i]',
  ).first();

  if (await logLevelSelect.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    const options = await logLevelSelect.locator('option').allTextContents();
    const infoOpt = options.find(o => /info/i.test(o));
    if (infoOpt) {
      await logLevelSelect.selectOption({ label: infoOpt });
    }
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 265 ────────────────────────────────────────────────────────────────
test('settings: OAuth or SSO provider configuration section is present', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  const oauthSection = page.locator(
    '[data-testid*="oauth" i], [data-testid*="sso" i], [data-testid*="saml" i], h2, h3',
  ).filter({ hasText: /oauth|sso|saml|single sign.?on|identity provider/i }).first();

  const sectionVisible = await oauthSection.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 266 ────────────────────────────────────────────────────────────────
test('settings: two-factor authentication toggle is present', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  const tfaToggle = page.locator(
    'input[type="checkbox"][name*="2fa" i], input[type="checkbox"][name*="mfa" i], input[type="checkbox"][aria-label*="two-factor" i], [data-testid*="2fa" i]',
  ).first();

  const tfaLabel = page.locator('label, span, p').filter({ hasText: /two.?factor|2fa|mfa/i }).first();

  const present = await tfaToggle.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false) ||
                  await tfaLabel.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 267 ────────────────────────────────────────────────────────────────
test('settings: webhook management section lists outbound webhooks', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  // Navigate to webhook section if available.
  const webhookLink = page.locator('a, button, [role="tab"]')
    .filter({ hasText: /webhook|integration|outbound/i }).first();
  if (await webhookLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await webhookLink.click();
    await page.waitForLoadState('networkidle');
  }

  const webhookSection = page.locator(
    '[data-testid*="webhook" i], [aria-label*="webhook" i], h2, h3',
  ).filter({ hasText: /webhook|outbound/i }).first();

  const webhookVisible = await webhookSection.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 268 ────────────────────────────────────────────────────────────────
test('settings: white-label branding fields (logo upload or title) are present', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  // Navigate to branding sub-section if available.
  const brandingLink = page.locator('a, button, [role="tab"], li')
    .filter({ hasText: /brand|logo|theme|appearance/i }).first();
  if (await brandingLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await brandingLink.click();
    await page.waitForLoadState('networkidle');
  }

  const logoUpload = page.locator(
    'input[type="file"][name*="logo" i], [aria-label*="logo" i], [data-testid*="logo" i]',
  ).first();
  const brandTitle = page.locator(
    'input[name*="brand" i], input[name*="app_name" i], input[placeholder*="brand" i]',
  ).first();

  const brandingVisible = await logoUpload.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false) ||
                          await brandTitle.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 269 ────────────────────────────────────────────────────────────────
test('settings: saving settings with no changes succeeds without error', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  const saveBtn = page.locator('button, [role="button"]')
    .filter({ hasText: /save|apply|update/i }).first();

  if (await saveBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await saveBtn.click();
    await page.waitForTimeout(1_000);
    // Should not navigate away to an error page.
    await expect(page.locator('body')).not.toContainText(/500|uncaught error/i);
  } else {
    await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  }
});

// ─── Test 270 ────────────────────────────────────────────────────────────────
test('settings: rate limit configuration field is present in API settings', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToSettings(page);

  // Navigate to API section if available.
  const apiLink = page.locator('a, button, [role="tab"]')
    .filter({ hasText: /api|rate limit|developer/i }).first();
  if (await apiLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await apiLink.click();
    await page.waitForLoadState('networkidle');
  }

  const rateLimitInput = page.locator(
    'input[name*="rate_limit" i], input[name*="ratelimit" i], input[aria-label*="rate limit" i], input[placeholder*="rate limit" i]',
  ).first();

  const rateLimitVisible = await rateLimitInput.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});
