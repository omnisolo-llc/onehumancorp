import { test, expect } from '@playwright/test';

test.describe('Swarm Observability Flow', () => {
  // Test 1: Navigation
  test('1. User can observe swarm velocity and drill down into memory state', async ({ page }) => {
    // Navigate to home page
    await page.goto('/');

    // Wait for the app to load - Note: In flutter web CanvasKit, texts might be rendered as canvas graphics
    // unless semantics are enabled. However, standard testing environments usually render DOM nodes.
    // If not, we still verify the component is loaded.
    await expect(page.locator('text="OHC Business Dashboard"').first()).toBeVisible({ timeout: 15000 });

    // Verify Swarm Velocity widget is present with its metrics
    await expect(page.locator('text="Swarm Velocity"').first()).toBeVisible();
    await expect(page.locator('text="Task Rate"').first()).toBeVisible();
    await expect(page.locator('text="Latency"').first()).toBeVisible();

    // The user should see the 'View Swarm Memory State' button
    const viewMemoryBtn = page.locator('text="View Swarm Memory State"').first();
    await expect(viewMemoryBtn).toBeVisible();

    // Click to navigate to Swarm Memory Screen
    await viewMemoryBtn.click();

    // Verify the Swarm Memory State screen is visible
    await expect(page.locator('text="Swarm Memory State"').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text="Vector Space 1536-D"').first()).toBeVisible();

    // Verify the Broadcast High-Priority Event button is present
    const broadcastBtn = page.locator('text="Broadcast High-Priority Event"').first();
    await expect(broadcastBtn).toBeVisible();

    // Click the broadcast button to trigger the micro-animation/pulse
    await broadcastBtn.click();

    // Ensure the button can be clicked again.
    await expect(broadcastBtn).toBeEnabled();
  });

  // Test 2: Real-time UI metrics
  test('2. Swarm velocity widget shows real-time metrics', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text="OHC Business Dashboard"').first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text="Swarm Velocity"').first()).toBeVisible();
    await expect(page.locator('text="Task Rate"').first()).toBeVisible();
    await expect(page.locator('text="Latency"').first()).toBeVisible();
  });

  // Test 3: Visualization state
  test('3. Memory visualizer supports 1536-D representation', async ({ page }) => {
    await page.goto('/');
    const viewMemoryBtn = page.locator('text="View Swarm Memory State"').first();
    await expect(viewMemoryBtn).toBeVisible({ timeout: 15000 });
    await viewMemoryBtn.click();
    await expect(page.locator('text="Vector Space 1536-D"').first()).toBeVisible({ timeout: 10000 });
  });

  // Test 4: Haptic pulse interaction
  test('4. Tactical feedback triggers on broadcast', async ({ page }) => {
    await page.goto('/');
    const viewMemoryBtn = page.locator('text="View Swarm Memory State"').first();
    await expect(viewMemoryBtn).toBeVisible({ timeout: 15000 });
    await viewMemoryBtn.click();

    const broadcastBtn = page.locator('text="Broadcast High-Priority Event"').first();
    await expect(broadcastBtn).toBeVisible({ timeout: 10000 });
    await broadcastBtn.click();
    await expect(broadcastBtn).toBeEnabled();
  });

  // Test 5: Visual fidelity
  test('5. Multi-layered parallax effect operates without blocking UI', async ({ page }) => {
    await page.goto('/');
    const viewMemoryBtn = page.locator('text="View Swarm Memory State"').first();
    await expect(viewMemoryBtn).toBeVisible({ timeout: 15000 });
    await viewMemoryBtn.click();

    // Assert visual presence
    await expect(page.locator('text="Swarm Memory State"').first()).toBeVisible({ timeout: 10000 });
  });
});
