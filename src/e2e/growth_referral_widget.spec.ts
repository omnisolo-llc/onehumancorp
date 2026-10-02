import { test, expect } from './fixtures';

test.describe('Growth Referral Widget', () => {
  test('generates widget code and handles paywall correctly', async ({ page }) => {

    await page.goto('/team');

    // Wait for the Widget Builder button to appear under Invite & Earn section and click it
    const widgetBuilderBtn = page.getByRole('button', { name: /Invite to Cloud Team|Unlock Cloud Collaboration/ });

    // Explicitly wait for it to be attached/visible
    await widgetBuilderBtn.waitFor({ state: 'visible', timeout: 15000 });

    await expect(widgetBuilderBtn).toBeVisible();
    await widgetBuilderBtn.click();

    // Check that the invite link generated has the expected format
    const copyInput = page.locator('input#cloud-bridge-invite-link');
    await copyInput.waitFor({ state: 'visible', timeout: 15000 });
    await expect(copyInput).toBeVisible();

    // Check invite link value
    await expect(copyInput).toHaveValue(/invite/, { timeout: 15000 });
    const inviteLink = await copyInput.inputValue();
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

    // No verified order-count receipt exists in this page yet.
    await expect(page.getByRole('heading', { name: 'Order milestones unavailable' })).toBeVisible();
    await expect(page.getByRole('img', { name: '10th Order Milestone' })).toHaveCount(0);
    await expect(page.getByRole('link', { name: /Share to WhatsApp/i })).toHaveCount(0);
  });
});
