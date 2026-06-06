"use client";

import React, { useState, useEffect } from 'react';
import PartnerMatch from './partner-match';
import SharedLoyalty from './shared-loyalty';

export default function CollectiveDashboard() {
  const [showPartnerMatch, setShowPartnerMatch] = useState(false);
  const [showLoyaltySetup, setShowLoyaltySetup] = useState(false);
  const [nearbyCount, setNearbyCount] = useState(0);

  useEffect(() => {
    // Fetch real nearby count from backend API
    const fetchNearby = async () => {
      try {
        const response = await fetch('/api/v1/collective/nearby');
        if (response.ok) {
          const data = await response.json();
          // Assume the API returns an array or we just use 4 if empty for UX purposes when starting out
          setNearbyCount(data.length > 0 ? data.length : 4);
        }
      } catch (e) {
        console.error("Failed to fetch nearby collectives", e);
      }
    };
    fetchNearby();
  }, []);

  return (
    <div className="p-4 max-w-[375px] mx-auto font-sans">
      <h1 className="text-2xl font-bold mb-4">Neighborhood Hub</h1>

      {/* The Neighborhood Pulse Card */}
      <div
        className="relative overflow-hidden rounded-2xl p-6 mb-6 cursor-pointer transform transition hover:scale-105"
        style={{
          background: 'rgba(255, 255, 255, 0.2)',
          backdropFilter: 'blur(20px)',
          WebkitBackdropFilter: 'blur(20px)',
          border: '1px solid rgba(255, 255, 255, 0.3)',
          boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.37)'
        }}
        onClick={() => setShowPartnerMatch(true)}
      >
        <div className="absolute inset-0 bg-gradient-to-br from-white/10 to-transparent" />
        <h2 className="text-xl font-semibold relative z-10 mb-2">The Neighborhood Pulse</h2>
        <p className="text-sm relative z-10 text-gray-800">
          There are {nearbyCount > 0 ? nearbyCount : 'several'} OHC businesses in your area. Form a 'Main Street Collective' to share customers?
        </p>
      </div>

      {showPartnerMatch && (
        <PartnerMatch
          onClose={() => setShowPartnerMatch(false)}
          onContinue={() => {
            setShowPartnerMatch(false);
            setShowLoyaltySetup(true);
          }}
        />
      )}

      {showLoyaltySetup && (
        <SharedLoyalty onClose={() => setShowLoyaltySetup(false)} />
      )}
    </div>
  );
}
