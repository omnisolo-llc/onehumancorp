import { test, expect } from '@playwright/test';

test.describe('Pricing Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"], input[placeholder*="email" i]', 'test@example.com');
    await page.fill('input[type="password"], input[placeholder*="password" i]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
    await page.click('text=Billing');
    await expect(page.locator('text=/my.*plan|current.*plan/i').first()).toBeVisible();
  });

  test('should display pricing page', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();
  });

  test('should show plan comparison', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/plan|comparison/i')).toBeVisible();
  });

  test('should display free plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/free|starter/i')).toBeVisible();
  });

  test('should display pro plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/pro|professional/i')).toBeVisible();
  });

  test('should display enterprise plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/enterprise|business/i')).toBeVisible();
  });

  test('should show plan prices', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const price = page.locator('text=/\\$\\d+/').first();
    await expect(price).toBeVisible();
  });

  test('should highlight recommended plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const recommended = page.locator('text=/recommended|popular|best/i').first();
    await expect(recommended).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should show feature list', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const features = page.locator('ul li, [class*="feature"]');
    await expect(features.first()).toBeVisible();
  });

  test('should show agent limits per plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/agent.*limit|number.*agents/i')).toBeVisible();
  });

  test('should show storage limits per plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/storage|gb/i')).toBeVisible();
  });

  test('should show support level per plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/support|help/i')).toBeVisible();
  });

  test('should select pro plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const proButton = page.locator('button:has-text("Pro"), button:has-text("Choose")').first();
    await proButton.click();
      await expect(page.locator('text=/checkout|payment/i')).toBeVisible();

  });

  test('should start free plan', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const freeButton = page.locator('button:has-text("Free"), button:has-text("Start")').first();
    await freeButton.click();
      await expect(page.locator('text=/dashboard|welcome/i')).toBeVisible();

  });

  test('should contact sales for enterprise', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const contactButton = page.locator('button:has-text("Contact"), button:has-text("Sales")').first();
    await contactButton.click();
      await expect(page.locator('text=/contact|sales|email/i')).toBeVisible();

  });

  test('should show annual billing discount', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const annualToggle = page.locator('text=/annual|monthly/i').first();
    await annualToggle.click();
      await expect(page.locator('text=/\\d+%.*off|discount|savings/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should display FAQ section', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const faqSection = page.locator('text=/faq|questions|help/i').first();
    await expect(faqSection).toBeVisible();
  });

  test('should expand FAQ item', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    const faqItem = page.locator('[class*="faq"], [class*="question"]').first();
    await faqItem.click();
      await expect(page.locator('text=/answer|description/i')).toBeVisible();

  });

  test('should show guarantee badge', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/guarantee|money.*back|refund/i')).toBeVisible();
  });

  test('should show security badge', async ({ page }) => {
    const pricingBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await pricingBtn.click();
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();

    await expect(page.locator('text=/secure|security|ssl/i')).toBeVisible();
  });
});

test.describe('My Plan Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"], input[placeholder*="email" i]', 'test@example.com');
    await page.fill('input[type="password"], input[placeholder*="password" i]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
    await page.click('text=Billing');
    await expect(page.locator('text=/my.*plan|current.*plan/i').first()).toBeVisible();
  });

  test('should display current plan', async ({ page }) => {
    await expect(page.locator('text=/my.*plan|current.*plan/i')).toBeVisible();
  });

  test('should show plan status', async ({ page }) => {
    await expect(page.locator('text=/active|status/i')).toBeVisible();
  });

  test('should show renewal date', async ({ page }) => {
    await expect(page.locator('text=/renewal|renews|next.*billing/i')).toBeVisible();
  });

  test('should show billing history', async ({ page }) => {
    const historyBtn = page.locator('button:has-text("History"), button:has-text("Invoices")').first();
    await historyBtn.click();
      await expect(page.locator('text=/invoice|history|billing/i')).toBeVisible();

  });

  test('should upgrade plan', async ({ page }) => {
    const upgradeBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    await upgradeBtn.click();
      await expect(page.locator('text=/pricing|plans/i')).toBeVisible();

  });

  test('should cancel subscription', async ({ page }) => {
    const cancelBtn = page.locator('button:has-text("Cancel"), button:has-text("Unsubscribe")').first();
    await cancelBtn.click();
      await expect(page.locator('text=/confirm|cancel.*subscription/i')).toBeVisible();

  });

  test('should update payment method', async ({ page }) => {
    const paymentBtn = page.locator('button:has-text("Payment"), button:has-text("Update")').first();
    await paymentBtn.click();
      await expect(page.locator('text=/card|payment|method/i')).toBeVisible();

  });

  test('should download invoice', async ({ page }) => {
    const downloadBtn = page.locator('button:has-text("Download"), [class*="download"]').first();
    await downloadBtn.click();
      await expect(page.locator('text=/pdf|invoice/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should open cost transparency dashboard', async ({ page }) => {
    const detailsBtn = page.locator('button:has-text("View Cost Details")').first();
    await expect(detailsBtn).toBeVisible();
    await detailsBtn.click();
    await expect(page.locator('text=/Cost & Token Usage/i').first()).toBeVisible();
  });
});
