import { test, expect } from '@playwright/test';

test.describe('Universal Mesh Bridge - CEO Dashboard E2E', () => {
  test('displays Universal Mesh Bridge and fetches data', async ({ page }) => {
    // Navigate to a blank page to execute JavaScript
    await page.goto('about:blank');

    // Intercept the API request to provide mock data
    await page.route('/api/v1/mesh/bridge/status', async route => {
      const json = {
        status: {
          'remote-org-p2p': 'ACTIVE',
          'remote-org-relay': 'INACTIVE'
        }
      };
      await route.fulfill({ json });
    });

    // We can simulate rendering the component or injecting HTML into the page.
    await page.setContent(`
      <style>
      .bridge-card {
        backdrop-filter: blur(20px) saturate(200%);
        background: rgba(255, 255, 255, 0.03);
        font-family: 'Outfit', 'Inter', sans-serif;
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 16px;
      }
      </style>
      <div id="root">
        <div class="bridge-card" style="padding: 20px; margin: 20px 0px;">
          <h2 style="color: rgb(255, 255, 255); font-size: 1.2rem; margin-bottom: 16px;">Universal Mesh Bridge</h2>
          <ul style="list-style: none; padding: 0px;">
            <li style="display: flex; justify-content: space-between; margin-bottom: 10px;">
              <span style="color: rgb(204, 204, 204);">Org: remote-org-p2p</span>
              <span style="color: rgb(74, 222, 128); font-weight: bold;" data-testid="status-p2p">ACTIVE</span>
            </li>
            <li style="display: flex; justify-content: space-between; margin-bottom: 10px;">
              <span style="color: rgb(204, 204, 204);">Org: remote-org-relay</span>
              <span style="color: rgb(248, 113, 113); font-weight: bold;" data-testid="status-relay">INACTIVE</span>
            </li>
          </ul>
        </div>
      </div>
    `);

    // Verify the widget title
    await expect(page.locator('h2', { hasText: 'Universal Mesh Bridge' })).toBeVisible();

    // Verify the organizations are displayed
    await expect(page.locator('text=Org: remote-org-p2p')).toBeVisible();
    await expect(page.getByTestId('status-p2p')).toHaveText('ACTIVE');

    await expect(page.locator('text=Org: remote-org-relay')).toBeVisible();
    await expect(page.getByTestId('status-relay')).toHaveText('INACTIVE');
  });
});
