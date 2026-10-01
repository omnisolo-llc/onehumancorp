import { test, expect } from '../onboarding_fixtures';

test.describe('Zero-Click Onboarding to Agent Feed', () => {
  test('User completes chat onboarding and sees welcome card on feed', async ({ page }) => {
    // Navigate to the setup route
    await page.goto('/setup.html');
    const identityResponse = await page.request.get('/api/v1/auth/session-identity');
    expect(identityResponse.status()).toBe(200);
    const identity = await identityResponse.json();
    expect(typeof identity.userId).toBe('string');
    expect(identity.userId.length).toBeGreaterThan(0);
    expect(typeof identity.tenantId).toBe('string');
    expect(identity.tenantId.length).toBeGreaterThan(0);

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
    await chatInput.press('Enter');

    // Review the prepared profile before explicitly launching it.
    const approval = page.locator('#step-approval');
    await expect(approval.getByRole('heading', { name: 'Ready to Launch' })).toBeVisible({ timeout: 45000 });
    await expect(approval.locator('#approval-details')).not.toBeEmpty();
    const preparation = page.waitForResponse(response => response.url().endsWith('/api/v1/onboarding/start') && response.request().method() === 'POST');
    const launch = page.waitForResponse(response => response.url().endsWith('/api/v1/onboarding/launch') && response.request().method() === 'POST');
    await approval.getByRole('button', { name: 'Approve & Complete Setup' }).click();
    const preparedResponse = await preparation;
    expect(preparedResponse.status()).toBe(200);
    const prepared = await preparedResponse.json();
    expect(prepared.success).toBe(true);
    expect(typeof prepared.preparation_id).toBe('string');
    expect(prepared.preparation_id.length).toBeGreaterThan(0);
    expect(typeof prepared.organization_id).toBe('string');
    expect(prepared.organization_id.length).toBeGreaterThan(0);
    expect(typeof prepared.user_id).toBe('string');
    expect(prepared.user_id.length).toBeGreaterThan(0);
    expect(prepared.organization_id).toBe(identity.tenantId);
    expect(prepared.user_id).toBe(identity.userId);
    const launchedResponse = await launch;
    expect(launchedResponse.status()).toBe(200);
    const launched = await launchedResponse.json();
    expect(launched).toMatchObject({ success: true, status: 'launched', preparation_id: prepared.preparation_id, organization_id: prepared.organization_id, user_id: prepared.user_id });
    const stateResponse = await page.request.get('/api/v1/onboarding/state');
    expect(stateResponse.status()).toBe(200);
    expect((await stateResponse.json()).preparation).toMatchObject({ preparation_id: prepared.preparation_id, status: 'launched', organization_id: prepared.organization_id, user_id: prepared.user_id });
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
