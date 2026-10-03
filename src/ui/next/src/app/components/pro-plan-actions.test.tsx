import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, expect, it, vi } from 'vitest';
import PreOrder from '../pre-order-widget/page';
import SocialProof from '../social-proof-nudge/page';
import Savings from './AiTimeSavingsWidget';
const plan = vi.hoisted(() => ({ hasPro: false, claimTrial: vi.fn(), refreshPlan: vi.fn(), confirmPro: vi.fn(), claimError: 'Trial activation is unavailable.', verifiedOwner: { userId: 'a', tenantId: 'a' } }));
vi.mock('./useProPlan', () => ({ useProPlan: () => plan }));
vi.mock('next/navigation', () => ({ useRouter: () => ({ push: vi.fn() }) }));
beforeEach(() => { vi.clearAllMocks(); plan.claimTrial.mockResolvedValue(false); plan.refreshPlan.mockResolvedValue(false); vi.stubGlobal('fetch', vi.fn(async () => Response.json({ hours_saved: 3, inquiries_handled: 2, appointments_scheduled: 1 }))); vi.spyOn(window, 'open').mockImplementation(() => null); });
it.each([{ name: 'pre-order', Page: PreOrder }, { name: 'social proof', Page: SocialProof }])('$name cannot post a trial claim or flip Pro from a share action', async ({ Page }) => {
  render(<Page />); fireEvent.click(screen.getByRole('checkbox'));
  const action = screen.getByRole('button', { name: /Share on X|Check trial availability/ });
  await act(async () => fireEvent.click(action));
  expect(vi.mocked(fetch).mock.calls.some(([,options]) => options?.method === 'POST')).toBe(false);
  expect(plan.confirmPro).not.toHaveBeenCalled(); expect(window.open).not.toHaveBeenCalled();
  expect(screen.getByRole('checkbox')).not.toBeChecked();
  expect(screen.getByText('Trial activation is unavailable.')).toBeVisible();
});
it.each(['false', 'throw'])('savings cannot claim a duration or earned grant after a %s claim result', async outcome => {
  if (outcome === 'throw') plan.claimTrial.mockRejectedValue(new Error('unavailable'));
  render(<Savings />);
  const action = await screen.findByRole('button', { name: /Share to get|Check trial availability/ });
  await act(async () => fireEvent.click(action));
  await waitFor(() => expect(plan.claimTrial).toHaveBeenCalledTimes(1));
  expect(screen.queryAllByText(/Trial Extended|successfully extended by 7 days/)).toHaveLength(0);
  expect(window.open).not.toHaveBeenCalled();
  expect(screen.getByRole('button', { name: 'Check trial availability' })).toBeVisible();
});
