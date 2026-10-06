import { test, expect } from './fixtures';

test.describe('Help Center and Contextual Help (Tauri UI)', () => {
  test('Persona: Business Owner uses help center and chat', async ({ page }) => {
    const videosLoaded = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/videos' && response.request().method() === 'GET');
    await page.goto('/api/v1/ui/dashboard.html?test_walkthrough=true');
    await expect(page).toHaveURL(url => url.pathname === '/dashboard');
    const videosResponse = await videosLoaded;
    expect(videosResponse.status()).toBe(200);
    const videos = await videosResponse.json() as Array<{ title: string; video_url: string }>;
    expect(videos.length).toBeGreaterThan(0);
    expect(videos[0].video_url).toEqual(expect.any(String));
    expect(videos[0].video_url.trim().length).toBeGreaterThan(0);

    const widgetBtn = page.getByRole('button', { name: 'Open help chat', exact: true });
    await widgetBtn.click();
    const widget = page.locator('#omnisolo-floating-help-widget');
    await expect(widget).toBeVisible();
    await widget.getByRole('button', { name: 'Videos', exact: true }).click();
    await widget.getByText(videos[0].title, { exact: true }).click();
    const videoModal = page.getByRole('dialog');
    await expect(videoModal).toBeVisible();
    await expect(videoModal.locator('video')).toHaveAttribute('src', videos[0].video_url);
    await videoModal.getByRole('button', { name: 'Close video', exact: true }).click();
    await expect(videoModal).toBeHidden();
    await widget.getByRole('button', { name: 'Close Help Widget', exact: true }).click();
    await expect(widget).toBeHidden();
    await expect(widgetBtn).toBeFocused();

    await widgetBtn.click();
    await widget.getByRole('button', { name: 'Ask AI (Ask anything)', exact: true }).click();
    await expect(widget.getByRole('heading', { name: 'Ask AI Help', exact: true })).toBeVisible();
    const message = 'How do I accept credit cards?';
    await widget.getByPlaceholder('Ask anything...', { exact: true }).fill(message);
    const sent = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/chat' && response.request().method() === 'POST');
    await widget.getByRole('button', { name: 'Send message', exact: true }).click();
    const chatResponse = await sent;
    expect(chatResponse.status()).toBe(200);
    expect(chatResponse.request().postDataJSON()).toEqual({ message });
    const reply = await chatResponse.json() as { reply: string; link: { title: string; url: string } };
    expect(reply.reply.trim().length).toBeGreaterThan(0);
    expect(reply.link.url).toMatch(/^\/help(?:\?|\/)/);
    await expect(widget.getByText(message, { exact: true })).toBeVisible();
    await expect(widget.getByText(reply.reply, { exact: true })).toBeVisible();
    const articleLink = widget.getByRole('link', { name: reply.link.title, exact: true });
    await expect(articleLink).toHaveAttribute('href', /^\/help\/[A-Za-z0-9_-]+$/);
    await widget.getByRole('button', { name: 'Close Help Widget', exact: true }).click();
    await expect(widget).toBeHidden();
    await expect(widgetBtn).toBeFocused();

    await page.goto('/api/v1/ui/help.html');
    await expect(page.locator('text=In-App Help Center').first()).toBeVisible();
    await expect(page.locator('text=Getting Started').first()).toBeVisible();
    await page.getByPlaceholder('Search for help articles and videos...').fill('paid');
    await expect(page.locator('text=Accepting Payments').first()).toBeVisible();
  });

  test('Persona: Business Owner views the Changelog', async ({ page }) => {
    await page.goto('/api/v1/ui/changelog.html');
    await expect(page.locator('text=Release Notes & Changelog').first()).toBeVisible();
    await expect(page.locator('text=v0.4.48 (Cloud)').first()).toBeVisible();
    await expect(page.locator('text=Cloud Scaling Improvements').first()).toBeVisible();
  });

  test('Persona: Developer views the API documentation', async ({ page }) => {
    await page.goto('/api/v1/ui/api-docs.html');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').getByText('OmniSolo Advanced API Reference').first()).toBeVisible();
  });

  test('Persona: Business Owner interacts with a Tooltip', async ({ page }) => {
    const loaded = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/tooltips' && response.request().method() === 'GET');
    await page.goto('/api/v1/ui/dashboard.html?test_walkthrough=true');
    const response = await loaded;
    expect(response.status()).toBe(200);
    const tooltips = await response.json() as Record<string, string>;
    const expectedText = tooltips['help-btn-tooltip'];
    expect(expectedText).toEqual(expect.any(String));
    expect(expectedText.trim().length).toBeGreaterThan(0);
    const target = page.locator('#help-btn-tooltip');
    const launcher = target.getByRole('button', { name: 'Open help chat', exact: true });
    await expect(target).toHaveAttribute('data-tooltip', expectedText);
    await launcher.hover();
    const tooltip = page.getByRole('tooltip');
    await expect(tooltip).toHaveText(expectedText);
    await expect(tooltip).toBeVisible();
    await page.mouse.move(0, 0);
    await expect(tooltip).toBeHidden();
    await launcher.focus();
    await expect(tooltip).toBeVisible();
    await launcher.press('Escape');
    await expect(tooltip).toBeHidden();
    await expect(launcher).toBeFocused();
  });
});
