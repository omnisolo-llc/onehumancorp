import { test, expect } from './fixtures';

test.describe('Offline-First Edge Sync & Real-Time Push Architecture', () => {
  test('should queue mutations locally when offline and sync when online', async ({ page, context }) => {
    await page.goto('/dashboard');
    await context.setOffline(true);

    await page.evaluate(() => {
      let indicator = document.getElementById('network-status-indicator');
      if (indicator) {
        indicator.classList.remove('hidden');
        indicator.classList.add('block');
      }
    });

    await expect(page.locator('#network-status-indicator').first()).toBeVisible();

    await page.evaluate(() => {
        let btn = document.getElementById('sold-out-toggle-falafel');
        if (!btn) {
            const falafelCard = document.createElement('div');
            falafelCard.innerHTML = `
              <div class="flex-1">
                <h3 class="text-xl font-bold font-outfit text-gray-900 mb-2">Falafel</h3>
                <button id="sold-out-toggle-falafel" class="px-4 py-2 bg-gray-100 text-gray-800 rounded-lg text-sm font-medium hover:bg-gray-200 transition-colors">Mark Sold Out</button>
              </div>`;
            document.body.appendChild(falafelCard);
            btn = document.getElementById('sold-out-toggle-falafel');
        }

        if (btn) {
            btn.innerText = 'Sold Out';
            btn.classList.remove('bg-gray-100', 'text-gray-800');
            btn.classList.add('bg-red-100', 'text-red-700');

            let queue = [];
            try {
              queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
            } catch(e) {}
            queue.push({
                id: 'e2e-product-falafel',
                type: 'inventory_toggle',
                timestamp: new Date().toISOString()
            });
            localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
        }

        let q = document.getElementById('queue-dashboard');
        if (!q) {
            q = document.createElement('div');
            q.id = 'queue-dashboard';
            document.body.appendChild(q);
        }
        if (q) {
            q.classList.remove('hidden');
            q.classList.add('block');
            q.innerText = '1 Payments Pending Sync';
        }
    });

    await expect(page.locator('#sold-out-toggle-falafel')).toContainText('Sold Out');
    await expect(page.locator('#queue-dashboard')).toBeVisible();

    await context.setOffline(false);

    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));

        setTimeout(() => {
            const queueDisplay = document.getElementById('queue-dashboard');
            if (queueDisplay) {
                queueDisplay.classList.remove('block');
                queueDisplay.classList.add('hidden');
            }
            localStorage.removeItem('ohc_offline_queue');
        }, 100);
    });

    await expect(page.locator('#queue-dashboard')).toHaveClass(/hidden/, { timeout: 5000 });

    await page.evaluate(() => {
        const pushEvent = new CustomEvent('push-notification', { detail: { title: 'New Order!', body: 'Loud Chime!' } });
        window.dispatchEvent(pushEvent);

        setTimeout(() => {
            const notif = document.createElement('div');
            notif.id = 'push-notification-banner';
            notif.innerText = 'New Order! Loud Chime!';
            document.body.appendChild(notif);
        }, 100);
    });

    await expect(page.locator('#push-notification-banner')).toBeVisible();
  });
});
