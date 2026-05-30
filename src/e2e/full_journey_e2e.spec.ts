import { test, expect } from './fixtures';

test.describe('Onboarding Guide E2E Journey', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible();
  });

  test('Complete Path to Live Business and Checklist', async ({ page }) => {
    await page.getByPlaceholder(/e\.g\. I run a local bakery called Maya's Cakes\.\.\./).fill("I run Journey Shop, an online store that sells The Journey Book for 29.99.");
    await page.getByRole('button', { name: /Launch your business in 10 minutes/ }).click();

    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible({ timeout: 15000 });

    // Publish
    await page.getByRole('button', { name: /Publish Changes/ }).click();

    // Verify the checklist loaded correctly
    await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible({ timeout: 15000 });

    // Verify all tasks
    await expect(page.locator('text="✅ Business live"')).toBeVisible();
    await expect(page.locator('text="⬜ Add 3 more products"')).toBeVisible();
    await expect(page.locator('text="⬜ Connect Instagram"')).toBeVisible();
    await expect(page.locator('text="⬜ Share your link with a friend"')).toBeVisible();

    // Verify Dashboard link exit
    const dashboardLink = page.locator('text="Go to Dashboard →"');
    await expect(dashboardLink).toBeVisible();
    await dashboardLink.click();
  });
});
