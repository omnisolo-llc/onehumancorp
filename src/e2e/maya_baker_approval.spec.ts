import { test, expect } from '@playwright/test';

test('Maya The Baker - Customer Success Draft Approval Workflow', async ({ page }) => {
    // Context: Maya receives a DM about custom cakes.
    // The Ambassador agent drafts a reply and requires approval.




    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'maya-bakery',
            department: 'CustomerSuccess',
            description: 'Draft reply to IG user @sweet_tooth'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('maya@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="customer_success Action"')).toBeVisible();
    await expect(page.locator('text="Draft reply to IG user @sweet_tooth"')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
