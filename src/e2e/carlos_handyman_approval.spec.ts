import { test, expect } from '@playwright/test';

test('Carlos Handyman - Sales Quote Approval Workflow', async ({ page }) => {
    // Context: Carlos receives a voicemail requesting a quote for gutter cleaning.
    // The Salesperson agent drafts a quote and requires approval.




    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'carlos-handyman',
            department: 'Sales',
            description: 'Draft quote $250 for gutter cleaning'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('carlos@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="sales Action"')).toBeVisible();
    await expect(page.locator('text="Draft quote $250 for gutter cleaning"')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
