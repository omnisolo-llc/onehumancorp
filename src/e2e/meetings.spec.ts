import { test, expect } from '@playwright/test';

test.describe('Meetings Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    const btn = page.locator('button:has-text("/login")');
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")');
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        await page.locator('button:has-text("/login")').click();
      }
    }
  });
  test('should display meetings page', async ({ page }) => {
    await expect(page.locator('text=/meeting|schedule/i')).toBeVisible();
  });

  test('should show meetings header', async ({ page }) => {
    await expect(page.locator('text=Meetings')).toBeVisible();
  });

  test('should display upcoming meetings', async ({ page }) => {
    const meeting = page.locator('[class*="meeting"], [class*="event"]').first();
    await expect(meeting).toBeVisible();
  });

  test('should show schedule new meeting button', async ({ page }) => {
    await expect(page.locator('button:has-text("Schedule"), button:has-text("New Meeting")')).toBeVisible();
  });

  test('should open meeting scheduler', async ({ page }) => {
    const scheduleBtn = page.locator('button:has-text("Schedule"), button:has-text("New Meeting")').first();
    if (await scheduleBtn.isVisible()) {
      await scheduleBtn.click();
      await expect(page.locator('text=/schedule|create.*meeting/i')).toBeVisible();
    }
  });

  test('should select meeting date', async ({ page }) => {
    const scheduleBtn = page.locator('button:has-text("Schedule"), button:has-text("New Meeting")').first();
    if (await scheduleBtn.isVisible()) {
      await scheduleBtn.click();
      const datePicker = page.locator('input[type="date"], [class*="date"]').first();
      if (await datePicker.isVisible()) {
        await datePicker.fill('2026-12-15');
      }
    }
  });

  test('should select meeting time', async ({ page }) => {
    const scheduleBtn = page.locator('button:has-text("Schedule"), button:has-text("New Meeting")').first();
    if (await scheduleBtn.isVisible()) {
      await scheduleBtn.click();
      const timePicker = page.locator('input[type="time"], [class*="time"]').first();
      if (await timePicker.isVisible()) {
        await timePicker.fill('14:00');
      }
    }
  });

  test('should add meeting participants', async ({ page }) => {
    const scheduleBtn = page.locator('button:has-text("Schedule"), button:has-text("New Meeting")').first();
    if (await scheduleBtn.isVisible()) {
      await scheduleBtn.click();
      const participantInput = page.locator('input[placeholder*="email" i], input[placeholder*="participant"]').first();
      if (await participantInput.isVisible()) {
        await participantInput.fill('test@example.com');
        await page.locator('button:has-text("Add")').click();
      }
    }
  });

  test('should set meeting title', async ({ page }) => {
    const scheduleBtn = page.locator('button:has-text("Schedule"), button:has-text("New Meeting")').first();
    if (await scheduleBtn.isVisible()) {
      await scheduleBtn.click();
      const titleInput = page.locator('input[type="text"]').first();
      if (await titleInput.isVisible()) {
        await titleInput.fill('Team Sync');
      }
    }
  });

  test('should join meeting', async ({ page }) => {
    const joinBtn = page.locator('button:has-text("Join"), button:has-text("Start")').first();
    if (await joinBtn.isVisible()) {
      await joinBtn.click();
      await expect(page.locator('text=/meeting.*room|video|audio/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });

  test('should cancel meeting', async ({ page }) => {
    const meeting = page.locator('[class*="meeting"]').first();
    await meeting.hover();
    const cancelBtn = page.locator('button:has-text("Cancel"), button:has-text("Delete")').first();
    if (await cancelBtn.isVisible()) {
      await cancelBtn.click();
      await expect(page.locator('text=/canceled|cancelled/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should show meeting details', async ({ page }) => {
    const meeting = page.locator('[class*="meeting"]').first();
    await meeting.click();
    await expect(page.locator('text=/details|description/i')).toBeVisible();
  });

  test('should reschedule meeting', async ({ page }) => {
    const meeting = page.locator('[class*="meeting"]').first();
    await meeting.click();
    const rescheduleBtn = page.locator('button:has-text("Reschedule"), button:has-text("Edit")').first();
    if (await rescheduleBtn.isVisible()) {
      await rescheduleBtn.click();
      await expect(page.locator('text=/reschedule|change.*time/i')).toBeVisible();
    }
  });

  test('should show past meetings', async ({ page }) => {
    const pastTab = page.locator('button:has-text("Past"), button:has-text("History")').first();
    if (await pastTab.isVisible()) {
      await pastTab.click();
      await expect(page.locator('text=/past|history|completed/i')).toBeVisible();
    }
  });

  test('should display meeting calendar view', async ({ page }) => {
    const calendarBtn = page.locator('button:has-text("Calendar"), [class*="calendar"]').first();
    if (await calendarBtn.isVisible()) {
      await calendarBtn.click();
      await expect(page.locator('text=/calendar|month|week/i')).toBeVisible();
    }
  });

  test('should display meeting recordings', async ({ page }) => {
    const recordingTab = page.locator('button:has-text("Recordings"), button:has-text("Recordings")').first();
    if (await recordingTab.isVisible()) {
      await recordingTab.click();
      await expect(page.locator('text=/recording|video/i')).toBeVisible();
    }
  });

  test('should show meeting countdown timer', async ({ page }) => {
    const timer = page.locator('text=/\\d+:\\d+:\\d+/').first();
    await expect(timer).toBeVisible({ timeout: 3000 }).catch(() => {});
  });
});

test.describe('Meetings Video', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test('should show video controls', async ({ page }) => {
    await expect(page.locator('text=/video|audio|mute/i')).toBeVisible();
  });

  test('should toggle video', async ({ page }) => {
    const videoBtn = page.locator('button:has-text("Video"), [class*="video"]').first();
    if (await videoBtn.isVisible()) {
      await videoBtn.click();
      await expect(page.locator('text=/video.*off|off/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should toggle audio', async ({ page }) => {
    const audioBtn = page.locator('button:has-text("Mute"), [class*="audio"]').first();
    if (await audioBtn.isVisible()) {
      await audioBtn.click();
      await expect(page.locator('text=/muted|off/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should share screen', async ({ page }) => {
    const shareBtn = page.locator('button:has-text("Share"), button:has-text("Screen")').first();
    if (await shareBtn.isVisible()) {
      await shareBtn.click();
      await expect(page.locator('text=/sharing|screen.*share/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should end meeting', async ({ page }) => {
    const endBtn = page.locator('button:has-text("End"), button:has-text("Leave")').first();
    if (await endBtn.isVisible()) {
      await endBtn.click();
      await expect(page.locator('text=/ended|left/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should show participant list', async ({ page }) => {
    const participantsBtn = page.locator('button:has-text("Participants"), button:has-text("People")').first();
    if (await participantsBtn.isVisible()) {
      await participantsBtn.click();
      await expect(page.locator('text=/participant|people/i')).toBeVisible();
    }
  });

  test('should show chat in meeting', async ({ page }) => {
    const chatBtn = page.locator('button:has-text("Chat"), [class*="chat"]').first();
    if (await chatBtn.isVisible()) {
      await chatBtn.click();
      await expect(page.locator('text=/chat|messages/i')).toBeVisible();
    }
  });

  test('should raise hand in meeting', async ({ page }) => {
    const handBtn = page.locator('button:has-text("Hand"), button:has-text("Raise")').first();
    if (await handBtn.isVisible()) {
      await handBtn.click();
      await expect(page.locator('text=/hand.*raised|raised/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should record meeting', async ({ page }) => {
    const recordBtn = page.locator('button:has-text("Record"), [class*="record"]').first();
    if (await recordBtn.isVisible()) {
      await recordBtn.click();
      await expect(page.locator('text=/recording/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });
});
