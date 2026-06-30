import { test, expect } from '@playwright/test';

test.describe('Help Components', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the backend API responses required for the help center to load correctly
    await page.route('**/api/help', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { category: "Getting Started", id: "getting-started-1", title: "Getting Started with Your Store", desc: "Welcome to OneHumanCorp!", link: "/help/getting-started-1" }
        ])
      });
    });

    await page.route('**/api/videos', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([{ id: 1, title: "Set up your store", duration: "1:15", video_url: "https://example.com/video.mp4" }])
      });
    });

    await page.route('**/api/tooltips', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          "pricing-tier-tooltip": "Select the plan that best fits your business needs."
        })
      });
    });

    await page.route('**/api/walkthrough/dashboard', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { target_id: "sales-card-target", title: "Business Analytics", content: "This panel shows your current sales and customer counts.", position: "bottom" },
          { target_id: "operations-map-target", title: "Operations Map", content: "Use this area to see the live state of your orders, messages, and inventory.", position: "bottom" }
        ])
      });
    });
  });

  test('Help Center page loads with articles', async ({ page }) => {
    await page.goto('/help');

    // Wait for at least one article title to appear
    await expect(page.locator('h1:has-text("In-App Help Center")')).toBeVisible();
    await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible();
    await expect(page.locator('h3:has-text("Getting Started with Your Store")')).toBeVisible();

    // Check videos loaded from API fallback
    await expect(page.locator('h2:has-text("Video Tutorials")')).toBeVisible();
  });

  test('Contextual Tooltip triggers correctly', async ({ page }) => {
    await page.goto('/pricing');

    const target = page.locator('h1:has-text("Pricing Plans")');
    await expect(target).toBeVisible();

    // Trigger the hover
    await target.hover();

    // Verify the tooltip text is visible
    const tooltipText = page.locator('text=Select the plan that best fits your business needs.');
    await expect(tooltipText).toBeVisible();
  });

  test('Help Chat opens and sends a message', async ({ page }) => {
    await page.goto('/help?test_chat=true');

    // Open chat
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();
    await expect(page.locator('text=Ask anything').first()).toBeVisible();

    // Fill message and send
    const input = page.locator('input[placeholder="Ask anything..."]');
    await input.fill('How do I accept credit cards?');
    await page.locator('button[aria-label="Send message"]').click();

    // Verify response
    await expect(page.locator('text=How do I accept credit cards?').first()).toBeVisible();
    await expect(page.locator('text=Sorry, I\'m having trouble connecting right now.').first()).toBeVisible({ timeout: 15000 });
  });

  test('Help Chat clears messages', async ({ page }) => {
    await page.goto('/help?test_chat=true');

    // Open chat
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    // Fill message and send
    const input = page.locator('input[placeholder="Ask anything..."]');
    await input.fill('How do I clear this chat?');
    await page.locator('button[aria-label="Send message"]').click();

    // Verify user message
    await expect(page.locator('text=How do I clear this chat?').first()).toBeVisible();

    // Click clear
    const clearButton = page.locator('button[aria-label="Clear chat"]');
    await expect(clearButton).toBeVisible();
    await clearButton.click();

    // Verify messages are gone
    await expect(page.locator('text=How do I clear this chat?')).not.toBeVisible();
    await expect(page.locator('text=Hi! I\'m your AI Help Agent. Need help setting up your store or understanding payments?').first()).toBeVisible();
    await expect(clearButton).not.toBeVisible();
  });

  test('Interactive Walkthrough functions correctly on dashboard', async ({ page }) => {
    await page.goto('/dashboard?test_walkthrough=true');

    const startTourBtn = page.locator('button:has-text("Start Tour")');
    await expect(startTourBtn).toBeVisible();
    await startTourBtn.click();

    // Verify the first walkthrough step appears
    const firstStepTitle = page.getByRole('dialog').getByText('Business Analytics');
    await expect(firstStepTitle).toBeVisible();

    // Advance to the next step
    const nextBtn = page.locator('button:has-text("Next")');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    // Verify the second walkthrough step appears
    const secondStepTitle = page.getByRole('dialog').getByText('Operations Map');
    await expect(secondStepTitle).toBeVisible();

    // Finish the walkthrough
    const finishBtn = page.locator('button:has-text("Finish")');
    await expect(finishBtn).toBeVisible();
    await finishBtn.click();

    // Verify the walkthrough bubble is no longer visible
    await expect(secondStepTitle).not.toBeVisible();
  });
});
