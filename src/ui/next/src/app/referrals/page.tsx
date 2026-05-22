"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";

export default function Referrals() {
  const router = useRouter();
  const [inviteCopied, setInviteCopied] = useState(false);
  const [tenant, setTenant] = useState("DEFAULT");

  useEffect(() => {
    const t = localStorage.getItem("tenant");
    if (t) {
      setTenant(t);
    }
  }, []);

  const handleCopyLink = () => {
    navigator.clipboard.writeText(`ohc://join?ref=${tenant}`);
    window.alert("Copied");
  };

  const handleCopyInvite = () => {
    navigator.clipboard.writeText(
      `Join me on One Human Corp! ohc://join?ref=${tenant}`
    );
    setInviteCopied(true);
    setTimeout(() => setInviteCopied(false), 3000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-4">
             <button onClick={() => router.back()} className="text-gray-500 hover:text-gray-900 transition-colors">
                 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
             </button>
             <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Referral Dashboard</h1>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">

        <section className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-4">
            <h2 className="text-xl font-semibold font-outfit text-gray-900">Your Referral Link</h2>
            <div className="flex flex-col sm:flex-row gap-3">
                <div
                    id="referral-link"
                    className="flex-1 bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-700 font-mono flex items-center"
                >
                    ohc://join?ref={tenant}
                </div>
                <button
                    onClick={handleCopyLink}
                    className="px-6 py-3 bg-gray-900 hover:bg-black text-white font-semibold rounded-xl shadow-sm transition-all"
                >
                    Copy
                </button>
            </div>
        </section>

        <section className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-6">
            <div className="flex justify-between items-center">
                <h2 className="text-xl font-semibold font-outfit text-gray-900">Share Tools</h2>
            </div>

            <div className="flex flex-col gap-4">
                <button
                    onClick={handleCopyInvite}
                    className="w-full sm:w-auto px-6 py-3 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 font-semibold rounded-xl transition-all flex justify-center items-center gap-2"
                >
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                    Copy Invite Message
                </button>
                {inviteCopied && (
                    <p className="text-green-600 text-sm font-medium animate-pulse text-center sm:text-left">Invite message copied!</p>
                )}

                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
                    <button className="flex items-center justify-center gap-2 px-4 py-3 bg-gradient-to-r from-purple-500 via-pink-500 to-orange-500 text-white font-semibold rounded-xl hover:opacity-90 transition-opacity">
                        Share to Instagram
                    </button>
                    {/* Placeholder for other networks */}
                    <button className="flex items-center justify-center gap-2 px-4 py-3 bg-[#1DA1F2] text-white font-semibold rounded-xl hover:opacity-90 transition-opacity">
                        Share to X
                    </button>
                    <button className="flex items-center justify-center gap-2 px-4 py-3 bg-[#25D366] text-white font-semibold rounded-xl hover:opacity-90 transition-opacity">
                        Share to WhatsApp
                    </button>
                </div>
            </div>
        </section>

        <section className="flex flex-col sm:flex-row gap-4 mt-4">
            <button className="flex-1 px-6 py-4 bg-white border border-gray-200 text-gray-700 hover:bg-gray-50 font-semibold rounded-xl shadow-sm transition-all flex justify-center items-center gap-2">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" /></svg>
                View Referral Logs
            </button>
            <button className="flex-1 px-6 py-4 bg-white border border-gray-200 text-gray-700 hover:bg-gray-50 font-semibold rounded-xl shadow-sm transition-all flex justify-center items-center gap-2">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
                Export Data
            </button>
        </section>

      </main>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
