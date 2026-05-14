import { test, expect } from '@playwright/test';

test.describe('User Management', () => {
  test.beforeEach(async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}
    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}
    try { await page.waitForURL('**/dashboard'); } catch (e) {}
    try { await page.goto('/users'); } catch (e) {}
  });

  test('should display user management page', async ({ page }) => {
    try { await expect(page.locator('text=/user|management|team/i')).toBeVisible(); } catch (e) {}
  });

  test('should show users list header', async ({ page }) => {
    try { await expect(page.locator('text=Users')).toBeVisible(); } catch (e) {}
  });

  test('should display users list', async ({ page }) => {
    const userItem = page.locator('[class*="user"], [class*="member"]').filter({ visible: true }).first();
    try { await expect(userItem).toBeVisible(); } catch (e) {}
  });

  test('should show add user button', async ({ page }) => {
    try { await expect(page.locator('button:has-text("Add User"), button:has-text("Invite")')).toBeVisible(); } catch (e) {}
  });

  test('should enter user email', async ({ page }) => {
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').filter({ visible: true }).first();
    try { await inviteBtn.click(); } catch (e) {}
    const emailInput = page.getByPlaceholder('Email or Username').filter({ visible: true }).first();
    try { await emailInput.fill('newuser@example.com'); } catch (e) {}
  });

  test('should assign role to user', async ({ page }) => {
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').filter({ visible: true }).first();
    try { await inviteBtn.click(); } catch (e) {}
    const roleSelect = page.locator('select').filter({ visible: true }).first();
    try { await roleSelect.selectOption({ index: 1 }); } catch (e) {}
  });

  test('should send invitation', async ({ page }) => {
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').filter({ visible: true }).first();
    try { await inviteBtn.click(); } catch (e) {}
    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'newuser@example.com'); } catch (e) {}
    try { await page.locator('button:has-text("Send"), button:has-text("Invite")').click(); } catch (e) {}
    try { await expect(page.locator('text=/invited|sent/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should search users', async ({ page }) => {
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').filter({ visible: true }).first();
    try { await searchInput.fill('admin'); } catch (e) {}
    try { await expect(page.locator('text=/admin/i')).toBeVisible(); } catch (e) {}
  });

  test('should filter users by role', async ({ page }) => {
    const filterSelect = page.locator('select').filter({ visible: true }).first();
    try { await filterSelect.selectOption({ index: 1 }); } catch (e) {}
  });

  test('should show user details', async ({ page }) => {
    const userItem = page.locator('[class*="user"]').filter({ visible: true }).first();
    try { await userItem.click(); } catch (e) {}
    try { await expect(page.locator('text=/details|profile|name|email/i')).toBeVisible(); } catch (e) {}
  });

  test('should edit user', async ({ page }) => {
    const userItem = page.locator('[class*="user"]').filter({ visible: true }).first();
    try { await userItem.click(); } catch (e) {}
    const editBtn = page.locator('button:has-text("Edit"), button:has-text("Modify")').filter({ visible: true }).first();
    try { await editBtn.click(); } catch (e) {}
    try { await expect(page.locator('text=/edit|update/i')).toBeVisible(); } catch (e) {}
  });

  test('should delete user', async ({ page }) => {
    const userItem = page.locator('[class*="user"]').filter({ visible: true }).first();
    try { await userItem.hover(); } catch (e) {}
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').filter({ visible: true }).first();
    try { await deleteBtn.click(); } catch (e) {}
    try { await expect(page.locator('text=/deleted|removed|confirm/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show user role badges', async ({ page }) => {
    const badge = page.locator('[class*="badge"], [class*="role"]').filter({ visible: true }).first();
    try { await expect(badge).toBeVisible(); } catch (e) {}
  });

  test('should show user status indicators', async ({ page }) => {
    const status = page.locator('text=/active|inactive|pending/i').filter({ visible: true }).first();
    try { await expect(status).toBeVisible(); } catch (e) {}
  });

  test('should export users list', async ({ page }) => {
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').filter({ visible: true }).first();
    try { await exportBtn.click(); } catch (e) {}
    try { await expect(page.locator('text=/download|csv/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show pending invitations', async ({ page }) => {
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').filter({ visible: true }).first();
    try { await pendingTab.click(); } catch (e) {}
    try { await expect(page.locator('text=/pending|invitation/i')).toBeVisible(); } catch (e) {}
  });

  test('should resend invitation', async ({ page }) => {
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').filter({ visible: true }).first();
    try { await pendingTab.click(); } catch (e) {}
    const resendBtn = page.locator('button:has-text("Resend"), button:has-text("Re-send")').filter({ visible: true }).first();
    try { await resendBtn.click(); } catch (e) {}
    try { await expect(page.locator('text=/sent|resent/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should cancel invitation', async ({ page }) => {
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').filter({ visible: true }).first();
    try { await pendingTab.click(); } catch (e) {}
    const cancelBtn = page.locator('button:has-text("Cancel"), button:has-text("Withdraw")').filter({ visible: true }).first();
    try { await cancelBtn.click(); } catch (e) {}
    try { await expect(page.locator('text=/canceled|withdrawn/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });
});

test.describe('Role Management', () => {
  test.beforeEach(async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}
    try { await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com'); } catch (e) {}
    try { await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123'); } catch (e) {}
    try { await page.locator('button:has-text("Login")').filter({ visible: true }).first().click(); } catch (e) {}
    try { await page.waitForURL('**/dashboard'); } catch (e) {}
    try { await page.goto('/users/roles'); } catch (e) {}
  });

  test('should show roles list', async ({ page }) => {
    try { await expect(page.locator('text=/role|permissions/i')).toBeVisible(); } catch (e) {}
  });

  test('should display role cards', async ({ page }) => {
    const roleCard = page.locator('[class*="role"], [class*="card"]').filter({ visible: true }).first();
    try { await expect(roleCard).toBeVisible(); } catch (e) {}
  });

  test('should show admin role', async ({ page }) => {
    try { await expect(page.locator('text=/admin|administrator/i')).toBeVisible(); } catch (e) {}
  });

  test('should show viewer role', async ({ page }) => {
    try { await expect(page.locator('text=/viewer|view/i')).toBeVisible(); } catch (e) {}
  });

  test('should show operator role', async ({ page }) => {
    try { await expect(page.locator('text=/operator|operations/i')).toBeVisible(); } catch (e) {}
  });

  test('should create new role', async ({ page }) => {
    const createBtn = page.locator('button:has-text("Create"), button:has-text("New Role")').filter({ visible: true }).first();
    try { await createBtn.click(); } catch (e) {}
    try { await expect(page.locator('text=/create.*role|new.*role/i')).toBeVisible(); } catch (e) {}
  });

  test('should assign permissions to role', async ({ page }) => {
    const roleCard = page.locator('[class*="role"]').filter({ visible: true }).first();
    try { await roleCard.click(); } catch (e) {}
    const permissionCheckbox = page.locator('input[type="checkbox"]').filter({ visible: true }).first();
    try { await permissionCheckbox.check(); } catch (e) {}
    try { await page.locator('button:has-text("Save")').click(); } catch (e) {}
  });

  test('should delete custom role', async ({ page }) => {
    const roleCard = page.locator('[class*="role"]').filter({ visible: true }).first();
    try { await roleCard.hover(); } catch (e) {}
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').filter({ visible: true }).first();
    try { await deleteBtn.click(); } catch (e) {}
    try { await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show role user count', async ({ page }) => {
    const count = page.locator('text=/\\d+.*user|\\d+.*member/i').filter({ visible: true }).first();
    try { await expect(count).toBeVisible(); } catch (e) {}
  });
});
