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
    await expect(page.locator('#cloud-bridge-invite-link')).toHaveValue(/https:\/\/ohc\.app\/invite\/.*/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy Link' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Close the modal
    await page.getByRole('button', { name: 'Close Cloud Bridge Invite' }).click();
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).not.toBeVisible();
  });

  test('should allow interacting with multiple invite workflows without issue', async ({ page }) => {
    await page.goto('/team');
    await page.getByRole('button', { name: 'Invite to Cloud Team' }).click();
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).toBeVisible();
    await page.getByRole('button', { name: 'Close Cloud Bridge Invite' }).click();
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).not.toBeVisible();
    await page.getByRole('button', { name: 'Invite to Cloud Team' }).click();
    await expect(page.getByRole('heading', { name: 'Cloud Bridge Invite' })).toBeVisible();
  });

  test('should show correct text and styling when modal is open', async ({ page }) => {
     await page.goto('/team');
     await page.getByRole('button', { name: 'Invite to Cloud Team' }).click();
     const copyButton = page.getByRole('button', { name: 'Copy Link' });
     await expect(copyButton).toBeVisible();
     await expect(copyButton).toHaveClass(/w-full.*bg-blue-600/);
  });

  test('copied state resets after 2 seconds', async ({ page }) => {
      await page.goto('/team');
      await page.getByRole('button', { name: 'Invite to Cloud Team' }).click();
      await page.getByRole('button', { name: 'Copy Link' }).click();
      await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
      await page.waitForTimeout(2100);
      await expect(page.getByRole('button', { name: 'Copy Link' })).toBeVisible();
  });

  test('does not break rest of team page functionality when modal interacts', async ({ page }) => {
      await page.goto('/team');
      // wait for departments to load
      await expect(page.getByText('The Manager')).toBeVisible();
      await page.getByRole('button', { name: 'Invite to Cloud Team' }).click();
      await page.getByRole('button', { name: 'Close Cloud Bridge Invite' }).click();
      await expect(page.getByText('The Manager')).toBeVisible();
  });
});
