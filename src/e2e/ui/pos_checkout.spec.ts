import { test, expect } from '../fixtures';

test.describe('Tap to Pay / POS Checkout UI Flow', () => {
  test('Processes POS sale via Tap to Pay and sends digital receipt', async ({ adminPage }) => {
    // Using adminPage from fixtures to be pre-authenticated as the owner
    // Start from the POS terminal path
    await adminPage.goto('/pos/terminal');

    // 1. Unlock the terminal (assuming default test PIN is 1234)
    await adminPage.click('button:has-text("1")');
    await adminPage.click('button:has-text("2")');
    await adminPage.click('button:has-text("3")');
    await adminPage.click('button:has-text("4")');

    // Wait for unlock
    await expect(adminPage.locator('text=Clocked In').or(adminPage.locator('text=Not Clocked In'))).toBeVisible();

    // 2. Connect to Reader
    await expect(adminPage.locator('text=Discover Readers')).toBeVisible();
    await adminPage.click('text=Discover Readers');

    await expect(adminPage.locator('text=Connect').first()).toBeVisible();
    await adminPage.click('text=Connect');

    // 3. Initiate tap to pay
    await expect(adminPage.locator('text=Charge $50.00')).toBeVisible();
    await adminPage.click('text=Charge $50.00');

    // 4. Digital receipt & Agent
    await adminPage.fill('input[placeholder="customer@email.com"]', 'pos-customer@test.com');
    await adminPage.click('text=Send Receipt & Add to CRM');

  });
});
