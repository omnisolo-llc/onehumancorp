import { test, expect } from './fixtures';

test.describe('Documentation Features', () => {
  test('Help center navigation and search', async ({ page }) => {
    await page.goto('/help');

    // Verify main help center elements
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.getByPlaceholder('Search for help articles...')).toBeVisible();

    // Verify some default articles are listed
    await expect(page.getByText('Getting Started')).toBeVisible();
    await expect(page.getByText('My Store')).toBeVisible();

    // Test search functionality
    await page.getByPlaceholder('Search for help articles...').fill('products');
    await expect(page.getByText('Getting Started')).toBeHidden();
    await expect(page.getByText('My Store')).toBeVisible();

    // Navigate to an article
    await page.getByText('My Store').click();
    await expect(page.getByRole('heading', { name: 'Managing My Store' })).toBeVisible();

    // Verify content of the article
    await expect(page.getByText('Adding Products')).toBeVisible();
  });

  test('Interactive Walkthrough is present', async ({ page }) => {
    await page.goto('/kairos?walkthrough=true');
    // Walkthrough should be visible
    await expect(page.getByText('Step 1 of')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Next' })).toBeVisible();

    // Click through
    await page.getByRole('button', { name: 'Next' }).click();
    // Finish or Next
    const finishBtn = page.getByRole('button', { name: 'Finish' });
    const nextBtn = page.getByRole('button', { name: 'Next' });
    if (await finishBtn.isVisible()) {
      await finishBtn.click();
    } else {
      await nextBtn.click();
    }
  });

  test('API documentation is accessible', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced:')).toBeVisible();
    await expect(page.getByText('This section is for developers directly integrating with our APIs.')).toBeVisible();
  });

  test('Release notes / Changelog is accessible', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes' }).first()).toBeVisible();
  });

});
