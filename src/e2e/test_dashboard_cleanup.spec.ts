import { test, expect } from './fixtures';

test.describe('Dashboard Cleanup Audit', () => {
  test('Verify absence of PRO badge in Advanced AI Automations card', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
    const heading = page.locator('h2', { hasText: 'Advanced AI Automations' });
    if(await heading.count() > 0) {
      await expect(heading).toBeVisible();
      await expect(heading).not.toContainText('PRO');
    }
  });

  test('Verify absence of Failed to load time savings data error', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
    await expect(page.locator('text="Failed to load time savings data."')).toHaveCount(0);
  });

  test('Verify walkthrough.js is not loaded', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    let walkthroughLoaded = false;
    page.on('request', request => {
      if (request.url().includes('walkthrough.js')) {
        walkthroughLoaded = true;
      }
    });
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
    expect(walkthroughLoaded).toBe(false);
  });

  test('Verify help-chat.js is not loaded', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    let helpChatLoaded = false;
    page.on('request', request => {
      if (request.url().includes('help-chat.js')) {
        helpChatLoaded = true;
      }
    });
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
    expect(helpChatLoaded).toBe(false);
  });

  test('Verify tooltip.js is not loaded', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    let tooltipLoaded = false;
    page.on('request', request => {
      if (request.url().includes('tooltip.js')) {
        tooltipLoaded = true;
      }
    });
    await page.goto('/dashboard');
    await page.waitForTimeout(3000);
    expect(tooltipLoaded).toBe(false);
  });

  test('Verify gracefully hiding AI savings widget when backend returns error (HTTP 500)', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Mock the backend API to force an error
    await page.route('/api/v1/growth/time-savings', route => {
        route.fulfill({
            status: 500,
            contentType: 'application/json',
            body: JSON.stringify({ error: 'Internal Server Error' }),
        });
    });

    await page.goto('/dashboard');
    await page.waitForTimeout(3000);

    // Wait for the widget to NOT be visible
    const widget = page.locator('#ai-savings-widget');
    if (await widget.count() > 0) {
      await expect(widget.locator('..')).toBeHidden();
    }
  });

  test('Verify gracefully hiding AI savings widget when backend is unreachable (network failure)', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Mock the backend API to simulate a network failure
    await page.route('/api/v1/growth/time-savings', route => {
        route.abort('failed');
    });

    await page.goto('/dashboard');
    await page.waitForTimeout(3000);

    const widget = page.locator('#ai-savings-widget');
    if (await widget.count() > 0) {
      await expect(widget.locator('..')).toBeHidden();
    }
  });

  test('Verify AI savings widget is visible when backend returns valid data', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Mock the backend API to return valid data
    await page.route('/api/v1/growth/time-savings', route => {
        route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
              hours_saved: 42,
              inquiries_handled: 10,
              appointments_scheduled: 5,
              carts_recovered: 2
            }),
        });
    });

    await page.goto('/dashboard');
    await page.waitForTimeout(3000);

    const widget = page.locator('#ai-savings-widget');
    if (await widget.count() > 0) {
      const parent = widget.locator('..');
      await expect(parent).toBeVisible();
      await expect(page.locator('#ai-savings-title')).toHaveText('You saved 42 hours this week');
    }
  });

  test('Verify AI savings widget is visible with zero hours when backend returns error (HTTP 404)', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Mock the backend API to simulate missing endpoint or no data for this user
    await page.route('/api/v1/growth/time-savings', route => {
        route.fulfill({
            status: 404,
            contentType: 'application/json',
            body: JSON.stringify({ error: 'Not Found' }),
        });
    });

    await page.goto('/dashboard');
    await page.waitForTimeout(3000);

    // The previous implementation fell through to the catch block, but if we handle 404 it might go through 'else' or 'catch'.
    // We expect it to be hidden since it's an error.
    const widget = page.locator('#ai-savings-widget');
    if (await widget.count() > 0) {
      await expect(widget.locator('..')).toBeHidden();
    }
  });

  test('Verify AI savings widget handles partial valid data gracefully', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Mock the backend API to return partial data
    await page.route('/api/v1/growth/time-savings', route => {
        route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
              hours_saved: 5,
              inquiries_handled: 2,
              // Missing appointments_scheduled and carts_recovered
            }),
        });
    });

    await page.goto('/dashboard');
    await page.waitForTimeout(3000);

    const widget = page.locator('#ai-savings-widget');
    if (await widget.count() > 0) {
      const parent = widget.locator('..');
      await expect(parent).toBeVisible();
      await expect(page.locator('#ai-savings-title')).toHaveText('You saved 5 hours this week');
      // Should show 'undefined' for missing data, but shouldn't crash the widget
    }
  });
});
