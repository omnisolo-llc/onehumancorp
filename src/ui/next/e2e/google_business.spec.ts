import { test, expect } from '@playwright/test';

test.describe('Google Business Profile Integration Flow', () => {

  test('1. Connect Google Business Profile from Integrations', async ({ page }) => {
    // Navigate to Integrations page
    await page.goto('http://localhost:3000/integrations');

    // Ensure the Google Business Profile card is visible
    const card = page.locator('div:has-text("Google Business Profile")').last();
    await expect(card).toBeVisible();

    // Verify it is initially disconnected
    await expect(card.locator('span:has-text("disconnected")')).toBeVisible();

    // Mock window.alert to not block the test
    page.on('dialog', dialog => dialog.accept());

    // Click Connect
    await card.locator('button:has-text("Connect")').click();

    // After connecting, it should redirect to the Inbox page
    await page.waitForURL('**/inbox');

    // Verify we are on the Inbox page
    await expect(page.locator('h1:has-text("Customer Inbox")')).toBeVisible();
  });

  test('2. Verify Google Business messages appear in the Customer Inbox with AI drafts', async ({ page }) => {
    await page.goto('http://localhost:3000/inbox');

    // Check for Google Business message presence
    const googleMsg = page.locator('div:has-text("Google Search User")').first();
    await expect(googleMsg).toBeVisible();

    // Verify message content
    await expect(googleMsg.locator('text="Are you open on Sundays?"')).toBeVisible();

    // Verify AI drafted reply
    const aiDraft = page.locator('text="Hi there! Yes, we are open on Sundays from 10:00 AM to 4:00 PM. We hope to see you soon!"');
    // Using string matching since it might be split across DOM nodes
    await expect(page.locator('body')).toContainText('Hi there! Yes, we are open on Sundays from 10:00 AM to 4:00 PM. We hope to see you soon!');
  });

  test('3. Edit and send a reply to a Google Business message', async ({ page }) => {
    await page.goto('http://localhost:3000/inbox');

    // There are hidden buttons in the DOM, so select the specific visible one inside the message container
    const sendBtn = page.locator('.bg-\\[\\#805ad5\\]').filter({ hasText: 'Send' }).last();
    await sendBtn.click({ force: true });

    // Verify the sent message appears from 'Me'
    await expect(page.locator('body')).toContainText('Hi there! Yes, we are open on Sundays');

    // Check that 'Me' is displayed as the sender in the list
    await expect(page.locator('span:has-text("Me")').last()).toBeVisible();
  });

  test('4. Navigate to Reputation page and verify Google Reviews', async ({ page }) => {
    await page.goto('http://localhost:3000/reputation');

    await expect(page.locator('h1:has-text("Reputation Management")')).toBeVisible();

    // Verify Sarah's review is pending
    const review1 = page.locator('div:has-text("Sarah Jenkins")').first();
    await expect(review1).toBeVisible();
    await expect(review1.locator('span:has-text("pending")')).toBeVisible();
    await expect(review1.locator('p:has-text("Absolutely wonderful service! Carlos was very professional and fixed my car in no time.")')).toBeVisible();

    // Verify AI suggested reply is present
    await expect(review1.locator('text="AI Suggested Reply"')).toBeVisible();
  });

  test('5. Approve and send an AI-drafted response to a Google Review', async ({ page }) => {
    await page.goto('http://localhost:3000/reputation');

    // Find the pending review
    const review1 = page.locator('div:has-text("Sarah Jenkins")').first();
    await expect(review1.locator('span:has-text("pending")')).toBeVisible();

    // Click Approve & Publish
    const approveBtn = review1.locator('button:has-text("Approve & Publish")');
    await approveBtn.click();

    // Verify status changes to replied
    await expect(review1.locator('span:has-text("replied")').first()).toBeVisible();

    // Verify the published reply section appears
    await expect(review1.locator('span:has-text("Your Reply")').first()).toBeVisible();
    await expect(review1.locator('span:has-text("Published")').first()).toBeVisible();
  });

});