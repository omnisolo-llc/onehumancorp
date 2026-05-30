import { expect, test } from './fixtures';

export function currentAppSmoke(label: string) {
  test(`current embedded app smoke: ${label}`, async ({ page, request }) => {
    // Basic navigation assertions just like other specs to make sure the app loads
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Verify Help Center navigation
    await page.goto('/help');

    // Check main title
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Check search input
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await expect(searchInput).toBeVisible();

    // verify Help Article rendering
    await page.goto('/help/getting-started');
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.getByText('Step 1: Tell us about your business')).toBeVisible();

    // verify API Docs advanced page
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced: This section is for developers directly integrating with our APIs.')).toBeVisible();

    // verify Changelog page
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();
  });
}

currentAppSmoke('help_docs');
