import { test, expect } from '../onboarding_fixtures';
import {captureSetupPost,completeManualSetup,expectLaunchedSetup,expectPreparedSetup,verifiedSetupOwner} from '../support/legacy_manual_setup';

test.describe('Zero-Click Onboarding to Agent Feed', () => {
  test('User completes chat onboarding and sees welcome card on feed', async ({ page }) => {
    // Navigate to the setup route
    await page.goto('/setup.html');
    const identity=await verifiedSetupOwner(page);

    // Make sure we're on a mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });

    // Click Conversational Setup
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Wait for chat input to be visible
    const chatInput = page.locator('input[placeholder*="e.g. I am a home baker"]');
    await expect(chatInput).toBeVisible();

    // Type a simple sentence and press Enter
    await chatInput.fill('I run a mobile dog grooming service in Austin');
    const chat=captureSetupPost(page,'chat');
    const [reply]=await Promise.all([chat,chatInput.press('Enter')]);
    if(reply.status===503){
      expect(reply.body).toMatchObject({error:'onboarding_ai_unconfigured'});
      expect(reply.body.success).not.toBe(true);
      await completeManualSetup(page,'chat','I run a mobile dog grooming service in Austin',identity);
    }else{
      expect(reply.status).toBe(200);expect(reply.body).toMatchObject({is_complete:true});
      expect(reply.body.error??null).toBeNull();
      const approval=page.locator('#step-approval');
      await expect(approval.getByRole('heading',{name:'Ready to Launch'})).toBeVisible({timeout:45000});
      await expect(approval.locator('#approval-details')).not.toBeEmpty();
      const preparation=captureSetupPost(page,'start');const launch=captureSetupPost(page,'launch');
      const [prepared,launched]=await Promise.all([preparation,launch,approval.getByRole('button',{name:'Approve & Complete Setup'}).click()]);
      const id=expectPreparedSetup(prepared,identity);await expectLaunchedSetup(page,launched,id,identity);
    }
    await expect(page).toHaveURL(/\/dashboard(?:\.html)?$/, { timeout: 60000 });
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();

    // Check horizontal scroll by verifying document width equals window innerWidth
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBeFalsy();
  });

  test('Conversational Setup prevents empty submissions', async ({ page }) => {
    await page.goto('/setup.html');

    // Click Conversational Setup
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Ensure input is empty and send
    const chatInput = page.locator('input[placeholder*="e.g. I am a home baker"]');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('');
    await page.locator('#chat-send-btn').click();

    // Message shouldn't appear in chat history
    const userMessages = page.locator('.chat-message.user');
    await expect(userMessages).toHaveCount(0);
  });

  test('Conversational Setup opens image upload input when toggled', async ({ page }) => {
    await page.goto('/setup.html');

    // Click Conversational Setup
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // The image container should be hidden by default
    const imageContainer = page.locator('#chat-image-container');
    await expect(imageContainer).toBeHidden();

    // Click the toggle button
    const uploadBtn = page.locator('#chat-upload-btn');
    await uploadBtn.click();

    // Image container should now be visible
    await expect(imageContainer).toBeVisible();
  });

  test('Conversational Setup maintains history after reload', async ({ page, onboardingOwner }) => {
    await page.goto('/setup.html');

    // Start conversational flow
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Type a message
    const chatInput = page.locator('input[placeholder*="e.g. I am a home baker"]');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('This is a test message to ensure history persistence.');
    await page.locator('#chat-send-btn').click();

    // Wait for the message to appear
    const userMessages = page.locator('.chat-message.user');
    await expect(userMessages).toHaveCount(1);

    // Wait for this verified owner's actual durable local snapshot, not a delay.
    await expect.poll(() => page.evaluate(({ userId, tenantId }) => {
      const key = 'omnisolo_onboarding_owned_v1:' + encodeURIComponent(JSON.stringify([userId, tenantId])) + ':legacy-draft';
      const saved = JSON.parse(localStorage.getItem(key) || '{}');
      return saved.state?.chat_history?.some((message: {role: string; content: string}) => message.role === 'user' && message.content === 'This is a test message to ensure history persistence.') ?? false;
    }, onboardingOwner)).toBe(true);

    // Reload page
    await page.reload();

    // The step and chat history should have persisted
    await expect(page.locator('#step-chat')).toBeVisible();
    await expect(page.locator('.chat-message.user').first()).toContainText('This is a test message to ensure history persistence.');
  });

  test('Conversational Setup renders user messages correctly', async ({ page }) => {
    await page.goto('/setup.html');

    // Start conversational flow
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Type a message
    const chatInput = page.locator('input[placeholder*="e.g. I am a home baker"]');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('Testing chat bubble formatting');
    await page.locator('#chat-send-btn').click();

    // Check message wrapper layout
    const lastUserMessage = page.locator('.chat-message.user').last();
    await expect(lastUserMessage).toBeVisible();

    const senderTitle = lastUserMessage.locator('.chat-sender');
    await expect(senderTitle).toHaveText('You');

    const bubbleContent = lastUserMessage.locator('.chat-bubble');
    await expect(bubbleContent).toHaveText('Testing chat bubble formatting');

  });
});
