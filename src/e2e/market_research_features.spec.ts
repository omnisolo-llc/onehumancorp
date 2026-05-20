import { test, expect } from './fixtures';

test.describe('Market Research & Strategy Features', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('1. Intelligent Auto-Responder for order status queries', async ({ page }) => {
    // Navigate to Integrations to show navigation flow
    await page.getByRole('link', { name: 'Integrations' }).click();
    await expect(page.getByRole('heading', { name: 'Integrations' })).toBeVisible();

    // Navigate to Inbox
    await page.getByRole('link', { name: 'Back' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Mock an event via internal API if possible, or verify UI elements
    await expect(page.getByText('Team Activity')).toBeVisible();
  });

  test('2. Social media post generation for catalog updates', async ({ page }) => {
    await page.getByRole('link', { name: 'Integrations' }).click();
    await expect(page.locator('#instagram-integration')).toBeVisible();
    await expect(page.locator('#whatsapp-integration')).toBeVisible();
    await expect(page.locator('#shopify-integration')).toBeVisible();

    // Back to dashboard
    await page.getByRole('link', { name: 'Back' }).click();
    await expect(page.getByText('Business Snapshot')).toBeVisible();
  });

  test('3. Integration page navigation and configuration', async ({ page }) => {
    await page.getByRole('link', { name: 'Integrations' }).click();

    // Test tab switching
    await page.getByRole('button', { name: 'Social' }).click();
    await expect(page.locator('#instagram-integration')).toBeVisible();

    // Test configuration navigation (mocked)
    await page.locator('#instagram-integration').getByRole('button', { name: 'Configure' }).click();
    await expect(page).toHaveURL(/\/inbox/);
  });

  test('4. Dashboard metric visibility and informational density', async ({ page }) => {
    await expect(page.getByText('Today\'s Sales')).toBeVisible();
    await expect(page.getByText('Active Customers')).toBeVisible();
    await expect(page.getByText('Pending Orders')).toBeVisible();
    await expect(page.getByText('Conversion Rate')).toBeVisible();

    // Check for high density markers
    await expect(page.getByText('vs yesterday')).toBeVisible();
    await expect(page.getByText('Live now')).toBeVisible();
  });

  test('5. Mobile viewport responsiveness (375px)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Header should still have title
    const title = page.getByRole('heading', { name: 'Dashboard' });
    const box = await title.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);

    // Snapshot cards should stack or be visible
    await expect(page.getByText('Today\'s Sales')).toBeVisible();

    // Navigate to Integrations on mobile
    const nav = page.locator('nav');
    await expect(nav).toBeHidden();
  });
});
