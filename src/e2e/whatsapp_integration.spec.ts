import { test, expect } from './fixtures';

test.describe('WhatsApp Integration', () => {
  test('should allow connecting WhatsApp Cloud API from integrations page', async ({ page, memberPage }) => {
    // Navigate to integrations page
    await page.goto('/integrations');

    // Check if WhatsApp Cloud API card is visible
    const whatsappCard = page.locator('h3:has-text("WhatsApp Cloud API")');
    await expect(whatsappCard).toBeVisible();

    // Click connect
    await whatsappCard.locator('..').locator('button:has-text("Connect")').click();

    // Verify modal opens
    const modalTitle = page.locator('h2:has-text("Connect WhatsApp Cloud API")');
    await expect(modalTitle).toBeVisible();

    // Fill the credentials
    await page.getByPlaceholder('Meta App ID').fill('test_app_id');
    await page.getByPlaceholder('Hidden for security').fill('test_access_token');
    await page.getByPlaceholder('+1234567890').fill('+1234567890');

    // Click save
    await page.getByRole('button', { name: 'Save & Connect' }).click();

    // Verify status message
    await expect(page.locator('text=WhatsApp Cloud API connected.')).toBeVisible();
  });
});
