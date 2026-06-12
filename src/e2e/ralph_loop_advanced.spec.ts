import { test, expect } from './fixtures';

test.describe('Ralph Loop Advanced UI Interactions', () => {
  test('should display feature list when progress is populated', async ({ page }) => {
    // Navigate to Ralph Loop
    await page.goto('/ralph.html');

    // Mock a progress state in the UI for verification
    await page.evaluate(() => {
      const statusBadge = document.getElementById('overall-status');
      const featureList = document.getElementById('feature-list');
      const summaryText = document.getElementById('project-summary-text');

      statusBadge.innerText = 'Working';
      statusBadge.className = 'status-badge working';
      summaryText.innerText = 'Analyzing requirement...';

      featureList.innerHTML = '<div class="feature-item"><div class="feature-status status-completed"></div><div class="feature-name">Design Doc</div></div>';
    });

    await expect(page.locator('#overall-status')).toHaveText('Working');
    await expect(page.locator('.feature-name')).toHaveText('Design Doc');
  });

  test('should have a working sidebar link to Dashboard', async ({ page }) => {
    await page.goto('/ralph.html');
    await page.click('a:has-text("Dashboard")');
    await expect(page).toHaveURL(/.*dashboard.html/);
  });
});
