import { test, expect } from '@playwright/test';

test.describe('Viral Waitlist Loop', () => {
  test('should allow user to join waitlist and share', async ({ page }) => {
    // Navigate to waitlist page
    await page.goto('/waitlist');

    // Fill out the form
    await page.fill('input[id="email"]', 'test@example.com');

    // Submit the form
    await Promise.all([
      page.waitForResponse(resp => resp.url().includes('/api/v1/growth/waitlist') && resp.status() === 200),
      page.click('button[type="submit"]')
    ]);

    // Verify success message and position
    // We match by regex to handle dynamic position numbers smoothly
    await expect(page.locator('h2', { hasText: /You're( #\d+)? on the list!/ })).toBeVisible();

    // Verify the viral loop section is present
    await expect(page.locator("text=Move up the list!")).toBeVisible();
    await expect(page.locator("text=Invite friends with your unique link.")).toBeVisible();

    // Verify the referral link input is visible and populated
    const referralInput = page.locator('input[readonly]');
    await expect(referralInput).toBeVisible();
    await expect(referralInput).toHaveValue(/https:\/\/ohc\.app\/waitlist\?ref=/);

    // Verify the Copy button
    await expect(page.locator('button', { hasText: 'Copy' })).toBeVisible();

    // Verify the Share on X button and its viral branding in the text
    const shareButton = page.locator('a', { hasText: 'Share on X' });
    await expect(shareButton).toBeVisible();
    const href = await shareButton.getAttribute('href');
    expect(href).toContain('Powered%20by%20OHC');

    // Verify the footer viral branding
    await expect(page.locator("text=⚡ Powered by OHC")).toBeVisible();
  });
});
