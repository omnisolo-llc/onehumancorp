import { test, expect } from '@playwright/test';

test.describe('Chaos Resilience', () => {

  test('handles 500 error on /api/v1/builder/geo_score gracefully', async ({ page }) => {
    // Intercept the request to mock a 500 failure
    await page.route('/api/v1/builder/geo_score', async route => {
      await route.fulfill({ status: 500, contentType: 'application/json', body: '{"error": "Internal Server Error"}' });
    });

    await page.goto('/builder');

    // We expect the page to load without crashing and show some error indication
    // Wait for the specific heading to confirm we are on the page
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' }).first()).toBeVisible();

    // As geo_score is a background metric for the builder, we ensure the builder UI itself hasn't crashed
    // Let's verify the main interaction is still possible (e.g. Next button or input fields)
    const inputs = page.locator('input[type="text"]');
    await expect(inputs.first()).toBeVisible();
  });

  test('handles disconnected network timeout on /api/v1/builder/auto_seo', async ({ page }) => {
    // Intercept with an abort to simulate disconnected network
    await page.route('/api/v1/builder/auto_seo', async route => {
      await route.abort('failed');
    });

    await page.goto('/builder');

    // Builder should still be usable even if auto_seo background task fails
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' }).first()).toBeVisible();
  });

  test('handles 503 error on /api/agents/approvals gracefully', async ({ page }) => {
    await page.route('/api/agents/approvals', async route => {
      await route.fulfill({ status: 503, contentType: 'application/json', body: '{"error": "Service Unavailable"}' });
    });

    await page.goto('/agents');

    // Wait for the UI to show the AI Departments
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();

    // Ideally there's an error message or the table is empty but the page itself should not crash
    // We confirm the page content is still visible
    await expect(page.getByRole('button', { name: /The Ambassador/ }).first()).toBeVisible();
  });

  test('handles failing /api/v1/growth/referrals/generate on checkout gracefully', async ({ page }) => {
    await page.route('/api/v1/growth/referrals/generate', async route => {
      await route.fulfill({ status: 500, contentType: 'application/json', body: '{"error": "Failed"}' });
    });

    await page.goto('/checkout');

    // Wait for the checkout heading
    await expect(page.getByRole('heading', { name: 'Checkout' }).first()).toBeVisible();
  });

  test('handles offline network for /api/chat gracefully', async ({ page }) => {
    await page.route('/api/chat', async route => {
       await route.abort('failed');
    });

    await page.goto('/dashboard');

    // Open Help Chat
    await page.locator('button.fixed.bottom-6.right-6').click();

    // The chat input should be visible
    const input = page.locator('input[placeholder="Ask me anything..."]');
    await expect(input).toBeVisible();

    // Try to send a message
    await input.fill('Hello');
    await input.press('Enter');

    // Expect an error message to show up in the chat
    await expect(page.getByText('Connection error. Please check your internet.')).toBeVisible();
  });

});
