import { test, expect } from '@playwright/test';

test('Alex Personal Trainer - Scheduling Approval Workflow', async ({ page }) => {



    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'alex-trainer',
            department: 'Operations',
            description: 'Draft schedule change for Tuesday class'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('alex@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="operations Action"')).toBeVisible();
    await expect(page.locator('text="Draft schedule change for Tuesday class"')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});

test('Sarah Consultant - Legal Policy Update Approval Workflow', async ({ page }) => {



    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'sarah-consultant',
            department: 'Legal',
            description: 'Draft updated Privacy Policy for 2024 compliance.'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('sarah@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="legal Action"')).toBeVisible();
    await expect(page.locator('text="Draft updated Privacy Policy for 2024 compliance."')).toBeVisible();


    await page.click('button:has-text("Reject / Edit")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
