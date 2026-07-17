import { test, expect } from './fixtures';

test.describe('Hardware-Free P2P Offline Mesh Sync for Multi-Device POS', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Persona: Fatima - Two POS devices sync inventory offline via P2P Mesh', async ({ browser }) => {
    const tenantId = `tenant-p2p-${Date.now()}`;
    const contextA = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const contextB = await browser.newContext({ viewport: { width: 375, height: 812 } });

    const pageA = await contextA.newPage();
    const pageB = await contextB.newPage();

    const mockDataSetup = async (page, deviceId) => {
        await page.goto('http://127.0.0.1:3000/').catch(() => {});
        await page.evaluate(({tenant, devId}) => {
            localStorage.setItem('tenant_id', tenant);
            localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Fatima', role: 'Owner', pin_hash: '1234', tenant_id: tenant }]));
            localStorage.setItem('ohc_pos_device_id', devId);

            const catalog = [{
                id: 'prod_falafel',
                title: 'Falafel Wrap',
                price_cents: 800,
                inventory_count: 5,
                stock: 5,
                available_quantity: 5
            }];
            localStorage.setItem('ohc_catalog_default', JSON.stringify(catalog));
            localStorage.setItem(`ohc_catalog_${tenant}`, JSON.stringify(catalog));
        }, {tenant: tenantId, devId: deviceId});
    };

    await mockDataSetup(pageA, 'device_A');
    await mockDataSetup(pageB, 'device_B');

    // 1. Navigate both devices to POS terminal
    const loginToPos = async (page) => {
        await page.goto('/pos.html');
        const pins = ['1', '2', '3', '4'];
        for (const p of pins) {
            await page.getByRole('button', { name: p, exact: true }).click();
        }
        await page.locator('button:has-text("Clock In")').click({ force: true, timeout: 5000 }).catch(() => {});
        await expect(page.locator('h3', { hasText: 'Product Catalog' })).toBeVisible({ timeout: 15000 });
        const productBtn = page.locator('button', { hasText: 'Falafel Wrap' });
        await expect(productBtn).toBeVisible();
        await expect(productBtn).toContainText('Stock: 5');
    };

    await loginToPos(pageA);
    await loginToPos(pageB);

    // 2. Take both devices offline
    await contextA.setOffline(true);
    await pageA.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(pageA.locator('text=Offline - Changes saved locally')).toBeVisible({ timeout: 10000 });

    await contextB.setOffline(true);
    await pageB.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(pageB.locator('text=Offline - Changes saved locally')).toBeVisible({ timeout: 10000 });

    // 3. Device B should discover Device A and show "Join Local Register Network"
    const joinModal = pageB.locator('#p2p-mesh-modal');
    await expect(joinModal).toBeVisible({ timeout: 15000 });

    // 4. Device B joins the mesh
    await pageB.getByRole('button', { name: 'Join Local Register Network' }).click();
    await expect(joinModal).toBeHidden();
    await expect(pageB.locator('#teammate-mesh-indicator')).toBeVisible();

    // Have Device A also join the mesh so they can communicate bi-directionally
    await pageA.evaluate(() => {
        const joinBtn = document.getElementById('join-mesh-btn');
        if (joinBtn) joinBtn.click();
    });
    await expect(pageA.locator('#teammate-mesh-indicator')).toBeVisible();

    // 5. Device A makes an offline cash sale
    await pageA.locator('button', { hasText: 'Falafel Wrap' }).click();
    const chargeBtnA = pageA.getByRole('button', { name: /Charge \$/ });
    await expect(chargeBtnA).toBeVisible();
    await chargeBtnA.click();
    const cashMethodBtnA = pageA.getByRole('button', { name: 'Cash' });
    await expect(cashMethodBtnA).toBeVisible();
    await cashMethodBtnA.click();
    const recordCashSaleBtnA = pageA.getByRole('button', { name: /Record Offline Cash Sale/ });
    await recordCashSaleBtnA.click();
    await expect(pageA.getByText('Cash sale saved offline')).toBeVisible({ timeout: 10000 });
    await pageA.getByRole('button', { name: 'No Receipt' }).click();

    // Verify stock goes down on Device A
    await expect(pageA.locator('button', { hasText: 'Falafel Wrap' })).toContainText('Stock: 4');

    // 6. Verify Device B instantly receives the P2P mesh update and updates UI
    await expect(pageB.locator('button', { hasText: 'Falafel Wrap' })).toContainText('Stock: 4', { timeout: 15000 });

    // Verify it's in the IndexedDB offline queue on Device A
    const queueData = await pageA.evaluate(async () => {
        return new Promise<string>((resolve) => {
            const req = window.indexedDB.open('OHC_Offline_Queue', 1);
            req.onsuccess = (e) => {
                const db = (e.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains('actions')) return resolve('[]');
                const tx = db.transaction('actions', 'readonly');
                const reqAll = tx.objectStore('actions').getAll();
                reqAll.onsuccess = () => resolve(JSON.stringify(reqAll.result));
            };
            req.onerror = () => resolve('[]');
        });
    });
    expect(queueData).toContain('cash_sale');

    // Restore Network
    await contextA.setOffline(false);
    await pageA.evaluate(() => window.dispatchEvent(new Event('online')));
    await contextB.setOffline(false);
    await pageB.evaluate(() => window.dispatchEvent(new Event('online')));

    await contextA.close();
    await contextB.close();
  });
});
