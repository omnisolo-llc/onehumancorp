"use client";

import React, { useState, useCallback } from 'react';

type RoleData = {
  id: string;
  name: string;
  count: number;
};

type Props = {
  initialRoles?: RoleData[];
};

// Premium CSS tokens from design doc
const tokens = {
  bgPanel: 'bg-[#1A1A1A]',
  textPrimary: 'text-[#F3F4F6]',
  textSecondary: 'text-[#9CA3AF]',
  accentHire: 'text-[#10B981]',
  accentFire: 'text-[#EF4444]',
  glowHire: 'shadow-[0_0_15px_rgba(16,185,129,0.4)]',
  glowFire: 'shadow-[0_0_15px_rgba(239,68,68,0.4)]',
  glassmorphism: 'bg-white/5 backdrop-blur-[20px] saturate-200 border border-white/10',
};

export default function DynamicScalingCard({
  initialRoles = [
    { id: 'sales_rep', name: 'Sales Representative', count: 2 },
    { id: 'customer_support', name: 'Customer Support Specialist', count: 3 }
  ]
}: Props) {
  const [roles, setRoles] = useState<RoleData[]>(initialRoles);
  const [loadingRoles, setLoadingRoles] = useState<Record<string, boolean>>({});
  const [traces, setTraces] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);

  const handleScale = useCallback(async (roleId: string, newCount: number) => {
    const roleIndex = roles.findIndex(r => r.id === roleId);
    if (roleIndex === -1) return;

    const role = roles[roleIndex];
    const previousCount = role.count;

    // Optimistic UI update
    const newRoles = [...roles];
    newRoles[roleIndex].count = newCount;
    setRoles(newRoles);

    setLoadingRoles(prev => ({ ...prev, [roleId]: true }));
    setError(null);

    const isHiring = newCount > previousCount;
    const action = isHiring ? "Hiring" : "Firing";
    const amount = Math.abs(newCount - previousCount);

    try {
      // Intent payload to Gateway API
      const res = await fetch('/api/v1/scale', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ role: roleId, count: newCount }),
      });

      if (!res.ok) {
        throw new Error("Failed to update scale");
      }

      // Simulated real-time trace updates (SSE)
      setTraces(prev => [`${action} ${amount} ${role.name}(s)...`, ...prev].slice(0, 5));

      // Simulate K8s reconciliation time
      setTimeout(() => {
        setTraces(prev => [`✅ ${action} complete for ${role.name}.`, ...prev].slice(0, 5));
        setLoadingRoles(prev => ({ ...prev, [roleId]: false }));
      }, 1500);

    } catch (err: any) {
      // Revert on failure
      const revertedRoles = [...roles];
      revertedRoles[roleIndex].count = previousCount;
      setRoles(revertedRoles);
      setError(err.message || "Failed to scale agent");
      setLoadingRoles(prev => ({ ...prev, [roleId]: false }));
    }
  }, [roles]);

  return (
    <div className={`w-full rounded-2xl p-6 ${tokens.glassmorphism}`}>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-bold font-outfit text-white">Workforce Scaling</h2>
        <span className="text-sm font-medium bg-blue-500/20 text-blue-300 px-3 py-1 rounded-full border border-blue-500/30">
          Live Sync
        </span>
      </div>

      {error && (
        <div className="mb-4 p-3 bg-red-500/10 border border-red-500/20 rounded-xl text-red-400 text-sm">
          {error}
        </div>
      )}

      <div className="space-y-6">
        {roles.map((role) => (
          <div key={role.id} className="relative group">
            <div className={`
              absolute inset-0 rounded-xl transition-all duration-300 opacity-0 group-hover:opacity-100
              ${loadingRoles[role.id] ? (role.count > initialRoles.find(r => r.id === role.id)!.count ? tokens.glowHire : tokens.glowFire) : ''}
            `} />

            <div className="relative bg-[#1A1A1A]/80 border border-white/5 rounded-xl p-5 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
              <div>
                <h3 className="font-semibold text-white font-outfit">{role.name}</h3>
                <p className="text-sm text-gray-400 mt-1">Active Replicas: {role.count}</p>
              </div>

              <div className="flex items-center gap-4">
                <button
                  onClick={() => handleScale(role.id, Math.max(0, role.count - 1))}
                  disabled={loadingRoles[role.id] || role.count <= 0}
                  className="w-10 h-10 flex items-center justify-center rounded-full bg-red-500/10 text-red-400 hover:bg-red-500/20 hover:text-red-300 active:scale-90 transition-all disabled:opacity-50"
                  aria-label={`Decrease ${role.name}`}
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 12H4" /></svg>
                </button>

                <div className="w-12 text-center font-bold text-xl text-white">
                  {role.count}
                </div>

                <button
                  onClick={() => handleScale(role.id, role.count + 1)}
                  disabled={loadingRoles[role.id]}
                  className="w-10 h-10 flex items-center justify-center rounded-full bg-emerald-500/10 text-emerald-400 hover:bg-emerald-500/20 hover:text-emerald-300 active:scale-90 transition-all disabled:opacity-50"
                  aria-label={`Increase ${role.name}`}
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
                </button>
              </div>
            </div>

            {loadingRoles[role.id] && (
              <div className="absolute bottom-0 left-0 h-1 bg-gradient-to-r from-transparent via-blue-500 to-transparent w-full animate-[shimmer_1.5s_infinite]" />
            )}
          </div>
        ))}
      </div>

      {traces.length > 0 && (
        <div className="mt-8 p-4 bg-black/40 rounded-xl border border-white/5">
          <h4 className="text-xs font-bold text-gray-500 uppercase tracking-wider mb-3">Real-time Trace Logs</h4>
          <div className="space-y-2 font-mono text-sm">
            {traces.map((trace, idx) => (
              <div key={idx} className={`text-gray-300 ${idx === 0 ? 'opacity-100' : 'opacity-60'}`}>
                <span className="text-gray-500 mr-2">[{new Date().toLocaleTimeString()}]</span>
                {trace}
              </div>
            ))}
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes shimmer {
          0% { transform: translateX(-100%); }
          100% { transform: translateX(100%); }
        }
      `}} />
    </div>
  );
}
