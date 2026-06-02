import { test, expect } from './fixtures';

test.describe('Analytics Viral Share Loop', () => {
  test('should display share milestone button and contain correct link', async ({ page }) => {
    await page.goto('/analytics');
    await expect(page.getByText('Total Revenue')).toBeVisible();

    const shareLink = page.getByRole('link', { name: 'Share Milestone to X' });
    await expect(shareLink).toBeVisible();

    const href = await shareLink.getAttribute('href');
    expect(href).toContain('twitter.com/intent/tweet');
    expect(decodeURIComponent(href || '')).toContain('ohc://join?ref=');
    expect(decodeURIComponent(href || '')).toContain('$4,250.00');
  });
});
