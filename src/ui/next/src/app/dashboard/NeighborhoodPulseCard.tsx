"use client";
import React, { useState, useEffect } from 'react';

type Neighbor = {
  id: string;
  name: string;
};

export const NeighborhoodPulseCard = ({ tenant }: { tenant: string }) => {
  const [neighbors, setNeighbors] = useState<Neighbor[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchNeighbors = async () => {
      try {
        const response = await fetch('/api/mesh/v2/collective?action=getNearby');
        const data = await response.json();
        if (data.neighbors) {
          setNeighbors(data.neighbors.map((id: string) => ({ id, name: id.replace('_', ' ').replace(/\b\w/g, l => l.toUpperCase()) })));
        }
      } catch (e) {
        console.error('Failed to fetch neighbors', e);
      } finally {
        setLoading(false);
      }
    };
    fetchNeighbors();
  }, [tenant]);

  const handleInvite = async (targetId: string) => {
    try {
      const response = await fetch('/api/mesh/v2/collective', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action: 'invite', target_tenant_id: targetId })
      });
      const data = await response.json();
      if (data.success) {
        alert('Invitation sent successfully!');
      } else {
        alert('Failed to send invitation');
      }
    } catch (e) {
      console.error(e);
      alert('Error occurred while inviting');
    }
  };

  if (loading) return null;
  if (neighbors.length === 0) return null;

  return (
    <div
      className="p-6 rounded-2xl mb-6 shadow-xl relative overflow-hidden text-white"
      style={{
        background: 'linear-gradient(135deg, rgba(99, 102, 241, 0.9), rgba(168, 85, 247, 0.9))',
        backdropFilter: 'blur(30px) saturate(210%)',
        border: '1px solid rgba(255, 255, 255, 0.2)'
      }}
    >
      {/* Decorative pulse element */}
      <div className="absolute top-0 right-0 w-64 h-64 bg-white opacity-10 rounded-full blur-3xl -mr-10 -mt-10 animate-pulse"></div>

      <div className="relative z-10">
        <h2 className="text-xl font-bold font-outfit mb-2">Neighborhood Pulse</h2>
        <p className="text-sm text-indigo-100 mb-6 font-inter">
          There are {neighbors.length} OHC businesses in your area. Form a "Main Street Collective" to share customers?
        </p>

        <div className="space-y-4">
          {neighbors.map(neighbor => (
            <div
              key={neighbor.id}
              className="flex items-center justify-between p-4 rounded-xl"
              style={{
                background: 'rgba(255, 255, 255, 0.15)',
                backdropFilter: 'blur(30px) saturate(210%)',
                border: '1px solid rgba(255, 255, 255, 0.2)'
              }}
            >
              <div>
                <h3 className="font-semibold">{neighbor.name}</h3>
                <p className="text-xs text-indigo-100">Complementary Vibe Match</p>
              </div>
              <button
                onClick={() => handleInvite(neighbor.id)}
                className="px-4 py-2 bg-white text-indigo-600 rounded-lg text-sm font-semibold hover:bg-indigo-50 transition-colors shadow-sm"
              >
                Invite Partner
              </button>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
