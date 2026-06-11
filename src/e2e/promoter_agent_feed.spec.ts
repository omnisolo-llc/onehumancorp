import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed - Promoter Auto-Draft', () => {
  test('owner logs in, sees Promoter social draft, and approves it', async ({ page }) => {
    // 1. Start by logging in via UI (mandatory for owner E2E flow)
    await page.goto('/login');
    // Ensure we are on the login page and use test credentials
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');

    // Ensure successful navigation to the dashboard after login
    await expect(page).toHaveURL(/\/dashboard/);

    // 2. Wait for the feed to be visible
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Generate draft
    await page.goto('/products/new');
    await expect(page.locator('h1').filter({ hasText: 'Add Product' })).toBeVisible();
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('span:has-text("Take a photo or upload")').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({ name: 'test-image.jpg', mimeType: 'image/jpeg', buffer: Buffer.from('fake image data') });
    await expect(page.locator('input[value="Artisan Cupcake"]')).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Looks Good")');
    await expect(page.locator('h2').filter({ hasText: 'Product Published!' })).toBeVisible();

    // Check feed
    await page.goto('/dashboard');
    const promoterDraft = page.locator('div.app-card:has-text("New product detected!")');
    await expect(promoterDraft).toBeVisible({ timeout: 15000 });

    // 5. Approve & Schedule
    const approveBtn = promoterDraft.locator('button[data-testid="approve-social-post"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // After approval, the card should either disappear or state should change (depending on implementation, here usually the card is removed from feed)
    await expect(promoterDraft).not.toBeVisible({ timeout: 10000 });
  });

  test('owner can dismiss Promoter social draft', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Generate draft
    await page.goto('/products/new');
    await expect(page.locator('h1').filter({ hasText: 'Add Product' })).toBeVisible();
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('span:has-text("Take a photo or upload")').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({ name: 'test-image.jpg', mimeType: 'image/jpeg', buffer: Buffer.from('fake image data') });
    await expect(page.locator('input[value="Artisan Cupcake"]')).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Looks Good")');
    await expect(page.locator('h2').filter({ hasText: 'Product Published!' })).toBeVisible();

    // Check feed
    await page.goto('/dashboard');
    const promoterDraft = page.locator('div.app-card:has-text("New product detected!")');
    await expect(promoterDraft).toBeVisible({ timeout: 15000 });

    // Dismiss
    const dismissBtn = promoterDraft.locator('button[data-testid="dismiss-social-post"]');
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();
    await expect(promoterDraft).not.toBeVisible({ timeout: 10000 });
  });

  test('owner can see edit draft button on Promoter social draft', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Generate draft
    await page.goto('/products/new');
    await expect(page.locator('h1').filter({ hasText: 'Add Product' })).toBeVisible();
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('span:has-text("Take a photo or upload")').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({ name: 'test-image.jpg', mimeType: 'image/jpeg', buffer: Buffer.from('fake image data') });
    await expect(page.locator('input[value="Artisan Cupcake"]')).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Looks Good")');
    await expect(page.locator('h2').filter({ hasText: 'Product Published!' })).toBeVisible();

    // Check feed
    await page.goto('/dashboard');
    const promoterDraft = page.locator('div.app-card:has-text("New product detected!")');
    await expect(promoterDraft).toBeVisible({ timeout: 15000 });

    // Verify edit button
    const editBtn = promoterDraft.locator('button[data-testid="edit-social-post"]');
    await expect(editBtn).toBeVisible();
  });

  test('owner can click edit draft button on Promoter social draft', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Generate draft
    await page.goto('/products/new');
    await expect(page.locator('h1').filter({ hasText: 'Add Product' })).toBeVisible();
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('span:has-text("Take a photo or upload")').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({ name: 'test-image.jpg', mimeType: 'image/jpeg', buffer: Buffer.from('fake image data') });
    await expect(page.locator('input[value="Artisan Cupcake"]')).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Looks Good")');
    await expect(page.locator('h2').filter({ hasText: 'Product Published!' })).toBeVisible();

    // Check feed
    await page.goto('/dashboard');
    const promoterDraft = page.locator('div.app-card:has-text("New product detected!")');
    await expect(promoterDraft).toBeVisible({ timeout: 15000 });

    // Verify edit button navigation
    const editBtn = promoterDraft.locator('button[data-testid="edit-social-post"]');
    await expect(editBtn).toBeVisible();
    await editBtn.click();
    await expect(page).toHaveURL(/\/promoter/);
  });

  test('owner can see correct social draft layout', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Generate draft
    await page.goto('/products/new');
    await expect(page.locator('h1').filter({ hasText: 'Add Product' })).toBeVisible();
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('span:has-text("Take a photo or upload")').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({ name: 'test-image.jpg', mimeType: 'image/jpeg', buffer: Buffer.from('fake image data') });
    await expect(page.locator('input[value="Artisan Cupcake"]')).toBeVisible({ timeout: 10000 });
    await page.click('button:has-text("Looks Good")');
    await expect(page.locator('h2').filter({ hasText: 'Product Published!' })).toBeVisible();

    // Check feed
    await page.goto('/dashboard');
    const promoterDraft = page.locator('div.app-card:has-text("New product detected!")');
    await expect(promoterDraft).toBeVisible({ timeout: 15000 });

    await expect(promoterDraft).toContainText('Schedule a post?');
    await expect(promoterDraft).toContainText('Instagram / TikTok Draft');

    const approveBtn = promoterDraft.locator('button[data-testid="approve-social-post"]');
    await expect(approveBtn).toBeVisible();
    const editBtn = promoterDraft.locator('button[data-testid="edit-social-post"]');
    await expect(editBtn).toBeVisible();
    const dismissBtn = promoterDraft.locator('button[data-testid="dismiss-social-post"]');
    await expect(dismissBtn).toBeVisible();
  });
});