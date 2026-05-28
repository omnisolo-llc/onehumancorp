import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept());
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Social Media Inbox' })).toBeVisible();
    await expect(page.getByText('Connect ManyChat to view and respond to all social media messages (Instagram, Facebook, WhatsApp) in one simple inbox.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect ManyChat' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendar Sync' })).toBeVisible();
    await expect(page.getByText('Connect Calendly to let clients book available time slots directly without double-booking your personal calendar.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shipping Labels' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Payment Processing' })).toBeVisible();
    await expect(page.getByText('Connect Shippo to instantly calculate shipping rates and generate printable PDF shipping labels from home.')).toBeVisible();
    await expect(page.getByText('Connect Stripe to generate simple invoice payment links and get paid securely without complex merchant accounts.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Email Marketing' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Video Conferencing' })).toBeVisible();
    await expect(page.getByText('Keep Mailchimp Contacts in Sync. Automatically push new and updated customer details to your email marketing audience.')).toBeVisible();
    await expect(page.getByText('Connect Zoom to auto-generate unique meeting links and automatically add them to your calendar invites.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Text Reminders' })).toBeVisible();
    await expect(page.getByText('Connect Twilio to send automatic SMS text message reminders to clients 24 hours before their scheduled appointments.')).toBeVisible();
  });

  test('can connect Social Media Inbox', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Social Media Inbox' }).getByRole('button', { name: 'Connect ManyChat' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to ManyChat...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Calendar Sync', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Calendar Sync' }).getByRole('button', { name: 'Connect Calendly' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Calendly...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Email Marketing and Payment Processing', async ({ page }) => {
    const emailBtn = page.locator('div.card.glass').filter({ hasText: 'Email Marketing' }).getByRole('button', { name: 'Connect Mailchimp' });
    page.once('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div.card.glass').filter({ hasText: 'Payment Processing' }).getByRole('button', { name: 'Connect Stripe' });
    page.once('dialog', dialog => dialog.accept());
    await paymentBtn.click();
  });

  test('can connect Shipping, Text Reminders, and Video Conferencing', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippingBtn = page.locator('div.card.glass').filter({ hasText: 'Shipping Labels' }).getByRole('button', { name: 'Connect Shippo' });
    await shippingBtn.click();
    const smsBtn = page.locator('div.card.glass').filter({ hasText: 'Text Reminders' }).getByRole('button', { name: 'Connect Twilio' });
    await smsBtn.click();
    const meetingBtn = page.locator('div.card.glass').filter({ hasText: 'Video Conferencing' }).getByRole('button', { name: 'Connect Zoom' });
    await meetingBtn.click();
  });
});
