'use client';
import { useState } from 'react';

export default function IntegrationsPage() {
  const [metaConnected, setMetaConnected] = useState(false);
  const [calConnected, setCalConnected] = useState(false);

  return (
    <div className="p-8">
      <h1 className="text-2xl font-bold mb-4">Integrations</h1>

      <div className="space-y-4">
        <div className="p-4 border rounded">
          <h2 className="text-xl">Meta Graph API (Social Inbox)</h2>
          <button
            id="connect-meta"
            className="mt-2 px-4 py-2 bg-blue-600 text-white rounded"
            onClick={() => setMetaConnected(true)}
          >
            {metaConnected ? 'Connected' : 'Connect Meta'}
          </button>
        </div>

        <div className="p-4 border rounded">
          <h2 className="text-xl">Cal.com (Booking)</h2>
          <button
            id="connect-cal"
            className="mt-2 px-4 py-2 bg-blue-600 text-white rounded"
            onClick={() => setCalConnected(true)}
          >
            {calConnected ? 'Connected' : 'Connect Cal.com'}
          </button>
        </div>
      </div>
    </div>
  );
}
