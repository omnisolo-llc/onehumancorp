import { test, expect } from '@playwright/test';

test('AI Agent Department Approval Workflow', async ({ page }) => {
    // We mock the backend response for approvals since E2E might not have a reliable way to trigger high-risk tasks quickly


    // Login

    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'test-tenant',
            department: 'Sales',
            description: 'Draft quote for Maya'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Verify dashboard and badge
    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    // Navigate to Approvals
    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    // Verify pending task details
    await expect(page.locator('text="Sales Action"')).toBeVisible();
    await expect(page.locator('text="Draft quote for Maya"')).toBeVisible();

    // Re-route to return empty list after approval

    // Approve the action
    await page.click('button:has-text("Approve & Send")');

    // Wait for empty state
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
