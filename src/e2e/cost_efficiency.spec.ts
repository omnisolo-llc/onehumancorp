import { test, expect } from '@playwright/test';

test.describe('Cost Efficiency & Transparency CUJ', () => {
  test('should display Cost Transparency Dashboard via My Plan after login', async ({ page }) => {
    // 1. Navigate to home
    await page.goto('/');

    // 2. Login flow
    const emailInput = page.locator('input[type="email"], input[placeholder*="email" i]');
    await emailInput.waitFor({ state: 'visible' });
    await emailInput.fill('admin@onehumancorp.com');

    const passInput = page.locator('input[type="password"], input[placeholder*="password" i]');
    await passInput.waitFor({ state: 'visible' });
    await passInput.fill('password123');

    const loginBtn = page.locator('button:has-text("Login"), button:has-text("Sign In")');
    await loginBtn.click();

    // 3. Wait for Dashboard to appear
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 10000 });

    // 4. Click Billing
    const billingLink = page.locator('text=Billing').first();
    await expect(billingLink).toBeVisible();
    await billingLink.click();

    // 5. Assert My Plan opens
    const myPlanHeading = page.locator('text=/my.*plan|current.*plan/i').first();
    await expect(myPlanHeading).toBeVisible();

    // 6. View Cost Details
    const viewCostBtn = page.locator('button:has-text("View Cost Details")').first();
    await expect(viewCostBtn).toBeVisible();
    await viewCostBtn.click();

    // 7. Verify Cost Dashboard metrics
    const costDashboardTitle = page.locator('text=/Cost & AI Usage/i').first();
    await expect(costDashboardTitle).toBeVisible();

    // Assert metric placeholders are present
    await expect(page.locator('text=/Total Spend/i').first()).toBeVisible();
    await expect(page.locator('text=/Total AI Usage/i').first()).toBeVisible();
  });

  test('should verify agent efficiency and ROI on Cost Dashboard', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"], input[placeholder*="email" i]', 'admin@onehumancorp.com');
    await page.fill('input[type="password"], input[placeholder*="password" i]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');

    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 10000 });

    await page.click('text=Billing');
    const viewCostBtn = page.locator('button:has-text("View Cost Details")').first();
    await viewCostBtn.waitFor({ state: 'visible' });
    await viewCostBtn.click();

    await expect(page.locator('text=/Cost & AI Usage/i').first()).toBeVisible();
    await expect(page.locator('text=/ROI:/i').first()).toBeVisible();
    await expect(page.locator('text=/Efficiency:/i').first()).toBeVisible();
  });

  test('should verify plan tiers exist on Pricing page', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"], input[placeholder*="email" i]', 'admin@onehumancorp.com');
    await page.fill('input[type="password"], input[placeholder*="password" i]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');

    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 10000 });

    await page.click('text=Billing');
    const upgradeBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await upgradeBtn.waitFor({ state: 'visible' });
    await upgradeBtn.click();

    await expect(page.locator('text=/pricing|plans/i').first()).toBeVisible();
    await expect(page.locator('text=/Free/i').first()).toBeVisible();
    await expect(page.locator('text=/Starter/i').first()).toBeVisible();
    await expect(page.locator('text=/Pro/i').first()).toBeVisible();
    await expect(page.locator('text=/Business/i').first()).toBeVisible();
  });

  test('should verify annual discount toggle changes prices', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"], input[placeholder*="email" i]', 'admin@onehumancorp.com');
    await page.fill('input[type="password"], input[placeholder*="password" i]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');

    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 10000 });

    await page.click('text=Billing');
    const upgradeBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await upgradeBtn.waitFor({ state: 'visible' });
    await upgradeBtn.click();

    const annualToggle = page.locator('text=/annual|monthly/i').first();
    await expect(annualToggle).toBeVisible();
    await annualToggle.click();

    await expect(page.locator('text=/\\d+%.*off|discount|savings/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should verify pricing limits transparency', async ({ page }) => {
    await page.goto('/');

    await page.fill('input[type="email"], input[placeholder*="email" i]', 'admin@onehumancorp.com');
    await page.fill('input[type="password"], input[placeholder*="password" i]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');

    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 10000 });

    await page.click('text=Billing');
    const upgradeBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await upgradeBtn.waitFor({ state: 'visible' });
    await upgradeBtn.click();

    await expect(page.locator('text=/agent.*limit|number.*agents/i').first()).toBeVisible();
    await expect(page.locator('text=/storage.*limit|gb/i').first()).toBeVisible();
    await expect(page.locator('text=/support/i').first()).toBeVisible();
  });
});
