import { test, expect } from '@playwright/test';

test.describe('Mobile Menu Offline Toggle & Agent Verification', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('Food cart operator toggles Chicken Shawarma to Sold Out offline and syncs', async ({ page, request, context }) => {
    const tenantId = 'fatima_cart_' + Date.now();

    // 1. Seed the database with the tenant and product
    const seederResponse = await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('fatima_user_id', 'fatima@example.com', 'Fatima', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'Fatima Food Cart', 'fatima@example.com')
          ON CONFLICT DO NOTHING;

          INSERT INTO products (id, tenant_id, title, description, price_cents, inventory_count, available_quantity)
          VALUES ('prod_shawarma', '${tenantId}', 'Chicken Shawarma', 'Delicious shawarma', 800, 50, 50)
          ON CONFLICT DO NOTHING;
        `
      }
    });

    // Ensure seeding worked
    expect(seederResponse.ok()).toBeTruthy();

    // 2. Set tenant in localStorage and go to mobile menu
    await page.goto(`/login?test_email=fatima@example.com`);
    await page.evaluate((t) => localStorage.setItem('tenant_id', t), tenantId);
    await page.goto('/mobile-menu');

    // Verify initial load
    await expect(page.locator('text=Chicken Shawarma')).toBeVisible({ timeout: 15000 });
    const productCard = page.locator('.bg-white\\/65, .app-card').filter({ hasText: 'Chicken Shawarma' }).first();
    const toggleBtn = productCard.locator('button');

    await expect(toggleBtn).toContainText('Available');

    // 3. Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Verify offline indicator
    await expect(page.locator('text=Offline - Changes saved locally')).toBeVisible({ timeout: 10000 });

    // 4. Toggle Sold Out
    await toggleBtn.click();
    await expect(toggleBtn).toContainText('Sold Out');

    // It should now show optimistic Sold Out UI
    await expect(productCard.locator('h3')).toHaveClass(/line-through/);

    // Verify it's in the queue
    const queueData = await page.evaluate(async () => {
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
    expect(queueData).toContain('TOGGLE_SOLD_OUT');

    // 5. Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for the sync to complete and the toast to appear
    await expect(page.locator('text=Menu updated online')).toBeVisible({ timeout: 10000 });

    // 6. Verify backend task creation (Operations Agent checks pre-orders)
    await page.waitForTimeout(2000); // Give backend a moment to process the sync event and insert into department_tasks

    const taskCheckResponse = await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          SELECT COUNT(*) as count FROM department_tasks
          WHERE tenant_id = '${tenantId}'
          AND event_type = 'InventoryConflictEvent'
          AND payload::text LIKE '%prod_shawarma%';
        `
      }
    });

    const taskCheckResult = await taskCheckResponse.json();
    expect(taskCheckResult.rows[0].count).toBeGreaterThan(0);
  });
});
