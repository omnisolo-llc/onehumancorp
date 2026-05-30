import { test, expect } from './fixtures';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('shows the single-prompt instant build step', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible();
    await expect(page.getByPlaceholder(/e\.g\. I run a local bakery called Maya's Cakes\.\.\./)).toBeVisible();
    await expect(page.getByRole('button', { name: /Launch your business in 10 minutes/ })).toBeVisible();
  });

  test('completes the publish path to the checklist', async ({ page }) => {
    await page.getByPlaceholder(/e\.g\. I run a local bakery called Maya's Cakes\.\.\./).fill("I run a test company that sells physical products.");
    await page.getByRole('button', { name: /Launch your business in 10 minutes/ }).click();

    // Wait for the generating animation to finish and storefront to render
    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible({ timeout: 10000 });

    // Trigger publish
    await page.getByRole('button', { name: /Publish Changes/ }).click();

    // Wait for confetti / checklist screen to appear
    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible({ timeout: 10000 });
  });
});
