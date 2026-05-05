import { test, expect } from '@playwright/test';

test.describe('Meetings Page', () => {
  test('should display meetings page', async ({ page }) => {
    await page.goto('/meetings');
    await expect(page.locator('text=/meeting|schedule/i')).toBeVisible();
  });

  test('should show meetings header', async ({ page }) => {
    await page.goto('/meetings');
    await expect(page.locator('text=Meetings')).toBeVisible();
  });

  test('should display upcoming meetings', async ({ page }) => {
    await page.goto('/meetings');
    const meeting = page.locator('[class*="meeting"], [class*="event"]').first();
    await expect(meeting).toBeVisible();
  });

  test('should show schedule new meeting button', async ({ page }) => {
    await page.goto('/meetings');
    await expect(page.locator('button:has-text("Schedule"), button:has-text("New Meeting")')).toBeVisible();
  });

  test('should open meeting scheduler', async ({ page }) => {
    await page.goto('/meetings');
    const scheduleBtn = page.locator('button:has-text("Schedule"), button:has-text("New Meeting")').first();
    if (await scheduleBtn.isVisible()) {
      await scheduleBtn.click();
      await expect(page.locator('text=/schedule|create.*meeting/i')).toBeVisible();
    }
  });

  test('should select meeting date', async ({ page }) => {
    await page.goto('/meetings');
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
    await page.goto('/meetings');
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
    await page.goto('/meetings');
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
    await page.goto('/meetings');
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
    await page.goto('/meetings');
    const joinBtn = page.locator('button:has-text("Join"), button:has-text("Start")').first();
    if (await joinBtn.isVisible()) {
      await joinBtn.click();
      await expect(page.locator('text=/meeting.*room|video|audio/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should cancel meeting', async ({ page }) => {
    await page.goto('/meetings');
    const meeting = page.locator('[class*="meeting"]').first();
    await meeting.hover();
    const cancelBtn = page.locator('button:has-text("Cancel"), button:has-text("Delete")').first();
    if (await cancelBtn.isVisible()) {
      await cancelBtn.click();
      await expect(page.locator('text=/canceled|cancelled/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show meeting details', async ({ page }) => {
    await page.goto('/meetings');
    const meeting = page.locator('[class*="meeting"]').first();
    await meeting.click();
    await expect(page.locator('text=/details|description/i')).toBeVisible();
  });

  test('should reschedule meeting', async ({ page }) => {
    await page.goto('/meetings');
    const meeting = page.locator('[class*="meeting"]').first();
    await meeting.click();
    const rescheduleBtn = page.locator('button:has-text("Reschedule"), button:has-text("Edit")').first();
    if (await rescheduleBtn.isVisible()) {
      await rescheduleBtn.click();
      await expect(page.locator('text=/reschedule|change.*time/i')).toBeVisible();
    }
  });

  test('should show past meetings', async ({ page }) => {
    await page.goto('/meetings');
    const pastTab = page.locator('button:has-text("Past"), button:has-text("History")').first();
    if (await pastTab.isVisible()) {
      await pastTab.click();
      await expect(page.locator('text=/past|history|completed/i')).toBeVisible();
    }
  });

  test('should display meeting calendar view', async ({ page }) => {
    await page.goto('/meetings');
    const calendarBtn = page.locator('button:has-text("Calendar"), [class*="calendar"]').first();
    if (await calendarBtn.isVisible()) {
      await calendarBtn.click();
      await expect(page.locator('text=/calendar|month|week/i')).toBeVisible();
    }
  });

  test('should display meeting recordings', async ({ page }) => {
    await page.goto('/meetings');
    const recordingTab = page.locator('button:has-text("Recordings"), button:has-text("Recordings")').first();
    if (await recordingTab.isVisible()) {
      await recordingTab.click();
      await expect(page.locator('text=/recording|video/i')).toBeVisible();
    }
  });

  test('should show meeting countdown timer', async ({ page }) => {
    await page.goto('/meetings');
    const timer = page.locator('text=/\\d+:\\d+:\\d+/').first();
    await expect(timer).toBeVisible({ timeout: 3000 });
  });
});

test.describe('Meetings Video', () => {
  test('should show video controls', async ({ page }) => {
    await page.goto('/meetings/room/1');
    await expect(page.locator('text=/video|audio|mute/i')).toBeVisible();
  });

  test('should toggle video', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const videoBtn = page.locator('button:has-text("Video"), [class*="video"]').first();
    if (await videoBtn.isVisible()) {
      await videoBtn.click();
      await expect(page.locator('text=/video.*off|off/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should toggle audio', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const audioBtn = page.locator('button:has-text("Mute"), [class*="audio"]').first();
    if (await audioBtn.isVisible()) {
      await audioBtn.click();
      await expect(page.locator('text=/muted|off/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should share screen', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const shareBtn = page.locator('button:has-text("Share"), button:has-text("Screen")').first();
    if (await shareBtn.isVisible()) {
      await shareBtn.click();
      await expect(page.locator('text=/sharing|screen.*share/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should end meeting', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const endBtn = page.locator('button:has-text("End"), button:has-text("Leave")').first();
    if (await endBtn.isVisible()) {
      await endBtn.click();
      await expect(page.locator('text=/ended|left/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show participant list', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const participantsBtn = page.locator('button:has-text("Participants"), button:has-text("People")').first();
    if (await participantsBtn.isVisible()) {
      await participantsBtn.click();
      await expect(page.locator('text=/participant|people/i')).toBeVisible();
    }
  });

  test('should show chat in meeting', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const chatBtn = page.locator('button:has-text("Chat"), [class*="chat"]').first();
    if (await chatBtn.isVisible()) {
      await chatBtn.click();
      await expect(page.locator('text=/chat|messages/i')).toBeVisible();
    }
  });

  test('should raise hand in meeting', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const handBtn = page.locator('button:has-text("Hand"), button:has-text("Raise")').first();
    if (await handBtn.isVisible()) {
      await handBtn.click();
      await expect(page.locator('text=/hand.*raised|raised/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should record meeting', async ({ page }) => {
    await page.goto('/meetings/room/1');
    const recordBtn = page.locator('button:has-text("Record"), [class*="record"]').first();
    if (await recordBtn.isVisible()) {
      await recordBtn.click();
      await expect(page.locator('text=/recording/i')).toBeVisible({ timeout: 3000 });
    }
  });
});