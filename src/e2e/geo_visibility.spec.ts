import { test, expect } from '@playwright/test';

const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

test('GEO Visibility Flow: scan and auto-apply recommendation', async ({ page }) => {
    // 1. Start from Home Page
    await page.goto(baseUrl);
    await expect(page).toHaveTitle(/One Human Corp/);

    // 2. Perform Login
    await page.fill('input[placeholder="name@company.com"]', 'test@ohc.local');
    await page.fill('input[placeholder="••••••••"]', 'test_password');
    await page.click('text=Sign In');

    // 3. Wait for Dashboard
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 10000 });

    // 4. Navigate to AI Visibility Tool via Quick Actions
    await page.click('text=AI Visibility');

    // 5. Verify Tool UI is open and shows correct initial state
    await expect(page.locator('text=Generative Engine Optimization (GEO)')).toBeVisible();
    await expect(page.locator('text=N/A')).toBeVisible(); // Initial score

    // 6. Start Scan
    await page.click('text=Scan Business Metadata');

    // 7. Verify scanning indicator and eventual recommendations list
    await expect(page.locator('text=Scanning Business Profile & Schema...')).toBeVisible();
    await expect(page.locator('text=Add Schema.org Markup')).toBeVisible({ timeout: 10000 });

    // 8. Auto-Apply the first recommendation
    await page.click('text=Auto-Apply (Pro)');

    // 9. Verify impact applied correctly
    await expect(page.locator('text=✓ Applied')).toBeVisible();
    await expect(page.locator('text=Optimization Applied!')).toBeVisible();

    // 10. Close GEO tool
    await page.click('text=Close');
});
