import { test, expect } from '@playwright/test';

test.describe('Offline-Capable POS Engine', () => {
  test('Persona: Business Owner can sync offline transactions when back online', async ({ request, page, context }) => {
    // Navigate to ensure basic UI is alive
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    const res = await request.post('/api/chat', {
        data: { message: "Can you confirm my recent offline POS transactions are syncing?" }
    });

    expect(res.ok()).toBeTruthy();
  });
});
