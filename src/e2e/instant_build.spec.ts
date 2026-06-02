import { test, expect } from './fixtures';

test.describe('Zero-Touch Storefront Generation API', () => {
  test('uses instant build to generate and launch a storefront', async ({ page }) => {
    const id = `instant-build-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_wizard_state');
    }, id);
    await page.goto('/website-builder');

    await expect(page.locator('#setup-screen')).toBeVisible();

    // Ensure the Instant Build button is visible before clicking it
    await expect(page.getByRole('button', { name: 'Instant Build' })).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: 'Instant Build' }).click();

    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible({ timeout: 10000 });

    const textarea = page.getByPlaceholder('e.g. I run a local bakery');
    await textarea.fill('I am a freelance handyman who needs to book more weekend repair jobs.');

    await expect(page.getByRole('button', { name: 'Approve & Launch' })).toBeVisible({ timeout: 10000 });
    await page.getByRole('button', { name: 'Approve & Launch' }).click();

    // UI should show it's generating
    await expect(page.getByText('Agents are building your store...')).toBeVisible();

    // Since this hits the real API it might take a few seconds
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 15000 });
  });
});
