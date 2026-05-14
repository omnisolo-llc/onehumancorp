import { test, expect } from '@playwright/test';

test('Leo Tutor - Finance Refund Approval Workflow', async ({ page }) => {
    // Context: A student missed a lesson with a valid excuse.
    // The Accountant agent drafts a $25 refund and requires approval.




    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'leo-tutor',
            department: 'Finance',
            description: 'Draft partial refund of $25.00'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('leo@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="finance Action"')).toBeVisible();
    await expect(page.locator('text="Draft partial refund of $25.00"')).toBeVisible();


    await page.click('button:has-text("Reject / Edit")'); // Rejecting this one
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
