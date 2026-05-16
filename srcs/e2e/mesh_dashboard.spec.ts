import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: Realtime Teammate Mesh APIs for KAIROS Coordination', () => {
  test('Complete CUJ: View mesh state in Swarm Observability Panel', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/dashboard');

    // 2. Navigate to Swarm Observability Panel
    await page.click('text=Swarm Observability');

    // 3. Verify connection to the new mesh API WebSocket
    await expect(page.locator('text=Mesh Connected')).toBeVisible();

    // 4. Verify a plain-language activity feed appears
    await expect(page.locator('text=✅ Your Support Agent replied to 3 customers')).toBeVisible();

    // Verify glassmorphism style presence
    const panel = page.locator('div', { hasText: 'Swarm Observability' });
    await expect(panel).toHaveCSS('backdrop-filter', 'blur(20px) saturate(200%)');
  });
});
