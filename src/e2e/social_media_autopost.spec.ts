import { test, expect } from '@playwright/test';

test('Promoter Agent automatically creates a social post draft on product creation and can be approved in Unified Feed', async ({ page, request }) => {
    // 1. Create a product via API to trigger the PromoterWorker
    const loginRes = await request.post('/api/auth/login', {
        data: {
            email: 'admin@ohc.local',
            password: 'changeme'
        }
    });
    expect(loginRes.ok()).toBeTruthy();

    const productRes = await request.post('/api/catalog/products', {
        data: {
            name: 'Viral Summer Dress',
            description: 'A beautiful summer dress',
            item_type: 'Physical'
        }
    });
    expect(productRes.ok()).toBeTruthy();

    // 2. Start by logging in via UI (mandatory for owner E2E flow)
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');

    // Ensure successful navigation to the dashboard after login
    await expect(page).toHaveURL(/\/dashboard/);

    // 3. Verify the Unified Agent Feed has the new social post draft
    // We expect the text we added in our UnifiedAgentFeed UI update
    await expect(page.locator('text=Social media posts ready for Viral Summer Dress')).toBeVisible({ timeout: 15000 });

    // 4. Verify variants exist
    await expect(page.locator('text=TikTok')).toBeVisible();
    await expect(page.locator('text=Instagram')).toBeVisible();
    await expect(page.locator('text=Facebook')).toBeVisible();

    // 5. Approve the post
    const approveBtn = page.locator('button[data-testid="approve-social-post"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 6. Verify the card is removed after approval
    await expect(page.locator('text=Social media posts ready for Viral Summer Dress')).not.toBeVisible();
});
