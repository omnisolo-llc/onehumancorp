import { test, expect } from './fixtures';

test.describe('Documentation User Journey', () => {
  test('Maya navigates the Help Center and views the Changelog', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/changelog');

    // Verify Changelog is loaded
    await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();

    // Now Maya navigates to the Help Center
    await page.goto('/help');

    // Verify Help Center is loaded
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    // Verify Categories from the fallback we added
    await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Payments' })).toBeVisible();

    // Verify Videos list loads
    await expect(page.locator('h2', { hasText: 'Video Tutorials' })).toBeVisible();

    // Maya searches for "products" to learn how to add products
    await page.fill('input[placeholder="Search for help articles and videos..."]', 'products');

    // Wait for the mock API response to update UI
    await page.waitForTimeout(500);

    // Verify empty state works correctly
    await page.fill('input[placeholder="Search for help articles and videos..."]', 'xyznonexistent123');
    await expect(page.getByText('No results found matching')).toBeVisible();
  });

  test('Tooltips appear on elements', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api-docs');
    const advancedText = page.locator('span', { hasText: 'Advanced:' });
    await expect(advancedText).toBeVisible();
    await advancedText.hover();
    await expect(page.getByText('Direct API access is only for custom integrations.')).toBeVisible();
  });

  test('Interactive Walkthrough can be started', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    test.setTimeout(10000);

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

  test('API Documentation is accessible', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api-docs');
    await expect(page.getByText('API Documentation')).toBeVisible();
  });

  test('AI Help Chat is functional', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/help');

    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();

    const chatHeader = page.locator('#ai-chat-header');
    await expect(chatHeader).toBeVisible();

    const chatInput = page.locator('input[placeholder="Ask me anything..."]');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('How do I add a product?');

    const sendButton = page.locator('button[aria-label="Send message"]');
    await expect(sendButton).toBeVisible();
    await sendButton.click();

    const sentMessage = page.locator('.msg-user', { hasText: 'How do I add a product?' }).last();
    await expect(sentMessage).toBeVisible();
  });
});
