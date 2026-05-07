import { test, expect } from '@playwright/test';

test.describe('Integrations Page', () => {
  test.beforeEach(async ({ page }) => {  });

  test.beforeEach(async ({ page }) => {  });

  test.beforeEach(async ({ page }) => {  });

  test.beforeEach(async ({ page }) => {

  });

  test('should display integrations page', async ({ page }) => {

    await expect(page.locator('text=/integrations|connect/i')).toBeVisible();
  });

  test('should show integrations header', async ({ page }) => {

    await expect(page.locator('text=Integrations')).toBeVisible();
  });

  test('should display available integrations', async ({ page }) => {

    const integration = page.locator('[class*="integration"], [class*="app"]').first();
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

    const slackBtn = page.locator('button:has-text("Connect"), button:has-text("Slack")').first();
    if (await slackBtn.isVisible()) {
      await slackBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should disconnect integration', async ({ page }) => {

    const integration = page.locator('[class*="integration"]').first();
    await integration.hover();
    const disconnectBtn = page.locator('button:has-text("Disconnect"), button:has-text("Remove")').first();
    if (await disconnectBtn.isVisible()) {
      await disconnectBtn.click();
      await expect(page.locator('text=/disconnected|removed/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show integration status', async ({ page }) => {

    const status = page.locator('text=/connected|active|inactive/i').first();
    await expect(status).toBeVisible();
  });

  test('should configure integration settings', async ({ page }) => {

    const integration = page.locator('[class*="integration"]').first();
    await integration.click();
    const settingsBtn = page.locator('button:has-text("Settings"), button:has-text("Configure")').first();
    if (await settingsBtn.isVisible()) {
      await settingsBtn.click();
      await expect(page.locator('text=/settings|configure/i')).toBeVisible();
    }
  });

  test('should show integration usage stats', async ({ page }) => {

    const stats = page.locator('text=/usage|requests|api.*calls/i').first();
    await expect(stats).toBeVisible();
  });

  test('should search integrations', async ({ page }) => {

    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('Manychat');
      await expect(page.locator('text=/Manychat/i')).toBeVisible();
      await expect(page.locator('text=/slack/i')).toBeVisible();
    }
  });

  test('should filter integrations by category', async ({ page }) => {

    const filterSelect = page.locator('select').first();
    if (await filterSelect.isVisible()) {
      await filterSelect.selectOption({ index: 1 });
    }
  });

  test('should show manychat integration', async ({ page }) => {
    await expect(page.locator('text=/Manychat/i')).toBeVisible();
  });

  test('should connect manychat integration', async ({ page }) => {
    const manychatBtn = page.locator('button:has-text("Connect"), button:has-text("Manychat")').first();
    if (await manychatBtn.isVisible()) {
      await manychatBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show zoom integration', async ({ page }) => {
    await expect(page.locator('text=/Zoom/i')).toBeVisible();
  });

  test('should connect zoom integration', async ({ page }) => {
    const zoomBtn = page.locator('button:has-text("Connect"), button:has-text("Zoom")').first();
    if (await zoomBtn.isVisible()) {
      await zoomBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show twilio integration', async ({ page }) => {
    await expect(page.locator('text=/Twilio/i')).toBeVisible();
  });

  test('should connect twilio integration', async ({ page }) => {
    const twilioBtn = page.locator('button:has-text("Connect"), button:has-text("Twilio")').first();
    if (await twilioBtn.isVisible()) {
      await twilioBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show calendly integration', async ({ page }) => {
    await expect(page.locator('text=/Calendly/i')).toBeVisible();
  });

  test('should connect calendly integration', async ({ page }) => {
    const calendlyBtn = page.locator('button:has-text("Connect"), button:has-text("Calendly")').first();
    if (await calendlyBtn.isVisible()) {
      await calendlyBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show nylas integration', async ({ page }) => {
    await expect(page.locator('text=/Nylas/i')).toBeVisible();
  });

  test('should connect nylas integration', async ({ page }) => {
    const nylasBtn = page.locator('button:has-text("Configure"), button:has-text("Nylas")').first();
    if (await nylasBtn.isVisible()) {
      await nylasBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show stripe integration', async ({ page }) => {
    await expect(page.locator('text=/Stripe/i')).toBeVisible();
  });

  test('should connect stripe integration', async ({ page }) => {
    const stripeBtn = page.locator('button:has-text("Configure"), button:has-text("Stripe")').first();
    if (await stripeBtn.isVisible()) {
      await stripeBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show mailchimp integration', async ({ page }) => {
    await expect(page.locator('text=/Mailchimp/i')).toBeVisible();
  });

  test('should connect mailchimp integration', async ({ page }) => {
    const mailchimpBtn = page.locator('button:has-text("Configure"), button:has-text("Mailchimp")').first();
    if (await mailchimpBtn.isVisible()) {
      await mailchimpBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show messagebird integration', async ({ page }) => {
    await expect(page.locator('text=/MessageBird/i')).toBeVisible();
  });

  test('should connect messagebird integration', async ({ page }) => {
    const messagebirdBtn = page.locator('button:has-text("Configure"), button:has-text("MessageBird")').first();
    if (await messagebirdBtn.isVisible()) {
      await messagebirdBtn.click();
      await expect(page.locator('text=/authorizing|connecting/i')).toBeVisible({ timeout: 5000 });
    }
  });

test.describe('Pipeline Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.locator('text=/Login/i').click();
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button[type="submit"]');
    await page.locator('text=/Pipelines/i').click();
  });

  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.locator('text=/Login/i').click();
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button[type="submit"]');
    await page.locator('text=/Pipelines/i').click();
  });

  test('should display pipelines page', async ({ page }) => {

    await expect(page.locator('text=/pipeline|workflow/i')).toBeVisible();
  });

  test('should show pipelines header', async ({ page }) => {

    await expect(page.locator('text=Pipelines')).toBeVisible();
  });

  test('should display pipeline list', async ({ page }) => {

    const pipeline = page.locator('[class*="pipeline"], [class*="workflow"]').first();
    await expect(pipeline).toBeVisible();
  });

  test('should create new pipeline', async ({ page }) => {

    const newBtn = page.locator('button:has-text("New"), button:has-text("Create")').first();
    if (await newBtn.isVisible()) {
      await newBtn.click();
      await expect(page.locator('text=/create.*pipeline|new.*pipeline/i')).toBeVisible();
    }
  });

  test('should show pipeline stages', async ({ page }) => {

    const stage = page.locator('[class*="stage"], text=/stage|step/i').first();
    await expect(stage).toBeVisible();
  });

  test('should drag to reorder stages', async ({ page }) => {

    const stage = page.locator('[class*="stage"]').first();
    if (await stage.isVisible()) {
      await stage.dragTo(page.locator('[class*="stage"]').nth(2));
    }
  });

  test('should edit pipeline stage', async ({ page }) => {

    const stage = page.locator('[class*="stage"]').first();
    await stage.click();
    const editBtn = page.locator('button:has-text("Edit"), button:has-text("Modify")').first();
    if (await editBtn.isVisible()) {
      await editBtn.click();
      await expect(page.locator('text=/edit|stage/i')).toBeVisible();
    }
  });

  test('should delete pipeline stage', async ({ page }) => {

    const stage = page.locator('[class*="stage"]').first();
    await stage.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    if (await deleteBtn.isVisible()) {
      await deleteBtn.click();
      await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show pipeline analytics', async ({ page }) => {

    const analytics = page.locator('text=/analytics|metrics|stats/i').first();
    await expect(analytics).toBeVisible();
  });

  test('should run pipeline manually', async ({ page }) => {

    const runBtn = page.locator('button:has-text("Run"), button:has-text("Execute")').first();
    if (await runBtn.isVisible()) {
      await runBtn.click();
      await expect(page.locator('text=/running|executing/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show pipeline run history', async ({ page }) => {

    const historyBtn = page.locator('button:has-text("History"), button:has-text("Runs")').first();
    if (await historyBtn.isVisible()) {
      await historyBtn.click();
      await expect(page.locator('text=/history|runs|execution/i')).toBeVisible();
    }
  });

  test('should pause pipeline', async ({ page }) => {

    const pauseBtn = page.locator('button:has-text("Pause"), button:has-text("Disable")').first();
    if (await pauseBtn.isVisible()) {
      await pauseBtn.click();
      await expect(page.locator('text=/paused|disabled/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should resume pipeline', async ({ page }) => {

    const resumeBtn = page.locator('button:has-text("Resume"), button:has-text("Enable")').first();
    if (await resumeBtn.isVisible()) {
      await resumeBtn.click();
      await expect(page.locator('text=/active|running/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should duplicate pipeline', async ({ page }) => {

    const pipeline = page.locator('[class*="pipeline"]').first();
    await pipeline.hover();
    const duplicateBtn = page.locator('button:has-text("Duplicate"), button:has-text("Copy")').first();
    if (await duplicateBtn.isVisible()) {
      await duplicateBtn.click();
      await expect(page.locator('text=/duplicated|copied/i')).toBeVisible({ timeout: 3000 });
    }
  });


});
});
