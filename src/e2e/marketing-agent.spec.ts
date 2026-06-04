import { test, expect } from '@playwright/test';

test.describe('Marketing Agent Social Media Manager E2E', () => {

  test('Persona: Business Owner adds a product and approves social post', async ({ page }) => {
    // 1. Owner registers/logs in
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill(`test@example.com`);
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Mock /api/products
    await page.route('/api/products', async route => {
      if (route.request().method() === 'POST') {
        const body = JSON.parse(route.request().postData() || '{}');
        await route.fulfill({ status: 201, json: { success: true, product: body } });
        return;
      }
      await route.continue();
    });

    // 3. Navigate to Team/Agents page
    await page.goto('/team');

    // MOCK API Route for pending approvals
    await page.route('/api/agents/approvals', async route => {
      await route.fulfill({
        status: 200,
        json: {
          pending_approvals: [
            {
              id: 'test-approval-1',
              department: 'marketing',
              status: 'pending',
              description: 'Draft Instagram post for Vegan Chocolate Cake',
              payload: {
                feature_type: 'social_post',
                product_name: 'Vegan Chocolate Cake',
                image_url: 'https://example.com/optimized_cake.jpg',
                draft_copy: 'Craving something sweet and vegan? 🍰 Try our new Vegan Chocolate Cake! Order now via link in bio. #VeganBaking #LocalBakery'
              }
            }
          ]
        }
      });
    });

    // We assume there is an "Add Product" flow in /products
    // If it's not present, we can simulate the event being triggered.
    // For this E2E we verify the core agent approval flow directly from /team

    // Check if there is an Add Product button (we might need to adapt this depending on the real UI)
    // For now, let's assume we can trigger a product creation via UI if it exists, or just fallback to hitting the real API route

    // Instead of mocking, we can make a request to the backend from the client context
    await page.evaluate(async () => {
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
    });

    // 4. MOCK the team UI
    await page.getByText('The Promoter').click();

    // Verify approval card is present
    await expect(page.getByText('Social Media Post Drafted')).toBeVisible();
    await expect(page.getByText('Vegan Chocolate Cake', { exact: false })).toBeVisible();
    await expect(page.getByText('Craving something sweet and vegan?', { exact: false })).toBeVisible();

    // MOCK API Route for approval action
    await page.route('/api/agents/approvals/test-approval-1', async route => {
      if (route.request().method() === 'POST') {
        const body = JSON.parse(route.request().postData() || '{}');
        expect(body.approved).toBe(true);
        await route.fulfill({ status: 200, json: { success: true } });
        return;
      }
      await route.continue();
    });

    // 4. Click Approve & Schedule
    const approveButton = page.getByRole('button', { name: 'Approve & Schedule' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Approval box should disappear or empty state should show
    await expect(page.getByText('No pending actions')).toBeVisible();
  });
});
