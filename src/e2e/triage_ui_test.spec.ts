import { test, expect } from './fixtures';

test.describe('Work Triage Agent CUJ', () => {
    test.beforeEach(async ({ page }) => {
        // Clear mock database before each test to ensure hermeticity
        await page.request.post('/api/dev/reset-triage-items?tenant_id=default');
    });

    test('Owner views feed, approves AI action, and clears feed', async ({ page, loginAs, adminUser }) => {
        // Step 1: Log in
        await loginAs(page, adminUser);

        // Step 2: Seed the database with a mock incoming message
        const mockPayload = {
            source: 'Instagram DM',
            priority: 'High',
            context: 'Customer asked: Do you make vegan cakes for this Saturday?',
            action_type: 'Draft Reply',
            action_payload: 'Hi! Yes, we have 2 vegan chocolate cakes left for this weekend. Would you like to reserve one? Here is a $50 deposit link: [Link]'
        };

        const response = await page.request.post('/api/dev/simulate-triage-item?tenant_id=default', {
            data: mockPayload
        });
        expect(response.status()).toBe(200);
        const { id: triageItemId } = await response.json();

        // Step 3: Navigate to the Feed
        await page.goto('/dashboard');

        // Wait for the feed to appear
        const feed = page.locator('[data-testid="work-triage-feed"]');
        await expect(feed).toBeVisible({ timeout: 10000 });

        // Step 4: Verify the Triage Card contents
        const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
        await expect(card).toBeVisible();
        await expect(card).toContainText('Instagram DM');
        await expect(card).toContainText('vegan cakes');
        await expect(card).toContainText('Draft Reply');

        // Step 5: Expand the card (if collapsed) and verify touch target
        // We ensure touch targets are >= 44px
        const box = await card.boundingBox();
        expect(box?.height).toBeGreaterThanOrEqual(44);
        expect(box?.width).toBeGreaterThanOrEqual(44);

        // Step 6: Click "Approve and Send"
        const approveButton = page.locator(`[data-testid="triage-approve-${triageItemId}"]`);
        await expect(approveButton).toBeVisible();
        await approveButton.click();

        // Step 7: Verify it disappears from the feed
        await expect(card).not.toBeVisible({ timeout: 5000 });

        // The empty state should eventually show up if it's the only item
        const emptyState = page.locator('text=caught up');
        await expect(emptyState).toBeVisible({ timeout: 10000 });
    });

    test('Triage feed renders correctly on mobile viewport (375px)', async ({ page, loginAs, adminUser }) => {
        await page.setViewportSize({ width: 375, height: 812 });
        await loginAs(page, adminUser);

        const mockPayload = {
            source: 'Email',
            priority: 'Normal',
            context: 'New catering request',
            action_type: 'Draft Quote',
            action_payload: 'Sending quote for $150.'
        };
        const response = await page.request.post('/api/dev/simulate-triage-item?tenant_id=default', {
            data: mockPayload
        });
        expect(response.status()).toBe(200);

        await page.goto('/dashboard');

        const feed = page.locator('[data-testid="work-triage-feed"]');
        await expect(feed).toBeVisible({ timeout: 10000 });

        // Verify the feed does not cause horizontal scroll
        const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
        expect(bodyWidth).toBeLessThanOrEqual(375);
    });
});
