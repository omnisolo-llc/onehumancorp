import { useState } from 'react';
import { Button } from '@/components/ui/button';

export function WhatsAppSetup() {
  const [isConnecting, setIsConnecting] = useState(false);

  const handleConnect = async () => {
    setIsConnecting(true);
    // Real implementation would launch Facebook Embedded Signup flow here
    setTimeout(() => {
      setIsConnecting(false);
      alert('WhatsApp connected successfully!');
    }, 1500);
  };

  return (
    <div className="flex flex-col space-y-4 p-4 min-w-[375px] max-w-full">
      <h2 className="text-xl font-bold">Connect WhatsApp</h2>
      <p className="text-sm text-gray-500">
        Connect your WhatsApp Business account to receive messages and reply directly from OneHumanCorp.
      </p>
      <Button
        onClick={handleConnect}
        disabled={isConnecting}
        className="w-full min-h-[44px]"
      >
        {isConnecting ? 'Connecting...' : 'Connect with Facebook'}
      </Button>
    </div>
  );
}
