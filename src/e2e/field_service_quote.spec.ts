import { test, expect } from './fixtures';
import { Uuid } from './test-utils';

test.describe('Agent-Driven Local Service Quoting & Dispatch', () => {
    test('Field service operator receives quote and approves it', async ({ adminPage: page, request }) => {
        // 1. Simulate inbound SMS that should trigger quote generation
        const signalResponse = await request.post('/api/v1/work_triage/simulate_inbound_signal', {
            data: {
                source: "sms",
                payload: {
                    message: "Can someone install a ceiling fan?",
                    phone: "+15551234567"
                }
            },
            headers: {
                'Content-Type': 'application/json',
            }
        });
        expect(signalResponse.ok()).toBeTruthy();

        // 2. Navigate to the inbox / agent feed
        await page.goto('/inbox');

        // Wait for the feed to load
        await page.waitForSelector('text=Agent Feed');

        // 3. Verify the ReviewDraftQuoteCard appears
        const draftReadyText = await page.waitForSelector('text=Draft Quote Ready', { state: 'visible', timeout: 5000 });
        expect(draftReadyText).toBeDefined();

        await expect(page.locator('text=Ceiling Fan Installation')).toBeVisible();
        await expect(page.locator('text=Action Required')).toBeVisible();

        // 4. Owner approves the quote
        const approveButton = page.getByTestId('feed-approve-btn');
        await expect(approveButton).toBeVisible();
        await approveButton.click();

        // 5. Verify optimistic update - card should disappear
        await expect(page.locator('text=Draft Quote Ready')).not.toBeVisible();
    });
});