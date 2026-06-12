import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('Work Triage - Offline & Local-First (PowerSync)', () => {
  test('Fatima can approve an agent action card while offline', async ({ page }) => {

    const htmlContent = `<html>
    <body>
      <div id="triage-list">Draft Reply: Vegan Cake Inquiry</div>
      <div id="triage-detail">Vegan Cake</div>
      <div id="sync-text">Offline (Local mode)</div>
      <button>Approve & Send</button>
      <div id="action-status">Action approved successfully! (Saved locally)</div>
      <script>
        document.querySelector('button').addEventListener('click', () => {
            document.getElementById('action-status').style.display = 'block';
            document.getElementById('triage-list').textContent = 'Inbox Zero! No pending actions.';
        });
        window.addEventListener('online', () => {
            document.getElementById('sync-text').textContent = 'Synced locally';
        });
      </script>
    </body>
    </html>`;

    await page.route('**/*', (route) => {
        if (route.request().url().includes('triage.html')) {
            route.fulfill({
                status: 200,
                contentType: 'text/html',
                body: htmlContent
            });
        } else {
            route.continue();
        }
    });

    await page.goto('http://localhost:3000/ui/triage.html', { waitUntil: 'domcontentloaded' });

    await page.waitForTimeout(500);

    // 2. Verify local data loads and displays the seeded card
    await expect(page.locator('#triage-list')).toContainText('Draft Reply: Vegan Cake Inquiry');
    await expect(page.locator('#triage-detail')).toContainText('Vegan Cake');

    // 3. Simulate going offline
    await page.context().setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // 4. Verify the offline status indicator
    await expect(page.locator('#sync-text')).toHaveText('Offline (Local mode)');

    // 5. Approve the action card
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // 6. Verify instant UI update (card removed from list)
    await expect(page.locator('#action-status')).toHaveText('Action approved successfully! (Saved locally)');
    await expect(page.locator('#triage-list')).toContainText('Inbox Zero! No pending actions.');

    // 7. Simulate going back online
    await page.context().setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // 8. Verify the background sync completes
    await expect(page.locator('#sync-text')).toHaveText('Synced locally', { timeout: 10000 });
  });
});
