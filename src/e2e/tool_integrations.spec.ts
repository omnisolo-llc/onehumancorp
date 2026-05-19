import { test, expect } from './fixtures';

test.describe('Integrations E2E Flow', () => {
  test('should allow setting up and using EasyPost for a shipment', async ({ page }) => {
    // 1. Setup EasyPost in Settings
    await page.goto('/settings/integrations');
    await expect(page.getByText('EasyPost Shipping')).toBeVisible();
    await page.getByRole('button', { name: 'Configure EasyPost' }).click();

    // Mock entering an API key
    await page.getByPlaceholder('EasyPost API Key').fill('test_api_key_123');
    await page.getByRole('button', { name: 'Save' }).click();
    await expect(page.getByText('EasyPost connected successfully')).toBeVisible();

    // 2. Go to an Order and generate a label
    await page.goto('/orders');
    await page.getByText('Order #1001').click();
    await expect(page.getByText('Pending Shipment')).toBeVisible();

    // Trigger label generation
    await page.getByRole('button', { name: 'Generate Shipping Label' }).click();

    // Verify success and tracking info
    await expect(page.getByText('Label generated successfully')).toBeVisible();
    await expect(page.getByText('9400100000000000000000')).toBeVisible(); // From mock
  });

  test('should allow generating a Jitsi meeting for an online booking', async ({ page }) => {
    // 1. View a booking
    await page.goto('/bookings');
    await page.getByText('Piano Lesson with Leo').click();

    // 2. Generate Meeting Link
    await page.getByRole('button', { name: 'Create Meeting Room' }).click();

    // 3. Verify link is generated and visible
    await expect(page.getByText('meet.jit.si')).toBeVisible();
  });
});
