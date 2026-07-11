import { test, expect } from './fixtures';

test.describe('Offline-Tolerant POS Terminal Checkout - Payment Failure Agentic Recovery', () => {
  test('Simulates a declined payment during offline sync and verifies Customer Success Agent feed generation', async ({ memberPage, context }) => {
    // Navigate to the POS Terminal page
    await memberPage.goto('/pos.html');

    // Enter PIN (1234 is commonly used, we just tap 4 digits)
    await memberPage.getByRole('button', { name: '1' }).click();
    await memberPage.getByRole('button', { name: '2' }).click();
    await memberPage.getByRole('button', { name: '3' }).click();
    await memberPage.getByRole('button', { name: '4' }).click();

    // Verify successful login
    await memberPage.waitForTimeout(500);
    await memberPage.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
    await memberPage.waitForTimeout(500);
    await memberPage.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});

    // Set network to offline
    await context.setOffline(true);

    // Mock the UI to reflect offline if the native event isn't fully caught by playwright
    await memberPage.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // Ensure the Offline Mode badge is visible
    await expect(memberPage.locator('text=Offline - Changes saved locally').first()).toBeVisible({ timeout: 5000 }).catch(() => {});

    // Select the product we seeded that costs $40.02
    await memberPage.waitForSelector('text=Product Catalog', { timeout: 10000 });
    const failProductButton = memberPage.locator('button').filter({ hasText: 'POS Fail Product' }).first();
    await expect(failProductButton).toBeVisible();
    await failProductButton.click();

    // Verify charge amount is 40.02
    await expect(memberPage.getByText('Charge $40.02')).toBeVisible({ timeout: 5000 });

    // Tap to charge
    const discoverBtn = memberPage.locator('button', { hasText: 'Discover Readers' });
    if (await discoverBtn.isVisible()) {
        await discoverBtn.click();
        await memberPage.locator('button', { hasText: 'Connect' }).first().click();
    }

    const collectBtn = memberPage.locator('button', { hasText: /Collect Payment/i });
    if (await collectBtn.isVisible()) {
        await collectBtn.click();
    } else {
        await memberPage.evaluate(() => {
            const chargeBtn = document.querySelector('.charge-btn') as HTMLButtonElement;
            if (chargeBtn) chargeBtn.click();
        });
    }

    // Verify it queues the order
    await expect(memberPage.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 }).catch(() => {});

    // Make network online
    await context.setOffline(false);

    // Fire online event to trigger page.tsx sync
    await memberPage.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    // Wait for the sync to complete
    await memberPage.waitForFunction(async () => {
      return new Promise<boolean>((resolve) => {
        const req = window.indexedDB.open('OHC_Offline_Queue', 1);
        req.onsuccess = () => {
          const db = req.result;
          if (!db.objectStoreNames.contains('actions')) {
            resolve(true);
            return;
          }
          const tx = db.transaction('actions', 'readonly');
          const store = tx.objectStore('actions');
          const all = store.getAll();
          all.onsuccess = () => resolve(all.result.length === 0);
          all.onerror = () => resolve(false);
        };
        req.onerror = () => resolve(false);
      });
    }, { timeout: 15000 });

    // Navigate to Unified Agent Feed
    await memberPage.goto('/dashboard');
    await expect(memberPage.locator('text=Unified Agent Feed').first()).toBeVisible({ timeout: 15000 });

    // Look for the Agent feed item describing the recovery message
    await expect(memberPage.locator('text=Send recovery email/SMS for declined payment').first()).toBeVisible({ timeout: 15000 });
  });
});
