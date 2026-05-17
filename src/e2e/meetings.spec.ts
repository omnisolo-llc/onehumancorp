import { test, expect } from '@playwright/test';

test.describe('Meetings Page', () => {
  test('shows upcoming meeting and scheduler', async ({ page }) => {
    await page.goto('/meetings');
    await expect(page.locator('#meetings-screen')).toBeVisible();
    await expect(page.getByRole('button', { name: /Meetings Schedule New Meeting/ })).toBeVisible();
    await expect(page.getByText('Team Sync - 14:00')).toBeVisible();

    await page.getByRole('button', { name: /Meetings Schedule New Meeting/ }).click();
    await expect(page.getByRole('heading', { name: 'Plan Create' })).toBeVisible();
    await page.getByPlaceholder('Meeting Title').fill('Planning Call');
    await page.locator('#scheduler input[type="date"]').fill('2026-05-18');
    await page.locator('#scheduler input[type="time"]').fill('14:30');
    await page.getByPlaceholder('Participant Email').fill('team@example.com');
  });

  test('opens meeting room controls', async ({ page }) => {
    await page.goto('/meetings');
    await page.getByRole('button', { name: 'Join Start' }).click();

    await expect(page.locator('#meeting-room-screen')).toBeVisible();
    await page.getByRole('button', { name: 'Camera' }).click();
    await expect(page.locator('#status-text')).toContainText('Video Off');
    await page.getByRole('button', { name: 'Record' }).click();
    await expect(page.locator('#status-text')).toContainText('Recording');
  });
});
