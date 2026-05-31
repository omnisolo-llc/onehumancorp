import { test, expect } from './fixtures';

test.describe('Viral Invite Loop on Team Page', () => {
  test('should display Cloud Bridge invite modal and generate a link', async ({ page }) => {
    await page.goto('/team');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText('Bridge your local sovereignty with cloud-native collaboration.')).toBeVisible();

    // Click the invite button
    await page.getByRole('button', { name: 'Invite to Cloud Team' }).click();

    // Verify modal content
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).toBeVisible();
    await expect(page.getByText('Share this link to provision a temporary multi-tenant context')).toBeVisible();

    // Verify loading spinner (optional) and then the generated link
    await expect(page.locator('input[type="text"]')).toHaveValue(/https:\/\/ohc\.app\/invite\/.*/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy Link' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Close the modal
    await page.locator('button').filter({ hasText: /^$/ }).nth(1).click();
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).not.toBeVisible();
  });
});