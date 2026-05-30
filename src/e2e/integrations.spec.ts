import { test, expect } from './fixtures';

test.describe('Integrations Page UI', () => {
  test('displays Integrations page and filters correctly', async ({ page }) => {
    await page.goto('/integrations');

    await expect(page.locator('h1')).toHaveText('Tool Integrations');

    // Default 'all' tab shows ManyChat and Calendly
    await expect(page.locator('text=ManyChat')).toBeVisible();
    await expect(page.locator('text=Calendly')).toBeVisible();
    await expect(page.locator('text=Shippo')).toBeVisible();

    // Click on Operations tab
    await page.getByRole('button', { name: 'Operations' }).click();
    await expect(page.locator('text=Calendly')).toBeVisible();
    await expect(page.locator('text=Shippo')).toBeVisible();
    // ManyChat should be hidden
    await expect(page.locator('text=ManyChat')).not.toBeVisible();

    // Click on Marketing tab
    await page.getByRole('button', { name: 'Marketing' }).click();
    await expect(page.locator('text=ManyChat')).toBeVisible();
    await expect(page.locator('text=Mailchimp')).toBeVisible();
    await expect(page.locator('text=Calendly')).not.toBeVisible();

    // Click on Finance tab
    await page.getByRole('button', { name: 'Finance' }).click();
    await expect(page.locator('text=Mercado Pago')).toBeVisible();
    await expect(page.locator('text=ManyChat')).not.toBeVisible();
  });

  test('connects Twilio integration via modal', async ({ page }) => {
    await page.goto('/integrations');

    // Find the button directly by navigating from the text
    await page.locator('h3', { hasText: 'Twilio Conversations' }).locator('..').getByRole('button', { name: 'Connect' }).click();

    // Verify modal appears
    const modal = page.locator('text=Connect Twilio Conversations');
    await expect(modal).toBeVisible();

    // Toggle Instagram - this requires clicking the button next to the text
    const instagramRow = page.locator('div').filter({ hasText: /^instagram$/i }).first();
    await instagramRow.locator('button').click();

    // Click Save & Connect
    await page.getByRole('button', { name: 'Save & Connect' }).click();

    // Verify routing to /inbox
    await page.waitForURL('**/inbox');
    expect(page.url()).toContain('/inbox');
  });
});
