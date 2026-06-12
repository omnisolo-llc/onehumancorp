import { test, expect } from './fixtures';

function integrationCard(page: import('@playwright/test').Page, name: string) {
  return page
    .getByRole('heading', { name })
    .locator('xpath=ancestor::div[contains(@class, "rounded")][1]');
}

test.describe('Tool Integrations UI', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    const card = integrationCard(page, 'Meta Graph API');
    await expect(card).toBeVisible();
    await expect(card.getByText('Central Instagram and Facebook Inbox.')).toBeVisible();
    await expect(card.getByRole('button', { name: 'Connect' })).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    const card = integrationCard(page, 'Cal.com');
    await expect(card).toBeVisible();
    await expect(card.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(integrationCard(page, 'Shippo')).toContainText('Painless Shipping Labels & Tracking.');
    await expect(integrationCard(page, 'Mercado Pago')).toContainText('Accept credit cards and local payment methods in Latin America.');
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(integrationCard(page, 'Resend')).toContainText('Transactional and Marketing Emails.');
    await expect(integrationCard(page, 'Whereby')).toContainText('Zero-Setup Online Lessons and video conferencing.');
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(integrationCard(page, 'Twilio Conversations')).toContainText('Central omnichannel inbox via Twilio Conversations API for SMS, WhatsApp, and chat.');
  });

  test('displays front omnichannel inbox card', async ({ page }) => {
    await expect(integrationCard(page, 'Front')).toContainText('Central omnichannel inbox aggregating messages across all channels.');
  });

  test('can connect Ayrshare', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    await integrationCard(page, 'Ayrshare').getByRole('button', { name: 'Connect' }).click();
    await expect(page).toHaveURL(/\/inbox$/);
  });

  test('can connect Cal.com', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    await integrationCard(page, 'Cal.com').getByRole('button', { name: 'Connect' }).click();
    await expect(integrationCard(page, 'Cal.com').getByText('connected')).toBeVisible();
  });

  test('can connect Resend and Mercado Pago', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    await integrationCard(page, 'Resend').getByRole('button', { name: 'Connect' }).click();
    await expect(integrationCard(page, 'Resend').getByText('connected')).toBeVisible();

    await integrationCard(page, 'Mercado Pago').getByRole('button', { name: 'Connect' }).click();
    await expect(integrationCard(page, 'Mercado Pago').getByText('connected')).toBeVisible();
  });

  test('can connect Twilio Conversations and Whereby', async ({ page }) => {
    await integrationCard(page, 'Twilio Conversations').getByRole('button', { name: 'Connect' }).click();
    await expect(page.getByRole('heading', { name: 'Connect Twilio Conversations' })).toBeVisible();
    await page.getByRole('button', { name: 'Save & Connect' }).click();
    await expect(page).toHaveURL(/\/inbox$/);

    await page.goto('/integrations');
    page.on('dialog', dialog => dialog.accept());
    await integrationCard(page, 'Whereby').getByRole('button', { name: 'Connect' }).click();
    await expect(integrationCard(page, 'Whereby').getByText('connected')).toBeVisible();
  });

  test('can connect WhatsApp via Twilio', async ({ page }) => {
    await integrationCard(page, 'WhatsApp via Twilio').getByRole('button', { name: 'Connect' }).click();
    await expect(page.getByRole('heading', { name: 'Connect WhatsApp via Twilio' })).toBeVisible();
    await expect(page.getByText('Connect your WhatsApp Business number via your Twilio account')).toBeVisible();
    await page.getByRole('button', { name: 'Save & Connect' }).click();
    await expect(page).toHaveURL(/\/inbox$/);
  });

  test('can connect Front', async ({ page }) => {
    page.on('dialog', dialog => dialog.accept());
    await integrationCard(page, 'Front').getByRole('button', { name: 'Connect' }).click();
    await expect(integrationCard(page, 'Front').getByText('connected')).toBeVisible();
  });
});
