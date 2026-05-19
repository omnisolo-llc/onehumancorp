import { test, expect } from './fixtures';

test.describe('Canvas Integrations Hub', () => {

  test('displays pixel-perfect high fidelity integrations page', async ({ page }) => {
    await page.goto('/');
    // Must navigate to integrations page from home
    await page.getByRole('button', { name: 'Integrations' }).click();

    // Ensure "grandmother test" simple language
    await expect(page.getByRole('heading', { name: 'Integrations' })).toBeVisible();
    await expect(page.getByText('Manage your custom software connections here.')).toBeVisible();

    // Verify categories are present and easy to read
    const categories = [
        'Marketing & Advertising',
        'Operations',
        'Email Marketing',
        'Finance & Payments',
        'Shipping',
        'Video Services'
    ];

    for (const category of categories) {
        await expect(page.getByRole('button', { name: category })).toBeVisible();
    }
  });

  test('can navigate to social media integrations', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Integrations' }).click();

    // Verify Social Media (Ayrshare)
    await page.getByRole('button', { name: 'Marketing & Advertising' }).click();
    await expect(page.getByText('Ayrshare')).toBeVisible();
    await expect(page.getByText('Unified Social Media Inbox & Cross-Posting')).toBeVisible();
  });

  test('can navigate to operations and calendar integrations', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Integrations' }).click();

    // Verify Operations (Cal.com / Twilio)
    await page.getByRole('button', { name: 'Operations' }).click();
    await expect(page.getByText('Cal.com')).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync')).toBeVisible();
    await expect(page.getByText('Twilio')).toBeVisible();
  });

  test('can navigate to email and finance integrations', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Integrations' }).click();

    // Verify Email Marketing (Listmonk)
    await page.getByRole('button', { name: 'Email Marketing' }).click();
    await expect(page.getByText('Listmonk')).toBeVisible();

    // Verify Finance (Mercado Pago)
    await page.getByRole('button', { name: 'Finance & Payments' }).click();
    await expect(page.getByText('Mercado Pago')).toBeVisible();
  });

  test('can navigate to shipping and video integrations', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Integrations' }).click();

    // Verify Shipping (EasyPost)
    await page.getByRole('button', { name: 'Shipping' }).click();
    await expect(page.getByText('EasyPost')).toBeVisible();

    // Verify Video (Jitsi)
    await page.getByRole('button', { name: 'Video Services' }).click();
    await expect(page.getByText('Jitsi Meet')).toBeVisible();
  });
});
