import { test, expect } from '@playwright/test';

test.describe('Promoter Agent Social Media Workflow', () => {
  test('Creates a product and approves generated social post', async ({ page, request }) => {
    const baseUrl = 'http://localhost:3000';

    await request.post('http://127.0.0.1:18789/api/dev/db-execute', {
      data: {
        query: `
          INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload)
          VALUES ('e2e-promoter-post-1', 'e2e-tenant', 'marketing', 'The Promoter generated social media captions for your new product. Review and schedule.', 'PENDING', 'LOW',
            '{"feature_type": "social_post", "product_id": "prod-123", "product_name": "Test Vegan Cake", "instagram": "New arrival! Link in bio.", "tiktok": "Check out our new product!", "facebook": "We just added a new product to our store."}'
          ) ON CONFLICT(id) DO UPDATE SET status = 'PENDING';
        `
      }
    });

    await page.goto(`${baseUrl}/login`);
    await page.evaluate(() => {
      localStorage.setItem('token', 'e2e-token');
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('tenant', 'e2e-tenant');
    });

    await page.goto(`${baseUrl}/team`);
    await page.click('text=The Promoter');

    await expect(page.locator('text=Social Post Drafted')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Test Vegan Cake')).toBeVisible();
    await expect(page.locator('text=Instagram:')).toBeVisible();

    await page.click('button:has-text("Schedule Post")');

    await expect(page.locator('text=Test Vegan Cake')).not.toBeVisible({ timeout: 5000 });
  });
});
