import { test, expect } from '@playwright/test';

test.describe('Autonomous 1-Tap AI Catalog & Storefront Generator', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display the AI Camera Ingest Widget on the dashboard and handle the flow', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Check if the camera widget button is visible
    const cameraButton = page.locator('button', { hasText: '📷' });
    await expect(cameraButton).toBeVisible();

    // Click the button to invoke camera (we can't easily mock the file dialog here but we can test the UI state)
    // To simulate file selection we could use setInputFiles but since it's hidden we just interact with it.

    // Instead of actual file upload, we verify the presence of the hidden file input
    const fileInput = page.locator('input[type="file"][accept="image/*"]');
    await expect(fileInput).toBeAttached();

    // Simulate setting a file to trigger the flow
    await fileInput.setInputFiles('src/e2e/test-image.jpg');

    // Wait for the skeleton loading UI to appear
    const loadingText = page.locator('text=AI Agent is analyzing and writing descriptions...');
    await expect(loadingText).toBeVisible();

    // Wait for the mock response to return (3000ms in widget + buffer)
    const pendingApprovalText = page.locator('text=Pending Approval');
    await expect(pendingApprovalText).toBeVisible({ timeout: 10000 });

    // Check if the auto-generated data is present
    await expect(page.locator('input[defaultValue="Generated Item"]')).toBeVisible();
    await expect(page.locator('input[defaultValue="$45.00"]')).toBeVisible();

    // Setup dialog alert listener
    page.once('dialog', dialog => dialog.accept());

    // Click Approve & Publish
    const approveButton = page.locator('button', { hasText: 'Approve & Publish' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Widget should close
    await expect(pendingApprovalText).toBeHidden();
  });
});
