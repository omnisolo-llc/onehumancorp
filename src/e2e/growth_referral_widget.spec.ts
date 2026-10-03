import { test, expect } from './fixtures';
import { createRecordedOrderOwner, captureRecordedOrders } from './support/recorded_order_fixture';
import { createRecordedInvitation } from './support/recorded_invitation';

test.describe('Growth Referral Widget', () => {
  test('creates an actual invitation and preserves the empty recorded-order state', async ({ page, baseURL }) => {
    const owner = await createRecordedOrderOwner(page, baseURL, 0);
    const reading = captureRecordedOrders(page, owner);
    await page.goto('/team');
    await reading;

    // Wait for the Widget Builder button to appear under Invite & Earn section and click it
    const widgetBuilderBtn = page.getByRole('button', { name: /Invite to Cloud Team|Unlock Cloud Collaboration/ });

    // Explicitly wait for it to be attached/visible
    await widgetBuilderBtn.waitFor({ state: 'visible', timeout: 15000 });

    await expect(widgetBuilderBtn).toBeVisible();
    const copyInput = page.locator('input#cloud-bridge-invite-link');
    const inviteLink = await createRecordedInvitation(page, { button: widgetBuilderBtn, input: copyInput, invitee: 'pending-invite' });
    expect(inviteLink).toContain('/invite/');

    // Verify Copy button is present using exact text to avoid matching "Copy Embed Code"
    const copyBtn = page.getByRole('button', { name: 'Copy', exact: true });
    await expect(copyBtn).toBeVisible();

    // Verify WhatsApp and Twitter buttons are present
    const whatsappBtn = page.getByRole('button', { name: /Share on WhatsApp/i });
    await expect(whatsappBtn).toBeVisible();

    const twitterBtn = page.getByRole('button', { name: /Share on X/i });
    await expect(twitterBtn).toBeVisible();

    // Verify the embed section
    const embedHeading = page.getByRole('heading', { name: 'Embed Your Business' });
    await expect(embedHeading).toBeVisible();

    const copyEmbedBtn = page.getByRole('button', { name: 'Copy Embed Code' });
    await expect(copyEmbedBtn).toBeVisible();

    // The actual owned aggregate is empty even though the fixture has foreign orders.
    await expect(page.getByText('No recorded orders yet.', { exact: true })).toBeVisible();
    await expect(page.getByRole('img', { name: '10th Order Milestone' })).toHaveCount(0);
    await expect(page.getByRole('link', { name: /Share to WhatsApp/i })).toHaveCount(0);
  });
});
