import { test, expect } from './fixtures';

test.describe('Tooltip Registry', () => {
  test('Persona: Admin updates dynamic tooltips', async ({ page }) => {
    // We append /api/ui/ since this is served via backend in testing
    await page.goto('/tooltip-registry.html');
    await expect(page.locator('h1')).toHaveText('Tooltip Registry');

    await page.fill('#new-id', 'test-dynamic-id');
    await page.fill('#new-text', 'My test tooltip text');
    await page.click('#add-btn');

    // Wait for the UI to update the table
    await page.waitForTimeout(1000); // UI may take a bit to fetch again

    await expect(page.locator('input#input-test-dynamic-id')).toHaveValue('My test tooltip text');
  });
});
