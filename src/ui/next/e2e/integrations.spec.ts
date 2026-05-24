import { test, expect } from '@playwright/test';

test.describe('Tool Integrations Premium UI', () => {

  test.beforeEach(async ({ page }) => {
    // 1. Starts from the home page after user login
    await page.goto('http://localhost:3000/integrations');
  });

  test('should display all integrated tools correctly', async ({ page }) => {
    // 2. Navigate the feature flow
    await expect(page.getByRole('heading', { name: /Tool Integrations/i })).toBeVisible();

    // Verify specific tools are present based on research report
    await expect(page.getByText('Chatwoot')).toBeVisible();
    await expect(page.getByText('Cal.com')).toBeVisible();
    await expect(page.getByText('Resend')).toBeVisible();
    await expect(page.getByText('Stripe')).toBeVisible();
    await expect(page.getByText('Mercado Pago')).toBeVisible();
    await expect(page.getByText('Shippo')).toBeVisible();
    await expect(page.getByText('Twilio')).toBeVisible();
    await expect(page.getByText('Zoom')).toBeVisible();
    await expect(page.getByText('Google Meet')).toBeVisible();
  });

  test('should filter by marketing tab', async ({ page }) => {
    // 3. Click the Marketing tab
    await page.getByRole('button', { name: 'Marketing' }).click();

    // 4. Assert final product matches
    await expect(page.getByText('Resend')).toBeVisible();
    await expect(page.getByText('Stripe')).not.toBeVisible();
    await expect(page.getByText('Chatwoot')).not.toBeVisible();
  });

  test('should filter by finance tab', async ({ page }) => {
    await page.getByRole('button', { name: 'Finance' }).click();

    await expect(page.getByText('Stripe')).toBeVisible();
    await expect(page.getByText('Mercado Pago')).toBeVisible();
    await expect(page.getByText('Chatwoot')).not.toBeVisible();
  });

  test('should filter by operations tab', async ({ page }) => {
    await page.getByRole('button', { name: 'Operations' }).click();

    await expect(page.getByText('Chatwoot')).toBeVisible();
    await expect(page.getByText('Cal.com')).toBeVisible();
    await expect(page.getByText('Shippo')).toBeVisible();
    await expect(page.getByText('Twilio')).toBeVisible();
    await expect(page.getByText('Zoom')).toBeVisible();
    await expect(page.getByText('Google Meet')).toBeVisible();
    await expect(page.getByText('Resend')).not.toBeVisible();
  });

  test('should correctly toggle Dark Mode style', async ({ page }) => {
    // Initial Light Mode checks
    const toggleButton = page.getByRole('button', { name: 'Toggle Dark Mode' });
    await expect(toggleButton).toHaveText('Dark Mode');

    // Toggle Dark Mode
    await toggleButton.click();

    // Check if it updated
    await expect(toggleButton).toHaveText('Light Mode');

    // Toggle back
    await toggleButton.click();
    await expect(toggleButton).toHaveText('Dark Mode');
  });

  test('should verify connection status UI colors', async ({ page }) => {
    // Stripe should be connected and display "Manage Settings"
    const stripeCard = page.locator('[data-testid="integration-card-stripe"]');
    await expect(stripeCard.getByText('Manage Settings')).toBeVisible();

    // Chatwoot should be disconnected and display "Connect"
    const chatwootCard = page.locator('[data-testid="integration-card-chatwoot"]');
    await expect(chatwootCard.getByRole('button', { name: 'Connect' })).toBeVisible();
  });
});
