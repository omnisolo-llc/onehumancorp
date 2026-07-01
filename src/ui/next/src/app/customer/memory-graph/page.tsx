"use client";
import { Suspense } from "react";
import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'next/navigation';
import { PoweredByOHC } from '../../components/PoweredByOHC';
import { FaInstagram, FaRegEnvelope, FaStore, FaCalendarCheck, FaGlobe, FaRobot } from 'react-icons/fa';

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
  }, [tenantId, customerId]);

  if (loading) {
    return (
      <div className="flex h-screen w-full items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-[#0066FF]"></div>
        <span className="sr-only">Loading customer history...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex h-screen w-full items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
        <div className="text-red-500">{error}</div>
      </div>
    );
  }

  const interactions = data?.events ? data.events : (data?.interactions ? data.interactions : []);

  const summary = data?.summary || "No summary available.";
  const customerName = data?.customer_name || "Unknown Customer";

  const getIcon = (type: string) => {
    switch (type) {
      case 'ig_dm': return <FaInstagram className="text-pink-500" />;
      case 'email': return <FaRegEnvelope className="text-blue-500" />;
      case 'store_visit': return <FaStore className="text-green-500" />;
      case 'booking': return <FaCalendarCheck className="text-indigo-500" />;
      case 'agent_reply': return <FaRobot className="text-[#0066FF]" />;
      default: return <FaGlobe className="text-gray-500" />;
    }
  };

  return (
    <div className="flex flex-col h-screen w-full bg-gray-50 dark:bg-gray-900 text-[#1D1D1F] dark:text-[#F5F5F7]">
      <div className="flex-1 overflow-y-auto p-4 max-w-2xl mx-auto w-full space-y-6 pt-8 pb-20">

        <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-6 shadow-sm">
          <div className="flex items-center gap-4 mb-4">
             <div className="w-16 h-16 rounded-full bg-indigo-100 dark:bg-indigo-900/50 flex items-center justify-center text-2xl font-bold text-[#0066FF]">
               {customerName.charAt(0)}
             </div>
             <div>
               <h1 className="text-2xl font-bold">Customer Context</h1>
               <span className="inline-flex items-center rounded-md bg-green-50 px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20">High Intent</span>
               {data?.segments && data.segments.map((s: string) => <span key={s} className="ml-1 inline-flex items-center rounded-md bg-blue-50 px-2 py-1 text-xs font-medium text-blue-700 ring-1 ring-inset ring-blue-600/20">{s}</span>)}
             </div>
          </div>
          <div className="mt-4 border-t border-gray-200 dark:border-gray-700 pt-4">
            <h3 className="text-sm font-semibold mb-2">Agent Summary</h3>
            <p className="text-sm text-gray-600 dark:text-gray-300 leading-relaxed">
              {summary}
            </p>
            <p className="text-xs text-gray-500 mt-2">{data?.total_interactions || interactions.length} total interactions recorded.</p>
          </div>
        </div>

        <div>
           <h2 className="text-lg font-bold mb-4 ml-1">Timeline</h2>
           <div className="space-y-4 relative before:absolute before:inset-0 before:ml-5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-gray-300 dark:before:via-gray-700 before:to-transparent">

             {interactions.length === 0 ? <div className="text-center p-4">No interaction history found.</div> : interactions.map((interaction: any, idx: number) => (
               <div key={idx} className="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group is-active">
                  <div className="flex items-center justify-center w-10 h-10 rounded-full border-2 border-white dark:border-gray-900 bg-white dark:bg-gray-800 text-gray-500 shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 z-10">
                     {getIcon(interaction.channel || interaction.type)}
                  </div>
                  <div className="w-[calc(100%-4rem)] md:w-[calc(50%-2.5rem)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-4 shadow-sm">
                     <div className="flex items-center justify-between mb-1">
                        <span className="font-bold text-sm">{interaction.channel || (interaction.type === 'agent_reply' ? 'OHC Agent' : 'Customer')}</span>
                        <time className="text-xs text-gray-500">{interaction.date || (interaction.created_at ? new Date(interaction.created_at).toLocaleString() : '')}</time>
                     </div>
                     <p className="text-sm text-gray-600 dark:text-gray-300">{interaction.description || interaction.raw_content}</p>
                  </div>
               </div>
             ))}

           </div>

           <div className="mt-8 flex justify-center gap-4">
               <button className="px-4 py-2 bg-blue-600 text-white rounded-md">Draft Reply</button>
               <button className="px-4 py-2 bg-red-600 text-white rounded-md">Issue Refund</button>
           </div>
        </div>

      </div>
      <div className="fixed bottom-4 left-0 right-0 flex justify-center z-50">
          <PoweredByOHC tenantId={tenantId} />
      </div>
    </div>
  );
}

export default function CustomerMemoryGraph() {
  return (
    <Suspense fallback={
      <div className="flex h-screen w-full items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-[#0066FF]"></div>
        <span className="sr-only">Loading customer history...</span>
      </div>
    }>
      <CustomerMemoryGraphContent />
    </Suspense>
  );
}
