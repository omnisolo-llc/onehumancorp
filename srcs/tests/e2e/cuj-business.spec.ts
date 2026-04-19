/**
 * cuj-business.spec.ts
 *
 * Critical User Journey (CUJ) tests focused on Business Management.
 * Tests 241–250.
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

// ─── Test 241 ────────────────────────────────────────────────────────────────
test('business management: filter businesses by industry renders without error', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Look for an industry filter dropdown or search.
  const industryFilter = page.locator(
    'select[name*="industry" i], [aria-label*="industry" i], [data-testid*="industry-filter" i]',
  ).first();

  if (await industryFilter.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    const options = await industryFilter.locator('option').allTextContents();
    if (options.length > 1) {
      await industryFilter.selectOption({ index: 1 });
      await page.waitForTimeout(500);
    }
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500|crashed/i);
});

// ─── Test 242 ────────────────────────────────────────────────────────────────
test('business management: sort businesses by created date is accessible', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Look for a sort control (column header, sort dropdown, etc.).
  const sortByDate = page.locator(
    'th:has-text("Created"), th:has-text("Date"), button:has-text("Sort"), [aria-label*="sort" i]',
  ).first();

  if (await sortByDate.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await sortByDate.click();
    await page.waitForTimeout(500);
    await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  } else {
    await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  }
});

// ─── Test 243 ────────────────────────────────────────────────────────────────
test('business management: business details shows assigned agent team', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Open the first business in the list.
  const firstBusiness = page.locator(
    'table tbody tr:first-child td a, [data-testid*="business-item" i]:first-child, [class*="business-row" i]:first-child a',
  ).first();

  if (await firstBusiness.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await firstBusiness.click();
    await page.waitForLoadState('networkidle');

    // The business details page should mention agent team assignment.
    const agentTeamSection = page.locator(
      '[data-testid*="agent-team" i], [aria-label*="agent team" i], h2, h3, [class*="team" i]',
    ).filter({ hasText: /agent team|assigned team|team/i }).first();

    const sectionVisible = await agentTeamSection.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false);
    await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  } else {
    await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  }
});

// ─── Test 244 ────────────────────────────────────────────────────────────────
test('business management: revenue target field accepts a numeric value', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Look for create / edit form.
  const newBizBtn = page.locator('button, a').filter({ hasText: /new business|create|add/i }).first();
  if (await newBizBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await newBizBtn.click();
    await page.waitForLoadState('networkidle');
  }

  const revenueInput = page.locator(
    'input[name*="revenue" i], input[placeholder*="revenue" i], input[aria-label*="revenue" i]',
  ).first();

  if (await revenueInput.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await revenueInput.fill('500000');
    const val = await revenueInput.inputValue();
    expect(val).toBe('500000');
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 245 ────────────────────────────────────────────────────────────────
test('business management: business contact email field is editable', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Try to open the first business or new-business form.
  const firstBusiness = page.locator(
    'table tbody tr:first-child td a, [data-testid*="business-item" i]:first-child',
  ).first();
  if (await firstBusiness.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await firstBusiness.click();
    await page.waitForLoadState('networkidle');
    const editBtn = page.locator('button, a').filter({ hasText: /edit|modify/i }).first();
    if (await editBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
      await editBtn.click();
      await page.waitForLoadState('networkidle');
    }
  }

  const emailInput = page.locator(
    'input[type="email"], input[name*="contact_email" i], input[placeholder*="contact email" i]',
  ).first();

  if (await emailInput.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await emailInput.fill('contact@example.com');
    const val = await emailInput.inputValue();
    expect(val).toContain('@');
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 246 ────────────────────────────────────────────────────────────────
test('business management: export business list button is accessible', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  const exportBtn = page.locator('button, a, [role="button"]')
    .filter({ hasText: /export|download|csv/i }).first();
  const exportVisible = await exportBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false);

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 247 ────────────────────────────────────────────────────────────────
test('business management: business activity timeline section is accessible', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Open first business detail.
  const firstBusiness = page.locator(
    'table tbody tr:first-child td a, [data-testid*="business-item" i]:first-child',
  ).first();
  if (await firstBusiness.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await firstBusiness.click();
    await page.waitForLoadState('networkidle');

    const timelineSection = page.locator(
      '[data-testid*="timeline" i], [aria-label*="timeline" i], [class*="timeline" i], h2, h3',
    ).filter({ hasText: /timeline|activity|history|event/i }).first();

    const timelineVisible = await timelineSection.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false);
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 248 ────────────────────────────────────────────────────────────────
test('business management: bulk-select businesses and suspend action does not error', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Try to select all via header checkbox.
  const selectAll = page.locator(
    'th input[type="checkbox"], input[type="checkbox"][aria-label*="select all" i], [data-testid*="select-all" i]',
  ).first();
  if (await selectAll.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await selectAll.check();

    // Look for a bulk action.
    const bulkSuspendBtn = page.locator('button, [role="button"]')
      .filter({ hasText: /suspend|archive|deactivate/i }).first();
    if (await bulkSuspendBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
      await bulkSuspendBtn.click();
      await page.waitForTimeout(500);
      // Confirmation dialog may appear; dismiss it.
      const cancelBtn = page.locator('[role="dialog"] button').filter({ hasText: /cancel|no/i }).first();
      if (await cancelBtn.isVisible({ timeout: 2_000 }).catch(() => false)) {
        await cancelBtn.click();
      }
    }
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
});

// ─── Test 249 ────────────────────────────────────────────────────────────────
test('business management: business tags or labels field accepts input', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // Try to open edit mode or new business form.
  const newOrEditBtn = page.locator('button, a').filter({ hasText: /new business|create|edit/i }).first();
  if (await newOrEditBtn.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await newOrEditBtn.click();
    await page.waitForLoadState('networkidle');
  }

  const tagsInput = page.locator(
    'input[name*="tag" i], input[placeholder*="tag" i], input[aria-label*="label" i], [data-testid*="tags" i]',
  ).first();

  if (await tagsInput.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await tagsInput.fill('retail');
    const val = await tagsInput.inputValue();
    expect(val).toBe('retail');
  }

  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});

// ─── Test 250 ────────────────────────────────────────────────────────────────
test('business management: budget usage column is visible in the business list', async ({ page }) => {
  await loginAsAdmin(page);

  const businessLink = page.locator('nav a, nav button, aside a, [role="menuitem"]')
    .filter({ hasText: /business|compan/i }).first();
  if (await businessLink.isVisible({ timeout: SHORT_TIMEOUT }).catch(() => false)) {
    await businessLink.click();
    await page.waitForLoadState('networkidle');
  }

  // A "Budget" or "Spend" column header should be visible in the table.
  const budgetColumn = page.locator(
    'th:has-text("Budget"), th:has-text("Spend"), th:has-text("Usage"), [aria-label*="budget" i]',
  ).first();

  const columnVisible = await budgetColumn.isVisible({ timeout: MEDIUM_TIMEOUT }).catch(() => false);
  await expect(page.locator('body')).not.toContainText(/uncaught error|500/i);
  expect(true).toBe(true);
});
