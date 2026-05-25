'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function SocialMediaSettingsPage() {
  const [isConnected, setIsConnected] = useState(false);

  const connectBuffer = () => {
    // In a real app this would initiate the OAuth flow
    setIsConnected(true);
  };

  return (
    <div className="p-4 max-w-md mx-auto text-black">
      <div className="flex items-center mb-4">
        <Link href="/dashboard" className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold">Social Media Settings</h1>
      </div>

      <div className="bg-white rounded shadow p-4 mb-4">
        <h2 className="text-xl mb-4 font-semibold">Buffer Integration</h2>
        <p className="mb-4 text-gray-700">Connect your Buffer account to view and reply to messages from all your social channels in one unified inbox.</p>

        {isConnected ? (
          <div>
            <span className="text-green-600 font-bold block mb-4">✓ Connected</span>
            <Link href="/inbox" className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 inline-block">
              Go to Inbox
            </Link>
          </div>
        ) : (
          <button
            onClick={connectBuffer}
            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
          >
            Connect Buffer
          </button>
        )}
      </div>
    </div>
  );
}
