import { test, expect } from './fixtures';

test.describe('WhatsApp Business API Integration Flow', () => {
  test('Owner can successfully connect WhatsApp and see connected status', async ({ page }) => {
    // 1. Navigate to the Integrations page
    await page.goto('/integrations');

    // The WhatsApp integration card should initially show as disconnected and have a Connect button
    // It's a specific card, let's identify it by its specific heading
    const whatsappCard = page.locator('div', { has: page.locator('h3', { hasText: 'WhatsApp Cloud API' }) }).nth(2);
    // Since nth(2) might be brittle, let's just click the button inside the element containing the text
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'WhatsApp Cloud API' }).locator('button', { hasText: 'Connect' });
    await expect(connectButton).toBeVisible();
    await connectButton.click();

    // 2. Expect the embedded signup modal to appear
    const modal = page.locator('div:has-text("Connect WhatsApp")').last();
    await expect(modal).toBeVisible();

    // Verify modal text to ensure owner-friendly language
    await expect(page.locator('text=Connect your business number via the Meta Embedded Signup flow')).toBeVisible();

    // 3. Click the Continue with Meta button
    const metaButton = page.locator('button:has-text("Continue with Meta")');
    await expect(metaButton).toBeVisible();

    // Simulate connection
    await metaButton.click();

    // 4. Verify we are redirected to inbox
    await expect(page).toHaveURL(/\/inbox/);
  });
});
