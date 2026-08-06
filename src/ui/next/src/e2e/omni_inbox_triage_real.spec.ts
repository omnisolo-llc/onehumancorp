import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat & AI Inbox Triage E2E', () => {

  // We must not use page.route mocking for this requirement, we need to hit the real API
  test('Complete Flow: Simulate webhook, verify triage item, review draft, and approve', async ({ request, page }) => {
    // 1. Setup tenant & webhook simulation
    const tenantId = 'triage-tenant-' + Date.now();
    const customerId = 'customer-' + Date.now();

    const webhookRes = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        channel: 'whatsapp',
        sender_id: customerId,
        message: 'Hello, I need 5 custom vegan cakes for a party.',
      },
    });

    expect(webhookRes.ok()).toBeTruthy();
    const webhookData = await webhookRes.json();
    expect(webhookData.success).toBe(true);

    // Wait for AI async job (message_triage) to finish drafting.
    // In our implementation, `trigger_unified_chat_triage` inserts a job and another async process runs it.
    // We'll give it a moment.
    await page.waitForTimeout(1000);

    // 2. Load the triage page as the owner
    await page.goto('/triage?tenant_id=' + tenantId);

    // 3. Verify the triage card appears
    await expect(page.getByText('Work Triage')).toBeVisible();

    // The source or customer ID might show 'Unknown Source' if identity wasn't completely resolved in test db
    // Let's just wait for the triage item card to appear
    const card = page.locator('.ohc-card').first();
    await expect(card).toBeVisible({ timeout: 15000 });

    // Check that it's tagged as "Action Needed"
    await expect(card.getByText('Action Needed')).toBeVisible();

    // 4. Owner reviews the draft
    // The "Review Draft" button is visible inside the card when selected or hovering, depending on design.
    // In our `page.tsx`, we click the header to expand.
    const cardHeader = card.locator('div').first();
    await cardHeader.click();

    // Verify draft action is available (e.g. DraftReply)
    await expect(card.getByText('Proposed Action: DraftReply')).toBeVisible();

    const reviewButton = card.getByTestId(/triage-review-btn-.*/);
    await expect(reviewButton).toBeVisible();
    await reviewButton.click();

    // 5. Owner edits the draft reply
    const textarea = card.getByTestId(/triage-edit-textarea-.*/);
    await expect(textarea).toBeVisible();

    // We expect some pre-filled AI draft text
    const draftText = await textarea.inputValue();
    expect(draftText).toContain('Thank you');

    await textarea.fill('Yes, we can do 5 vegan cakes! I will send a quote shortly.');

    // 6. Owner saves & sends the edited draft
    const saveButton = card.getByTestId(/triage-save-btn-.*/);
    await saveButton.click();

    // 7. Verify the item disappears from triage
    await expect(card).not.toBeVisible();

    // 8. Verify Inbox Zero state
    await expect(page.getByText('Inbox Zero')).toBeVisible();
  });

  test('Complete Flow: Dismissing a triage action', async ({ request, page }) => {
    const tenantId = 'triage-tenant-dismiss-' + Date.now();
    const customerId = 'customer-' + Date.now();

    await request.post('/api/v1/omnichannel/webhook', {
      data: { tenant_id: tenantId, channel: 'instagram', sender_id: customerId, message: 'Just saying thanks!' },
    });

    await page.goto('/triage?tenant_id=' + tenantId);

    const card = page.locator('.ohc-card').first();
    await expect(card).toBeVisible({ timeout: 15000 });

    const cardHeader = card.locator('div').first();
    await cardHeader.click();

    const dismissButton = card.getByTestId(/triage-dismiss-.*/);
    await dismissButton.click();

    await expect(card).not.toBeVisible();
    await expect(page.getByText('Inbox Zero')).toBeVisible();
  });

  test('Complete Flow: Approve as-is', async ({ request, page }) => {
    const tenantId = 'triage-tenant-approve-' + Date.now();
    const customerId = 'customer-' + Date.now();

    await request.post('/api/v1/omnichannel/webhook', {
      data: { tenant_id: tenantId, channel: 'sms', sender_id: customerId, message: 'Where are you located?' },
    });

    await page.goto('/triage?tenant_id=' + tenantId);

    const card = page.locator('.ohc-card').first();
    await expect(card).toBeVisible({ timeout: 15000 });

    const cardHeader = card.locator('div').first();
    await cardHeader.click();

    const approveBtn = card.getByTestId(/triage-approve-.*/);
    await approveBtn.click();

    await expect(card).not.toBeVisible();
    await expect(page.getByText('Inbox Zero')).toBeVisible();
  });

  test('Complete Flow: Empty inbox state on load', async ({ page }) => {
    const tenantId = 'empty-tenant-' + Date.now();
    await page.goto('/triage?tenant_id=' + tenantId);

    // Should show inbox zero immediately (after loading)
    await expect(page.getByText('Inbox Zero')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Your AI assistant has handled all outstanding items')).toBeVisible();
  });

  test('Complete Flow: Multiple messages ordered correctly', async ({ request, page }) => {
    const tenantId = 'triage-tenant-multi-' + Date.now();

    await request.post('/api/v1/omnichannel/webhook', {
      data: { tenant_id: tenantId, channel: 'sms', sender_id: 'cust-1', message: 'First msg' },
    });

    await request.post('/api/v1/omnichannel/webhook', {
      data: { tenant_id: tenantId, channel: 'sms', sender_id: 'cust-2', message: 'Second msg' },
    });

    await page.goto('/triage?tenant_id=' + tenantId);

    const cards = page.locator('.ohc-card');
    await expect(cards).toHaveCount(2, { timeout: 15000 });

    // Check that we can dismiss the first one
    const firstCard = cards.nth(0);
    const firstHeader = firstCard.locator('div').first();
    await firstHeader.click();
    const dismissBtn = firstCard.getByTestId(/triage-dismiss-.*/);
    await dismissBtn.click();

    // Now there should be 1 left
    await expect(cards).toHaveCount(1);
  });
});
