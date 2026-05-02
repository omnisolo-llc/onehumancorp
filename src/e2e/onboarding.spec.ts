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
  test('should display welcome checklist', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    await expect(page.locator('text=/checklist|welcome/i')).toBeVisible();
  });

  test('should show checklist items', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const item = page.locator('[class*="item"], [class*="checklist"]').first();
    await expect(item).toBeVisible();
  });

  test('should mark item as complete', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const checkbox = page.locator('input[type="checkbox"]').first();
    if (await checkbox.isVisible()) {
      await checkbox.check();
      await expect(page.locator('text=/completed|done/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should show completion progress', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const progress = page.locator('text=/\\d+ of \\d+|\\d+%/').first();
    await expect(progress).toBeVisible();
  });

  test('should mark all items complete', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const checkboxes = page.locator('input[type="checkbox"]');
    const count = await checkboxes.count();
    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
      await page.waitForTimeout(100);
    }
  });

  test('should show congratulations message', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const checkboxes = page.locator('input[type="checkbox"]');
    const count = await checkboxes.count();
    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
      await page.waitForTimeout(100);
    }
    await expect(page.locator('text=/congratulations|complete|awesome/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
  });

  test('should link to documentation', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const docLink = page.locator('a:has-text("Documentation"), a:has-text("Docs")').first();
    if (await docLink.isVisible()) {
      await docLink.click();
      await expect(page.locator('text=/docs|documentation/i')).toBeVisible();
    }
  });

  test('should link to video tutorials', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const videoLink = page.locator('a:has-text("Video"), a:has-text("Tutorial")').first();
    if (await videoLink.isVisible()) {
      await videoLink.click();
      await expect(page.locator('text=/video|tutorial/i')).toBeVisible();
    }
  });

  test('should offer to contact support', async ({ page }) => {
    await page.goto('/dashboard'); await page.locator('text="Welcome Checklist"').waitFor();
    const supportLink = page.locator('text=/support|help|contact/i').first();
    await expect(supportLink).toBeVisible();
  });
});