import { test, expect } from '@playwright/test';

test.describe('Help Components', () => {
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
    await page.goto('/help');

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
    await page.goto('/help');

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
    await page.goto('/dashboard');

    const startTourBtn = page.locator('button:has-text("Start Tour")');
    await expect(startTourBtn).toBeVisible();
    await startTourBtn.click();

    // Verify the first walkthrough step appears
    const firstStepTitle = page.getByRole('dialog').getByText('Welcome to your dashboard! This is your control center.');
    await expect(firstStepTitle).toBeVisible();

    // Advance to the next step
    const nextBtn = page.locator('button:has-text("Next")');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    // Verify the second walkthrough step appears
    const secondStepTitle = page.getByRole('dialog').getByText('Here you can see the time and effort your agents have saved you.');
    await expect(secondStepTitle).toBeVisible();

    // Finish the walkthrough
    const finishBtn = page.locator('button:has-text("Finish")');
    await expect(finishBtn).toBeVisible();
    await finishBtn.click();

    // Verify the walkthrough bubble is no longer visible
    await expect(secondStepTitle).not.toBeVisible();
  });
});
