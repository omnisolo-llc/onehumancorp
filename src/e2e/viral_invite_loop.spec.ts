import { test, expect } from './fixtures';

test.describe('Viral Invite Loop on Team Page', () => {
  test('should display Cloud Bridge invite modal and generate a link', async ({ page }) => {
    await page.goto('/team');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Grow Your Team' })).toBeVisible();
    await expect(page.getByText('Bring your team online easily. Share access to your workspace securely.')).toBeVisible();

    // Click the invite button
    await page.getByRole('button', { name: 'Invite Team Member' }).click();

    // Verify modal content
    await expect(page.getByRole('heading', { name: 'Team Invite' })).toBeVisible();
    await expect(page.getByText('Share this secure link with your team member so they can collaborate with you online.')).toBeVisible();

    // Verify loading spinner (optional) and then the generated link
    await expect(page.locator('#team-invite-link')).toHaveValue(/https:\/\/ohc\.app\/invite\/.*/);

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy Link' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Close the modal
    await page.getByRole('button', { name: 'Close Team Invite' }).click();
    await expect(page.getByRole('heading', { name: 'Team Invite' })).not.toBeVisible();
  });
});
