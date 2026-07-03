import { test, expect } from './fixtures';

test('AI Scheduling and Subscription Workflow', async ({ page }) => {
  // Login to the OHC mobile view
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('/feed');

  // Trigger the simulation
  await page.click('data-testid=simulate-booking-sub-btn');

  // Wait for the simulated card to appear
  await expect(page.locator('text=Booking & Subscription from Sarah')).toBeVisible({ timeout: 10000 });

  // Verify the AI drafted email and times
  await expect(page.locator('text=AI Drafted Reply')).toBeVisible();
  await expect(page.locator('text=Tomorrow at 10:00 AM')).toBeVisible();
  await expect(page.locator('text=Tomorrow at 2:00 PM')).toBeVisible();
  await expect(page.locator('text=Wednesday at 11:00 AM')).toBeVisible();

  // Verify the subscription link
  await expect(page.locator('text=Start $100/mo Subscription')).toBeVisible();

  // Review the draft and approve it
  const approveButton = page.locator('data-testid=feed-approve-btn', { hasText: 'Approve & Send' });

  // Verify button is large enough for mobile touch target (44x44px minimum according to requirement)
  const box = await approveButton.boundingBox();
  expect(box?.width).toBeGreaterThanOrEqual(44);
  expect(box?.height).toBeGreaterThanOrEqual(44);

  await approveButton.click();

  // Card should be dismissed after approval
  await expect(page.locator('text=Booking & Subscription from Sarah')).toBeHidden();
});
