import { test, expect } from './fixtures';

function integrationCard(page: import('@playwright/test').Page, name: string) {
  return page
    .getByRole('heading', { name })
    .locator('xpath=ancestor::div[contains(@class, "rounded")][1]');
}

test.describe('Tool Integrations UI', () => {
  test.beforeEach(async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite marketing, finance, and operations tools.')).toBeVisible();
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

  for (const name of ['Ayrshare', 'Cal.com', 'Resend', 'Mercado Pago', 'Whereby', 'Front']) {
    test(`${name} stays disconnected when secure verification is unavailable`, async ({ page }) => {
      const writes: string[] = [];
      const dialogs: string[] = [];
      page.on('request', request => {
        if (request.method() === 'POST' && new URL(request.url()).pathname.startsWith('/api/v1/integrations/')) writes.push(request.url());
      });
      page.on('dialog', dialog => { dialogs.push(dialog.type()); void dialog.dismiss(); });
      const card = integrationCard(page, name);
      await card.getByRole('button', { name: 'Connect', exact: true }).click();
      await expect(page.getByText(`${name} connection is unavailable until secure provider verification is configured.`, { exact: true })).toBeVisible();
      await expect(card.getByText('disconnected', { exact: true })).toBeVisible();
      await expect(card.getByRole('button', { name: 'Connect', exact: true })).toBeVisible();
      await expect(page).toHaveURL(/\/integrations$/);
      expect(writes).toEqual([]);
      expect(dialogs).toEqual([]);
    });
  }

  test('blank Twilio credentials cannot claim a connection or navigate to an inbox', async ({ page }) => {
    const writes: string[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && new URL(request.url()).pathname === '/api/v1/integrations/twilio/connect') writes.push(request.url());
    });
    await integrationCard(page, 'Twilio Conversations').getByRole('button', { name: 'Connect', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Connect Twilio Conversations', exact: true })).toBeVisible();
    await expect(page.getByLabel('Twilio Account SID')).toBeEmpty();
    await expect(page.getByLabel('Twilio Auth Token')).toBeEmpty();
    await expect(page.getByRole('button', { name: 'Save & Connect', exact: true })).toBeDisabled();
    await expect(page.getByText('Twilio Conversations connected.', { exact: true })).toHaveCount(0);
    await expect(page).toHaveURL(/\/integrations$/);
    expect(writes).toEqual([]);
  });
});
