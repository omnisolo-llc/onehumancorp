import { test, expect } from '@playwright/test';

test.describe('Dashboard Core', () => {
  test('should load dashboard page', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show dashboard header', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
  });

  test('should display todays sales stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
  });

  test('should display new orders stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Orders to Ship')).toBeVisible();
  });

  test('should display active AI helpers stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Team Members')).toBeVisible();
  });

  test('should display tasks in progress stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Ongoing Tasks')).toBeVisible();
  });

  test('should display store health score stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Store Health')).toBeVisible();
  });

  test('should show quick actions section', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Store Tips')).toBeVisible();
  });

  test('should display add product quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Add Product')).toBeVisible();
  });

  test('should display view orders quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=View Orders')).toBeVisible();
  });

  test('should display messages quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Messages')).toBeVisible();
  });

  test('should display customer messages inbox section', async ({ page }) => {
    // 1. Log in to access the dashboard
    await page.goto('/');

    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // 2. Ensure the new Customer Messages section is visible on the dashboard
    await expect(page.locator('text=Customer Messages').first()).toBeVisible();
    await expect(page.locator('button:has-text("View Inbox")')).toBeVisible();

    // 3. Click the View Inbox button to open the Unified Inbox modal
    await page.click('button:has-text("View Inbox")');
    await expect(page.locator('text="Customer Inbox"')).toBeVisible();

    // 4. Verify that we can interact with a Chatwoot unified conversation
    await expect(page.locator('text="Select a conversation"')).toBeVisible();

    // Click on a contact from the inbox list
    await page.click('text="Maya"');
    await expect(page.locator('text="Do you do vegan cakes?"')).toBeVisible();

    // 5. Test AI Draft integration on the Chatwoot inbox UI
    await page.click('button:has-text("✨ AI Draft")');
    await expect(page.locator('input[placeholder="Type a message..."]')).toHaveValue(/vegan/i, { timeout: 10000 });

    // 6. Send the message and verify it appears in the chat
    await page.fill('input[placeholder="Type a message..."]', 'Yes, we have 3 vegan options!');
    await page.click('button:has-text("Send")');
    await expect(page.locator('text="Yes, we have 3 vegan options!"').last()).toBeVisible();
  });

  test('should display analytics quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Analytics')).toBeVisible();
  });

  test('should display share store quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Share Store')).toBeVisible();
  });

  test('should show drafts ready for review section', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Needs Your Approval')).toBeVisible();
  });


  test('should show my team section', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=My Team')).toBeVisible();
  });

  test('should display help buttons', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Ask AI')).toBeVisible();
    await expect(page.locator('text=Help')).toBeVisible();
  });

  test('should display billing button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Billing')).toBeVisible();
  });

  test('should display docs button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Docs')).toBeVisible();
  });

  test('should display videos button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Videos')).toBeVisible();
  });

  test('should display tour button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Tour')).toBeVisible();
  });

  test('should display whats new button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=What\'s New')).toBeVisible();
  });

  test('should navigate to dashboard from home', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Dashboard');
    await expect(page).toHaveURL(/\//);
  });

  test('should show draft approval buttons', async ({ page }) => {
    await page.goto('/');
    const approveBtn = page.locator('button:has-text("Approve")').first();
    const editBtn = page.locator('button:has-text("Edit")').first();
    await expect(approveBtn).toBeVisible();
    await expect(editBtn).toBeVisible();
  });

});

test.describe('Dashboard Mobile', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display correctly on mobile', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
  });

  test('should verify plain language labels on mobile', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=Orders to Ship')).toBeVisible();
    await expect(page.locator('text=Store Health')).toBeVisible();
    await expect(page.locator('text=Needs Your Approval')).toBeVisible();
  });

  test('should display quick actions on mobile', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Store Tips')).toBeVisible();
  });
});