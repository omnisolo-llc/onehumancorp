import { test, expect } from '@playwright/test';
import { test as customTest } from './fixtures';

test.describe('Viral Event RSVP Builder Loop', () => {
  test.skip('User can configure event RSVP and see embed code with viral loop', async ({ browser }) => {
    const page = await customTest(browser);

    // Navigate to the Dashboard
    await page.goto('/dashboard');

    // Click on the Event RSVP card link
    await page.click('a[href="/event-rsvp-builder"]');

    // Expect to be on the builder page
    await expect(page).toHaveURL('/event-rsvp-builder');
    await expect(page.locator('h1')).toContainText('Event RSVP Builder');

    // Fill out the form
    await page.fill('input[placeholder="e.g. Summer Pop-up"]', 'My Awesome Webinar');
    await page.fill('input[placeholder="e.g. Aug 15 @ 12 PM"]', 'Oct 31 @ 2 PM');
    await page.fill('input[placeholder="e.g. Main Street Plaza or Zoom Link"]', 'https://zoom.us/j/123456');

    // Select dark theme
    await page.click('button:has-text("Dark")');

    // Try to remove branding (without pro, will trigger soft paywall)
    await page.check('input[id="removeBranding"]');

    // Wait for paywall modal
    await expect(page.locator('h2', { hasText: 'Upgrade to Remove Branding' })).toBeVisible();

    // Close paywall
    await page.locator('button', { hasText: '×' }).click();

    // Wait for paywall modal to disappear
    await expect(page.locator('h2', { hasText: 'Upgrade to Remove Branding' })).toBeHidden();

    // Generate Embed Code
    await page.click('button:has-text("Get Widget Code")');

    // Expect embed modal to appear
    await expect(page.locator('h2', { hasText: 'Embed RSVP Widget' })).toBeVisible();

    // Verify the textarea contains the correct iframe URL
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible();
    const embedCode = await textarea.inputValue();

    // Verify parameters in the URL
    expect(embedCode).toContain('/api/v1/growth/event-rsvp/embed');
    expect(embedCode).toContain('title=My%20Awesome%20Webinar');
    expect(embedCode).toContain('theme=dark');
    expect(embedCode).toContain('branding=true'); // Still true because we closed the paywall and didn't upgrade

    // Close the embed modal
    await page.locator('button[aria-label="Close modal"]').click();
  });
});
