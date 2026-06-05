import { test, expect } from './fixtures';

test.describe('Promoter Auto-Post CUJ', () => {
  test('Owner adds a product and approves the auto-generated social post', async ({ page, request }) => {
    test.skip(process.env.CI === 'true', 'Skip in CI until webhook/event endpoints run reliably');

    // We assume the user is already logged in due to the global setup / fixtures.
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // 1. Trigger the Promoter's draft copy via a product creation
    // The Marketing agent listens for tenant.product.created
    const tenantId = 'e2e-tenant'; // Match the standard fixture tenant
    const productName = 'New Summer Collection T-Shirt';

    // Instead of clicking through the UI which can be flaky, we hit the exact endpoint to simulate a product creation
    // The marketing agent will pick this up
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

    // 3. Navigate to Team Page / Agents Approval Inbox
    await page.goto('/team');

    // Navigate to The Promoter
    await page.getByRole('button', { name: 'The Promoter' }).first().click();

    // Ensure we are viewing the Promoter inbox specifically
    await expect(page.getByRole('heading', { name: 'The Promoter' })).toBeVisible({ timeout: 5000 });

    // Wait for the specific text to appear, indicating the drafted card is loaded
    // Depending on the AI output, we check for a fragment we expect based on the logic:
    // Either "Draft Instagram post for" or the product name itself.
    const inquiryLocator = page.getByText(productName).first();
    await expect(inquiryLocator).toBeVisible({ timeout: 15000 });

    // Click Approve
    await page.getByRole('button', { name: 'Approve' }).first().click();

    // Validate empty state or removal
    await expect(page.getByText(productName)).toBeHidden();
  });
});
