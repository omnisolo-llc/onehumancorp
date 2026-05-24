import { test, expect } from './fixtures';

test.describe('Dashboard UX', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display correctly on mobile and verify plain language labels', async ({ page }) => {
    // Navigate to login page
    await page.goto('/login');

    // Fill in credentials and sign in
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();

    // Wait for Dashboard to load
    await page.waitForURL('**/*');

    // Some apps navigate to '/' or '/dashboard', we will just wait for navigation
    // and verify the labels.
    await expect(page.locator('text=My Business').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=Orders to Ship')).toBeVisible();
    await expect(page.locator('text=Team Members')).toBeVisible();
    await expect(page.locator('text=Ongoing Tasks')).toBeVisible();

    // Verify softer wording for drafts
    await expect(page.locator('text=Needs Your Approval')).toBeVisible();
  });
});

test('should display Quick Actions on mobile', async ({ page }) => {
  await page.goto('/login');
  await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
  await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
  await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();
  await page.waitForURL('**/*');

  // Verify navigation actions
  await expect(page.locator('text=Store Tips')).toBeVisible();

  // Verify First-Time User Tour ? icon toggle
  const questionMarkBtn = page.locator('button:has-text("?")');
  await expect(questionMarkBtn).toBeVisible();

  // Verify tap targets are appropriately sized (>= 44px)
  const box = await questionMarkBtn.boundingBox();
  expect(box?.height).toBeGreaterThanOrEqual(44);
  expect(box?.width).toBeGreaterThanOrEqual(44);

  await questionMarkBtn.click();
  await expect(page.locator('text=These buttons are shortcuts to your most common daily tasks.')).toBeVisible();

  // Verify bottom navigation bar buttons are present
  const btnAdd = page.locator('button:has-text("Add")');
  await expect(btnAdd).toBeVisible();
  const boxAdd = await btnAdd.boundingBox();
  expect(boxAdd?.height).toBeGreaterThanOrEqual(44);
  expect(boxAdd?.width).toBeGreaterThanOrEqual(44);

  const btnOrders = page.locator('button:has-text("Orders")');
  await expect(btnOrders).toBeVisible();
  const boxOrders = await btnOrders.boundingBox();
  expect(boxOrders?.height).toBeGreaterThanOrEqual(44);
  expect(boxOrders?.width).toBeGreaterThanOrEqual(44);

  const btnChat = page.locator('button:has-text("Chat")');
  await expect(btnChat).toBeVisible();
  const boxChat = await btnChat.boundingBox();
  expect(boxChat?.height).toBeGreaterThanOrEqual(44);
  expect(boxChat?.width).toBeGreaterThanOrEqual(44);

  const btnStats = page.locator('button:has-text("Stats")');
  await expect(btnStats).toBeVisible();
  const boxStats = await btnStats.boundingBox();
  expect(boxStats?.height).toBeGreaterThanOrEqual(44);
  expect(boxStats?.width).toBeGreaterThanOrEqual(44);

  const btnShare = page.locator('button:has-text("Share")');
  await expect(btnShare).toBeVisible();
  const boxShare = await btnShare.boundingBox();
  expect(boxShare?.height).toBeGreaterThanOrEqual(44);
  expect(boxShare?.width).toBeGreaterThanOrEqual(44);
});

test('should display Menu toggle on mobile and have expected links', async ({ page }) => {
  await page.goto('/login');
  await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
  await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
  await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();
  await page.waitForURL('**/*');

  // Verify navigation actions
  const menuBtn = page.locator('button:has-text("Menu")');
  await expect(menuBtn).toBeVisible();
  await menuBtn.click();

  await expect(page.locator('button:has-text("Help Center")')).toBeVisible();
  await expect(page.locator('button:has-text("Billing")')).toBeVisible();
  await expect(page.locator('button:has-text("Connect Apps")')).toBeVisible();
  await expect(page.locator('button:has-text("Video Tutorials")')).toBeVisible();
  await expect(page.locator('button:has-text("How to use this app")')).toBeVisible();
  await expect(page.locator('button:has-text("What\'s New")')).toBeVisible();
});

test.describe('Dashboard Flow Completeness UX', () => {
  test('Grandmother test: complete critical journey starting from login', async ({ page }) => {
    // Navigate to login page per constraints
    await page.goto('/login');

    // Fill in credentials and sign in
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();

    // Wait for Dashboard to load
    await page.waitForURL('**/*');

    await expect(page.locator('text=My Business').filter({ visible: true }).first()).toBeVisible();

    const addProductBtn = page.locator('button:has-text("Add")').filter({ visible: true }).first();
    await expect(addProductBtn).toBeVisible();

    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('Grandmother test: Check Orders from login', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();
    await page.waitForURL('**/*');

    const ordersBtn = page.locator('button:has-text("Orders")').filter({ visible: true }).first();
    await expect(ordersBtn).toBeVisible();
  });

  test('Grandmother test: Check Messages from login', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();
    await page.waitForURL('**/*');

    const messagesBtn = page.locator('button:has-text("Chat")').filter({ visible: true }).first();
    await expect(messagesBtn).toBeVisible();
  });

  test('Grandmother test: Check Analytics from login', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();
    await page.waitForURL('**/*');

    const analyticsBtn = page.locator('button:has-text("Stats")').filter({ visible: true }).first();
    await expect(analyticsBtn).toBeVisible();
  });

  test('Grandmother test: Share Store from login', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();
    await page.waitForURL('**/*');

    const shareBtn = page.locator('button:has-text("Share")').filter({ visible: true }).first();
    await expect(shareBtn).toBeVisible();
  });
});

import { Client } from 'pg';

test.describe('Dashboard Approvals', () => {
  test('should display Action Required, allow toggling advanced settings, and processing approvals with state verified', async ({ page }) => {
    // 1. Sign in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();

    // 2. Wait for dashboard and verify "Action Required"
    await page.waitForURL('**/*');
    await expect(page.locator('h2:has-text("Action Required")').first()).toBeVisible();

    // 3. Toggle Advanced Settings to see the technical payload
    const advancedSettingsBtn = page.locator('button').filter({ has: page.locator('span.absolute') }).first();
    await advancedSettingsBtn.click();

    // Look for Technical Payload text
    await expect(page.locator('div:has-text("Technical Payload:")').first()).toBeVisible();

    // 4. Click Approve on the first item
    const approveBtns = page.locator('button:has-text("Approve")');
    const initialCount = await approveBtns.count();
    expect(initialCount).toBeGreaterThan(0);

    await approveBtns.first().click();

    // 5. Verify the UI updates (the item is removed from the list)
    await expect(approveBtns).toHaveCount(initialCount - 1);

    // 6. Assert the database state correctly matches the processed UI action
    const client = new Client({
      connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc',
    });
    await client.connect();
    // We approved the first item. The seeded approvals are: e2e-approval-1, e2e-approval-social, e2e-approval-cart.
    // At least one of them should now have status 'APPROVED'
    const res = await client.query("SELECT count(*) FROM agent_approvals WHERE status = 'APPROVED' AND tenant_id = 'e2e-tenant'");
    await client.end();

    expect(parseInt(res.rows[0].count, 10)).toBeGreaterThan(0);
  });
});
