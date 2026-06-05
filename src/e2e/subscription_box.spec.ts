import { test, expect } from '@playwright/test';

test.describe('Autonomous Subscription Box Lifecycle', () => {

  test('Maya creates and manages a monthly cake subscription', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[placeholder="Email address"]', 'maya@example.com');
    await page.click('button:has-text("Continue")');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign in")');

    await expect(page).toHaveURL('/dashboard');

    await page.goto('/products/new');

    await page.waitForSelector('text=Take a photo or upload');

    // Mute missing assets that might not be loaded initially in UI tests
    await page.fill('input[type="text"]', 'Vegan Cake');
    await page.fill('input[type="text"]', '10.00');

    // Simulate clicking the Offer as Subscription toggle by clicking its parent label
    await page.locator('text=Offer as Subscription').click();

    await expect(page.locator('text=Deliver every')).toBeVisible();

    await page.click('button:has-text("Publish Product")');

    await expect(page.locator('text=Product Published!')).toBeVisible();
    await page.click('text=Return to Dashboard');

    await expect(page).toHaveURL('/dashboard');
  });

  test('Customer manages their subscription using magic link', async ({ page }) => {
    const magicToken = "mock-magic-token-123";
    await page.goto(`/subscriptions/manage?token=${magicToken}`);

    await expect(page.locator('text=Manage Subscription')).toBeVisible();
    await expect(page.locator('text=You are authenticated via a secure magic link.')).toBeVisible();

    await page.click('button:has-text("Pause Subscription")');
    await expect(page.locator('text=Subscription successfully updated to pause.')).toBeVisible();

    await page.click('button:has-text("Resume Subscription")');
    await expect(page.locator('text=Subscription successfully updated to resume.')).toBeVisible();

    await page.click('button:has-text("Cancel Subscription")');
    await expect(page.locator('text=Subscription successfully updated to cancel.')).toBeVisible();
  });
});
