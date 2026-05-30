import { test, expect } from './fixtures';

test.describe('Viral Booking Loop E2E', () => {
  test('user can book an appointment and is exposed to the viral growth loop', async ({ page, context }) => {
    // Navigate to the booking page
    await page.goto('/booking');
    await page.waitForLoadState('networkidle');

    // Verify the booking page form renders correctly
    await expect(page.getByRole('heading', { name: 'Schedule your session' })).toBeVisible();

    // Fill out the booking form
    await page.getByPlaceholder('John Doe').fill('Leo Tutor');
    await page.getByPlaceholder('john@example.com').fill('leo@music.com');

    // Playwright `fill` on date inputs expects YYYY-MM-DD
    await page.locator('input[name="date"]').fill('2024-12-01');

    // Select time
    await page.locator('select[name="time"]').selectOption('10:00');

    // Submit the form
    await page.getByRole('button', { name: 'Confirm Booking' }).click();

    // Verify success screen is displayed
    await expect(page.getByRole('heading', { name: 'Booking Confirmed!' })).toBeVisible({ timeout: 5000 });

    // Verify the viral loop elements are present
    await expect(page.getByText('Want your own booking page?')).toBeVisible();

    // Verify CTA points to the correct viral referral link (default is 'DEFAULT' because we are not setting it in localStorage in the test, so it falls back to 'DEFAULT')
    const createYoursBtn = page.getByRole('link', { name: 'Create yours for free' });
    await expect(createYoursBtn).toBeVisible();
    await expect(createYoursBtn).toHaveAttribute('href', 'ohc://join?ref=DEFAULT');

    // Verify the Powered by OHC footer
    const poweredByLink = page.getByRole('link', { name: '⚡ Powered by OHC' });
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toHaveAttribute('href', 'ohc://join?ref=DEFAULT');

    // Intercept Twitter intent window to verify Share on X functionality
    const [newPage] = await Promise.all([
      context.waitForEvent('page'),
      page.getByRole('button', { name: 'Share on X' }).click()
    ]);

    await newPage.waitForLoadState();
    expect(newPage.url()).toContain('twitter.com/intent/tweet');
    expect(newPage.url()).toContain(encodeURIComponent('ohc://join?ref=DEFAULT'));

    await newPage.close();
  });
});
