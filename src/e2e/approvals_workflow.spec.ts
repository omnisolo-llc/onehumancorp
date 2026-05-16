import { test, expect } from '@playwright/test';

test.describe('Approvals Workflow E2E', () => {
    test.beforeEach(async ({ page }) => {
        // Go to login and authenticate
        await page.goto('/');

        // Force dashboard display
        await page.evaluate(() => {
            // @ts-ignore
            if (window.showScreen) {
                // @ts-ignore
                window.showScreen('dashboard-screen');
            }
        });

        // Wait for dashboard to be visible
        await expect(page.locator('#dashboard-screen')).toBeVisible();
    });

    test('should navigate to approvals screen', async ({ page }) => {
        // 1. Click "Review Pending Approvals" button
        await page.click('button:has-text("Review Pending Approvals")');

        // 2. Expect approvals screen to be visible
        await expect(page.locator('#approvals-screen')).toBeVisible();
        await expect(page.locator('h1', { hasText: 'Pending AI Approvals' })).toBeVisible();
    });

    test('should display loading state initially', async ({ page }) => {
        // Mock the API to delay response
        await page.route('**/api/agents/approvals', async route => {
            // Delay for 500ms
            setTimeout(() => route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ pending_approvals: [] })
            }), 500);
        });

        await page.click('button:has-text("Review Pending Approvals")');
        await expect(page.locator('#approvals-list-container p', { hasText: 'Loading approvals...' })).toBeVisible();
    });

    test('should display "No pending approvals" if empty', async ({ page }) => {
        await page.route('**/api/agents/approvals', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ pending_approvals: [] })
            });
        });

        await page.click('button:has-text("Review Pending Approvals")');
        await expect(page.locator('#approvals-list-container p', { hasText: 'No pending approvals. All good!' })).toBeVisible();
    });

    test('should approve a pending action and display toast', async ({ page }) => {
        const mockApprovals = {
            pending_approvals: [
                {
                    id: 'task-123',
                    department: 'Customer Success',
                    description: 'Drafted thank you email to Priya.'
                }
            ]
        };

        await page.route('**/api/agents/approvals', async route => {
            await route.fulfill({ json: mockApprovals });
        });

        let approvalRequestRecieved = false;
        await page.route('**/api/agents/approvals/task-123', async route => {
            const req = route.request();
            if (req.method() === 'POST') {
                const body = req.postDataJSON();
                if (body && body.approved === true) {
                    approvalRequestRecieved = true;
                }
            }
            await route.fulfill({ json: { success: true } });
        });

        await page.click('button:has-text("Review Pending Approvals")');

        // Expect the card to be visible
        const card = page.locator('#approval-card-task-123');
        await expect(card).toBeVisible();
        await expect(card.locator('p', { hasText: 'Drafted thank you email to Priya.' })).toBeVisible();

        // Click approve
        await card.locator('button:has-text("Approve & Send")').click();

        // Expect toast
        const toast = page.locator('#toast-notification');
        await expect(toast).toBeVisible();
        await expect(toast).toHaveText('Action approved and executed!');

        // Expect request was sent
        expect(approvalRequestRecieved).toBe(true);

        // Expect card is removed eventually
        await expect(card).not.toBeVisible();

        // Expect empty state to show up
        await expect(page.locator('#approvals-list-container p', { hasText: 'No pending approvals. All good!' })).toBeVisible();
    });

    test('should reject a pending action and display toast', async ({ page }) => {
        const mockApprovals = {
            pending_approvals: [
                {
                    id: 'task-456',
                    department: 'Operations',
                    description: 'Drafted full refund for Order #999.'
                }
            ]
        };

        await page.route('**/api/agents/approvals', async route => {
            await route.fulfill({ json: mockApprovals });
        });

        let approvalRequestRecieved = false;
        await page.route('**/api/agents/approvals/task-456', async route => {
            const req = route.request();
            if (req.method() === 'POST') {
                const body = req.postDataJSON();
                if (body && body.approved === false) {
                    approvalRequestRecieved = true;
                }
            }
            await route.fulfill({ json: { success: true } });
        });

        await page.click('button:has-text("Review Pending Approvals")');

        // Expect the card to be visible
        const card = page.locator('#approval-card-task-456');
        await expect(card).toBeVisible();

        // Click reject
        await card.locator('button:has-text("Reject")').click();

        // Expect toast
        const toast = page.locator('#toast-notification');
        await expect(toast).toBeVisible();
        await expect(toast).toHaveText('Action rejected and discarded.');

        // Expect request was sent
        expect(approvalRequestRecieved).toBe(true);

        // Expect card is removed eventually
        await expect(card).not.toBeVisible();
    });
});
