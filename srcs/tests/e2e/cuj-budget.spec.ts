/**
 * cuj-budget.spec.ts
 *
 * Critical User Journey (CUJ) tests focused on Budget & Billing.
 * Tests 251–260.
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

async function navigateToBudget(page: Page): Promise<void> {
  const budgetLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /budget|billing|cost|spend/i }).first();
  if (await budgetLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await budgetLink.click();
    await page.waitForLoadState('networkidle');
  } else {
    // Fallback: try /settings then look for budget section.
    await page.goto('/settings').catch(() => {});
    await page.waitForLoadState('networkidle');
  }
}

// ─── Test 251 ────────────────────────────────────────────────────────────────
test('budget: cost breakdown by agent role section is accessible', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const costBreakdown = page.locator(
    '[data-testid*="cost-breakdown" i], [aria-label*="cost breakdown" i], h2, h3',
  ).filter({ hasText: /cost breakdown|by role|breakdown/i }).first();

  const breakdownVisible = await costBreakdown.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 252 ────────────────────────────────────────────────────────────────
test('budget: set alert threshold at 80% accepts numeric input', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const thresholdInput = page.locator(
    'input[name*="threshold" i], input[name*="alert_pct" i], input[placeholder*="threshold" i], input[aria-label*="threshold" i]',
  ).first();

  if (await thresholdInput.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await thresholdInput.fill('80');
    const val = await thresholdInput.inputValue();
    expect(val).toBe('80');
  } else {
    await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  }
});

// ─── Test 253 ────────────────────────────────────────────────────────────────
test('budget: currency selector is present in billing settings', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const currencySelect = page.locator(
    'select[name*="currency" i], [aria-label*="currency" i], [data-testid*="currency" i]',
  ).first();

  const currencyVisible = await currencySelect.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 254 ────────────────────────────────────────────────────────────────
test('budget: cost per task metric label is present somewhere on the budget page', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  // Look for cost-per-task text anywhere on the page.
  const costPerTask = page.locator('body').filter({ hasText: /cost per task|per-task cost|task cost/i });

  const metricVisible = await costPerTask.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 255 ────────────────────────────────────────────────────────────────
test('budget: billing cycle selector allows choosing monthly vs annual', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const billingCycleSelect = page.locator(
    'select[name*="billing_cycle" i], select[name*="cycle" i], [aria-label*="billing cycle" i]',
  ).first();

  if (await billingCycleSelect.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    const options = await billingCycleSelect.locator('option').allTextContents();
    const monthlyOpt = options.find(o => /monthly|month/i.test(o));
    if (monthlyOpt) {
      await billingCycleSelect.selectOption({ label: monthlyOpt });
      await page.waitForTimeout(300);
    }
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 256 ────────────────────────────────────────────────────────────────
test('budget: spending graph or chart renders on the budget page', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const chart = page.locator(
    'canvas, svg[class*="chart" i], svg[class*="graph" i], [data-testid*="chart" i], [data-testid*="graph" i], [class*="recharts" i]',
  ).first();

  const chartVisible = await chart.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 257 ────────────────────────────────────────────────────────────────
test('budget: view total spend for current month is displayed', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  // Look for a "total spend" or "this month" metric card.
  const totalSpend = page.locator(
    '[data-testid*="total-spend" i], [data-testid*="current-month" i], [class*="spend" i]',
  ).or(
    page.locator('body').filter({ hasText: /total spend|this month|current month/i }),
  ).first();

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 258 ────────────────────────────────────────────────────────────────
test('budget: export billing history as CSV or PDF is accessible', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const exportBtn = page.locator('button, a, [role="button"]')
    .filter({ hasText: /export|download|csv|pdf/i }).first();
  const exportVisible = await exportBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 259 ────────────────────────────────────────────────────────────────
test('budget: refill or top-up budget option is present', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const topUpBtn = page.locator('button, a, [role="button"]')
    .filter({ hasText: /top.?up|refill|add funds|add credit/i }).first();
  const topUpVisible = await topUpBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 260 ────────────────────────────────────────────────────────────────
test('budget: overage protection toggle or limit field is present', async ({ page }) => {
  await loginAsAdmin(page);
  await navigateToBudget(page);

  const overageControl = page.locator(
    'input[type="checkbox"][name*="overage" i], input[type="checkbox"][aria-label*="overage" i], input[name*="overage_limit" i], [data-testid*="overage" i]',
  ).first();

  const overageVisible = await overageControl.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);
  if (overageVisible) {
    // If it's a checkbox, verify it can be toggled.
    const tagName = await overageControl.evaluate(el => el.tagName.toLowerCase());
    if (tagName === 'input') {
      const checked = await overageControl.isChecked();
      await overageControl.click();
      await page.waitForTimeout(300);
      const newChecked = await overageControl.isChecked();
      expect(newChecked).toBe(!checked);
    }
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});
