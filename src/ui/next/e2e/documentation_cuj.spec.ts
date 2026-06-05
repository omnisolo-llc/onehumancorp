import { test, expect } from '@playwright/test';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/changelog');
=======
    // Navigate starting from home page exactly as requested by instructions
    await page.goto('/');

    // Proceed through onboarding as a new business owner
    await page.getByPlaceholder('What does your business do?').fill('Maya Bakes Custom Cakes');
    await page.getByRole('button', { name: 'Generate My Store' }).click();
    await page.waitForTimeout(1000); // Give the mock some time
    await page.getByRole('button', { name: 'Launch My Store' }).click();

    // Verify we arrived at the dashboard
    await expect(page.getByRole('heading', { name: 'Welcome to your Dashboard' })).toBeVisible();

    // From dashboard, she wants to find the changelog
    const changelogLink = page.locator('a', { hasText: 'Changelog ✨' });
    await expect(changelogLink).toBeVisible();
    await changelogLink.click();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))

    // Verify Changelog is loaded
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible();

    // Now Maya navigates to the Help Center (using the generic help widget since it's the standard entrypoint)
    await page.goto('/help'); // Playwright can't easily click floating elements if they animate

    // Verify Help Center is loaded
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Maya searches for "products" to learn how to add products
<<<<<<< HEAD
    await page.fill('input[placeholder="Search for help articles and videos..."]', 'products');
=======
    await page.fill('input[placeholder="Search for help articles..."]', 'products');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))

    // "My Store" should be visible because it contains instructions on products
    const myStoreLink = page.locator('h2', { hasText: 'My Store' });
    await expect(myStoreLink).toBeVisible();

    // Click on the article
    await myStoreLink.click();

    // Verify the article loaded
    await expect(page.locator('h1', { hasText: 'Managing My Store' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Adding Products' })).toBeVisible();
  });
});
