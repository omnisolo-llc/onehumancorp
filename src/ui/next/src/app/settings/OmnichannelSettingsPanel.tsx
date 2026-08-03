'use client';

import React, { useState } from 'react';
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import { Label } from '../components/ui/label';

export default function OmnichannelSettingsPanel() {
  const [twilioSid, setTwilioSid] = useState('');
  const [twilioToken, setTwilioToken] = useState('');
  const [twilioPhone, setTwilioPhone] = useState('');

  const [whatsappToken, setWhatsappToken] = useState('');
  const [whatsappPhoneId, setWhatsappPhoneId] = useState('');

  const [status, setStatus] = useState<{message: string; type: 'success'|'error'|''} | null>(null);

  const handleConnectTwilio = async () => {
    try {
      const res = await fetch('/api/v1/settings/integrations/whatsapp_twilio', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          bot_token: twilioSid,
          api_token: twilioToken,
          from_phone: twilioPhone
        })
      });
      if (res.ok) {
        setStatus({ message: 'Twilio connected successfully', type: 'success' });
      } else {
        setStatus({ message: 'Failed to connect Twilio', type: 'error' });
      }
    } catch (e) {
      setStatus({ message: 'Error connecting Twilio', type: 'error' });
    }
  };

  const handleConnectWhatsapp = async () => {
    try {
      const res = await fetch('/api/v1/settings/integrations/whatsapp_cloud_api', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          api_token: whatsappToken,
          from_phone: whatsappPhoneId
        })
      });
      if (res.ok) {
        setStatus({ message: 'WhatsApp Cloud connected successfully', type: 'success' });
      } else {
        setStatus({ message: 'Failed to connect WhatsApp Cloud', type: 'error' });
      }
    } catch (e) {
      setStatus({ message: 'Error connecting WhatsApp Cloud', type: 'error' });
    }
  };

  return (
    <div className="space-y-6">
      {status && (
        <div className={`p-4 rounded-md ${status.type === 'success' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
          {status.message}
        </div>
      )}
      <Card>
        <CardHeader>
          <CardTitle>Twilio SMS Settings</CardTitle>
          <CardDescription>
            Connect your Twilio account to receive and send SMS directly from your Work Triage inbox.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="twilio-sid">Account SID</Label>
            <Input
              id="twilio-sid"
              type="text"
              placeholder="AC..."
              value={twilioSid}
              onChange={(e) => setTwilioSid(e.target.value)}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="twilio-token">Auth Token</Label>
            <Input
              id="twilio-token"
              type="password"
              placeholder="Token..."
              value={twilioToken}
              onChange={(e) => setTwilioToken(e.target.value)}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="twilio-phone">From Phone Number</Label>
            <Input
              id="twilio-phone"
              type="text"
              placeholder="+1234567890"
              value={twilioPhone}
              onChange={(e) => setTwilioPhone(e.target.value)}
            />
          </div>
          <Button onClick={handleConnectTwilio}>Connect Twilio SMS</Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>WhatsApp Business (Cloud API)</CardTitle>
          <CardDescription>
            Connect your WhatsApp Business account to sync messages directly with your Work Triage inbox.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="whatsapp-phone">Phone Number ID</Label>
            <Input
              id="whatsapp-phone"
              type="text"
              placeholder="e.g., 1234567890"
              value={whatsappPhoneId}
              onChange={(e) => setWhatsappPhoneId(e.target.value)}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="whatsapp-token">System User Access Token</Label>
            <Input
              id="whatsapp-token"
              type="password"
              placeholder="EAA..."
              value={whatsappToken}
              onChange={(e) => setWhatsappToken(e.target.value)}
            />
          </div>
          <Button onClick={handleConnectWhatsapp}>Connect WhatsApp</Button>
        </CardContent>
      </Card>
    </div>
  );
}
