"use client";

import React, { useEffect, useState, Suspense } from 'react';
import Link from 'next/link';
import { useSearchParams } from 'next/navigation';
import { PoweredByOHC } from '../../components/PoweredByOHC';

function ProposalViewContent() {
  const searchParams = useSearchParams();
  const [data, setData] = useState<any>(null);
  const [error, setError] = useState(false);
  const [isApproving, setIsApproving] = useState(false);
  const [approvalSuccess, setApprovalSuccess] = useState(false);

  const proposalId = searchParams.get('id');

  useEffect(() => {
    if (proposalId) {
      fetch(`/api/v1/proposals/${proposalId}`)
        .then((res) => {
          if (!res.ok) throw new Error("Failed to fetch");
          return res.json();
        })
        .then((json) => setData(json))
        .catch((e) => {
          console.error("Failed to load proposal data", e);
          setError(true);
        });
    } else {
      // Fallback for older links or backwards compatibility
      const encodedData = searchParams.get('data');
      if (encodedData) {
        try {
          const base64Str = encodedData.replace(/-/g, '+').replace(/_/g, '/');
          const utf8Encoded = escape(atob(base64Str));
          const decoded = JSON.parse(decodeURIComponent(utf8Encoded));
          setData(decoded);
        } catch (e) {
          console.error("Failed to decode proposal data");
          setError(true);
        }
      } else {
        setError(true);
      }
    }
  }, [proposalId, searchParams]);

  const handleApprove = async () => {
    if (!proposalId) {
       alert("Cannot approve a legacy proposal link. Please ask for a new link.");
       return;
    }

    setIsApproving(true);
    try {
      const res = await fetch(`/api/v1/proposals/${proposalId}/approve`, {
        method: 'POST',
      });

      if (!res.ok) {
        throw new Error("Failed to approve proposal");
      }

      setApprovalSuccess(true);
    } catch (e) {
      console.error(e);
      alert("An error occurred while approving the proposal.");
    } finally {
      setIsApproving(false);
    }
  };

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center font-inter bg-gray-50">
        <div className="text-red-500 font-medium">Error: Invalid or corrupted proposal data.</div>
      </div>
    );
  }

  if (!data) {
    return (
      <div className="min-h-screen flex items-center justify-center font-inter bg-gray-50">
        <div className="animate-pulse text-gray-500 font-medium">Loading Proposal...</div>
      </div>
    );
  }

  const tenant = data.tenant || data.tenant_id || "my-store";
  const clientName = data.clientName || data.client_name || "Client";
  const projectScope = data.projectScope || data.project_scope || "Project details";
  const amount = data.amount || (data.total_amount_cents ? (data.total_amount_cents / 100).toString() : "0");
  const timeline = data.timeline || data.estimated_timeline || "TBD";

  const today = new Date().toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' });

  if (approvalSuccess) {
     return (
        <div className="min-h-screen flex items-center justify-center font-inter bg-gray-50 p-4">
           <div className="bg-white rounded-2xl shadow-lg border border-gray-100 p-8 md:p-12 text-center max-w-lg animate-fade-in">
              <div className="text-5xl mb-6">🎉</div>
              <h2 className="text-2xl font-bold text-gray-900 font-outfit mb-4">Proposal Approved!</h2>
              <p className="text-gray-600 mb-8">
                 Thank you for approving the proposal. An invoice has been automatically generated and sent to you.
              </p>
              <div className="text-center flex flex-col items-center">
                 <PoweredByOHC tenantId={tenant} />
              </div>
           </div>
           <style dangerouslySetInnerHTML={{__html: `
            @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@400;500;600;700;800&display=swap');
            .font-inter { font-family: 'Inter', sans-serif; }
            .font-outfit { font-family: 'Outfit', sans-serif; }
            @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
            .animate-fade-in { animation: fadeIn 0.5s ease-out forwards; }
          `}} />
        </div>
     );
  }

  return (
    <div className="min-h-screen bg-gray-50 py-10 px-4 sm:px-6 lg:px-8 font-inter">
      <main className="max-w-3xl mx-auto">
        <section className="bg-white rounded-2xl shadow-lg border border-gray-100 p-8 md:p-12 mb-8 relative overflow-hidden">
          {/* Decorative element */}
          <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -mr-10 -mt-10"></div>

          <div className="flex justify-between items-start mb-12 relative z-10">
            <div>
              <h1 className="text-4xl font-extrabold text-indigo-900 font-outfit mb-2">PROJECT PROPOSAL</h1>
              <p className="text-gray-500 font-medium">Date: {today}</p>
            </div>
          </div>

          <div className="mb-10 p-6 bg-gray-50 rounded-xl border border-gray-100">
            <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-2">Prepared For</h2>
            <p className="text-2xl font-bold text-gray-900">{clientName}</p>
          </div>

          <div className="mb-10">
             <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-3">Project Scope & Details</h2>
             <div className="text-gray-800 leading-relaxed whitespace-pre-wrap">
               {projectScope}
             </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
              <div className="p-6 bg-indigo-50 rounded-xl border border-indigo-100">
                  <h2 className="text-sm font-bold text-indigo-400 uppercase tracking-wider mb-2">Estimated Timeline</h2>
                  <p className="text-xl font-bold text-indigo-900">{timeline}</p>
              </div>
          </div>

          <div className="flex justify-between items-center border-t border-gray-200 pt-8 mt-8">
            <span className="text-xl font-bold text-gray-900 font-outfit uppercase tracking-wider">Total Investment</span>
            <span className="text-4xl font-extrabold text-indigo-600 font-outfit">${parseFloat(amount).toFixed(2)}</span>
          </div>

          <button
             onClick={handleApprove}
             disabled={isApproving}
             className="w-full mt-10 py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-lg hover:shadow-xl transition-all text-lg flex items-center justify-center gap-2 transform hover:-translate-y-1 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
            {isApproving ? "Approving..." : "Approve Proposal"}
          </button>
        </section>

        {/* Viral Growth Loop Footer */}
        <div className="text-center pb-8 animate-fade-in flex flex-col items-center">
          <PoweredByOHC tenantId={tenant} />
          <Link
            href={`/onboarding?ref=${tenant}&source=proposal_generator`}
            target="_blank"
            className="inline-flex flex-col items-center gap-1 group mt-3"
          >
            <span className="text-sm font-medium text-indigo-600 group-hover:text-indigo-800 transition-colors bg-indigo-50 px-4 py-2 rounded-full border border-indigo-100 hover:border-indigo-200">
              Create your own professional proposals for free →
            </span>
          </Link>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@400;500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.5s ease-out forwards; }
      `}} />
    </div>
  );
}

export default function ProposalViewPage() {
  return (
    <Suspense fallback={<div className="min-h-screen bg-gray-50 flex items-center justify-center">Loading...</div>}>
      <ProposalViewContent />
    </Suspense>
  );
}