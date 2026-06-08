import { test, expect } from '@playwright/test';

test.describe('Tauri Onboarding Wizard Flow', () => {
  test('Completes the onboarding flow', async ({ page }) => {
    // Navigate to the index.html page being served by tauri
    await page.goto('/index.html');

    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Setup page
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Test Business");
    await page.getByRole('button', { name: 'Next' }).click();

    // Success page
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(page.getByText('Workspace created for Test Business.')).toBeVisible();
  });
});
