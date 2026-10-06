import { test, expect } from './fixtures';

test.describe('Help Chat Flow', () => {
  test('should open help chat, type message, and see response', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // loginAs opens the authenticated dashboard.

    // Check that the floating chat button exists
    const chatButton = page.locator('#omnisolo-floating-help-btn').first();
    await expect(chatButton).toBeVisible();

    // Open chat
    await chatButton.click();

    // Click the Ask AI tab
    await page.locator('button[data-target="tab-chat"]').click();

    // Verify chat UI appears
    const chatHeader = page.locator('#ohc-floating-help-header h3');
    await expect(chatHeader).toBeVisible();
    await expect(page.getByText('In-App Help Center')).toBeVisible();
    await expect(page.getByText("Hi! I'm your Help Agent. How can I assist you today? You can ask me anything about using OmniSolo.")).toBeVisible();

    // Type a message
    const input = page.locator('#ohc-help-chat-input');
    await input.fill('What is Operations?');

    // Submit
    const sendButton = page.locator('#ohc-help-chat-send');
    await sendButton.click();

    // Verify the current backend response and its follow-up link
    await expect(page.locator('text=I have routed your request to the Operations department.')).toBeVisible();

    // Verify link exists
    await expect(page.getByRole('link', { name: 'Check your inbox for updates →' })).toBeVisible();
  });
});

test.describe('Help Center Complete UI Flow', () => {
  test('should load Help Center, find videos, and click video to play', async ({ page, loginAs, unlimitedAdminUser, baseURL }) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    const origin = new URL(baseURL).origin;
    await loginAs(page, unlimitedAdminUser);
    const videosRead = page.waitForEvent('requestfinished', { predicate: request => {
      const url = new URL(request.url());
      return url.origin === origin && url.pathname === '/api/v1/videos' && request.method() === 'GET';
    } }).then(request => request.response());
    await page.goto('/help');
    const videosResponse = await videosRead;
    if (!videosResponse) throw new Error('Completed video catalog request has no response');
    expect(videosResponse.status()).toBe(200);
    const videos: { title: string; video_url: string }[] = await videosResponse.json();
    expect(Array.isArray(videos)).toBe(true);
    const title = 'Connecting a bank account to accept payments';
    const selectedVideo = videos.find(video => video.title === title)!;
    expect(selectedVideo).toBeDefined();
    expect(selectedVideo.video_url).toEqual(expect.any(String));
    expect(selectedVideo.video_url.length).toBeGreaterThan(0);

    // The title is present before filtering too. Wait for the actual debounced
    // search to finish before actionability checks track its final layout.
    const searchRead = page.waitForEvent('requestfinished', { predicate: request => {
      const url = new URL(request.url());
      return url.origin === origin && url.pathname === '/api/v1/help/search' && url.searchParams.get('q') === 'payment'
        && request.method() === 'GET';
    } }).then(request => request.response());
    await page.getByPlaceholder('Search for help articles and videos...').fill('payment');
    const searchResponse = await searchRead;
    if (!searchResponse) throw new Error('Completed Help search request has no response');
    expect(searchResponse.status()).toBe(200);
    expect(Array.isArray(await searchResponse.json())).toBe(true);

    const play = page.getByRole('button', { name: `Play video: ${title}`, exact: true });
    await expect(play).toBeVisible();
    await play.click();
    const videoModal = page.locator('video');
    await expect(videoModal).toBeVisible();
    await expect(videoModal).toHaveAttribute('src', selectedVideo.video_url);
    await expect(videoModal).toHaveAttribute('controls', '');
    await page.getByRole('button', { name: 'Close video', exact: true }).click();
    await expect(videoModal).not.toBeVisible();

    // Closing must restore a usable control, including after the search reflow.
    await play.click();
    await expect(videoModal).toBeVisible();
    await expect(videoModal).toHaveAttribute('src', selectedVideo.video_url);
    await page.getByRole('button', { name: 'Close video', exact: true }).click();
    await expect(videoModal).not.toBeVisible();
  });
});

test.describe('Tooltip functionality', () => {
  test('should display tooltip on hover', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    // Wait until tooltips load dynamically or are preloaded on Help page
    await page.goto('/api-docs');

    const tooltipTrigger = page.locator('#api-docs-tooltip');
    await tooltipTrigger.waitFor({ state: "visible", timeout: 10000 });
    await expect(tooltipTrigger).toBeVisible();

    await tooltipTrigger.hover();

    // Check if the tooltip wrapper gets rendered
    await expect(page.getByText('Direct API access is only for custom integrations.').first()).toBeVisible({ timeout: 10000 });
  });

  test('should display tooltip on dashboard hover', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // We expect the tooltip with text "View your daily sales and overall business health." to appear
    const dashboardTooltipTrigger = page.locator('.cursor-help', { hasText: 'Dashboard' }).first();
    await expect(dashboardTooltipTrigger).toBeVisible();

    await dashboardTooltipTrigger.hover();

    await expect(page.getByText('View your daily sales and overall business health.').first()).toBeVisible({ timeout: 10000 });
  });
});

test.describe('Changelog UX', () => {
  test('should ensure changelog renders beautiful design without placeholder text', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Version 1.1 (Latest)' })).toBeVisible();
    // Check that we removed the test line
    await expect(page.locator('text=This is a plain paragraph test line.')).not.toBeVisible();
  });
});

test.describe('API Documentation', () => {
  test('should navigate to API Documentation and load Swagger UI', async ({ page }) => {
    await page.goto('/api-docs');

    // Check for advanced warning badge
    await expect(page.getByText('Advanced:')).toBeVisible();

    // Check for swagger-ui wrapper
    const swaggerUI = page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui');
    await expect(swaggerUI).toBeVisible({ timeout: 15000 });
  });
});

test.describe('AppShell Help Button', () => {
  test('should display Help Center link and navigate successfully', async ({ page, loginAs, unlimitedAdminUser, baseURL }) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    const origin = new URL(baseURL).origin;
    const dashboardReady = Promise.all([
      '/api/v1/ui/dashboard/unified-feed', '/api/v1/onboarding/state',
    ].map(path => page.waitForEvent('requestfinished', { predicate: request => {
      const url = new URL(request.url());
      return url.origin === origin && url.pathname === path && request.method() === 'GET';
    } }).then(async request => {
      const response = await request.response();
      if (!response) throw new Error('Completed dashboard request has no response');
      expect(response.status()).toBe(200);
      expect(await response.json()).toEqual(expect.any(Object));
    })));
    // Observe early failures immediately; awaiting the original promise below
    // still reports transport failures without masking a login failure.
    void dashboardReady.catch(() => undefined);
    // loginAs already opens Dashboard; do not replace that document while its
    // hydration and initial status reads are still finishing.
    await loginAs(page, unlimitedAdminUser);
    await dashboardReady;
    const statusStrip = page.locator('.app-topbar .app-status-strip');
    await expect(statusStrip).not.toContainText('Loading');
    await expect(statusStrip).not.toContainText('Unknown');

    const helpLink = page.getByRole('link', { name: 'Help Center', exact: true });
    await expect(helpLink).toHaveAttribute('href', '/help');
    await expect(helpLink).toBeVisible();
    await helpLink.click();
    await expect(page).toHaveURL(`${origin}/help`);
    await expect(page.getByRole('heading', { name: 'In-App Help Center', level: 1, exact: true })).toBeVisible();
    await expect(page.getByPlaceholder('Search for help articles and videos...')).toBeVisible();
  });
});
