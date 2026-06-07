import { test, expect } from './fixtures';

test.describe('WhatsApp Integration UI', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
  });

  test('displays whatsapp business api integration card and handles connection flow', async ({ page }) => {
    const card = page.getByRole('heading', { name: 'WhatsApp Business API' }).locator('xpath=ancestor::div[contains(@class, "rounded")][1]');
    await expect(card).toBeVisible();
    await expect(card.getByText('Central WhatsApp Business Inbox and AI Auto-Replies.')).toBeVisible();

    await card.getByRole('button', { name: 'Connect' }).click();

    // Fill out the WhatsApp modal
    await expect(page.getByRole('heading', { name: 'Connect WhatsApp Business' })).toBeVisible();
    await page.getByPlaceholder('e.g. 100000123456').fill('123456789');
    await page.getByPlaceholder('e.g. 200000654321').fill('987654321');

    await page.getByRole('button', { name: 'Save & Connect' }).click();

    // Check redirection to inbox and status message
    await expect(page).toHaveURL(/\/inbox/);
    await expect(page.getByText('WhatsApp Business API connected.')).toBeVisible();
  });
});
