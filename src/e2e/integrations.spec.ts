import { test, expect } from '@playwright/test';

test.describe('Integrations Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    const loginLink = page.locator('text=/Login/i');
    await loginLink.click();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(500); // give some time

    // Navigate to Integrations
    const integrationsMenu = page.locator('text=/Integrations/i, text=/Connect/i').filter({ visible: true }).first();
    await integrationsMenu.click();
  });

          test('should display integrations page', async ({ page }) => {

    await expect(page.locator('text=/integrations|connect/i')).toBeVisible();
  });

  test('should show integrations header', async ({ page }) => {

    await expect(page.locator('text=Integrations')).toBeVisible();
  });

  test('should display available integrations', async ({ page }) => {

    const integration = page.locator('[class*="integration"], [class*="app"]').filter({ visible: true }).first();
    await expect(integration).toBeVisible();
  });

  test('should show slack integration', async ({ page }) => {

    await expect(page.locator('text=Slack')).toBeVisible();
  });

  test('should show github integration', async ({ page }) => {

    await expect(page.locator('text=GitHub')).toBeVisible();
  });

  test('should show zapier integration', async ({ page }) => {

    await expect(page.locator('text=Zapier')).toBeVisible();
  });

  test('should show google workspace integration', async ({ page }) => {

    await expect(page.locator('text=/google|workspace/i')).toBeVisible();
  });

  test('should show microsoft teams integration', async ({ page }) => {

    await expect(page.locator('text=/microsoft|teams/i')).toBeVisible();
  });

  test('should connect slack integration', async ({ page }) => {

    const slackBtn = page.locator('button:has-text("Connect"), button:has-text("Slack")').filter({ visible: true }).first();
    await slackBtn.click();
    await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
  });

  test('should disconnect integration', async ({ page }) => {

    const integration = page.locator('[class*="integration"]').filter({ visible: true }).first();
    await integration.hover();
    const disconnectBtn = page.locator('button:has-text("Disconnect"), button:has-text("Remove")').filter({ visible: true }).first();
    await disconnectBtn.click();
    await expect(page.locator('text=/disconnected|removed/i')).toBeVisible({ timeout: 3000 });
  });

  test('should show integration status', async ({ page }) => {

    const status = page.locator('text=/connected|active|inactive/i').filter({ visible: true }).first();
    await expect(status).toBeVisible();
  });

  test('should configure integration settings', async ({ page }) => {

    const integration = page.locator('[class*="integration"]').filter({ visible: true }).first();
    await integration.click();
    const settingsBtn = page.locator('button:has-text("Settings"), button:has-text("Configure")').filter({ visible: true }).first();
    await settingsBtn.click();
    await expect(page.locator('text=/settings|configure/i')).toBeVisible();
  });

  test('should show integration usage stats', async ({ page }) => {

    const stats = page.locator('text=/usage|requests|api.*calls/i').filter({ visible: true }).first();
    await expect(stats).toBeVisible();
  });

  test('should search integrations', async ({ page }) => {

    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').filter({ visible: true }).first();
    await searchInput.fill('Meta');
    await expect(page.locator('text=/Meta/i')).toBeVisible();
    await expect(page.locator('text=/slack/i')).toBeVisible();
  });

  test('should filter integrations by category', async ({ page }) => {

    const filterSelect = page.locator('select').filter({ visible: true }).first();
    await filterSelect.selectOption({ index: 1 });
  });

  test('should show meta integration', async ({ page }) => {
    await expect(page.locator('text=/Meta/i')).toBeVisible();
  });

  test('should connect meta integration', async ({ page }) => {
    const metaBtn = page.locator('button:has-text("Connect"), button:has-text("Meta")').filter({ visible: true }).first();
    await metaBtn.click();
    await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
  });

  test('should show zoom integration', async ({ page }) => {
    await expect(page.locator('text=/Zoom/i')).toBeVisible();
  });

  test('should connect zoom integration', async ({ page }) => {
    const zoomBtn = page.locator('button:has-text("Connect"), button:has-text("Zoom")').filter({ visible: true }).first();
    await zoomBtn.click();
    await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
  });

  test('should show twilio integration', async ({ page }) => {
    await expect(page.locator('text=/Twilio/i')).toBeVisible();
  });

  test('should connect twilio integration', async ({ page }) => {
    const twilioBtn = page.locator('button:has-text("Connect"), button:has-text("Twilio")').filter({ visible: true }).first();
    await twilioBtn.click();
    await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
  });

  test('should show calendly integration', async ({ page }) => {
    await expect(page.locator('text=/Calendly/i')).toBeVisible();
  });

  test('should connect calendly integration', async ({ page }) => {
    const calendlyBtn = page.locator('button:has-text("Connect"), button:has-text("Calendly")').filter({ visible: true }).first();
    await calendlyBtn.click();
    await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
  });


  test('should show Chatwoot integration', async ({ page }) => {
    await expect(page.locator('text=/Chatwoot/i')).toBeVisible();
  });

  test('should connect Chatwoot integration', async ({ page }) => {
    const chatwootBtn = page.locator('text=Chatwoot').locator('..').locator('button:has-text("Configure")').filter({ visible: true }).first();
    await chatwootBtn.click();
    await expect(page.locator('text=/Chatwoot/i')).toBeVisible();
  });

  test('should show Cal.com integration', async ({ page }) => {
    await expect(page.locator('text=/Cal\\.com/i')).toBeVisible();
  });

  test('should connect Cal.com integration', async ({ page }) => {
    const calcomBtn = page.locator('text=Cal.com').locator('..').locator('button:has-text("Configure")').filter({ visible: true }).first();
    await calcomBtn.click();
    await expect(page.locator('text=/Cal\\.com/i')).toBeVisible();
  });

  test('should show Resend integration', async ({ page }) => {
    await expect(page.locator('text=/Resend/i')).toBeVisible();
  });

  test('should connect Resend integration', async ({ page }) => {
    const resendBtn = page.locator('text=Resend').locator('..').locator('button:has-text("Configure")').filter({ visible: true }).first();
    await resendBtn.click();
    await expect(page.locator('text=/Resend/i')).toBeVisible();
  });

  test('should show Shippo integration', async ({ page }) => {
    await expect(page.locator('text=/Shippo/i')).toBeVisible();
  });

  test('should connect Shippo integration', async ({ page }) => {
    const shippoBtn = page.locator('text=Shippo').locator('..').locator('button:has-text("Configure")').filter({ visible: true }).first();
    await shippoBtn.click();
    await expect(page.locator('text=/Shippo/i')).toBeVisible();
  });

  test('should show Mercado Pago integration', async ({ page }) => {
    await expect(page.locator('text=/Mercado Pago/i')).toBeVisible();
  });

  test('should connect Mercado Pago integration', async ({ page }) => {
    const mpBtn = page.locator('text=Mercado Pago').locator('..').locator('button:has-text("Configure")').filter({ visible: true }).first();
    await mpBtn.click();
    await expect(page.locator('text=/Mercado Pago/i')).toBeVisible();
  });

  test('should show Razorpay integration', async ({ page }) => {
    await expect(page.locator('text=/Razorpay/i')).toBeVisible();
  });

  test('should connect Razorpay integration', async ({ page }) => {
    const razorpayBtn = page.locator('text=Razorpay').locator('..').locator('button:has-text("Configure")').filter({ visible: true }).first();
    await razorpayBtn.click();
    await expect(page.locator('text=/Razorpay/i')).toBeVisible();
  });



  test('should show Mailchimp integration', async ({ page }) => {
    await expect(page.locator('text=/Mailchimp/i')).toBeVisible();
  });

  test('should connect Mailchimp integration', async ({ page }) => {
    const btn = page.locator('text=Mailchimp').locator('..').locator('button:has-text("Configure"), button:has-text("Connect")').filter({ visible: true }).first();
    await btn.click();
    await expect(page.locator('text=/Mailchimp/i')).toBeVisible();
  });


test.describe('Pipeline Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    const loginLink = page.locator('text=/Login/i');
    await loginLink.click();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password');
    await page.click('button[type="submit"]');
    await page.waitForTimeout(500); // give some time

    const pipelinesMenu = page.locator('text=/Pipelines/i').filter({ visible: true }).first();
    await pipelinesMenu.click();
  });

  test('should display pipelines page', async ({ page }) => {

    await expect(page.locator('text=/pipeline|workflow/i')).toBeVisible();
  });

  test('should show pipelines header', async ({ page }) => {

    await expect(page.locator('text=Pipelines')).toBeVisible();
  });

  test('should display pipeline list', async ({ page }) => {

    const pipeline = page.locator('[class*="pipeline"], [class*="workflow"]').filter({ visible: true }).first();
    await expect(pipeline).toBeVisible();
  });

  test('should create new pipeline', async ({ page }) => {

    const newBtn = page.locator('button:has-text("New"), button:has-text("Create")').filter({ visible: true }).first();
    await newBtn.click();
    await expect(page.locator('text=/create.*pipeline|new.*pipeline/i')).toBeVisible();
  });

  test('should show pipeline stages', async ({ page }) => {

    const stage = page.locator('[class*="stage"], text=/stage|step/i').filter({ visible: true }).first();
    await expect(stage).toBeVisible();
  });

  test('should drag to reorder stages', async ({ page }) => {

    const stage = page.locator('[class*="stage"]').filter({ visible: true }).first();
    await stage.dragTo(page.locator('[class*="stage"]').nth(2));
  });

  test('should edit pipeline stage', async ({ page }) => {

    const stage = page.locator('[class*="stage"]').filter({ visible: true }).first();
    await stage.click();
    const editBtn = page.locator('button:has-text("Edit"), button:has-text("Modify")').filter({ visible: true }).first();
    await editBtn.click();
    await expect(page.locator('text=/edit|stage/i')).toBeVisible();
  });

  test('should delete pipeline stage', async ({ page }) => {

    const stage = page.locator('[class*="stage"]').filter({ visible: true }).first();
    await stage.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').filter({ visible: true }).first();
    await deleteBtn.click();
    await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 3000 });
  });

  test('should show pipeline analytics', async ({ page }) => {

    const analytics = page.locator('text=/analytics|metrics|stats/i').filter({ visible: true }).first();
    await expect(analytics).toBeVisible();
  });

  test('should run pipeline manually', async ({ page }) => {

    const runBtn = page.locator('button:has-text("Run"), button:has-text("Execute")').filter({ visible: true }).first();
    await runBtn.click();
    await expect(page.locator('text=/running|executing/i')).toBeVisible({ timeout: 3000 });
  });

  test('should show pipeline run history', async ({ page }) => {

    const historyBtn = page.locator('button:has-text("History"), button:has-text("Runs")').filter({ visible: true }).first();
    await historyBtn.click();
    await expect(page.locator('text=/history|runs|execution/i')).toBeVisible();
  });

  test('should pause pipeline', async ({ page }) => {

    const pauseBtn = page.locator('button:has-text("Pause"), button:has-text("Disable")').filter({ visible: true }).first();
    await pauseBtn.click();
    await expect(page.locator('text=/paused|disabled/i')).toBeVisible({ timeout: 3000 });
  });

  test('should resume pipeline', async ({ page }) => {

    const resumeBtn = page.locator('button:has-text("Resume"), button:has-text("Enable")').filter({ visible: true }).first();
    await resumeBtn.click();
    await expect(page.locator('text=/active|running/i')).toBeVisible({ timeout: 3000 });
  });

  test('should duplicate pipeline', async ({ page }) => {

    const pipeline = page.locator('[class*="pipeline"]').filter({ visible: true }).first();
    await pipeline.hover();
    const duplicateBtn = page.locator('button:has-text("Duplicate"), button:has-text("Copy")').filter({ visible: true }).first();
    await duplicateBtn.click();
    await expect(page.locator('text=/duplicated|copied/i')).toBeVisible({ timeout: 3000 });
  });


});
});

test.describe('Additional Integrations Verification', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
    await page.waitForURL('**/*');
  });

  test('verify calendar and scheduling Google/Outlook connection', async ({ page }) => {
    await page.click('button:has-text("Integrations")');
    await expect(page.locator('text="📅 Calendar & Scheduling"')).toBeVisible();
    await page.click('button:has-text("Connect Google Calendar")');
    await expect(page.locator('text="Google Workspace Connected"')).toBeVisible();
  });

  test('verify email marketing UI', async ({ page }) => {
    await page.click('button:has-text("Marketing")');
    await expect(page.locator('text="Email Marketing"')).toBeVisible();
    await page.fill('input[placeholder="e.g., Send a 10% off coupon to VIP customers"]', 'Promote new arrivals');
    await page.click('button:has-text("Send to VIP Customers")');
    await expect(page.locator('text="Campaign dispatched via Amazon SES/Resend!"')).toBeVisible();
  });

  test('verify shipping labels UI via Orders', async ({ page }) => {
    await page.click('button:has-text("Orders")');
    await expect(page.locator('text="Order Management"')).toBeVisible();
    await expect(page.locator(`text="Order #1024 - Priya's Boutique"`)).toBeVisible();
    await page.click('button:has-text("Buy Shipping Label")');
    await expect(page.locator('text="Label PDF Generated via EasyPost/Shippo. Open Print Dialog..."')).toBeVisible();
  });

  test('verify SMS notifications UI via Orders', async ({ page }) => {
    await page.click('button:has-text("Orders")');
    await expect(page.locator(`text="Order #1025 - Fatima's Cart"`)).toBeVisible();
    await page.click('button:has-text("Notify Ready for Pickup")');
    await expect(page.locator('text="SMS Sent via Twilio: Your order is ready at the cart!"')).toBeVisible();
  });

  test('verify video conferencing generation UI', async ({ page }) => {
    await page.click('button:has-text("Agenda")');
    await expect(page.locator('text="Meetings"')).toBeVisible();
    await page.click('button:has-text("Meetings Schedule New Meeting")');

    // Select dropdown
    const select = page.locator('#scheduler select');
    await select.selectOption({ label: 'Location: Online (Zoom)' });

    await page.click('button:has-text("Save")');
    await expect(page.locator('text="Booking confirmed. Meeting URL auto-generated & injected into email/calendar."')).toBeVisible();
  });

  test('verify global payment gateways UI on checkout', async ({ page }) => {
    // Navigate to pricing and then checkout
    await page.goto('/pricing');
    await page.click('button:has-text("Choose Pro")');
    await expect(page.locator('text="Checkout"')).toBeVisible();

    const select = page.locator('#checkout-screen select');
    await select.selectOption({ label: 'Pix (Mercado Pago)' });

    await page.click('button:has-text("Pay Now")');
    await expect(page.locator('text="Payment Pending Webhook Confirmation..."')).toBeVisible();
  });
});
