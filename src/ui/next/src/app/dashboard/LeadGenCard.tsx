import React, { useState } from 'react';

export const LeadGenCard = () => {
  const [budget, setBudget] = useState('');
  const [radius, setRadius] = useState('10');
  const [zipCode, setZipCode] = useState('');
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState(false);

  const handleStartCampaign = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/v1/growth/lead_gen_campaign', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          budget: parseFloat(budget),
          radius_miles: parseFloat(radius),
          zip_code: zipCode
        })
      });
      const data = await response.json();
      if (data.success) {
        setSuccess(true);
      } else {
        alert('Failed to start campaign.');
      }
    } catch (e) {
      console.error(e);
      alert('Error occurred while starting campaign');
    } finally {
      setLoading(false);
    }
  };

  if (success) {
    return (
      <div
        className="p-6 rounded-2xl mb-6 shadow-xl relative overflow-hidden text-white"
        style={{
          background: 'rgba(255, 255, 255, 0.65)',
          backdropFilter: 'blur(30px) saturate(210%)',
          border: '1px solid rgba(255, 255, 255, 0.4)'
        }}
      >
        <h2 className="text-xl font-bold font-outfit text-indigo-900 mb-2">Campaign Active!</h2>
        <p className="text-sm text-gray-700 font-inter">
          Your AI Marketing Agent is now finding jobs for you. You'll receive a notification when a new lead is booked.
        </p>
      </div>
    );
  }

  return (
    <div
      className="p-6 rounded-2xl mb-6 shadow-xl relative overflow-hidden"
      style={{
        background: 'rgba(255, 255, 255, 0.65)',
        backdropFilter: 'blur(30px) saturate(210%)',
        border: '1px solid rgba(255, 255, 255, 0.4)'
      }}
    >
      <div className="relative z-10">
        <h2 className="text-xl font-bold font-outfit text-indigo-900 mb-2">Want more local jobs this week?</h2>
        <p className="text-sm text-gray-700 mb-6 font-inter">
          Set a weekly budget and radius. Our AI agent will find prospects, qualify them via DM, and book appointments directly to your calendar.
        </p>

        <div className="space-y-4">
          <div>
            <label className="block text-xs font-semibold text-gray-600 mb-1">Weekly Budget ($)</label>
            <input
              type="number"
              value={budget}
              onChange={(e) => setBudget(e.target.value)}
              placeholder="e.g. 50"
              className="w-full px-4 py-2 rounded-lg bg-white bg-opacity-50 border border-gray-200 text-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              inputMode="decimal"
            />
          </div>
          <div className="flex gap-4">
            <div className="flex-1">
              <label className="block text-xs font-semibold text-gray-600 mb-1">Service Zip Code</label>
              <input
                type="text"
                value={zipCode}
                onChange={(e) => setZipCode(e.target.value)}
                placeholder="e.g. 90210"
                className="w-full px-4 py-2 rounded-lg bg-white bg-opacity-50 border border-gray-200 text-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                inputMode="numeric"
              />
            </div>
            <div className="flex-1">
              <label className="block text-xs font-semibold text-gray-600 mb-1">Radius (Miles)</label>
              <input
                type="number"
                value={radius}
                onChange={(e) => setRadius(e.target.value)}
                className="w-full px-4 py-2 rounded-lg bg-white bg-opacity-50 border border-gray-200 text-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                inputMode="numeric"
              />
            </div>
          </div>

          <button
            onClick={handleStartCampaign}
            disabled={loading || !budget || !zipCode}
            className={`w-full py-3 rounded-lg font-semibold text-white transition-colors shadow-md ${
              loading || !budget || !zipCode ? 'bg-indigo-300 cursor-not-allowed' : 'bg-[#0066FF] hover:bg-indigo-600'
            }`}
          >
            {loading ? 'Activating Agent...' : 'Start Finding Jobs'}
          </button>
        </div>
      </div>
    </div>
  );
};
