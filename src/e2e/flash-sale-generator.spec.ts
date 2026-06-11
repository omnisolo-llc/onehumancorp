import { test, expect } from '@playwright/test';


test.describe('Growth & Virality: Flash Sale Generator', () => {
  test('User can create and preview a flash sale widget', async ({ browser }) => {
    const page = await adminPage(browser);

    // Navigate from Dashboard to Flash Sale Generator
    await page.click('text=Flash Sale Generator');
    await expect(page).toHaveURL(/.*\/flash-sale-generator/);

    // Verify initial state
    await expect(page.locator('text=Flash Sale Generator ⚡')).toBeVisible();
    await expect(page.locator('text=Widget Settings')).toBeVisible();
    await expect(page.locator('text=Live Preview')).toBeVisible();

    // The live preview should contain the default title
    await expect(page.locator('h3:has-text("Weekend Flash Sale!")')).toBeVisible();
    // Verify default percent
    await expect(page.locator('p:has-text("20% OFF")')).toBeVisible();
    // Verify default discount code
    await expect(page.locator('span:has-text("SAVE20")')).toBeVisible();

    // Modify the settings
    await page.fill('input[placeholder="e.g. 24-Hour Flash Sale!"]', 'Cyber Monday Mega Deal');
    await page.fill('input[placeholder="e.g. FLASH20"]', 'CYBER50');
    await page.fill('input[placeholder="20"]', '50');

    // Change theme to dark
    await page.click('button:has-text("Dark")');

    // Verify Live Preview updates
    await expect(page.locator('h3:has-text("Cyber Monday Mega Deal")')).toBeVisible();
    await expect(page.locator('p:has-text("50% OFF")')).toBeVisible();
    await expect(page.locator('span:has-text("CYBER50")')).toBeVisible();

    // Click "Get Widget" to open the embed modal
    await page.click('button:has-text("Get Widget")');

    // Verify modal appears
    await expect(page.locator('h2:has-text("Embed Flash Sale")')).toBeVisible();

    // Verify embed code contains our custom config
    const embedTextarea = page.locator('textarea');
    const embedCode = await embedTextarea.inputValue();
    expect(embedCode).toContain('theme=dark');
    expect(embedCode).toContain('percent=50');
    expect(embedCode).toContain('code=CYBER50');
    expect(embedCode).toContain('title=Cyber%20Monday%20Mega%20Deal');

    // Close the modal
    await page.click('button:has-text("Close")');
    await expect(page.locator('h2:has-text("Embed Flash Sale")')).not.toBeVisible();
  });
});
