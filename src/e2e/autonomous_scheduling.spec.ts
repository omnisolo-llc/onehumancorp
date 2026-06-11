import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('autonomous_scheduling');

test.describe('Autonomous Work Scheduling & Routing', () => {
  test('Carlos the Handyman checks his daily run and completes a job early', async ({ page }) => {
    // 1. Navigate to the Field Ops Daily Run page
    await page.goto('/field-ops/jobs');

    // 2. Verify we are on the Daily Route screen
    await expect(page.locator('h1', { hasText: "Today's Route" }).first()).toBeVisible({ timeout: 15000 });

    // 3. Verify the first mock job is visible (Alice Smith - Plumbing Repair)
    const job1Card = page.locator('.bg-white.rounded-xl.shadow-sm').first();
    await expect(job1Card).toContainText('Alice Smith');
    await expect(job1Card).toContainText('Plumbing Repair');

    // It should initially be in Scheduled state, so the button should say "Heading to Job"
    const headingBtn = job1Card.getByRole('button', { name: 'Heading to Job' });
    await expect(headingBtn).toBeVisible();

    // 4. Carlos taps "Heading to Job"
    await headingBtn.click();

    // Status should change to EN-ROUTE and button should now be "Start Work"
    await expect(job1Card.locator('span.bg-purple-100')).toContainText('EN-ROUTE');
    const startWorkBtn = job1Card.getByRole('button', { name: 'Start Work' });
    await expect(startWorkBtn).toBeVisible();

    // 5. Carlos arrives and taps "Start Work"
    await startWorkBtn.click();

    // Status should change to IN-PROGRESS and button should now be "Job Done"
    await expect(job1Card.locator('span.bg-yellow-100')).toContainText('IN-PROGRESS');
    const jobDoneBtn = job1Card.getByRole('button', { name: 'Job Done' });
    await expect(jobDoneBtn).toBeVisible();

    // 6. Carlos finishes the job and taps "Job Done"
    await jobDoneBtn.click();

    // Status should change to COMPLETED
    await expect(job1Card.locator('span.bg-green-100')).toContainText('COMPLETED');

    // 7. Verify the Operations Agent intervenes because the job was finished early (mocked behavior)
    // The agent suggestion card should appear at the top
    const agentSuggestionCard = page.locator('.bg-blue-50.border-blue-200');
    await expect(agentSuggestionCard).toBeVisible();
    await expect(agentSuggestionCard).toContainText('You finished early!');
    await expect(agentSuggestionCard).toContainText('Should I text the next client');

    // Verify interaction buttons on the suggestion
    await expect(agentSuggestionCard.getByRole('button', { name: 'Yes, text them' })).toBeVisible();
    await expect(agentSuggestionCard.getByRole('button', { name: 'No, stick to schedule' })).toBeVisible();

    // 8. Dismiss the suggestion
    await agentSuggestionCard.getByRole('button', { name: 'No, stick to schedule' }).click();
    await expect(agentSuggestionCard).not.toBeVisible();
  });
});
