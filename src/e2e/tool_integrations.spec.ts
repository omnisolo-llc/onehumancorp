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
    await expect(page.getByRole('heading', { name: 'Connect Facebook & Instagram' })).toBeVisible();
    await expect(page.getByText('Manage all customer messages in one unified inbox.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect Meta' })).toBeVisible();
  });

  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Add Online Booking' })).toBeVisible();
    await expect(page.getByText('Sync with Google Calendar or add a Cal.com booking link.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect Cal.com' })).toBeVisible();
  });

  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Automated Shipping Labels' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect EasyPost' })).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Global Payment Methods' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Add Payment Provider' })).toBeVisible();
  });

  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Email Marketing' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Setup Email Engine' })).toBeVisible();

    await expect(page.getByRole('heading', { name: 'Automated Video Links' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect Video' })).toBeVisible();
  });
});
