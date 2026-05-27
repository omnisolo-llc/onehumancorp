import { test, expect } from './fixtures';
import { judgeGeneratedOutput } from './ai-judge';

test.describe('Twilio Omnichannel Unified Inbox', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss any alert/dialog automatically to avoid hanging
    page.on('dialog', dialog => dialog.accept());
  });

  test('1. Navigate to the integration page and open the Twilio Conversations connect modal', async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();

    // Find Twilio card
    await expect(page.getByRole('heading', { name: 'Twilio Conversations' })).toBeVisible();

    // Click connect to open modal
    await page.getByRole('button', { name: 'Connect' }).click();
    await expect(page.getByRole('heading', { name: 'Connect Twilio Conversations' })).toBeVisible();
  });

  test('2. Toggle specific channels within the Twilio connection modal', async ({ page }) => {
    await page.goto('/integrations');
    await page.getByRole('button', { name: 'Connect' }).click();
    await expect(page.getByRole('heading', { name: 'Connect Twilio Conversations' })).toBeVisible();

    // Find and toggle Instagram
    const instagramToggle = page.locator('div').filter({ hasText: /^instagram$/ }).getByRole('button');
    await instagramToggle.click();

    // Toggle WhatsApp (it defaults to true, so clicking it will turn it off)
    const whatsappToggle = page.locator('div').filter({ hasText: /^whatsapp$/ }).getByRole('button');
    await whatsappToggle.click();
  });

  test('3. Test graceful error handling when attempting to connect with an invalid channel', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

    // Open Settings Modal
    await page.getByTitle('Channel Settings').click();
    await expect(page.getByRole('heading', { name: 'Channel Settings' })).toBeVisible();

    // Ensure we see Facebook channel toggle
    const fbToggle = page.locator('div').filter({ hasText: /^facebook$/ }).getByRole('button');

    // Un-toggle facebook
    await fbToggle.click();

    // Toggle back on to trigger our fake connection error logic
    await fbToggle.click();

    // Verify error message is visible but no raw API code
    await expect(page.getByText('Could not connect to Facebook at this time. Please try again later.')).toBeVisible();
  });

  test('4. Verify messages from different channels are correctly displayed in the unified inbox', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

    // Ensure messages exist from Facebook, Instagram, and WhatsApp
    await expect(page.getByText('Facebook User')).toBeVisible();
    await expect(page.getByText('Do you have vegan birthday cake options?')).toBeVisible();

    await expect(page.getByText('Instagram User')).toBeVisible();
    await expect(page.getByText('When will my order be shipped?')).toBeVisible();

    await expect(page.getByText('WhatsApp User')).toBeVisible();
    await expect(page.getByText('Can I change my delivery address?')).toBeVisible();
  });

  test('5. Verify that a user can reply to a message from the unified inbox and the UI reflects the sent message', async ({ page }, testInfo) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

    // AI Draft a reply
    await page.getByRole('button', { name: /AI Draft/ }).first().click();

    const draft = await page.locator('#reply-input').inputValue();
    expect(draft).toBeTruthy();

    await page.getByRole('button', { name: 'Send' }).first().click();

    // Expect the sent message to appear in the list under "Me"
    await expect(page.locator('#messages-list')).toContainText(draft);
    await expect(page.getByText('Just now')).toBeVisible();
  });
});
