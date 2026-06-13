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

    // Verify success message
    await expect(page.locator("text=You're on the list!")).toBeVisible();

    // Verify viral loop elements
    await expect(page.getByText(/You are #\d+ in line/)).toBeVisible();

    // Verify share link is present
    const shareLink = page.locator('input[readonly]');
    await expect(shareLink).toBeVisible();
    const linkValue = await shareLink.inputValue();
    expect(linkValue).toContain('waitlist?ref=');

    // Verify copy button
    const copyBtn = page.getByRole('button', { name: 'Copy Link' });
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify social share buttons
    await expect(page.getByRole('link', { name: /Share on X/ })).toBeVisible();
    await expect(page.getByRole('link', { name: /Share to WhatsApp/ })).toBeVisible();

    // Verify "Powered by OHC" footer
    const footerLink = page.getByRole('link', { name: '⚡ Powered by OHC' });
    await expect(footerLink).toBeVisible();
    const footerHref = await footerLink.getAttribute('href');
    expect(footerHref).toContain('/onboarding');
  });
});
