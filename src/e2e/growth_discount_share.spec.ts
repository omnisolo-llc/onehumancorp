import { test, expect } from '@playwright/test';

test.describe('Growth Discount Share CUJ', () => {

  test('Persona: Maya the Home Baker shares a discount to X (Twitter)', async ({ page, context }) => {
    // 1. Owner starts from the home page after user login via the UI
    await page.goto('/login');
    // Login flow
    await page.getByPlaceholder(/Email/i).fill('maya@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Land on the home page dashboard
    await expect(page.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });

    // 3. Click the Referrals button to navigate to the Growth dashboard
    await page.getByRole('button', { name: /Referrals/i }).click();

    // 4. Wait for Growth dashboard to be visible
    await expect(page.getByRole('heading', { name: /Referral Dashboard/i })).toBeVisible();

    // 5. Locate the 🐦 Share 10% Off on X (Twitter) button and click it
    // When clicking it, it opens a new window, so we need to intercept the new page event
    const [newPage] = await Promise.all([
      context.waitForEvent('page'),
      page.getByRole('button', { name: /Share 10% Off on X \(Twitter\)/i }).click()
    ]);

    // 6. Verify the user-facing outcome: assert that a new window opens to the Twitter intent URL containing the generated discount share link
    await newPage.waitForLoadState();
    const url = newPage.url();
    expect(url).toContain('twitter.com/intent/tweet');
    expect(url).toContain('https%3A%2F%2Fohc.store%2Fdiscount%2F');
  });

});
