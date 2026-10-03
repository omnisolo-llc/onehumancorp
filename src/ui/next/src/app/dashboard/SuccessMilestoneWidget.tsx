"use client";

import { useCloudInvitation } from '../referrals/useCloudInvitation';
import { RecordedOrderMilestone } from '../components/RecordedOrderMilestone';

const noPrivateDraft = () => {};

/** The dashboard uses the same recorded-order and confirmed-invitation boundary. */
export function SuccessMilestoneWidget() {
  const invitation = useCloudInvitation(noPrivateDraft);
  return <RecordedOrderMilestone invitation={invitation} showInvitationAction />;
}
