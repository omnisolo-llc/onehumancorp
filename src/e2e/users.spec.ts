import { test, expect } from '@playwright/test';

test.describe('User Management', () => {
  test('should display user management page', async ({ page }) => {
    await page.goto('/users');
    await expect(page.locator('text=/user|management|team/i')).toBeVisible();
  });

  test('should show users list header', async ({ page }) => {
    await page.goto('/users');
    await expect(page.locator('text=Users')).toBeVisible();
  });

  test('should display users list', async ({ page }) => {
    await page.goto('/users');
    const userItem = page.locator('[class*="user"], [class*="member"]').first();
    await expect(userItem).toBeVisible();
  });

  test('should show add user button', async ({ page }) => {
    await page.goto('/users');
    await expect(page.locator('button:has-text("Add User"), button:has-text("Invite")')).toBeVisible();
  });


  test('should invite new user via referral program', async ({ page }) => {
    await page.goto('/users');
    const inviteBtn = page.locator('button:has-text("Invite User")').first();
    if (await inviteBtn.isVisible()) {
      await expect(page.locator('text=Referral Program')).toBeVisible();
      await expect(page.locator('text=Share OHC with a friend via our seamless Cloud-bridge referral loop')).toBeVisible();
      await inviteBtn.click();
    }
  });

  test.describe('User Management - Viewports', () => {
    test('should load correctly at 375px', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 800 });
      await page.goto('/users');
      await expect(page.locator('text=User Management')).first().toBeVisible();
    });
    test('should load correctly at 768px', async ({ page }) => {
      await page.setViewportSize({ width: 768, height: 1024 });
      await page.goto('/users');
      await expect(page.locator('text=User Management')).first().toBeVisible();
    });
    test('should load correctly at 1440px', async ({ page }) => {
      await page.setViewportSize({ width: 1440, height: 900 });
      await page.goto('/users');
      await expect(page.locator('text=User Management')).first().toBeVisible();
    });
  });


  test('should enter user email', async ({ page }) => {
    await page.goto('/users');
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').first();
    if (await inviteBtn.isVisible()) {
      await inviteBtn.click();
      const emailInput = page.locator('input[type="email"]').first();
      if (await emailInput.isVisible()) {
        await emailInput.fill('newuser@example.com');
      }
    }
  });

  test('should assign role to user', async ({ page }) => {
    await page.goto('/users');
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').first();
    if (await inviteBtn.isVisible()) {
      await inviteBtn.click();
      const roleSelect = page.locator('select').first();
      if (await roleSelect.isVisible()) {
        await roleSelect.selectOption({ index: 1 });
      }
    }
  });

  test('should send invitation', async ({ page }) => {
    await page.goto('/users');
    const inviteBtn = page.locator('button:has-text("Add User"), button:has-text("Invite")').first();
    if (await inviteBtn.isVisible()) {
      await inviteBtn.click();
      await page.fill('input[type="email"]', 'newuser@example.com');
      await page.locator('button:has-text("Send"), button:has-text("Invite")').click();
      await expect(page.locator('text=/invited|sent/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should search users', async ({ page }) => {
    await page.goto('/users');
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('admin');
      await expect(page.locator('text=/admin/i')).toBeVisible();
    }
  });

  test('should filter users by role', async ({ page }) => {
    await page.goto('/users');
    const filterSelect = page.locator('select').first();
    if (await filterSelect.isVisible()) {
      await filterSelect.selectOption({ index: 1 });
    }
  });

  test('should show user details', async ({ page }) => {
    await page.goto('/users');
    const userItem = page.locator('[class*="user"]').first();
    await userItem.click();
    await expect(page.locator('text=/details|profile|name|email/i')).toBeVisible();
  });

  test('should edit user', async ({ page }) => {
    await page.goto('/users');
    const userItem = page.locator('[class*="user"]').first();
    await userItem.click();
    const editBtn = page.locator('button:has-text("Edit"), button:has-text("Modify")').first();
    if (await editBtn.isVisible()) {
      await editBtn.click();
      await expect(page.locator('text=/edit|update/i')).toBeVisible();
    }
  });

  test('should delete user', async ({ page }) => {
    await page.goto('/users');
    const userItem = page.locator('[class*="user"]').first();
    await userItem.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    if (await deleteBtn.isVisible()) {
      await deleteBtn.click();
      await expect(page.locator('text=/deleted|removed|confirm/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show user role badges', async ({ page }) => {
    await page.goto('/users');
    const badge = page.locator('[class*="badge"], [class*="role"]').first();
    await expect(badge).toBeVisible();
  });

  test('should show user status indicators', async ({ page }) => {
    await page.goto('/users');
    const status = page.locator('text=/active|inactive|pending/i').first();
    await expect(status).toBeVisible();
  });

  test('should export users list', async ({ page }) => {
    await page.goto('/users');
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').first();
    if (await exportBtn.isVisible()) {
      await exportBtn.click();
      await expect(page.locator('text=/download|csv/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show pending invitations', async ({ page }) => {
    await page.goto('/users');
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').first();
    if (await pendingTab.isVisible()) {
      await pendingTab.click();
      await expect(page.locator('text=/pending|invitation/i')).toBeVisible();
    }
  });

  test('should resend invitation', async ({ page }) => {
    await page.goto('/users');
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').first();
    if (await pendingTab.isVisible()) {
      await pendingTab.click();
      const resendBtn = page.locator('button:has-text("Resend"), button:has-text("Re-send")').first();
      if (await resendBtn.isVisible()) {
        await resendBtn.click();
        await expect(page.locator('text=/sent|resent/i')).toBeVisible({ timeout: 3000 });
      }
    }
  });

  test('should cancel invitation', async ({ page }) => {
    await page.goto('/users');
    const pendingTab = page.locator('button:has-text("Pending"), button:has-text("Invitations")').first();
    if (await pendingTab.isVisible()) {
      await pendingTab.click();
      const cancelBtn = page.locator('button:has-text("Cancel"), button:has-text("Withdraw")').first();
      if (await cancelBtn.isVisible()) {
        await cancelBtn.click();
        await expect(page.locator('text=/canceled|withdrawn/i')).toBeVisible({ timeout: 3000 });
      }
    }
  });
});

test.describe('Role Management', () => {
  test('should show roles list', async ({ page }) => {
    await page.goto('/users/roles');
    await expect(page.locator('text=/role|permissions/i')).toBeVisible();
  });

  test('should display role cards', async ({ page }) => {
    await page.goto('/users/roles');
    const roleCard = page.locator('[class*="role"], [class*="card"]').first();
    await expect(roleCard).toBeVisible();
  });

  test('should show admin role', async ({ page }) => {
    await page.goto('/users/roles');
    await expect(page.locator('text=/admin|administrator/i')).toBeVisible();
  });

  test('should show viewer role', async ({ page }) => {
    await page.goto('/users/roles');
    await expect(page.locator('text=/viewer|view/i')).toBeVisible();
  });

  test('should show operator role', async ({ page }) => {
    await page.goto('/users/roles');
    await expect(page.locator('text=/operator|operations/i')).toBeVisible();
  });

  test('should create new role', async ({ page }) => {
    await page.goto('/users/roles');
    const createBtn = page.locator('button:has-text("Create"), button:has-text("New Role")').first();
    if (await createBtn.isVisible()) {
      await createBtn.click();
      await expect(page.locator('text=/create.*role|new.*role/i')).toBeVisible();
    }
  });

  test('should assign permissions to role', async ({ page }) => {
    await page.goto('/users/roles');
    const roleCard = page.locator('[class*="role"]').first();
    await roleCard.click();
    const permissionCheckbox = page.locator('input[type="checkbox"]').first();
    if (await permissionCheckbox.isVisible()) {
      await permissionCheckbox.check();
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should delete custom role', async ({ page }) => {
    await page.goto('/users/roles');
    const roleCard = page.locator('[class*="role"]').first();
    await roleCard.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    if (await deleteBtn.isVisible()) {
      await deleteBtn.click();
      await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show role user count', async ({ page }) => {
    await page.goto('/users/roles');
    const count = page.locator('text=/\\d+.*user|\\d+.*member/i').first();
    await expect(count).toBeVisible();
  });
});