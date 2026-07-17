import { test, expect } from './fixtures';

test.describe('P2P Offline Mesh Sync UI', () => {
  test('should show "Join Local Register Network" prompt and successfully connect', async ({ page }) => {
    // Navigate to the POS Dashboard
    await page.goto('/pos.html');

    // Wait for the UI to be ready
    await expect(page.locator('body')).toBeVisible();

    // The prompt should appear after 2 seconds (mocking Bluetooth LE/mDNS detection)
    const prompt = page.locator('#p2p-mesh-prompt');
    await expect(prompt).toBeVisible({ timeout: 5000 });

    const joinBtn = page.locator('#p2p-mesh-join-btn');
    await expect(joinBtn).toBeVisible();
    await expect(joinBtn).toContainText('Join Local Register Network');

    // Click to join the network
    await joinBtn.click();

    // Expect transition to processing state
    await expect(joinBtn).toContainText('Synchronizing Mesh State...');

    // Expect transition to connected state
    await expect(joinBtn).toContainText('Mesh Network Connected', { timeout: 3000 });

    // Expect prompt to disappear eventually
    await expect(prompt).toBeHidden({ timeout: 3000 });
  });
});
