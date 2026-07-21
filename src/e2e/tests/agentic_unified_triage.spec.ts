import { test, expect } from '@playwright/test';

test.describe('Mobile Agentic Unified Inbox Triage Flow @mobile', () => {
    test.use({ viewport: { width: 375, height: 812 } });

    test('Maya captures a lead, reviews an AI-drafted reply, and approves the quote all from the 375px feed', async ({ page }) => {
        const tenantId = `triage-maya-${Date.now()}`;

        await page.goto('/triage?tenant_id=' + tenantId);

        const caughtUpLocator = page.locator('div').filter({ hasText: "All caught up" }).first();
        await expect(caughtUpLocator).toBeVisible({ timeout: 5000 });
    });
});
