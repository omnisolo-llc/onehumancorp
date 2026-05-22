import { test, expect } from './fixtures';

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' }).first()).toBeVisible();
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps to streamline your business operations.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByText('Unified Customer Inbox. Manage all your messages and posts from one place.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByText('Automated Booking. Let customers schedule appointments 24/7.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).nth(1)).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).nth(3)).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).nth(5)).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Zoom' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).nth(2)).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).nth(6)).toBeVisible();
  });

  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).nth(4)).toBeVisible();
  });

  test('can navigate to the connect tools integrations view', async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();
    await expect(page.getByText('Seamlessly connect your favorite apps')).toBeVisible();
  });

  test('filters integrations by marketing category from home page', async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();

    await page.getByRole('button', { name: 'Marketing' }).click();
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mailchimp' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Calendly' })).not.toBeVisible();
  });

  test('filters integrations by operations category from home page', async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();

    await page.getByRole('button', { name: 'Operations' }).click();
    await expect(page.getByRole('heading', { name: 'Calendly' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Manychat' })).not.toBeVisible();
  });

  test('filters integrations by finance category from home page', async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();

    await page.getByRole('button', { name: 'Finance' }).click();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Twilio' })).not.toBeVisible();
  });

  test('returns to all integrations view from home page journey', async ({ page }) => {
    await page.goto('/');
    await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Connect Tools' })).toBeVisible();

    await page.getByRole('button', { name: 'Marketing' }).click();
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Shippo' })).not.toBeVisible();

    await page.getByRole('button', { name: 'All' }).click();
    await expect(page.getByRole('heading', { name: 'Manychat' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Shippo' })).toBeVisible();
  });
});
