import React, { useState } from 'react';
import { useAuth } from '@/lib/auth';

export default function WhatsAppIntegrationButton() {
  const { session } = useAuth();
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  // Future integration with real Meta OAuth Flow
  const handleConnect = async () => {
    setIsLoading(true);
    // Simulate Meta OAuth popup and integration setup
    setTimeout(() => {
      setIsConnected(true);
      setIsLoading(false);
    }, 1500);
  };

  return (
    <div className="flex items-center justify-between p-4 border border-gray-200 rounded-md bg-gray-50">
      <div className="flex items-center gap-3">
        <div className="w-10 h-10 bg-[#25D366] text-white rounded-full flex items-center justify-center font-bold text-xl">
          W
        </div>
        <div>
          <div className="font-semibold text-gray-900">WhatsApp Business Cloud API</div>
          <div className="text-sm text-gray-500">Connect to receive and reply to WhatsApp messages directly in Work Triage.</div>
        </div>
      </div>
      <div>
        {isConnected ? (
           <span className="app-badge good text-sm">Connected</span>
        ) : (
          <button
            onClick={handleConnect}
            disabled={isLoading}
            className="app-button primary"
            type="button"
          >
            {isLoading ? "Connecting..." : "Connect WhatsApp"}
          </button>
        )}
      </div>
    </div>
  );
}
