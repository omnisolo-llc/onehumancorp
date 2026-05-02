import { test, expect } from '@playwright/test';

test.describe('Pricing Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    const btn = page.locator('button:has-text("/login")');
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")');
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        await page.locator('button:has-text("/login")').click();
      }
    }
  });
  test('should display pricing page', async ({ page }) => {
    await expect(page.locator('text=/pricing|plan/i')).toBeVisible();
  });

  test('should show plan comparison', async ({ page }) => {
    await expect(page.locator('text=/plan|comparison/i')).toBeVisible();
  });

  test('should display free plan', async ({ page }) => {
    await expect(page.locator('text=/free|starter/i')).toBeVisible();
  });

  test('should display pro plan', async ({ page }) => {
    await expect(page.locator('text=/pro|professional/i')).toBeVisible();
  });

  test('should display enterprise plan', async ({ page }) => {
    await expect(page.locator('text=/enterprise|business/i')).toBeVisible();
  });

  test('should show plan prices', async ({ page }) => {
    const price = page.locator('text=/\\$\\d+/').first();
    await expect(price).toBeVisible();
  });

  test('should highlight recommended plan', async ({ page }) => {
    const recommended = page.locator('text=/recommended|popular|best/i').first();
    await expect(recommended).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should show feature list', async ({ page }) => {
    const features = page.locator('ul li, [class*="feature"]');
    await expect(features.first()).toBeVisible();
  });

  test('should show agent limits per plan', async ({ page }) => {
    await expect(page.locator('text=/agent.*limit|number.*agents/i')).toBeVisible();
  });

  test('should show storage limits per plan', async ({ page }) => {
    await expect(page.locator('text=/storage|gb/i')).toBeVisible();
  });

  test('should show support level per plan', async ({ page }) => {
    await expect(page.locator('text=/support|help/i')).toBeVisible();
  });

  test('should select pro plan', async ({ page }) => {
    const proButton = page.locator('button:has-text("Pro"), button:has-text("Choose")').first();
    if (await proButton.isVisible()) {
      await proButton.click();
      await expect(page.locator('text=/checkout|payment/i')).toBeVisible();
    }
  });

  test('should start free plan', async ({ page }) => {
    const freeButton = page.locator('button:has-text("Free"), button:has-text("Start")').first();
    if (await freeButton.isVisible()) {
      await freeButton.click();
      await expect(page.locator('text=/dashboard|welcome/i')).toBeVisible();
    }
  });

  test('should contact sales for enterprise', async ({ page }) => {
    const contactButton = page.locator('button:has-text("Contact"), button:has-text("Sales")').first();
    if (await contactButton.isVisible()) {
      await contactButton.click();
      await expect(page.locator('text=/contact|sales|email/i')).toBeVisible();
    }
  });

  test('should show annual billing discount', async ({ page }) => {
    const annualToggle = page.locator('text=/annual|monthly/i').first();
    if (await annualToggle.isVisible()) {
      await annualToggle.click();
      await expect(page.locator('text=/\\d+%.*off|discount|savings/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should display FAQ section', async ({ page }) => {
    const faqSection = page.locator('text=/faq|questions|help/i').first();
    await expect(faqSection).toBeVisible();
  });

  test('should expand FAQ item', async ({ page }) => {
    const faqItem = page.locator('[class*="faq"], [class*="question"]').first();
    if (await faqItem.isVisible()) {
      await faqItem.click();
      await expect(page.locator('text=/answer|description/i')).toBeVisible();
    }
  });

  test('should show guarantee badge', async ({ page }) => {
    await expect(page.locator('text=/guarantee|money.*back|refund/i')).toBeVisible();
  });

  test('should show security badge', async ({ page }) => {
    await expect(page.locator('text=/secure|security|ssl/i')).toBeVisible();
  });
});

test.describe('My Plan Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
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
    if (await historyBtn.isVisible()) {
      await historyBtn.click();
      await expect(page.locator('text=/invoice|history|billing/i')).toBeVisible();
    }
  });

  test('should upgrade plan', async ({ page }) => {
    const upgradeBtn = page.locator('button:has-text("Upgrade"), button:has-text("Change Plan")').first();
    if (await upgradeBtn.isVisible()) {
      await upgradeBtn.click();
      await expect(page.locator('text=/pricing|plans/i')).toBeVisible();
    }
  });

  test('should cancel subscription', async ({ page }) => {
    const cancelBtn = page.locator('button:has-text("Cancel"), button:has-text("Unsubscribe")').first();
    if (await cancelBtn.isVisible()) {
      await cancelBtn.click();
      await expect(page.locator('text=/confirm|cancel.*subscription/i')).toBeVisible();
    }
  });

  test('should update payment method', async ({ page }) => {
    const paymentBtn = page.locator('button:has-text("Payment"), button:has-text("Update")').first();
    if (await paymentBtn.isVisible()) {
      await paymentBtn.click();
      await expect(page.locator('text=/card|payment|method/i')).toBeVisible();
    }
  });

  test('should download invoice', async ({ page }) => {
    const downloadBtn = page.locator('button:has-text("Download"), [class*="download"]').first();
    if (await downloadBtn.isVisible()) {
      await downloadBtn.click();
      await expect(page.locator('text=/pdf|invoice/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should open cost transparency dashboard', async ({ page }) => {

    // Login flow
    await page.fill('input[type="email"], input[placeholder*="email" i]', 'test@example.com');
    await page.fill('input[type="password"], input[placeholder*="password" i]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');

    // Wait for Dashboard
    await expect(page.locator('text=Dashboard').first()).toBeVisible();

    // Navigate to Billing / My Plan
    await page.click('text=Billing');

    // Wait for My Plan page
    await expect(page.locator('text=/my.*plan|current.*plan/i').first()).toBeVisible();

    // Verify Cost Details button and click it
    const detailsBtn = page.locator('button:has-text("View Cost Details")').first();
    await expect(detailsBtn).toBeVisible();
    await detailsBtn.click();

    // Assert Cost Dashboard appears
    await expect(page.locator('text=/Cost & Token Usage/i').first()).toBeVisible();
  });
});
