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
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'ManyChat' })).toBeVisible();
    await expect(page.getByText('Unified Social Inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByText('Automated Scheduling for your clients.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Stripe' })).toBeVisible();
    await expect(page.getByText('Automated Shipping Label Generation.')).toBeVisible();
    await expect(page.getByText('Simple Invoice Payments via Stripe.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Keep Mailchimp Contacts in Sync.')).toBeVisible();
    await expect(page.getByText('Auto-Generate Zoom Links for Meetings.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Automated Appointment Reminders via Twilio SMS.')).toBeVisible();
  });

  test('can connect ManyChat', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'ManyChat' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to ManyChat...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Calendly', async ({ page }) => {
    const connectButton = page.locator('div.card.glass').filter({ hasText: 'Calendly' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to Calendly...');
      dialog.accept();
    });
    await connectButton.click();
  });

  test('can connect Mailchimp and Stripe', async ({ page }) => {
    const mailchimpBtn = page.locator('div.card.glass').filter({ hasText: 'Mailchimp' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mailchimpBtn.click();

    const stripeBtn = page.locator('div.card.glass').filter({ hasText: 'Stripe' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await stripeBtn.click();
  });

  test('can connect Shippo, Twilio, and Zoom', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippoBtn = page.locator('div.card.glass').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' });
    await shippoBtn.click();
    const twBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const zoomBtn = page.locator('div.card.glass').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' });
    await zoomBtn.click();
  });
});
