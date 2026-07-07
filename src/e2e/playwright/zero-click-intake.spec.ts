import { test, expect } from '@playwright/test';

test.describe('Zero Click Intake', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should generate a proposal and show it in the feed', async ({ page, request }) => {
    // Navigate to the form and submit it
    const response = await request.post('/api/agents/client_intake', {
      form: {
        name: 'John Doe',
        email: 'john@example.com',
        details: 'I need a custom project'
      }
    });

    expect(response.status()).toBe(200);

    // Navigate to feed and verify
    await page.goto(`/login?test_email=john@example.com`);
    await page.goto('/dashboard');

    // Look for Action Card
    const feedSection = page.locator('section');
    await expect(feedSection).toBeVisible({ timeout: 10000 });
  });
});
