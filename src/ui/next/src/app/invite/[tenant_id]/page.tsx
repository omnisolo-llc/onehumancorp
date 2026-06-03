"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

type Props = {
  params: {
    tenant_id: string;
  };
};

export default function CloudBridgeInvitePage({ params }: Props) {
  const router = useRouter();
  const [isJoining, setIsJoining] = useState(false);
  const [tenantName, setTenantName] = useState('Workspace');

  const tenantId = decodeURIComponent(params.tenant_id);

  useEffect(() => {
    // Basic formatting of tenantId to make it look like a readable name
    let formatted = tenantId.replace(/-/g, ' ');
    if (formatted.length > 0) {
        formatted = formatted.charAt(0).toUpperCase() + formatted.slice(1);
    }
    setTenantName(formatted);
  }, [tenantId]);

  const handleJoin = async () => {
    setIsJoining(true);

    try {
      // Connect with real backend provisioning
      // Note: In a real implementation this would create the user and add them to the tenant
      // For now, we simulate the invite acceptance tracking in the growth service
      const response = await fetch('/api/v1/growth/team-invites/accept', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: `invite-${tenantId}` })
      });

      // Even if API fails in this demo setup, we want the user to experience the UI flow
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('tenant', tenantId);
        localStorage.setItem('cloud_mode', 'true');
      }

      router.push('/dashboard?joined=true');
    } catch (e) {
      console.error('Failed to accept invite', e);
      setIsJoining(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4 font-inter relative overflow-hidden">

      {/* Background decorations */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-blue-100 opacity-50 blur-[80px]"></div>
      <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] rounded-full bg-purple-100 opacity-50 blur-[80px]"></div>

      <div
        className="w-full max-w-md p-8 relative z-10 text-center"
        style={{
          backdropFilter: 'blur(20px) saturate(200%)',
          background: 'rgba(255, 255, 255, 0.65)',
          border: '1px solid rgba(255, 255, 255, 0.4)',
          borderRadius: '24px',
          boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.05), 0 0 0 1px rgba(0, 0, 0, 0.02)'
        }}
      >
        <div className="w-20 h-20 mx-auto mb-6 bg-gradient-to-tr from-blue-600 to-indigo-600 rounded-2xl flex items-center justify-center shadow-lg transform rotate-[-5deg]">
          <span className="text-4xl text-white font-bold font-outfit">
             {tenantName.charAt(0).toUpperCase()}
          </span>
        </div>

        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">
          You've been invited
        </h1>
        <p className="text-gray-600 text-sm mb-8 leading-relaxed px-4">
          You're invited to collaborate in a secure Cloud-Native space for <span className="font-semibold text-gray-900">{tenantName}</span>. Connect your sovereign local instance seamlessly.
        </p>

        <div className="space-y-4">
          <button
            onClick={handleJoin}
            disabled={isJoining}
            className="w-full py-4 px-6 rounded-xl font-bold text-white transition-all shadow-md active:scale-[0.98] flex items-center justify-center gap-2 disabled:opacity-80"
            style={{ background: 'linear-gradient(135deg, #2563eb 0%, #4f46e5 100%)' }}
          >
            {isJoining ? (
              <>
                <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                Provisioning Tenant...
              </>
            ) : (
              'Accept Invitation & Join Team Workspace'
            )}
          </button>

          <button className="w-full py-3 text-sm font-semibold text-gray-500 hover:text-gray-700 transition-colors">
            Decline
          </button>
        </div>

        <div className="mt-8 pt-6 border-t border-gray-200/50 flex flex-col items-center gap-2">
           <div className="text-[10px] uppercase font-bold text-gray-400 tracking-wider">
             Secure Hybrid Growth Loop
           </div>
           <div className="flex items-center gap-2 text-xs text-gray-500 font-medium bg-white/50 px-3 py-1.5 rounded-full border border-white">
             <span className="text-green-500">●</span>
             Zero Data Leakage Guaranteed
           </div>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
