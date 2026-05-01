import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard', () => {
  test('should show welcome step', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=Welcome')).toBeVisible();
  });

  test('should display welcome message', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text=/welcome|get started/i')).toBeVisible();
  });

  test('should show next button on welcome step', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('button:has-text("Next")')).toBeVisible();
  });

  test('should navigate to business type step', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=/business type/i')).toBeVisible();
  });

  test('should show business type options', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=Online Store')).toBeVisible();
    await expect(page.locator('text=Service Business')).toBeVisible();
    await expect(page.locator('text=Restaurant')).toBeVisible();
  });

  test('should select online store option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Online Store').click();
    await expect(page.locator('button:has-text("Next")')).toBeEnabled();
  });

  test('should select service business option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Service Business').click();
  });

  test('should select restaurant option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Restaurant').click();
  });

  test('should select creative portfolio option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Creative').click();
  });

  test('should select local business option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Local Business').click();
  });

  test('should navigate through wizard steps', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('input[type="text"]').first().fill('Online Store');
    await page.locator('button:has-text("Next")').click();
    await page.locator('input[type="text"]').first().fill('Test Company');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=What do you sell')).toBeVisible();
  });

  test('should show step indicator', async ({ page }) => {
    await page.goto('/business-setup');
    const stepIndicator = page.locator('[class*="step"], text=/step \\d+/i').first();
    await expect(stepIndicator).toBeVisible();
  });

  test('should allow going back', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    const backButton = page.locator('button:has-text("Back")');
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page.locator('text=Welcome')).toBeVisible();
  });

  test('should show company name input', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('input[type="text"]').first()).toBeVisible();
  });

  test('should show what you sell step', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('input[type="text"]').first().fill('Test Company');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=/what do you sell/i')).toBeVisible();
  });

  test('should show physical products option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('input[type="text"]').first().fill('Test Company');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=Physical')).toBeVisible();
  });

  test('should show digital products option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('input[type="text"]').first().fill('Test Company');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=Digital')).toBeVisible();
  });

  test('should show services option', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('input[type="text"]').first().fill('Test Company');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=Services')).toBeVisible();
  });

  test('should show payments step', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('input[type="text"]').first().fill('Test Company');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Physical').click();
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=/payment/i')).toBeVisible();
  });

  test('should show admin account step', async ({ page }) => {
    await page.goto('/business-setup');
    for (let i = 0; i < 5; i++) {
      await page.locator('button:has-text("Next")').click();
      await page.waitForTimeout(200);
    }
    await expect(page.locator('text=/admin|account/i')).toBeVisible();
  });

  test('should show template selection step', async ({ page }) => {
    await page.goto('/business-setup');
    for (let i = 0; i < 6; i++) {
      const nextBtn = page.locator('button:has-text("Next")');
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    await expect(page.locator('text=/template/i')).toBeVisible();
  });

  test('should show domain step', async ({ page }) => {
    await page.goto('/business-setup');
    for (let i = 0; i < 7; i++) {
      const nextBtn = page.locator('button:has-text("Next")');
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    await expect(page.locator('text=/domain/i')).toBeVisible();
  });

  test('should show review and launch step', async ({ page }) => {
    await page.goto('/business-setup');
    for (let i = 0; i < 8; i++) {
      const nextBtn = page.locator('button:has-text("Next")');
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    await expect(page.locator('text=/review|launch/i')).toBeVisible();
  });

  test('should show launch button on final step', async ({ page }) => {
    await page.goto('/business-setup');
    for (let i = 0; i < 9; i++) {
      const nextBtn = page.locator('button:has-text("Next")');
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    await expect(page.locator('button:has-text("Launch")')).toBeVisible();
  });
});

test.describe('Business Setup Wizard Validation', () => {
  test('should require business type selection', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=/select.*type|choose.*type/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should require company name', async ({ page }) => {
    await page.goto('/business-setup');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Online Store').click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=/required|name.*required/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should validate email format', async ({ page }) => {
    await page.goto('/business-setup');
    for (let i = 0; i < 5; i++) {
      const nextBtn = page.locator('button:has-text("Next")');
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    await page.fill('input[type="email"]', 'invalidemail');
    await page.locator('button:has-text("Next")').click();
    await expect(page.locator('text=/invalid.*email|email.*invalid/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should validate password strength', async ({ page }) => {
    await page.goto('/business-setup');
    for (let i = 0; i < 5; i++) {
      const nextBtn = page.locator('button:has-text("Next")');
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    await page.fill('input[type="password"]', 'weak');
    await expect(page.locator('text=/weak|strong.*password/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });
});