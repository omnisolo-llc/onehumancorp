import { expect, test } from './fixtures';

test.describe('Unified Inbox E2E', () => {

  test('Persona: Business Owner uses unified inbox to view AI drafted responses', async ({ page }) => {
    // Start from login if needed
    await page.goto('/inbox');

    // Wait for the page to load
    await expect(page.locator('h1')).toHaveText('Customer Inbox');

    // Check if the messages are visible
    await expect(page.locator('text=Facebook User')).toBeVisible();
    await expect(page.locator('text=Instagram User')).toBeVisible();
    await expect(page.locator('text=WhatsApp User')).toBeVisible();

    // Check for AI Drafts
    await expect(page.getByText('AI Draft').first()).toBeVisible();
    await expect(page.getByText('"Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in."').first()).toBeVisible();

    // Perform swipe right to "Approve & Send"
    const draftBubble1 = page.locator('[data-testid="ai-draft-1"]');

    // Simulate swipe right using mouse
    const box = await draftBubble1.boundingBox();
    if (box) {
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.down();
      await page.mouse.move(box.x + box.width / 2 + 150, box.y + box.height / 2); // Swipe right
      await page.mouse.up();
    }

    // After swiping right, the draft message should be added to the messages list as sent by "Me"
    await expect(page.getByText('Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.').last()).toBeVisible();

    // Check that there is a "Me" sender now
    await expect(page.locator('span.font-semibold:has-text("Me")').first()).toBeVisible();
  });

  test('Persona: Business Owner uses unified inbox to swipe left and edit AI drafted responses', async ({ page }) => {
    await page.goto('/inbox');

    await expect(page.getByText('AI Draft').first()).toBeVisible();

    // Perform swipe left to "Edit" on the second draft message
    const draftBubble2 = page.locator('[data-testid="ai-draft-2"]');

    // Simulate swipe left using mouse
    const box = await draftBubble2.boundingBox();
    if (box) {
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.down();
      await page.mouse.move(box.x + box.width / 2 - 150, box.y + box.height / 2); // Swipe left
      await page.mouse.up();
    }

    // Edit textarea should become visible
    const editTextArea = page.locator('#reply-input-edit');
    await expect(editTextArea).toBeVisible();

    // And it should have the draft text
    await expect(editTextArea).toHaveValue('Your order is currently being prepared and will be shipped within 24 hours. You will receive a tracking link shortly.');

    // Let's change the text and click Send
    await editTextArea.fill('Your order will be shipped today!');
    await draftBubble2.locator('button:has-text("Send")').click();

    // Verify it was sent
    await expect(page.getByText('Your order will be shipped today!')).toBeVisible();
    await expect(page.locator('span.font-semibold:has-text("Me")').first()).toBeVisible();
  });

  test('Persona: Business Owner can schedule a post from unified inbox', async ({ page }) => {
    await page.goto('/inbox');

    // Check for Schedule Outbound Post button
    const scheduleButton = page.locator('button:has-text("Schedule Outbound Post")');
    await expect(scheduleButton).toBeVisible();
    await scheduleButton.click();

    // Wait for Modal to open
    await expect(page.locator('h2:has-text("Schedule Post")')).toBeVisible();

    // Type in text
    await page.fill('textarea[placeholder="What do you want to post?"]', 'We are having a 20% off sale on all items this weekend!');

    // Click Schedule
    await page.locator('button:has-text("Schedule")').nth(1).click();

    // Verify it was scheduled
    await expect(page.locator('h2:has-text("Scheduled Posts")')).toBeVisible();
    await expect(page.locator('text=We are having a 20% off sale on all items this weekend!')).toBeVisible();
  });
});
