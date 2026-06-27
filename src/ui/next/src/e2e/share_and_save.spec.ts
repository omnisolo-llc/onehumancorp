import { test, expect } from '@playwright/test';
import { memberPage } from './fixtures';

test.describe('Share & Save Widget Growth Feature', () => {

  test('Owner can navigate to the Share & Save widget builder from the dashboard', async ({ page }) => {
    // Start at dashboard
    await page.goto('/dashboard.html');

    // Check if the link exists
    const widgetLink = page.locator('a#share-and-save-link');
    await expect(widgetLink).toBeVisible();
    await expect(widgetLink).toHaveText(/Share & Save Widget/);
  });

  test('Owner can configure the Share & Save widget and generate embed code', async ({ page }) => {
    // Navigate directly to the builder
    await page.goto('/share-and-save-widget.html');

    // Wait for the page to load
    await expect(page.locator('h1')).toContainText('Share & Save Widget');

    // Verify default state
    const typePercentBtn = page.locator('#type-percent');
    const valueInput = page.locator('#discount-value');
    const codeInput = page.locator('#discount-code');

    await expect(typePercentBtn).toHaveClass(/active/);
    await expect(valueInput).toHaveValue('10');
    await expect(codeInput).toHaveValue('SHARE10');

    // Verify preview iframe exists
    const previewIframe = page.locator('#preview-iframe');
    await expect(previewIframe).toBeVisible();

    // Change configuration
    const typeFixedBtn = page.locator('#type-fixed');
    await typeFixedBtn.click();
    await valueInput.fill('15');
    await codeInput.fill('SAVE15');

    // Verify iframe src updated correctly (it's updated immediately on change)
    await expect(previewIframe).toHaveAttribute('src', /type=fixed/);
    await expect(previewIframe).toHaveAttribute('src', /val=15/);
    await expect(previewIframe).toHaveAttribute('src', /code=SAVE15/);

    // Open modal to get code
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // Wait for modal
    const embedModal = page.locator('#embed-modal');
    await expect(embedModal).toHaveClass(/active/);

    // Check code textarea
    const embedCode = page.locator('#embed-code');
    const codeValue = await embedCode.inputValue();
    expect(codeValue).toContain('<iframe');
    expect(codeValue).toContain('type=fixed');
    expect(codeValue).toContain('val=15');
    expect(codeValue).toContain('code=SAVE15');

    // Try copy button
    const copyBtn = page.locator('#copy-btn');
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');
  });

  test('Customer widget displays correctly and unlocks reward', async ({ page }) => {
    // Load the embed widget page directly with params
    await page.goto('/share-and-save-embed.html?val=20&type=fixed&code=VIP20&theme=light');

    // Check title and desc
    const title = page.locator('#widget-title');
    await expect(title).toContainText('Share & Get $20 Off');

    // Check social buttons
    const shareWa = page.locator('#share-wa');
    await expect(shareWa).toBeVisible();

    // Simulate share action (it should reveal the code after a delay)
    // In playwright we can't easily test window.open without complex setup,
    // but clicking the button triggers the timeout that shows the reward.
    await shareWa.click();

    // Wait for reward section to appear (timeout is 2000ms)
    const rewardSection = page.locator('#reward-section');
    await expect(rewardSection).toBeVisible({ timeout: 3000 });

    // Verify the code is revealed
    const codeDisplay = page.locator('#discount-code-display');
    await expect(codeDisplay).toHaveText('VIP20');

    // Verify title changed
    await expect(title).toHaveText("You're all set!");
  });
});
