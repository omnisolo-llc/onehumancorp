import { fetchForOnboardingOwner, type DraftOwner } from './draftSession';
import { captureOnboardingDraftWrite, acknowledgeOnboardingDraft, markOnboardingDraftPending } from './store';

export async function sendOnboardingDraft(url: string, options: RequestInit, owner: DraftOwner | null): Promise<Response> {
  const stamp = captureOnboardingDraftWrite(options.body);
  if (!stamp) markOnboardingDraftPending(owner);
  // Acknowledgement runs inside the origin write lock; a later write cannot overtake it.
  return fetchForOnboardingOwner(url, options, owner, version => {
    if (stamp) acknowledgeOnboardingDraft(stamp, version);
  });
}
