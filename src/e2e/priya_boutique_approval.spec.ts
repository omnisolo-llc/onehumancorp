import { test, expect } from '@playwright/test';

test('Priya Boutique - Marketing Post Approval Workflow', async ({ page }) => {
    // Context: Priya needs to post about a new dress collection.
    // The Promoter agent drafts a social post and requires approval.




    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'priya-boutique',
            department: 'Marketing',
            description: 'Draft Instagram post: "New Summer Dresses arrived!'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('priya@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="marketing Action"')).toBeVisible();
    await expect(page.locator('text="Draft Instagram post: \\"New Summer Dresses arrived!"')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
