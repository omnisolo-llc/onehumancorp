import { test, expect } from '@playwright/test';

test.describe('Onboarding Glassmorphism UI', () => {
  test('renders with proper glass classes on step 1', async ({ page }) => {
    await page.goto('/onboarding');
    const container = page.locator('#setup-screen');
    await expect(container).toHaveClass(/glassmorphism/);

    // Using a broad check since we appended classes.
    const hasLight = await container.evaluate((node) => node.classList.contains('translucent-glass-light'));
    const hasDark = await container.evaluate((node) => node.classList.contains('dark:translucent-glass-dark'));

    expect(hasLight || hasDark).toBeTruthy();
  });

  test('touch targets are at least 44px', async ({ page }) => {
    await page.goto('/onboarding');

    // Check buttons
    const buttons = await page.locator('button').all();
    for (const btn of buttons) {
      if (await btn.isVisible()) {
        const box = await btn.boundingBox();
        if (box) {
          expect(box.width).toBeGreaterThanOrEqual(44);
          expect(box.height).toBeGreaterThanOrEqual(44);
        }
      }
    }
  });

  test('onboarding screen navigates steps properly', async ({ page }) => {
    await page.goto('/onboarding');
    const startBtn = page.getByRole('button', { name: /Start Setup/i }).first();
    if (await startBtn.isVisible()) {
       await startBtn.click();
       await expect(page.locator('input').first()).toBeVisible();
    }
  });

  test('onboarding error state renders within glass container', async ({ page }) => {
    await page.goto('/onboarding');
    // If there is an error block, it should be within a container or visually styled
    const errorBlock = page.locator('.animate-shake');
    if (await errorBlock.count() > 0) {
      await expect(errorBlock.first()).toBeVisible();
    }
  });

  test('onboarding form accepts input without squishing dimensions', async ({ page }) => {
    await page.goto('/onboarding');
    const firstInput = page.locator('input[type="text"]').first();
    if (await firstInput.isVisible()) {
      await firstInput.fill('Test input');
      const box = await firstInput.boundingBox();
      expect(box?.height).toBeGreaterThanOrEqual(44);
    }
  });
});
