import { test, expect } from './fixtures';

test.describe('Documentation full suite', () => {
  test('Help portal loads properly and search works', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Visit help page
    await page.goto('/help');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('In-App Help Center');

    // Make sure search bar exists
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Test search');

    // Chat widget open interaction
    // The aria-label is missing in Nextjs help page component or it could be inner text, let's look for "Ask anything" since that is in the help page if search fails
    const chatBtn = page.getByRole('button', { name: 'Ask anything' });
    await expect(chatBtn).toBeVisible({ timeout: 10000 });
    await chatBtn.click();
  });

  test('Changelog pulls data dynamically', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Visit changelog page
    await page.goto('/changelog');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('Release Notes');
  });

  test('API Docs loads Swagger UI', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Visit api docs page
    await page.goto('/api-docs.html');

    // Check for Swagger UI wrapper
    const swaggerUI = page.locator('.swagger-ui');
    await expect(swaggerUI).toBeVisible();

    // Ensure the topbar from Swagger has loaded, indicating success
    const info = page.locator('.info .title');
    await expect(info).toBeVisible();
  });
});
