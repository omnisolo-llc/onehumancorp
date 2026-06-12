import { test, expect } from './fixtures';

test.describe('Voice Assistant Command Center', () => {
  test.use({ permissions: ['microphone'] });

  test('Carlos can initiate a quote via voice command hands-free', async ({ page }) => {
    // Override MediaRecorder in the browser
    await page.addInitScript(() => {
      class MockMediaRecorder {
        state = 'inactive';
        ondataavailable = null;
        onstop = null;
        start() {
          this.state = 'recording';
          if (this.ondataavailable) {
            this.ondataavailable({ data: new Blob(['mock audio data'], { type: 'audio/webm' }) });
          }
        }
        stop() {
          this.state = 'inactive';
          if (this.onstop) {
            this.onstop();
          }
        }
      }
      (window as any).MediaRecorder = MockMediaRecorder;

      const mockStream = {
        getTracks: () => [{ stop: () => {} }]
      };
      if (!navigator.mediaDevices) {
        (navigator as any).mediaDevices = {};
      }
      navigator.mediaDevices.getUserMedia = () => Promise.resolve(mockStream as any);
    });

    await page.goto('/dashboard');

    const voiceBtn = page.getByLabel('Voice Assistant').first();
    await expect(voiceBtn).toBeVisible();

    await voiceBtn.evaluate((el) => el.click());

    // Evaluate stop logic to simulate the 1s timeout behavior
    await page.evaluate(() => {
      setTimeout(() => {
         const el = document.querySelector('[aria-label="Voice Assistant"]');
         if (el) el.dispatchEvent(new MouseEvent('mouseup', { bubbles: true }));
      }, 500);
    });

    await expect(page.locator('text=Action Prepared!')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Create a $150 repair quote')).toBeVisible();

    const feed = page.locator('section').filter({ hasText: /Unified Agent Feed|All caught up!/ });
    await expect(feed).toBeVisible();

    const actionCard = page.getByTestId('draft-quote-card').first();
    await expect(actionCard).toBeVisible();
    await expect(actionCard).toContainText('$150');

    const approveBtn = page.getByTestId('approve-send-proposal').first();
    await approveBtn.click();

    // After approval, it moves to the activity tab. We need to click the activity tab first to see it.
    const activityTab = page.getByTestId('tab-activity');
    await activityTab.click();
    await expect(page.locator('text=APPROVED').first()).toBeVisible();
  });

  test('Voice Assistant button follows glassmorphism design tokens', async ({ page }) => {
    await page.goto('/dashboard');
    const voiceBtn = page.getByLabel('Voice Assistant').first();

    const box = await voiceBtn.boundingBox();
    expect(box?.width).toBe(64);
    expect(box?.height).toBe(64);

    const computedStyle = await voiceBtn.evaluate((el) => {
        return window.getComputedStyle(el).backdropFilter || window.getComputedStyle(el).webkitBackdropFilter;
    });
    expect(computedStyle).toContain('blur');
  });
});
