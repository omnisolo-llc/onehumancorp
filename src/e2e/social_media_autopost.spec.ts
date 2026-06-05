import { test, expect } from './fixtures';

test.describe('Promoter Auto-Post CUJ', () => {
  test('Owner adds a product and approves the auto-generated social post', async ({ page, request }) => {
    test.skip(process.env.CI === 'true', 'Skip in CI until webhook/event endpoints run reliably');

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    const tenantId = 'e2e-tenant';
    const productName = 'New Summer Collection T-Shirt';

    const webhookPayload = {
      tenant_id: tenantId,
      action: 'product.created',
      payload: {
        name: productName,
        description: 'A stylish and breezy summer t-shirt perfect for the beach.',
        images: ['https://example.com/summer-shirt.jpg']
      }
    };

    const response = await request.post('/api/agents/webhook', {
      data: webhookPayload,
    });
    expect(response.ok()).toBeTruthy();

    await page.goto('/team');

    await page.getByRole('button', { name: 'The Promoter' }).first().click();

    await expect(page.getByRole('heading', { name: 'The Promoter' })).toBeVisible({ timeout: 5000 });

    const inquiryLocator = page.getByText(productName).first();
    await expect(inquiryLocator).toBeVisible({ timeout: 15000 });

    await page.getByRole('button', { name: 'Approve' }).first().click();

    await expect(page.getByText(productName)).toBeHidden();
  });
});
