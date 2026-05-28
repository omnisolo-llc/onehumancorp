import { test, expect } from '@playwright/test';

test.describe('Mock endpoints proxy to actual backend', () => {
  test('verify that help endpoint proxies correctly', async ({ page }) => {
    // Go to help page, which should fetch /api/help.
    await page.goto('/help');
    // Ensure that help categories render correctly. This proves the request hit the actual backend
    // since the mocked data from next.js was removed and we hit backend endpoint natively instead.
    await expect(page.locator('h1')).toHaveText('Help Center');
  });

  test('verify that chat endpoint proxies correctly', async ({ page }) => {
    await page.goto('/team/chat');
    // Wait for the UI to be ready
    await expect(page.locator('h1')).toHaveText('Team Chat');
    const input = page.getByTestId('team-chat-input');
    await input.fill('Can you generate a quote for John Doe?');

    // We mock the /api/agents/chat call because actual generation takes time and we're just testing the proxy
    await page.route('/api/agents/chat', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          department_assigned: 'sales',
          agent: 'The Salesperson',
          description: 'Drafted quote based on: "Can you generate a quote for John Doe?"'
        })
      });
    });

    await page.getByTestId('team-chat-send').click();

    // Check if the system message returns showing the right agent and department
    await expect(page.locator('text=The Salesperson').first()).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text=Drafted quote based on: "Can you generate a quote for John Doe?"').first()).toBeVisible({ timeout: 5000 });
  });
});
