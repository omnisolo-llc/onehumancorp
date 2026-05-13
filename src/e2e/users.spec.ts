import { test, expect } from '@playwright/test';

test.describe('User Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');
    await page.goto('/users');
  });

  test('should display user management page', async ({ page }) => {
    await expect(page.locator('text=/user|management|team/i')).toBeVisible();
  });

  test('should show users list header', async ({ page }) => {
    await expect(page.locator('text=Users')).toBeVisible();
  });

  test('should display users list', async ({ page }) => {
    const userItem = page.locator('[class*="user"], [class*="member"]').first();
    await expect(userItem).toBeVisible();
  });

  test('should show add user button', async ({ page }) => {
    await expect(page.locator('button:has-text("Add User"), button:has-text("Invite")')).toBeVisible();
  });

  test('should enter user email', async ({ page }) => {
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').first();
    await inviteBtn.click();
    const emailInput = page.getByPlaceholder('Email or Username').first().first();
    await emailInput.fill('newuser@example.com');
  });

  test('should assign role to user', async ({ page }) => {
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').first();
    await inviteBtn.click();
    const roleSelect = page.locator('select').first();
    await roleSelect.selectOption({ index: 1 });
  });

  test('should send invitation', async ({ page }) => {
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').first();
    await inviteBtn.click();
    await page.getByPlaceholder('Email or Username').first().fill( 'newuser@example.com');
    await page.locator('button:has-text("Send"), button:has-text("Invite")').click();
    await expect(page.locator('text=/invited|sent/i')).toBeVisible({ timeout: 3000 });
  });

  test('should search users', async ({ page }) => {
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    await searchInput.fill('admin');
    await expect(page.locator('text=/admin/i')).toBeVisible();
  });

  test('should filter users by role', async ({ page }) => {
    const filterSelect = page.locator('select').first();
    await filterSelect.selectOption({ index: 1 });
  });

  test('should show user details', async ({ page }) => {
    const userItem = page.locator('[class*="user"]').first();
    await userItem.click();
    await expect(page.locator('text=/details|profile|name|email/i')).toBeVisible();
  });

  test('should edit user', async ({ page }) => {
    const userItem = page.locator('[class*="user"]').first();
    await userItem.click();
    const editBtn = page.locator('button:has-text("Edit"), button:has-text("Modify")').first();
    await editBtn.click();
    await expect(page.locator('text=/edit|update/i')).toBeVisible();
  });

  test('should delete user', async ({ page }) => {
    const userItem = page.locator('[class*="user"]').first();
    await userItem.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    await deleteBtn.click();
    await expect(page.locator('text=/deleted|removed|confirm/i')).toBeVisible({ timeout: 3000 });
  });

  test('should show user role badges', async ({ page }) => {
    const badge = page.locator('[class*="badge"], [class*="role"]').first();
    await expect(badge).toBeVisible();
  });

  test('should show user status indicators', async ({ page }) => {
    const status = page.locator('text=/active|inactive|pending/i').first();
    await expect(status).toBeVisible();
  });

  test('should export users list', async ({ page }) => {
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').first();
    await exportBtn.click();
    await expect(page.locator('text=/download|csv/i')).toBeVisible({ timeout: 3000 });
  });

  test('should show pending invitations', async ({ page }) => {
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').first();
    await pendingTab.click();
    await expect(page.locator('text=/pending|invitation/i')).toBeVisible();
  });

  test('should resend invitation', async ({ page }) => {
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').first();
    await pendingTab.click();
    const resendBtn = page.locator('button:has-text("Resend"), button:has-text("Re-send")').first();
    await resendBtn.click();
    await expect(page.locator('text=/sent|resent/i')).toBeVisible({ timeout: 3000 });
  });

  test('should cancel invitation', async ({ page }) => {
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').first();
    await pendingTab.click();
    const cancelBtn = page.locator('button:has-text("Cancel"), button:has-text("Withdraw")').first();
    await cancelBtn.click();
    await expect(page.locator('text=/canceled|withdrawn/i')).toBeVisible({ timeout: 3000 });
  });
});

test.describe('Role Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');
    await page.goto('/users/roles');
  });

  test('should show roles list', async ({ page }) => {
    await expect(page.locator('text=/role|permissions/i')).toBeVisible();
  });

  test('should display role cards', async ({ page }) => {
    const roleCard = page.locator('[class*="role"], [class*="card"]').first();
    await expect(roleCard).toBeVisible();
  });

  test('should show admin role', async ({ page }) => {
    await expect(page.locator('text=/admin|administrator/i')).toBeVisible();
  });

  test('should show viewer role', async ({ page }) => {
    await expect(page.locator('text=/viewer|view/i')).toBeVisible();
  });

  test('should show operator role', async ({ page }) => {
    await expect(page.locator('text=/operator|operations/i')).toBeVisible();
  });

  test('should create new role', async ({ page }) => {
    const createBtn = page.locator('button:has-text("Create"), button:has-text("New Role")').first();
    await createBtn.click();
    await expect(page.locator('text=/create.*role|new.*role/i')).toBeVisible();
  });

  test('should assign permissions to role', async ({ page }) => {
    const roleCard = page.locator('[class*="role"]').first();
    await roleCard.click();
    const permissionCheckbox = page.locator('input[type="checkbox"]').first();
    await permissionCheckbox.check();
    await page.locator('button:has-text("Save")').click();
  });

  test('should delete custom role', async ({ page }) => {
    const roleCard = page.locator('[class*="role"]').first();
    await roleCard.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    await deleteBtn.click();
    await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 3000 });
  });

  test('should show role user count', async ({ page }) => {
    const count = page.locator('text=/\\d+.*user|\\d+.*member/i').first();
    await expect(count).toBeVisible();
  });
});
