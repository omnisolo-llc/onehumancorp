import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed - Promoter Auto-Draft', () => {
  test('owner logs in, sees Promoter social draft, and approves it', async ({ page }) => {
    // 1. Start by logging in via UI (mandatory for owner E2E flow)
    await page.goto('/login');
    // Ensure we are on the login page and use test credentials
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');

    // Ensure successful navigation to the dashboard after login
    await expect(page).toHaveURL(/\/dashboard/);

    // 2. Wait for the feed to be visible
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // In a real environment, we would first create a product.
    // We will simulate it by going to the products flow if the draft isn't there,
    // but the task just asked to verify the agent feed flow.
    // Let's create a product first just to make sure the flow is end-to-end if needed.

    await page.goto('/products/new');
    await expect(page.locator('h1').filter({ hasText: 'Add Product' })).toBeVisible();

    // Trigger file upload to bypass text flow
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('span:has-text("Take a photo or upload")').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({
      name: 'test-image.jpg',
      mimeType: 'image/jpeg',
      buffer: Buffer.from('fake image data')
    });

    // Wait for AI parsing
    await expect(page.locator('p').filter({ hasText: 'The Promoter is working its magic...' })).toBeVisible();

    // Wait for form
    await expect(page.locator('input[value="Artisan Cupcake"]')).toBeVisible({ timeout: 10000 });

    // Publish
    await page.click('button:has-text("Looks Good")');
    await expect(page.locator('h2').filter({ hasText: 'Product Published!' })).toBeVisible();

    // 3. Navigate back to dashboard to check the agent feed
    await page.goto('/dashboard');
    await expect(page).toHaveURL(/\/dashboard/);
    await expect(feedSection).toBeVisible();

    // 4. Verify the Promoter Card exists in the feed
    // The feed title is "Draft Social Post: Artisan Cupcake"
    // And content has "New product detected!"
    const promoterDraft = page.locator('div.app-card:has-text("New product detected!")');
    await expect(promoterDraft).toBeVisible({ timeout: 15000 }); // give worker time to process

    // Check content details
    await expect(promoterDraft).toContainText('Schedule a post?');
    await expect(promoterDraft).toContainText('Instagram / TikTok Draft');

    // 5. Approve & Schedule
    const approveBtn = promoterDraft.locator('button[data-testid="approve-social-post"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // After approval, the card should either disappear or state should change (depending on implementation, here usually the card is removed from feed)
    await expect(promoterDraft).not.toBeVisible({ timeout: 10000 });
  });
});
