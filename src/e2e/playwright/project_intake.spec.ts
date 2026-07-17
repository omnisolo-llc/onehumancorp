import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Agentic Project Intake & Smart Proposal Engine CUJ', () => {
  // Test from persona Nora (Agency Principal)
  test('Owner reviews a proposal drafted from a new intake inquiry', async ({ page, request }) => {
    // 1. Simulate a new customer inquiry hitting the backend webhook endpoint
    const tenantId = 'tenant-' + randomUUID().substring(0, 8);
    const inquiryText = 'I need a full website redesign for my local cafe. Needs to be done in 3 weeks.';

    const res = await request.post('/api/v1/work-intake/submit?tenant=' + tenantId, {
      form: {
        name: 'John Cafe',
        email: 'john@cafe.com',
        details: inquiryText
      }
    });

    expect(res.status()).toBe(200);
    const resultJson = await res.json();
    expect(resultJson.success).toBe(true);
    expect(resultJson.proposal_drafted).toBe(true);

    // 2. Owner logs into the mobile-first unified feed
    await page.setViewportSize({ width: 375, height: 812 });

    // Auth bypass trick for the E2E env to set localStorage
    await page.goto('/');
    await page.evaluate(`localStorage.setItem('tenant_id', '${tenantId}')`);
    await page.evaluate(`localStorage.setItem('user_id', 'nora-agency')`);

    // Go to the unified feed
    await page.goto('/unified-feed');

    // We should see the proposal drafted in the unified feed
    await expect(page.locator('text=Approve Estimate')).toBeVisible({ timeout: 10000 });

    // Expand to see details
    await page.locator('text=Approve Estimate').click();

    // Look for the edit and approve buttons
    await expect(page.getByTestId('edit-proposal')).toBeVisible();
    await expect(page.getByTestId('feed-approve-btn')).toBeVisible();

    // 3. Edit the drafted text
    await page.getByTestId('edit-proposal').click();
    await page.getByTestId('edit-draft-textarea').fill('This is an updated proposal message for John.');

    // 4. Save & Approve & Send
    await page.getByTestId('save-edit-approve-btn').click();

    // Wait for feed item to disappear (it's resolved)
    await expect(page.locator('text=Approve Estimate')).not.toBeVisible();

    // 5. Verify the customer view URL is valid
    // Assuming the customer view is /proposals/[quote_id]
    const proposalUrl = `/proposals/${resultJson.quote_id}`;
    await page.goto(proposalUrl);

    // Check customer view matches
    await expect(page.locator('text=ACCEPTED')).toBeVisible(); // since we just approved it
    await expect(page.locator('text=Approve & Send')).not.toBeVisible(); // only visible if DRAFT
  });
});
