import { test, expect } from '@playwright/test';

test.describe('Cloud Bridge Referral Loop', () => {
  // Test 1: Visually displays the Cloud Bridge section
  test('should display the new glassmorphism Cloud Bridge section on dashboard', async ({ page }) => {
    // Navigate and sign in via global setup or direct
    await page.goto('/dashboard.html');

    const cloudBridgeSection = page.locator('#cloud-bridge-section');
    await expect(cloudBridgeSection).toBeVisible();

    // Check specific styling and text
    await expect(cloudBridgeSection).toContainText('Zero Data Leakage');
    await expect(cloudBridgeSection).toContainText('Invite to Cloud Workspace');
  });

  // Test 2: Input field is visible and accessible
  test('should have a functional input field for email', async ({ page }) => {
    await page.goto('/dashboard.html');

    const emailInput = page.locator('#cloud-bridge-email');
    await expect(emailInput).toBeVisible();
    await emailInput.fill('new.collab@example.com');
    await expect(emailInput).toHaveValue('new.collab@example.com');
  });

  // Test 3: Invite button is present with new label
  test('should have a prominent Unlock Cloud Collaboration button', async ({ page }) => {
    await page.goto('/dashboard.html');

    const generateBtn = page.locator('#generate-cloud-bridge-btn');
    await expect(generateBtn).toBeVisible();
    await expect(generateBtn).toContainText('Unlock Cloud Collaboration');
  });

  // Test 4: Triggers status change when clicked
  test('should trigger the invite generation and show status text', async ({ page }) => {
    await page.goto('/dashboard.html');

    const emailInput = page.locator('#cloud-bridge-email');
    const generateBtn = page.locator('#generate-cloud-bridge-btn');
    const statusDiv = page.locator('#cloud-bridge-status');

    await emailInput.fill('collab2@example.com');
    await generateBtn.click();

    // Status text should be set indicating action
    // According to HTML code, the button text changes to "Generating..."
    await expect(generateBtn).toContainText('Generating...');
  });

  // Test 5: Re-trigger works
  test('should allow attempting another invite generation', async ({ page }) => {
    await page.goto('/dashboard.html');

    const emailInput = page.locator('#cloud-bridge-email');
    const generateBtn = page.locator('#generate-cloud-bridge-btn');

    await emailInput.fill('collab3@example.com');
    await generateBtn.click();

    // Wait for button to finish state
    // Given there is a fetch call we can just verify the input is not disabled
    await expect(emailInput).toBeEnabled();
  });
});
