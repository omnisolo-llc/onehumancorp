import { test, expect } from '../fixtures';

test.describe('Cost Transparency Dashboard CUJ', () => {
    test('owner can view their tier limits and storage usage', async ({ page, adminUser, loginAs }) => {
        // 0. Login as admin user
        await loginAs(page, adminUser);

        // 1. Navigate to cost dashboard
        await page.goto(`${process.env.BASE_URL || 'http://127.0.0.1:18789'}/api/v1/ui/cost-dashboard.html?tenant=e2e-tenant`);

        // 2. Wait for the billing summary to load
        await expect(page.locator('#my-plan-name')).not.toHaveText('--', { timeout: 10000 });

        // 3. Verify that the correct limit information is displayed
        const storageText = await page.locator('#storage-text').innerText();

        // As a seeded Free tenant, the limit should be 500 MB.
        // Wait until it appears (we expect it to be 500 MB because 500 MB = 500 MB or 2048MB depending on formatting)
        // formatBytes converts 500 MB (524288000 bytes) -> 500 MB
        expect(storageText).toContain('500 MB');
    });
});
    test('owner can navigate back to their plan from cost dashboard', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto(`${process.env.BASE_URL || 'http://127.0.0.1:18789'}/api/v1/ui/cost-dashboard.html?tenant=e2e-tenant`);
        await expect(page.locator('#my-plan-name')).not.toHaveText('--', { timeout: 10000 });

        // Check back button
        const backBtn = page.locator('#back-to-my-plan');
        await expect(backBtn).toBeVisible();
    });

    test('owner can view total costs metric', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto(`${process.env.BASE_URL || 'http://127.0.0.1:18789'}/api/v1/ui/cost-dashboard.html?tenant=e2e-tenant`);
        await expect(page.locator('#my-plan-name')).not.toHaveText('--', { timeout: 10000 });

        const totalCosts = await page.locator('#cost-dashboard-total-costs').innerText();
        expect(totalCosts).toContain('$');
    });

    test('owner can view projected monthly cost metric', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto(`${process.env.BASE_URL || 'http://127.0.0.1:18789'}/api/v1/ui/cost-dashboard.html?tenant=e2e-tenant`);
        await expect(page.locator('#my-plan-name')).not.toHaveText('--', { timeout: 10000 });

        const projectedCost = await page.locator('#cost-dashboard-projected').innerText();
        expect(projectedCost).toContain('$');
    });

    test('owner can view llm cost metric', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto(`${process.env.BASE_URL || 'http://127.0.0.1:18789'}/api/v1/ui/cost-dashboard.html?tenant=e2e-tenant`);
        await expect(page.locator('#my-plan-name')).not.toHaveText('--', { timeout: 10000 });

        const llmCost = await page.locator('#cost-dashboard-llm').innerText();
        expect(llmCost).toContain('$');
    });
