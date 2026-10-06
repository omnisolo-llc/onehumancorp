import { test, expect, readTeamPage, sendTeamRequest, assertPendingRequests, recordTeamApproval,
  assertRecordedApproval, assertForeignRequestUnchanged, approvalNotice } from './support/team_fixture';

const departments = ['The Manager', 'The Promoter', 'The Salesperson', 'The Ambassador', 'The Accountant', 'The Protector', 'The Advisor'];

test.describe('Team Departments Page', () => {
  test('department controls reflect persisted owner approvals before and after a recorded decision', async ({ page, teamOwner }) => {
    const card = (name: string) => page.getByRole('button').filter({ has: page.getByRole('heading', { name, exact: true }) });
    await readTeamPage(page, teamOwner, [], () => page.goto('/team'));
    for (const name of departments) {
      await expect(card(name)).toBeVisible();
      await expect(card(name)).toBeDisabled();
      await expect(card(name)).toHaveAttribute('aria-disabled', 'true');
      await expect(card(name)).toContainText('No pending approvals');
    }
    await expect(page.getByText('Active and running', { exact: true })).toHaveCount(0);
    await readTeamPage(page, teamOwner, [], () => page.getByRole('button', { name: 'Team Chat', exact: true }).click());
    const request = await sendTeamRequest(page, teamOwner, `Review operations department ${teamOwner.tenantId}`);
    await assertPendingRequests(page, teamOwner, [request]);
    await readTeamPage(page, teamOwner, [request], () => page.getByRole('link', { name: 'Back to Team', exact: true }).click());
    await expect(card('The Manager')).toBeEnabled();
    await expect(card('The Manager')).toHaveAttribute('aria-disabled', 'false');
    await expect(card('The Manager')).toContainText('1 item awaiting approval');
    for (const name of departments.slice(1)) {
      await expect(card(name)).toBeDisabled();
      await expect(card(name)).toContainText('No pending approvals');
    }
    await card('The Manager').click();
    await expect(page.getByRole('heading', { name: 'The Manager', exact: true })).toBeVisible();
    await expect(page.getByText('Approval Inbox', { exact: true })).toBeVisible();
    await expect(page.getByLabel('Recorded action payload')).toHaveText(JSON.stringify(request.payload, null, 2));
    await recordTeamApproval(page, teamOwner, request);
    await expect(page.getByText('There are no pending actions requiring your review.', { exact: true })).toBeVisible();
    await readTeamPage(page, teamOwner, [], () => page.reload());
    await expect(page.getByRole('status').filter({ hasText: approvalNotice })).toBeVisible();
    for (const name of departments) {
      await expect(card(name)).toBeDisabled();
      await expect(card(name)).toHaveAttribute('aria-disabled', 'true');
      await expect(card(name)).toContainText('No pending approvals');
    }
    await assertPendingRequests(page, teamOwner, []);
    await assertRecordedApproval(page, teamOwner, request);
    await assertForeignRequestUnchanged(page, teamOwner);
  });
});
