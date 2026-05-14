import { test, expect } from '@playwright/test';

test('David Plumber - Business Advisory Approval Workflow', async ({ page }) => {



    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'david-plumber',
            department: 'BusinessAdvisory',
            description: 'Draft strategy email for expanding service area'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('david@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="business_advisory Action"')).toBeVisible();
    await expect(page.locator('text="Draft strategy email for expanding service area"')).toBeVisible();


    await page.click('button:has-text("Approve & Send")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});

test('Emma Freelancer - Operations Invoice Approval Workflow', async ({ page }) => {



    await page.request.post('/api/test/inject_approval', {
        data: {
            tenant_id: 'emma-freelancer',
            department: 'Finance',
            description: 'Draft invoice for $1500 to ACME Corp'
        }
    });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('emma@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('#approval-badge')).toBeVisible();
    await expect(page.locator('#approval-badge')).toHaveText('1');

    await page.click('button:has-text("Pending Approvals")');
    await expect(page.locator('text="Pending Approvals"').first()).toBeVisible();

    await expect(page.locator('text="finance Action"')).toBeVisible();
    await expect(page.locator('text="Draft invoice for $1500 to ACME Corp"')).toBeVisible();


    await page.click('button:has-text("Reject / Edit")');
    await expect(page.locator('text="No pending approvals."')).toBeVisible();
});
