import { test, expect } from '@playwright/test';

test.describe('Assistant Insights Widget', () => {
  test('displays insights and allows interaction', async ({ page }) => {
    // Mock the insights API
    await page.route('/api/v1/assistant/insights', async (route) => {
      const json = {
        insights: [
          {
            id: 'action-1',
            title: 'Draft quote for Carlos',
            description: 'Carlos requested a quote for a custom cake delivery.',
            actionLabel: 'Approve & Send',
            urgency: 'high'
          },
          {
            id: 'action-2',
            title: 'Follow up on abandoned cart for Priya',
            description: 'Priya left $150 worth of items in her cart yesterday.',
            actionLabel: 'Send Reminder',
            urgency: 'medium'
          }
        ]
      };
      await route.fulfill({ json });
    });

    // Go to dashboard

    // Set a mock session cookie or localStorage if needed, or route around auth
    await page.route('**/*', async (route) => {
        if (route.request().url().includes('/api/v1/auth')) {
            await route.fulfill({ status: 200, json: { user: { id: '1' } } });
        } else {
            await route.continue();
        }
    });

    // We can also try bypassing middleware redirect loop by using the browser context to set a fake cookie
    await page.context().addCookies([
      { name: 'ohc_session', value: 'fake_session_token', domain: 'localhost', path: '/' }
    ]);


    // Mock the walkthrough API
    await page.route('/api/v1/walkthrough/dashboard', async (route) => {
        await route.fulfill({ status: 200, json: [] });
    });


    await page.goto('/dashboard?skipAuth=true');




    // Check if the widget is rendered
    await expect(page.getByText('Assistant Insights')).toBeVisible();

    // Check if the insights are rendered
    await expect(page.getByText('Draft quote for Carlos')).toBeVisible();
    await expect(page.getByText('Carlos requested a quote for a custom cake delivery.')).toBeVisible();

    await expect(page.getByText('Follow up on abandoned cart for Priya')).toBeVisible();

    // Check if the badge shows 2 actions
    await expect(page.getByText('2 Actions')).toBeVisible();

    // Click on action button
    const actionButton = page.getByRole('button', { name: 'Approve & Send' });
    await actionButton.click();

    // The insight should disappear
    await expect(page.getByText('Draft quote for Carlos')).not.toBeVisible();

    // The counter should decrease
    await expect(page.getByText('1 Actions')).toBeVisible();
  });
});
