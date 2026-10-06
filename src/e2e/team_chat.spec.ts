import { test, expect, readTeamPage, sendTeamRequest, assertPendingRequests, recordTeamApproval,
  assertRecordedApproval, assertForeignRequestUnchanged, captureTeamResponse, approvalNotice } from './support/team_fixture';
import { currentAppSmoke } from './current_app_smoke';

test('current app smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'team_chat');
});

test.describe('Team Chat E2E', () => {
  test('loads the verified owner’s empty review inbox without an invented greeting', async ({ page, teamOwner }) => {
    await readTeamPage(page, teamOwner, [], () => page.goto('/team/chat'));
    await expect(page.getByRole('heading', { name: 'Team Chat', exact: true })).toBeVisible();
    await expect(page.getByText('No pending approvals were returned.', { exact: true })).toBeVisible();
    await expect(page.getByText("I'm your central team interface", { exact: false })).toHaveCount(0);
    await expect(page.getByTestId('action-card')).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Send', exact: true })).toBeDisabled();
    await assertPendingRequests(page, teamOwner, []);
    await assertForeignRequestUnchanged(page, teamOwner);
  });

  test('sends and reloads the exact persisted owner request', async ({ page, teamOwner }) => {
    await readTeamPage(page, teamOwner, [], () => page.goto('/team/chat'));
    const request = await sendTeamRequest(page, teamOwner, `Review operations for ${teamOwner.tenantId}`);
    await assertPendingRequests(page, teamOwner, [request]);
    await readTeamPage(page, teamOwner, [request], () => page.reload());
    await expect(page.getByTestId('action-card')).toHaveCount(1);
    await expect(page.getByTestId('action-card')).toContainText(request.payload.original_request);
    await assertPendingRequests(page, teamOwner, [request]);
  });

  test('renders separate persisted review requests without a canned action draft', async ({ page, teamOwner }) => {
    await readTeamPage(page, teamOwner, [], () => page.goto('/team/chat'));
    const first = await sendTeamRequest(page, teamOwner, `First operations review ${teamOwner.tenantId}`);
    const second = await sendTeamRequest(page, teamOwner, `Second operations review ${teamOwner.tenantId}`);
    expect(second.id).not.toBe(first.id);
    await assertPendingRequests(page, teamOwner, [first, second]);
    await readTeamPage(page, teamOwner, [first, second], () => page.reload());
    await expect(page.getByTestId('action-card')).toHaveCount(2);
    for (const request of [first, second]) {
      const card = page.getByTestId('action-card').filter({ hasText: request.payload.original_request });
      await expect(card).toHaveCount(1);
      await expect(card.locator('pre')).toHaveText(JSON.stringify(request.payload, null, 2));
      await expect(card.getByRole('button', { name: 'Record approval', exact: true })).toBeEnabled();
      await expect(card.getByRole('button', { name: 'Record dismissal', exact: true })).toBeEnabled();
    }
  });

  test('records and reconciles the exact approval without claiming execution or replaying it', async ({ page, teamOwner }) => {
    await readTeamPage(page, teamOwner, [], () => page.goto('/team/chat'));
    const request = await sendTeamRequest(page, teamOwner, `Approve operations review ${teamOwner.tenantId}`);
    await assertPendingRequests(page, teamOwner, [request]);
    const writes: string[] = [];
    page.on('request', request => { if (request.method() === 'PUT') writes.push(request.url()); });
    const receipt = await recordTeamApproval(page, teamOwner, request);
    await expect(page.getByTestId('action-card')).toHaveCount(0);
    await assertPendingRequests(page, teamOwner, []);
    const reconciled = captureTeamResponse(page, teamOwner, `/api/v1/agent-feed/${request.id}/decision`);
    await readTeamPage(page, teamOwner, [], () => page.reload());
    expect(await (await reconciled).json()).toEqual(receipt);
    await expect(page.locator(`[role="status"][data-approval-id="${request.id}"]`)).toHaveText(approvalNotice);
    await expect(page.getByTestId('action-card')).toHaveCount(0);
    await assertRecordedApproval(page, teamOwner, request);
    expect(writes).toEqual([`${teamOwner.origin}/api/v1/agent-feed/${request.id}`]);
    await assertForeignRequestUnchanged(page, teamOwner);
  });

  test('navigates through the Back to Team link and browser history without losing the request', async ({ page, teamOwner }) => {
    await readTeamPage(page, teamOwner, [], () => page.goto('/team/chat'));
    const request = await sendTeamRequest(page, teamOwner, `Keep operations review ${teamOwner.tenantId}`);
    const back = page.getByRole('link', { name: 'Back to Team', exact: true });
    await expect(back).toHaveAttribute('href', '/team');
    await readTeamPage(page, teamOwner, [request], () => back.click());
    await expect(page).toHaveURL(`${teamOwner.origin}/team`);
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();
    await readTeamPage(page, teamOwner, [request], () => page.goBack());
    await expect(page).toHaveURL(`${teamOwner.origin}/team/chat`);
    await expect(page.getByTestId('action-card')).toContainText(request.payload.original_request);
    await readTeamPage(page, teamOwner, [request], () => page.goForward());
    await expect(page).toHaveURL(`${teamOwner.origin}/team`);
    await assertPendingRequests(page, teamOwner, [request]);
  });
});
