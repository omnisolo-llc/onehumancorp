import { test, expect } from '@playwright/test';

test('Zack Developer - IT Services Approval Workflow', async ({ page }) => {



    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'zack-dev',
            department: 'Operations',
            description: 'Draft quote for custom React application development.'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('zack@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="operations Action"')).toBeVisible();
    await expect(page.locator('text="Draft quote for custom React application development."')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});

test('Nina Photographer - Portfolio Marketing Approval Workflow', async ({ page }) => {



    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'nina-photo',
            department: 'Marketing',
            description: 'Draft newsletter to subscribers: "New Wedding Packages for Spring 2024".'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('nina@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="marketing Action"')).toBeVisible();
    await expect(page.locator('text="Draft newsletter to subscribers: \\"New Wedding Packages for Spring 2024\\"."')).toBeVisible();


    await page.click('button:has-text("Reject / Edit")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});

test('Sam Chef - Operations Custom Order Approval Workflow', async ({ page }) => {



    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'sam-chef',
            department: 'Operations',
            description: 'Draft response to catering request for 50 people on Friday.'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('sam@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="operations Action"')).toBeVisible();
    await expect(page.locator('text="Draft response to catering request for 50 people on Friday."')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
