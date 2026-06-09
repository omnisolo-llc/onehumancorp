import { test, expect } from '@playwright/test';

/**
 * CUJ: Proactive Social Media Promotion
 * Persona: Priya (Boutique Owner)
 * Scenario: Priya adds a new product to her store. The Promoter Agent automatically
 * detects this and drafts social media posts for her review in the Unified Agent Feed.
 * Priya reviews the captions and approves them for scheduling.
 */
test('Promoter Agent proactive flow: Product creation to feed approval', async ({ page }) => {
    // 1. Login
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);

    // 2. Navigate to Inventory and Create a Product
    await page.click('nav a:has-text("Inventory"), a:has-text("Inventory")');
    await expect(page).toHaveURL(/\/inventory/);

    const productName = `E2E Silk Scarf ${Date.now()}`;
    await page.click('button:has-text("Add Product")');
    await page.fill('input[placeholder*="Product Name"], input[name="name"]', productName);
    await page.fill('textarea[placeholder*="Description"], textarea[name="description"]', 'A beautiful hand-woven silk scarf.');
    await page.fill('input[name="price"], input[placeholder*="0.00"]', '45.00');
    await page.click('button:has-text("Save Product"), button:has-text("Create")');

    // Wait for product to be saved
    await expect(page.locator(`text=${productName}`)).toBeVisible();

    // 3. Go back to Dashboard and check Unified Agent Feed
    await page.click('nav a:has-text("Dashboard"), a[href="/dashboard"]');
    await expect(page).toHaveURL(/\/dashboard/);

    // The agent might take a moment to process the event and generate the draft
    // We poll for the card appearing in the Proposals tab
    const socialPromoCard = page.locator(`text=Draft multi-platform social posts for ${productName}`);
    await expect(socialPromoCard).toBeVisible({ timeout: 15000 });

    // 4. Interact with the Social Promotion Card
    // Verify platform tabs exist
    await expect(page.locator('button:has-text("Instagram")')).toBeVisible();
    await expect(page.locator('button:has-text("Tiktok")')).toBeVisible();
    await expect(page.locator('button:has-text("Facebook")')).toBeVisible();

    // Check default caption (Instagram)
    await expect(page.locator('text=Link in bio')).toBeVisible();

    // Switch to TikTok
    await page.click('button:has-text("Tiktok")');
    await expect(page.locator('text=Check out our new')).toBeVisible();

    // 5. Approve & Schedule
    const approveBtn = page.locator('button[data-testid="approve-social-promotion"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify card is removed from proposals (optimistic UI)
    await expect(socialPromoCard).not.toBeVisible();

    // 6. Verify in Activity Feed
    await page.click('button:has-text("Activity Feed")');
    const activityEntry = page.locator(`text=Draft multi-platform social posts for ${productName}`);
    await expect(activityEntry).toBeVisible();
    await expect(page.locator('span:has-text("APPROVED")')).toBeVisible();
});
