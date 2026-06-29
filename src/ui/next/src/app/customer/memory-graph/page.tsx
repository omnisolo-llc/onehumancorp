'use client';

import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'next/navigation';
import { PoweredByOHC } from '../../components/PoweredByOHC';
import { FaInstagram, FaRegEnvelope, FaStore, FaCalendarCheck, FaGlobe, FaRobot } from 'react-icons/fa';

export default function CustomerMemoryGraph() { return <React.Suspense fallback={<div>Loading...</div>}><CustomerMemoryGraphContent /></React.Suspense>; }

function CustomerMemoryGraphContent() {
  const searchParams = useSearchParams();
  const customerId = searchParams.get('customerId') || 'default-customer-id';
  const tenantId = searchParams.get('tenantId') || 'default-tenant-id';

  const [loading, setLoading] = useState(true);
  const [data, setData] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchMemoryGraph = async () => {
      try {
        const res = await fetch(`/api/inbox/summary/${tenantId}/${customerId}`);
        if (res.ok) {
          const json = await res.json();
          setData(json);
        } else {
          setError('Failed to fetch customer history.');
        }
      } catch (err) {
        console.error('Failed to fetch memory graph', err);
        setError('An error occurred.');
      } finally {
        setLoading(false);
      }
    };
    fetchMemoryGraph();
  }, [customerId, tenantId]);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 font-inter">
        <p className="text-gray-500 font-medium">Loading customer history...</p>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 font-inter p-4">
        <div className="text-center">
          <p className="text-[#FF3B30] font-medium mb-2">{error || 'Customer not found.'}</p>
          <p className="text-gray-500 text-sm">Make sure the customer ID is correct.</p>
        </div>
      </div>
    );
  }

  const getChannelIcon = (channel: string) => {
    switch (channel.toLowerCase()) {
      case 'instagram': return <FaInstagram className="text-pink-500" />;
      case 'email': return <FaRegEnvelope className="text-blue-500" />;
      case 'pos': case 'in-store': return <FaStore className="text-emerald-500" />;
      case 'booking': return <FaCalendarCheck className="text-purple-500" />;
      case 'web': case 'online': return <FaGlobe className="text-indigo-500" />;
      case 'ai': case 'agent': return <FaRobot className="text-orange-500" />;
      default: return <div className="w-2 h-2 rounded-full bg-gray-400" />;
    }
  };

  const getChannelColor = (channel: string) => {
    switch (channel.toLowerCase()) {
      case 'instagram': return 'bg-pink-100 text-pink-700 border-pink-200';
      case 'pos': case 'in-store': return 'bg-emerald-100 text-emerald-700 border-emerald-200';
      case 'booking': return 'bg-purple-100 text-purple-700 border-purple-200';
      case 'ai': case 'agent': return 'bg-orange-100 text-orange-700 border-orange-200';
      default: return 'bg-gray-100 text-gray-700 border-gray-200';
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] font-inter p-4 pb-20 sm:p-6 sm:pb-24">
      <div className="max-w-[480px] mx-auto">
        <header className="mb-6">
          <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Customer Context</h1>
          <p className="text-sm text-gray-500 mt-1">Unified history across all channels</p>
        </header>

        {data.segments && data.segments.length > 0 && (
          <div className="mb-6 p-4 rounded-xl border bg-blue-50 border-blue-200" role="status">
            <h3 className="text-xs font-bold text-blue-800 uppercase tracking-wider mb-2 flex items-center gap-2">
              <FaRobot /> AI Insights
            </h3>
            <div className="flex flex-wrap gap-2">
              {data.segments.map((segment: string, i: number) => (
                <span key={i} className="px-2.5 py-1 text-xs font-semibold rounded-full bg-white text-blue-700 border border-blue-100 shadow-sm">
                  {segment}
                </span>
              ))}
            </div>
            {data.total_interactions > 0 && (
              <p className="text-sm text-blue-800 mt-3 font-medium">
                {data.total_interactions} total interactions recorded.
              </p>
            )}
          </div>
        )}

        <div className="relative p-6 rounded-[24px] shadow-[0_8px_30px_rgb(0,0,0,0.04)] mb-6 overflow-hidden"
             style={{
                background: 'rgba(255, 255, 255, 0.7)',
                backdropFilter: 'blur(40px) saturate(210%)',
                border: '1px solid rgba(255, 255, 255, 0.8)'
             }}>

          <h2 className="text-lg font-bold font-outfit text-gray-900 mb-6 border-b border-gray-100/80 pb-4">Timeline</h2>

          {data.events && data.events.length > 0 ? (
            <div className="relative border-l-2 border-gray-100 ml-3 space-y-8 pb-4">
              {data.events.map((event: any, index: number) => (
                <div key={event.id || index} className="relative pl-6 animate-fade-in-up" style={{ animationDelay: `${index * 100}ms` }}>
                  <div className="absolute -left-[9px] top-1 w-4 h-4 rounded-full bg-white border-2 border-gray-200 flex items-center justify-center shadow-sm">
                    <div className="w-1.5 h-1.5 rounded-full bg-gray-400"></div>
                  </div>

                  <div className="mb-1 flex items-center justify-between gap-4">
                    <span className={`px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider rounded border flex items-center gap-1.5 w-fit ${getChannelColor(event.channel)}`}>
                      {getChannelIcon(event.channel)} {event.channel}
                    </span>
                    <span className="text-xs text-gray-400 font-medium whitespace-nowrap">
                      {new Date(event.created_at).toLocaleDateString(undefined, { month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit' })}
                    </span>
                  </div>

                  <div className="bg-white/60 p-3 rounded-xl border border-gray-100/50 shadow-sm backdrop-blur-sm mt-2">
                    <p className="text-sm text-gray-800 font-medium leading-relaxed">{event.raw_content}</p>
                  </div>
                </div>
              ))}
            </div>
          ) : (
             <div className="text-center py-8">
               <div className="w-12 h-12 rounded-full bg-gray-50 flex items-center justify-center mx-auto mb-3">
                 <FaGlobe className="text-gray-300 text-xl" />
               </div>
               <p className="text-gray-500 font-medium text-sm">No interaction history found.</p>
               <p className="text-gray-400 text-xs mt-1">Events will appear here automatically.</p>
             </div>
          )}
        </div>

        <div className="flex gap-3">
          <button className="flex-1 py-3.5 px-4 bg-[#0066FF] text-white font-semibold rounded-xl shadow-sm hover:bg-blue-600 transition-all active:scale-[0.98] flex items-center justify-center gap-2">
            <FaRegEnvelope /> Draft Reply
          </button>
          <button className="flex-1 py-3.5 px-4 bg-white border border-gray-200 text-gray-900 font-semibold rounded-xl shadow-sm hover:bg-gray-50 transition-all active:scale-[0.98]">
            Issue Refund
          </button>
        </div>

        <div className="mt-8 flex justify-center">
            <PoweredByOHC tenantId={tenantId} />
        </div>
      </div>
    </div>
  );
}
