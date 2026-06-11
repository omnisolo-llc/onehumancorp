import { test, expect } from '@playwright/test';

test('End-to-End Unified Offering Creation Flow', async ({ page }) => {
  // 1. User logs in and sees the Home Feed
  await page.goto('/dashboard');
  await expect(page.locator('h2').filter({ hasText: /Welcome back/ })).toBeVisible();

  // 2. User taps the primary "+" FAB (Floating Action Button)
  const fab = page.locator('button').filter({ hasText: '+' });
  await fab.click();

  // 3. User taps "New Offering"
  const newOfferingBtn = page.locator('a').filter({ hasText: 'New Offering' });
  await newOfferingBtn.click();

  // 4. User is prompted: "What do you want to offer?"
  await expect(page.locator('label').filter({ hasText: 'What do you want to offer?' })).toBeVisible();

  // 5. User types: "Guitar lessons for beginners, 1 hour"
  await page.fill('textarea[placeholder="e.g. Guitar lessons for beginners, 1 hour"]', 'Guitar lessons for beginners, 1 hour');

  // 6. User clicks Generate Details
  await page.click('button:has-text("Generate Details")');

  // 7. Loading state (glassmorphism shimmer)
  await expect(page.locator('p').filter({ hasText: 'AI is preparing your offering...' })).toBeVisible();

  // 8. Form appears pre-filled
  await expect(page.locator('input[value="Beginner Guitar Lesson (1 Hour)"]')).toBeVisible({ timeout: 5000 });
  await expect(page.locator('input[value="Service"]')).toBeVisible();
  await expect(page.locator('input[value="50.00"]')).toBeVisible();


  // 9. User toggles "Split this payment", enters partner, and slides percentage
  await page.locator('label', { hasText: 'Split this payment' }).locator('..').locator('input[type="checkbox"]').check({ force: true });
  await page.fill('input[placeholder="Partner name, phone, or email"]', 'Sarah');
  await page.fill('input[type="range"]', '70');
  await page.dispatchEvent('input[type="range"]', 'change');

  // Verify split preview text
  await expect(page.locator('div').filter({ hasText: /If this sells for \$50\.00, Sarah gets \$35\.00, you get \$15\.00/ })).toBeVisible();

  // 10. User modifies price to $45 and taps "Publish"
  const priceInput = page.locator('input[value="50.00"]');

  await priceInput.fill('45.00');
  await page.click('button:has-text("Publish Offering")');

  // 10. Success toast. The new offering is immediately visible on the live public storefront.
  await expect(page.locator('h2').filter({ hasText: 'Offering Published!' })).toBeVisible();
  await expect(page.locator('p').filter({ hasText: 'Your new offering is now live on your storefront.' })).toBeVisible();
});
