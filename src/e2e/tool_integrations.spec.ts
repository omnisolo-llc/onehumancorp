import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to integrations directly to test the new UI
    await page.goto('/integrations');
    // We expect the heading to be Tool Integrations and not Connect Tools in the NextJS UI
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByText('Unified social media inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Manychat' }).getByRole('button', { name: 'Connect' })).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByText('Automated booking widget and calendar sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Automated Shipping Labels & Tracking.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByText('Customer re-engagement via automated email campaigns.')).toBeVisible();
    await expect(page.getByText('Auto-generated video meeting links for appointments.')).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Global SMS notifications for orders and alerts.')).toBeVisible();
  });

  test('can connect Manychat', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Manychat' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting to ManyChat...');
      dialog.accept();
    });
    await connectButton.click();
    await expect(page).toHaveURL(/.*\/inbox/);
  });

  test('can enable Calendly', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Calendly' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => {
      expect(dialog.message()).toContain('Connecting Calendly via OAuth...');
      dialog.accept();
    });
    await connectButton.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });

  test('can connect Mailchimp and Mercado Pago', async ({ page }) => {
    const emailBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mailchimp' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await emailBtn.click();

    const paymentBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', dialog => dialog.accept());
    await paymentBtn.click();
  });

  test('can connect Shipping, Twilio, and Zoom', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    const shippingBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Shippo' }).getByRole('button', { name: 'Connect' });
    await shippingBtn.click();

    // Twilio brings up a modal
    const smsBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await smsBtn.click();
    await expect(page.getByRole('heading', { name: 'Connect Twilio Conversations' })).toBeVisible();
    await page.getByRole('button', { name: 'Save & Connect' }).click();

    await page.goto('/integrations'); // reset route back

    const meetingBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Zoom' }).getByRole('button', { name: 'Connect' });
    await meetingBtn.click();
  });
});
