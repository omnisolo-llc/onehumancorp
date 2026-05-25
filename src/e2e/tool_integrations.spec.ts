import { test as baseTest, expect } from '@playwright/test';

// Define a simple test without using the login fixture, to directly hit the Next.js dev server.
const test = baseTest;

test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate straight to the nextjs integrations page route instead of via navigation
    await page.goto('http://localhost:3000/integrations');
  });

  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });

  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Meta Graph API' })).toBeVisible();
    await expect(page.getByText('Unified Native Social Media Inbox for Instagram, Facebook, and WhatsApp.')).toBeVisible();
    await expect(page.locator('div').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });

  test('can connect Meta Graph API', async ({ page }) => {
    const connectButton = page.locator('div').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Connect' }).first();
    await connectButton.click();

    // Increase timeout to give the fetch time to finish. The delay is ~2s.
    await expect(page.getByText('We found 12 products. Added to your OHC store.')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('div').filter({ hasText: 'Meta Graph API' }).getByRole('button', { name: 'Manage' }).first()).toBeVisible({ timeout: 10000 });
  });

  test('can connect Google Calendar', async ({ page }) => {
    // There might be multiple Google Calendar text, get the Connect button directly via the heading.
    const heading = page.getByRole('heading', { name: 'Google Calendar' });
    const card = heading.locator('..');
    const connectButton = card.getByRole('button', { name: 'Connect' });
    await connectButton.click();

    // Check for the updated connected dashboard instead of a floating text
    await expect(page.getByRole('heading', { name: 'Unified Dashboard' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Your calendar is synced and protecting your time slots.')).toBeVisible({ timeout: 10000 });
    await expect(card.getByRole('button', { name: 'Manage' })).toBeVisible({ timeout: 10000 });
  });
});
