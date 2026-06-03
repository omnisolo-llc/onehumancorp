import { test, expect } from './fixtures';

test.describe('Growth Loop: Waitlist Viral Mechanics', () => {
  test('User can sign up and get a referral link', async ({ page, request }) => {
    // Navigate to waitlist with an optional ref parameter
    await page.goto('/waitlist?ref=mockref123');

    // Wait for the waitlist header
    await expect(page.locator('h1', { hasText: 'The AI platform for' }).first()).toBeVisible();

    // Fill in email
    const emailInput = page.locator('input[type="email"]');
    await emailInput.fill(`test+${Date.now()}@example.com`);

    // Wait for the waitlist to process
    await page.locator('button[type="submit"]').click();

    // The success screen should appear with the position and referral link
    await expect(page.locator('text=You\'re on the list!')).toBeVisible({ timeout: 15000 });

    // Check for the position rendering
    const positionText = page.locator('text=in line');
    await expect(positionText).toBeVisible();

    // Check for the referral link
    const linkInput = page.locator('input[readonly]');
    await expect(linkInput).toBeVisible();
    const linkValue = await linkInput.inputValue();
    expect(linkValue).toContain('https://ohc.app/waitlist?ref=');

    // Check for the share buttons
    await expect(page.locator('text=Skip the Line')).toBeVisible();
    await expect(page.locator('text=Share on X')).toBeVisible();
  });
});
