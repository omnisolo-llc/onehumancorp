import { test, expect } from '@playwright/test';

test.describe('Marketing Agent Social Media Manager E2E', () => {
  // Use a unique tenant ID for this test run to avoid collisions
  const testTenantId = `tenant-${Date.now()}`;

  test('Persona: Business Owner adds a product and approves social post', async ({ page }) => {
    // 1. Owner registers/logs in
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill(`owner-${testTenantId}@example.com`);
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Add a new product (this hits the real backend API)
    await page.goto('/products');

    // Check if there is an Add Product button (we might need to adapt this depending on the real UI)
    // For now, let's assume we can trigger a product creation via UI if it exists, or just fallback to hitting the real API route

    // Instead of mocking, we can make a request to the backend from the client context
    await page.evaluate(async (tenant) => {
      const res = await fetch('/api/products', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'Vegan Chocolate Cake',
          description: 'A delicious vegan chocolate cake.',
          price: 45.0,
          images: ['https://example.com/cake.jpg']
        })
      });
      if (!res.ok) {
        console.error('Failed to create product');
      }
    }, testTenantId);

    // Give the backend event bus a moment to process the tenant.product.created event
    await page.waitForTimeout(2000);

    // 3. Navigate to Team/Agents page to view approvals
    await page.goto('/team');

    // Select Marketing department (The Promoter)
    await page.getByText('The Promoter').click();

    // The backend should have generated an approval request for the social post
    await expect(page.getByText('Social Media Post Drafted')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Vegan Chocolate Cake')).toBeVisible();

    // 4. Click Approve & Schedule
    const approveButton = page.getByRole('button', { name: 'Approve & Schedule' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify approval goes through
    await expect(page.getByText('Social Media Post Drafted')).not.toBeVisible();
  });
});
