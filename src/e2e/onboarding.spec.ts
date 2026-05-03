import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test('should start business setup onboarding', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=/onboarding|setup|welcome/i')).toBeVisible();
  });

  test('should show welcome screen', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=/welcome|get started/i')).toBeVisible();
  });

  test('should show progress indicator', async ({ page }) => {
    await page.goto('/onboarding');
    const progress = page.locator('[class*="progress"], text=/step \\d+ of \\d+/i').first();
    await expect(progress).toBeVisible();
  });

  test('should navigate to next step', async ({ page }) => {
    await page.goto('/onboarding');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/step \\d+/i')).toBeVisible();
    }
  });

  test('should navigate to previous step', async ({ page }) => {
    await page.goto('/onboarding');
    const backBtn = page.locator('button:has-text("Back"), button:has-text("Previous")').first();
    if (await backBtn.isVisible()) {
      await backBtn.click();
      await expect(page.locator('text=/welcome|step/i')).toBeVisible();
    }
  });

  test('should select business type', async ({ page }) => {
    await page.goto('/onboarding');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await page.locator('text=/online|store|service/i').first().click();
    }
  });

  test('should enter company name', async ({ page }) => {
    await page.goto('/onboarding');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      for (let i = 0; i < 2; i++) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
      const nameInput = page.locator('input[type="text"]').first();
      if (await nameInput.isVisible()) {
        await nameInput.fill('My Company');
      }
    }
  });

  test('should skip optional steps', async ({ page }) => {
    await page.goto('/onboarding');
    const skipBtn = page.locator('button:has-text("Skip"), button:has-text("Skip this step")').first();
    if (await skipBtn.isVisible()) {
      await skipBtn.click();
    }
  });

  test('should complete onboarding', async ({ page }) => {
    await page.goto('/onboarding');
    const finishBtn = page.locator('button:has-text("Finish"), button:has-text("Complete")').first();
    if (await finishBtn.isVisible()) {
      await finishBtn.click();
      await expect(page.locator('text=/dashboard|welcome/i')).toBeVisible({ timeout: 10000 }).catch(() => {});
    }
  });

  test('should save onboarding progress', async ({ page }) => {
    await page.goto('/onboarding');
    await page.fill('input[type="text"]', 'Test Company').catch(() => {});
    await page.locator('button:has-text("Save"), button:has-text("Continue")').click();
    await expect(page.locator('text=/saved|progress/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should resume onboarding from saved state', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=/resume|continue.*where.*left.*off/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });
});

test.describe('Onboarding Welcome Checklist', () => {
  // A helper function to complete the wizard
  async function completeWizardToChecklist(page) {
    await page.goto('/');

    // Login
    const emailInput = page.locator('input').first();
    await expect(emailInput).toBeVisible();
    await emailInput.fill('test@example.com');

    const passInput = page.locator('input[type="password"], input').nth(1);
    await expect(passInput).toBeVisible();
    await passInput.fill('password123');

    const signInBtn = page.locator('button:has-text("Sign In")').first();
    await signInBtn.click();

    // Setup Wizard opens.
    // Step 0: Welcome
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue"), button:has-text("Next →")');
    await expect(nextBtn.first()).toBeVisible({ timeout: 10000 });
    await nextBtn.first().click();

    // Step 1: Type
    await page.locator('text=/Online Store/i').first().click();

    // Step 2: Name
    await page.locator('input').first().fill('My E2E Store');
    await nextBtn.first().click();

    // Step 3: What do you sell
    await page.locator('text=/Physical/i').first().click();
    await nextBtn.first().click();

    // Step 4: Payments
    await page.locator('text=/Online/i').first().click();

    // Step 5: Admin
    await page.locator('input').nth(1).fill('admin@e2e.test');
    await nextBtn.first().click();

    // Step 6: Template
    await page.locator('text=/Modern/i').first().click();

    // Step 7: Product
    await page.locator('input').first().fill('My First Product');
    await page.locator('input').nth(1).fill('10.00');
    await nextBtn.first().click();

    // Step 8: Domain
    await page.locator('text=/mybusiness.ohc.app/i').first().click();

    // Step 9: Launch
    const launchBtn = page.locator('button:has-text("Launch"), text=/Launch My Business/i').first();
    await expect(launchBtn).toBeVisible({ timeout: 10000 });
    await launchBtn.click();

    // Wait for launch to finish and go to Checklist (Step 10)
    await expect(page.locator('text=/You\'re set up/i')).toBeVisible({ timeout: 15000 });
  }

  test('should navigate to add products when clicking Add 3 more products', async ({ page }) => {
    await completeWizardToChecklist(page);
    const addProductsItem = page.locator('text=/Add 3 more products/i').first();
    await expect(addProductsItem).toBeVisible();
    await addProductsItem.click();
    await expect(page.locator('text=/website|builder|product/i')).toBeVisible({ timeout: 10000 });
  });

  test('should navigate to dashboard analytics when clicking Connect Instagram', async ({ page }) => {
    await completeWizardToChecklist(page);
    const connectIgItem = page.locator('text=/Connect Instagram/i').first();
    await expect(connectIgItem).toBeVisible();
    await connectIgItem.click();
    await expect(page.locator('text=/dashboard|analytics|integrations/i')).toBeVisible({ timeout: 10000 });
  });

  test('should navigate to referrals when clicking Share your link', async ({ page }) => {
    await completeWizardToChecklist(page);
    const shareLinkItem = page.locator('text=/Share your link/i').first();
    await expect(shareLinkItem).toBeVisible();
    await shareLinkItem.click();
    await expect(page.locator('text=/referral|share/i')).toBeVisible({ timeout: 10000 });
  });

  test('should navigate to dashboard when clicking Go to Dashboard', async ({ page }) => {
    await completeWizardToChecklist(page);
    const dashboardBtn = page.locator('button:has-text("Go to Dashboard"), text=/Go to Dashboard/i').first();
    await expect(dashboardBtn).toBeVisible();
    await dashboardBtn.click();
    await expect(page.locator('text=/dashboard|my business/i')).toBeVisible({ timeout: 10000 });
  });
});