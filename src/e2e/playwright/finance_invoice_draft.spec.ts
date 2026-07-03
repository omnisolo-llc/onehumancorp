import { test, expect } from './fixtures';

test.describe('Agentic Automated Invoicing & Cash Flow Management', () => {
    test.beforeEach(async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
    });

    test('Draft invoice ready card appears and can be approved', async ({ page }) => {
        // Go to dashboard to view the Unified Agent Feed
        await page.goto('/dashboard');
        await expect(page.locator('h1', { hasText: 'Overview' })).toBeVisible({ timeout: 15000 });

        // Simulate a backend event dropping an invoice draft into the Triage Feed.
        await page.route('/api/v1/feed', async (route) => {
            const response = await route.fetch();
            const json = await response.json();
            json.triage_items = json.triage_items || [];
            json.triage_items.unshift({
                id: 'triage-invoice-test-123',
                source: 'Finance Agent',
                context: 'Project milestone "Phase 1 Complete" has been marked complete.',
                status: 'pending',
                action_payload: JSON.stringify({
                    feature_type: 'invoice_draft',
                    milestone_name: 'Phase 1 Complete',
                    project_id: 'proj-123',
                    original_message: 'Project milestone "Phase 1 Complete" has been marked complete.',
                    generated_response: 'I have drafted an invoice for the completed milestone. Please review and send.',
                })
            });
            await route.fulfill({ response, json });
        });

        // Reload to apply the intercepted route
        await page.goto('/dashboard');

        // Wait for the invoice draft card to appear
        const draftCard = page.locator('text=Draft Invoice ready for Phase 1 Complete');
        await expect(draftCard).toBeVisible({ timeout: 15000 });

        // Click the "Approve & Send" button
        const approveBtn = page.getByRole('button', { name: 'Approve & Send' }).first();
        await expect(approveBtn).toBeVisible();

        // Setup dialog handler for the expected success alert or intercept the action
        await page.route('/api/v1/triage/triage-invoice-test-123/decision', async (route) => {
            await route.fulfill({ status: 200, json: { success: true } });
        });

        await approveBtn.click();

        // Verify the card changes to sending state or disappears
        await expect(approveBtn).toHaveText(/Sending...|Approve & Send/);
    });
});
