import React, { useState } from 'react';

export default function AgentAutomations() {
  const [socialMediaEnabled, setSocialMediaEnabled] = useState(false);
  const [dmResponderEnabled, setDmResponderEnabled] = useState(false);
  const [weeklyInsightsEnabled, setWeeklyInsightsEnabled] = useState(false);
  const [loading, setLoading] = useState<string | null>(null);

  const toggleAgent = (agent: string, current: boolean, setter: React.Dispatch<React.SetStateAction<boolean>>) => {
    setLoading(agent);
    setTimeout(() => {
      setter(!current);
      setLoading(null);
    }, 500);
  };

  return (
    <div className="space-y-6">
      {/* Autonomous Social Media Agent */}
      <div className="rounded-[16px] shadow-sm p-6 glassmorphism/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-bold text-gray-900 font-outfit mb-1">Autonomous Social Media Agent</h3>
            <p className="text-sm text-gray-600">
              Agent detects new product addition, generates 3 Instagram post options (image + caption), and asks you to tap "Approve & Post".
            </p>
          </div>
          <button
            onClick={() => toggleAgent('social', socialMediaEnabled, setSocialMediaEnabled)}
            disabled={loading === 'social'}
            className={`px-4 py-2 rounded-xl text-sm font-semibold transition-colors ${
              socialMediaEnabled
                ? 'bg-red-50 text-red-600 hover:bg-red-100'
                : 'bg-indigo-600 text-white hover:bg-indigo-700 shadow-sm'
            }`}
          >
            {loading === 'social' ? '...' : socialMediaEnabled ? 'Disable Agent' : 'Enable Agent'}
          </button>
        </div>
        {socialMediaEnabled && (
          <div className="mt-4 p-4 rounded-xl bg-indigo-50/50 border border-indigo-100/50">
            <p className="text-sm font-medium text-indigo-900">✨ Agent is monitoring your catalog. Approvals will appear in your Inbox.</p>
          </div>
        )}
      </div>

      {/* DM Auto-Responder */}
      <div className="rounded-[16px] shadow-sm p-6 glassmorphism/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-bold text-gray-900 font-outfit mb-1">DM Auto-Responder</h3>
            <p className="text-sm text-gray-600">
              Agent connects to IG DMs, uses store FAQ/Inventory data to answer questions ("Do you have this in red?").
            </p>
          </div>
          <button
            onClick={() => toggleAgent('dm', dmResponderEnabled, setDmResponderEnabled)}
            disabled={loading === 'dm'}
            className={`px-4 py-2 rounded-xl text-sm font-semibold transition-colors ${
              dmResponderEnabled
                ? 'bg-red-50 text-red-600 hover:bg-red-100'
                : 'bg-indigo-600 text-white hover:bg-indigo-700 shadow-sm'
            }`}
          >
            {loading === 'dm' ? '...' : dmResponderEnabled ? 'Disable Agent' : 'Enable Agent'}
          </button>
        </div>
        {dmResponderEnabled && (
           <div className="mt-4 p-4 rounded-xl bg-indigo-50/50 border border-indigo-100/50">
           <p className="text-sm font-medium text-indigo-900">✨ Agent is connected to DMs. Fallback messages will route to you.</p>
         </div>
        )}
      </div>

      {/* Weekly Push Notification Insight */}
      <div className="rounded-[16px] shadow-sm p-6 glassmorphism/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-bold text-gray-900 font-outfit mb-1">Weekly Push Notification Insight</h3>
            <p className="text-sm text-gray-600">
              Agent sends a weekly push notification with 1 metric and 1 actionable suggestion.
            </p>
          </div>
          <button
            onClick={() => toggleAgent('insight', weeklyInsightsEnabled, setWeeklyInsightsEnabled)}
            disabled={loading === 'insight'}
            className={`px-4 py-2 rounded-xl text-sm font-semibold transition-colors ${
              weeklyInsightsEnabled
                ? 'bg-red-50 text-red-600 hover:bg-red-100'
                : 'bg-indigo-600 text-white hover:bg-indigo-700 shadow-sm'
            }`}
          >
            {loading === 'insight' ? '...' : weeklyInsightsEnabled ? 'Disable Agent' : 'Enable Agent'}
          </button>
        </div>
        {weeklyInsightsEnabled && (
           <div className="mt-4 p-4 rounded-xl bg-indigo-50/50 border border-indigo-100/50">
           <p className="text-sm font-medium text-indigo-900">✨ Agent will notify you every Monday morning.</p>
         </div>
        )}
      </div>
    </div>
  );
}
