import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    const id = `business-setup-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
      localStorage.removeItem('onboardingState');
    }, id);
    await page.goto('/website-builder');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('.glassmorphism').first()).toBeVisible();
  });

  test('shows the current setup welcome step', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();
        await expect(page.getByRole('button', { name: /Instant Build/ })).toBeVisible();
  });

  test('moves through business type and name steps', async ({ page }) => {
    await page.getByRole('button', { name: 'Back' }).click();
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await expect(page.getByRole('heading', { name: "What kind of business are you building?" })).toBeVisible();

    await page.getByRole('button', { name: /Online Store/ }).click();
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').waitFor({ state: 'visible', timeout: 10000 });
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('Custom cookies and cakes');
    await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });

  test('completes the publish path to the checklist', async ({ page }) => {
    const email = `maya+${Date.now()}@example.com`;
    await page.getByRole('button', { name: 'Back' }).click();
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').waitFor({ state: 'visible', timeout: 10000 });
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('Custom cookies and cakes');
    await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical Products/).check();
    await page.locator('#step-4').getByRole('button', { name: /Next/ }).click();
    await page.getByPlaceholder('What is the name of this product?').fill('Custom Cookies');
    await page.getByPlaceholder('0.00').fill('24.99');
    await page.locator('#step-5').getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    await page.getByRole('button', { name: 'Online', exact: true }).click();
    await page.getByPlaceholder('e.g. Maya Smith').fill('Maya Smith');
    await page.getByPlaceholder('you@email.com').fill(email);
    await page.getByPlaceholder('Password').fill('password123');
    await page.locator('#step-7').getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.locator('#step-8').getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.locator('#step-9').getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
    await page.getByRole('button', { name: /View Welcome Checklist/ }).click();
    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible();
  });
});

  test('validates minimum touch targets and disabled states', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    const startBtn = page.getByRole('button', { name: /Start My Business/ });
    await expect(startBtn).toBeVisible();
    const box1 = await startBtn.boundingBox();
    expect(box1?.height).toBeGreaterThanOrEqual(54);

    await page.getByRole('button', { name: 'Back' }).click();
    await startBtn.click();
    await page.getByRole('button', { name: /Online Store/ }).click();

    // Verify next is disabled initially
    const nextBtn = page.locator('#step-3').getByRole('button', { name: /Next/ });
    await expect(nextBtn).toBeDisabled();

    // Fill business name
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await expect(nextBtn).toBeEnabled();

    // Verify touch target
    const inputName = page.getByPlaceholder('What is your business called?');
    const box2 = await inputName.boundingBox();
    expect(box2?.height).toBeGreaterThanOrEqual(54);

    await nextBtn.click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });
