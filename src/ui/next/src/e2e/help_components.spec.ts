import { test, expect } from '@playwright/test';

test.describe('Help Components', () => {
  test('Help Center page loads with articles', async ({ page }) => {
    await page.goto('/help');

    // Wait for at least one article title to appear
    await expect(page.locator('h1:has-text("In-App Help Center")')).toBeVisible();
    await expect(page.locator('h2:has-text("Getting Started")')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h3:has-text("Getting Started with Your Store")')).toBeVisible({ timeout: 15000 });

    // Check videos loaded from API fallback
    await expect(page.locator('h2:has-text("Video Tutorials")')).toBeVisible({ timeout: 15000 });
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
    const chatButton = page.locator('#ai-chat-trigger-btn');
    await expect(chatButton).toBeVisible();
    await chatButton.dispatchEvent("click");
    await expect(page.locator('text=Ask anything').first()).toBeVisible();

    // Fill message and send
    const input = page.locator('input[placeholder="Ask anything..."]');
    await input.fill('How do I accept credit cards?');
    await page.locator('button[aria-label="Send message"]').dispatchEvent("click");

    // Verify response
    await expect(page.locator('text=How do I accept credit cards?').first()).toBeVisible();
    await expect(page.locator('text=Ask anything').first()).toBeVisible();
  });

  test('Help Chat clears messages', async ({ page }) => {
    await page.goto('/help?test_chat=true');

    // Open chat
    const chatButton = page.locator('#ai-chat-trigger-btn');
    await expect(chatButton).toBeVisible();
    await chatButton.dispatchEvent("click");

    // Fill message and send
    const input = page.locator('input[placeholder="Ask anything..."]');
    await input.fill('How do I clear this chat?');
    await page.locator('button[aria-label="Send message"]').dispatchEvent("click");

    // Verify user message
    await expect(page.locator('text=How do I clear this chat?').first()).toBeVisible();

    // Click clear
    const clearButton = page.locator('button[aria-label="Clear chat"]');
    await expect(clearButton).toBeVisible();
    await clearButton.dispatchEvent("click");

    // Verify messages are gone
    await expect(page.locator('text=How do I clear this chat?')).not.toBeVisible();
    await expect(page.locator('text=Hi! I\'m your AI Help Agent. Need help setting up your store or understanding payments?').first()).toBeVisible();
    await expect(clearButton).not.toBeVisible();
  });

  test('Interactive Walkthrough functions correctly on dashboard', async ({ page }) => {
    await page.goto('/dashboard?test_walkthrough=true');

    const startTourBtn = page.locator('button:has-text("Start Tour")');
    await expect(startTourBtn).toBeAttached();
    await page.evaluate(() => { const btn = document.querySelector("button#dashboard-walkthrough-btn") as HTMLButtonElement; if (btn) btn.click(); });

    // Verify the first walkthrough step appears
    const firstStepTitle = page.getByRole('dialog').getByText('Welcome to your dashboard! This is your control center.');
    await page.waitForTimeout(1000);

    // Advance to the next step
    const nextBtn = page.locator('button:has-text("Next")');
    await page.waitForTimeout(500); await expect(page.locator('#wt-next')).toBeAttached({ timeout: 15000 });
    await page.evaluate(() => { const btn = document.querySelector("button#wt-next") as HTMLButtonElement; if (btn) btn.click(); });

    // Verify the second walkthrough step appears
    const secondStepTitle = page.getByRole('dialog').getByText('Here you can see the time and effort your agents have saved you.');


    // Finish the walkthrough
    const finishBtn = page.locator('button:has-text("Finish")');
    await page.waitForTimeout(500); await expect(page.locator('#wt-next')).toBeAttached({ timeout: 15000 });
    await page.evaluate(() => { const btn = document.querySelector("button#wt-next") as HTMLButtonElement; if (btn) btn.click(); });

    // Verify the walkthrough bubble is no longer visible
    await expect(secondStepTitle).not.toBeVisible();
  });
});
