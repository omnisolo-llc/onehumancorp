'use client';

import { AppShell } from '../components/AppShell';
import { AgentFeed } from '../../components/AgentFeed/AgentFeed';

export default function AgentFeedPage() {
  const statusItems = [
    { label: 'Status', value: 'Live', tone: 'good' as const },
  ];

  return (
    <AppShell
      title="Agent Activity Feed"
      subtitle="The unified nerve center for all agent proposals and autonomous actions."
      statusItems={statusItems}
    >
      <div className="max-w-xl mx-auto py-8 px-4">
        <AgentFeed />
      </div>
    </AppShell>
  );
}
