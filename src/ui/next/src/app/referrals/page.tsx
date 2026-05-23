"use client";

import { useState } from "react";
import Link from "next/link";

export default function ReferralDashboard() {
  const [inviteCopied, setInviteCopied] = useState(false);

  const handleCopyInvite = () => {
    navigator.clipboard.writeText("Launch your business online instantly with OHC! Use my invite link: ohc://join?ref=DEFAULT"); setInviteCopied(true);
    setTimeout(() => setInviteCopied(false), 2000);
  };

  const handleCopyLink = () => {
    navigator.clipboard.writeText("ohc://join?ref=DEFAULT"); alert("Copied");
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Referral Dashboard</h1>
        <Link href="/dashboard" className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md text-sm font-medium hover:bg-gray-200 transition-colors">
          Back to Dashboard
        </Link>
      </header>

      <main className="flex-1 max-w-4xl w-full mx-auto p-4 sm:p-6 lg:p-8 space-y-8">

        {/* Hero Section */}
        <div className="bg-white rounded-2xl p-6 sm:p-8 shadow-sm border border-gray-100">
          <div className="flex flex-col md:flex-row gap-8 items-start md:items-center">
            <div className="flex-1 space-y-4">
              <h2 className="text-3xl font-bold font-outfit text-gray-900">Invite & Earn</h2>
              <p className="text-gray-600 leading-relaxed">
                Share OHC with other business owners. When they sign up using your link, you both get a $50 credit towards premium tools.
              </p>
            </div>
            <div className="w-full md:w-auto bg-indigo-50 p-4 rounded-xl border border-indigo-100">
              <p className="text-sm font-semibold text-indigo-900 mb-2">Your Referral Link</p>
              <div className="flex flex-col sm:flex-row gap-2">
                <div id="referral-link" className="bg-white px-4 py-3 rounded-lg text-indigo-600 font-mono text-sm border border-indigo-200 flex-1 truncate">
                  ohc://join?ref=DEFAULT
                </div>
                <button
                  onClick={handleCopyLink}
                  className="bg-indigo-600 text-white px-6 py-3 rounded-lg font-semibold hover:bg-indigo-700 transition-colors whitespace-nowrap"
                >
                  Copy
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Share Tools */}
        <div className="bg-white rounded-2xl p-6 sm:p-8 shadow-sm border border-gray-100">
          <h3 className="text-xl font-bold font-outfit text-gray-900 mb-6">Share Tools</h3>

          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            <button onClick={() => { navigator.clipboard.writeText("ohc://join?ref=DEFAULT"); alert("Link copied to clipboard for Instagram!"); }} className="flex items-center justify-center gap-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white p-4 rounded-xl font-semibold hover:opacity-90 transition-opacity">
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zM12 0C8.741 0 8.333.014 7.053.072 2.695.272.273 2.69.073 7.052.014 8.333 0 8.741 0 12c0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98C8.333 23.986 8.741 24 12 24c3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98C15.668.014 15.259 0 12 0zm0 5.838a6.162 6.162 0 100 12.324 6.162 6.162 0 000-12.324zM12 16a4 4 0 110-8 4 4 0 010 8zm6.406-11.845a1.44 1.44 0 100 2.881 1.44 1.44 0 000-2.881z"/></svg>
              Share to Instagram
            </button>

            <a href="https://wa.me/?text=Launch%20your%20business%20online%20instantly%20with%20OHC!%20Use%20my%20invite%20link:%20ohc://join?ref=DEFAULT" target="_blank" rel="noopener noreferrer" className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-4 rounded-xl font-semibold hover:bg-[#20bd5a] transition-colors">
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
              Share via WhatsApp
            </a>

            <button
              onClick={handleCopyInvite}
              className="flex items-center justify-center gap-2 bg-gray-900 text-white p-4 rounded-xl font-semibold hover:bg-black transition-colors w-full"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/></svg>
              Copy Invite Message
            </button>
          </div>

          {inviteCopied && (
            <p className="mt-4 text-center text-green-600 font-medium">
              Invite message copied!
            </p>
          )}
        </div>

        {/* Management Tools */}
        <div className="bg-white rounded-2xl p-6 sm:p-8 shadow-sm border border-gray-100 flex flex-col sm:flex-row gap-4 justify-between items-center">
          <div>
            <h3 className="text-lg font-bold text-gray-900">Manage Referrals</h3>
            <p className="text-gray-500 text-sm">Track your successful invites and rewards</p>
          </div>
          <div className="flex gap-3 w-full sm:w-auto">
            <button className="flex-1 sm:flex-none px-4 py-2 border border-gray-200 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors">
              View Referral Logs
            </button>
            <button className="flex-1 sm:flex-none px-4 py-2 border border-gray-200 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors">
              Export Data
            </button>
          </div>
        </div>

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
