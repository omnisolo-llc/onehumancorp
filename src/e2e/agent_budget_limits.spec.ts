import { expect, test } from './fixtures';

test.describe('Agent Budget Limits', () => {
  test('CUJ: Check free tier limits enforcement', async ({ page, request }) => {
<<<<<<< HEAD
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
    // Navigate to agents page
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();

    // Just verify the page loads, as full budget simulation requires backend triggers
    await expect(page.getByRole('button', { name: /The Ambassador/ }).first()).toBeVisible();
  });
});
