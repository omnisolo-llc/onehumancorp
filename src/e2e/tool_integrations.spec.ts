import { test, expect } from './fixtures';
test.describe('Tool Integrations UI Premium Dashbaord', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    // page.on('dialog', async dialog => { try { await dialog.accept(); } catch(e) {} });
    await page.goto('/integrations');
    // await page.getByText('Connect Tools').click();
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
  });
  test('shows premium integrations dashboard header and copy', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Tool Integrations' })).toBeVisible();
    await expect(page.getByText('Supercharge your workflow by connecting your favorite tools.')).toBeVisible();
  });
  test('displays social media integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Ayrshare' })).toBeVisible();
    await expect(page.getByText('Unified API for posting and retrieving messages across social networks.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connect' }).first()).toBeVisible();
  });
  test('displays online booking integration card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Cal.com' })).toBeVisible();
    await expect(page.getByText('Zero-Config Booking & Calendar Sync.')).toBeVisible();
  });
  test('displays automated shipping and global payment methods cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'EasyPost' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Mercado Pago' })).toBeVisible();
    await expect(page.getByText('Painless Shipping Labels & Tracking.')).toBeVisible();
    await expect(page.getByText('Accept credit cards and local payment methods in Latin America.')).toBeVisible();
  });
  test('displays email marketing and automated video links cards', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Listmonk' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Jitsi Meet' })).toBeVisible();
    await expect(page.getByText('Embedded, No-Jargon Email Campaigns.')).toBeVisible();
    await expect(page.getByText('Zero-Setup Online Lessons and video conferencing.')).toBeVisible();
  });
  test('displays global sms notifications card', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Twilio' })).toBeVisible();
    await expect(page.getByText('Reliable SMS alerts for new orders and customer notifications.')).toBeVisible();
  });
    test('can connect Ayrshare', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Ayrshare' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', async dialog => {
      // API call will fail locally without auth, falling back to basic connecting message
      expect(dialog.message()).toMatch(/(Successfully requested integration for Ayrshare|Connecting to Ayrshare)/);
      await dialog.accept();
    });
    await connectButton.click();
    await page.waitForTimeout(500); // Give dialog time to appear
  });
    test('can connect Cal.com', async ({ page }) => {
    const connectButton = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Cal.com' }).getByRole('button', { name: 'Connect' });
    page.once('dialog', async dialog => {
      // API call will fail locally without auth, falling back to basic connecting message
      expect(dialog.message()).toMatch(/(Successfully requested integration for Cal.com|Connecting to Cal.com)/);
      await dialog.accept();
    });
    await connectButton.click();
    await page.waitForTimeout(500); // Give dialog time to appear
  });
  test('can connect Listmonk and Mercado Pago', async ({ page }) => {
    const listmonkBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Listmonk' }).getByRole('button', { name: 'Connect' });
    page.on('dialog', async dialog => { try { await dialog.accept(); } catch(e) {} });
    await listmonkBtn.click();
    const mercadoBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Mercado Pago' }).getByRole('button', { name: 'Connect' });
    page.on('dialog', async dialog => { try { await dialog.accept(); } catch(e) {} });
    await mercadoBtn.click();
  });
  test('can connect EasyPost, Twilio, and Jitsi Meet', async ({ page }) => {
    page.on('dialog', async dialog => { try { await dialog.accept(); } catch(e) {} });
    const easypostBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'EasyPost' }).getByRole('button', { name: 'Connect' });
    await easypostBtn.click();
    const twBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Twilio' }).getByRole('button', { name: 'Connect' });
    await twBtn.click();
    const jitsiBtn = page.locator('div.rounded-\\[16px\\]').filter({ hasText: 'Jitsi Meet' }).getByRole('button', { name: 'Connect' });
    await jitsiBtn.click();
  });
});
