import { test, expect } from '@playwright/test';

test('Fatima Food Cart - Operations Cancel Order Approval Workflow', async ({ page }) => {
    // Context: An order was placed but they ran out of noodles.
    // The Operations Manager agent drafts a cancellation and SMS apology, requiring approval.




    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'fatima-food',
            department: 'Operations',
            description: 'Cancel order #899 (out of stock) and draft SMS apology'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('fatima@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="operations Action"')).toBeVisible();
    await expect(page.locator('text="Cancel order #899 (out of stock) and draft SMS apology"')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
