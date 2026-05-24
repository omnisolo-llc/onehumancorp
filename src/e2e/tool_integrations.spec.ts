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

  test('displays social media integration cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'ManyChat' })).toBeVisible();
    await expect(page.getByText('Automates initial inquiries and routes critical leads directly to your phone.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Sprout Social' })).toBeVisible();
    await expect(page.getByText('Unified inbox, allowing you to schedule posts and respond to clients across platforms.')).toBeVisible();
  });

  test('displays online booking integration cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByText('Lets clients book consultations directly on your website, automatically syncing with your calendar.')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Acuity Scheduling' })).toBeVisible();
    await expect(page.getByText('Handles custom bookings, allowing you to collect intake forms and deposits.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'ShipStation' })).toBeVisible();
    await expect(page.getByText('Automatically pulls orders and prints labels in bulk, saving you hours daily.')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Alipay' })).toBeVisible();
    await expect(page.getByText('Capture the Chinese market by supporting domestic digital wallets.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByText('Easily send out a monthly newsletter with photos and discount codes.')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Sendinblue (Brevo)' })).toBeVisible();
    await expect(page.getByText('Maintain a massive customer list without paying exorbitant fees.')).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Automatically generates a unique meeting link the moment a client books a slot.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Reliable SMS alerts for new orders and customer notifications.')).toBeVisible();
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

  test('can connect Mailchimp and Mercado Pago', async ({ page }) => {
    const mailchimpBtn = page.locator('div.card.glass').filter({ hasText: 'Mailchimp' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mailchimpBtn.click();

    const mercadoBtn = page.locator('div.card.glass').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await mercadoBtn.click();
  });

  test('can connect ShipStation, Twilio, and Zoom', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const ssBtn = page.locator('div.card.glass').filter({ hasText: 'ShipStation' }).getByRole('button', { name: 'Connect' });
    await ssBtn.click();
    const twBtn = page.locator('div.card.glass').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const zoomBtn = page.locator('div.card.glass').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' });
    await zoomBtn.click();
  });
});
