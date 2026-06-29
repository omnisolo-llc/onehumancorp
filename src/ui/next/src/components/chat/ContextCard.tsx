import React, { useEffect, useState } from 'react';

interface CustomerProfileSummary {
  total_interactions: number;
  last_interaction: string | null;
  segments: string[];
  preferences: string[];
  summary: string;
}

interface ContextCardProps {
  tenantId: string;
  customerId: string;
}

export const ContextCard: React.FC<ContextCardProps> = ({ tenantId, customerId }) => {
  const [profile, setProfile] = useState<CustomerProfileSummary | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchSummary = async () => {
      try {
        const res = await fetch(`/api/inbox/summary/${tenantId}/${customerId}`);
        if (res.ok) {
          const data = await res.json();
          setProfile(data);
        }
      } catch (e) {
        console.error("Failed to fetch customer profile summary", e);
      } finally {
        setLoading(false);
      }
    };
    fetchSummary();
  }, [tenantId, customerId]);

  if (loading) {
    return (
      <div className="w-full h-11 min-h-[44px] backdrop-blur-[30px] backdrop-saturate-[2.1] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 shadow-[0_4px_24px_rgba(0,0,0,0.04)] rounded-lg flex items-center justify-center animate-pulse">
        <span className="text-sm text-gray-500">Loading context...</span>
      </div>
    );
  }

  if (!profile) {
    return null;
  }

  return (
    <div className="w-full min-h-[44px] p-3 backdrop-blur-[30px] backdrop-saturate-[2.1] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 shadow-[0_4px_24px_rgba(0,0,0,0.04)] rounded-xl mb-4 transition-all duration-300">
      <div className="flex flex-col space-y-1">
        <div className="text-xs font-semibold text-gray-700 flex flex-wrap gap-1">
          {profile.segments.map((segment, idx) => (
            <span key={idx} className="bg-blue-100 text-blue-800 px-2 py-0.5 rounded-full">
              {segment}
            </span>
          ))}
          {profile.preferences.map((pref, idx) => (
            <span key={idx} className="bg-green-100 text-green-800 px-2 py-0.5 rounded-full">
              {pref}
            </span>
          ))}
          {profile.segments.length === 0 && profile.preferences.length === 0 && (
            <span className="text-gray-500">{profile.summary}</span>
          )}
        </div>
        <div className="text-xs text-gray-500 truncate">
          {profile.total_interactions} past orders &bull; Last order: {profile.last_interaction ? new Date(profile.last_interaction).toLocaleDateString() : 'N/A'}
        </div>
      </div>
    </div>
  );
};
