"use client";

import React, { useState, useEffect } from 'react';

export default function PublicProfilePage({ params }: { params: { tenantId: string } }) {
  const [profileData, setProfileData] = useState({
    name: 'OHC Demo Business',
    bio: 'Providing exceptional services and products.',
    tenantId: params?.tenantId || 'demo-tenant'
  });

  useEffect(() => {
    // We rely on the URL parameter now, but we can do a fallback
    if (params && params.tenantId) {
        setProfileData(prev => ({ ...prev, tenantId: params.tenantId }));
    }
  }, [params]);

  return (
    <div className="flex flex-col items-center min-h-screen bg-[#F5F5F7] font-inter relative pb-24">
      {/* Decorative background elements */}
      <div className="fixed top-0 left-0 w-full h-64 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 -z-10"></div>
      <div className="fixed top-20 left-1/4 w-64 h-64 bg-white/20 rounded-full blur-3xl pointer-events-none"></div>

      <main className="w-full max-w-[414px] mx-auto pt-16 px-4 flex flex-col items-center z-10">

        {/* Profile Header */}
        <div className="flex flex-col items-center text-center mb-8">
          <div className="w-24 h-24 rounded-full bg-white/80 backdrop-blur-md shadow-lg border border-white flex items-center justify-center text-3xl mb-4 p-1">
            <div className="w-full h-full rounded-full bg-gradient-to-tr from-indigo-100 to-purple-50 flex items-center justify-center font-outfit font-bold text-indigo-900">
               {profileData.name.substring(0, 2).toUpperCase()}
            </div>
          </div>
          <h1 className="text-2xl font-bold font-outfit text-white drop-shadow-md tracking-tight mb-2">
            {profileData.name}
          </h1>
          <p className="text-white/90 text-sm font-medium max-w-[280px] drop-shadow-sm">
            {profileData.bio}
          </p>
        </div>

        {/* Links Container (Glassmorphism) */}
        <div className="w-full flex flex-col gap-4">

          {/* Main Store Link */}
          <a
            href="/storefront-builder"
            className="group w-full p-4 rounded-2xl flex items-center justify-between shadow-lg transition-all active:scale-[0.98] hover:-translate-y-1"
            style={{
                background: 'rgba(255, 255, 255, 0.75)',
                backdropFilter: 'blur(30px) saturate(210%)',
                border: '1px solid rgba(255, 255, 255, 0.8)'
            }}
          >
            <div className="flex items-center gap-4">
              <div className="w-10 h-10 rounded-xl bg-indigo-100 flex items-center justify-center text-indigo-600 shadow-inner">
                🛍️
              </div>
              <span className="font-semibold text-gray-900">Shop Our Store</span>
            </div>
            <svg className="w-5 h-5 text-gray-400 group-hover:text-indigo-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
          </a>

          {/* Booking Link */}
          <a
            href="/booking"
            className="group w-full p-4 rounded-2xl flex items-center justify-between shadow-md transition-all active:scale-[0.98] hover:-translate-y-1"
            style={{
                background: 'rgba(255, 255, 255, 0.65)',
                backdropFilter: 'blur(20px) saturate(180%)',
                border: '1px solid rgba(255, 255, 255, 0.6)'
            }}
          >
            <div className="flex items-center gap-4">
              <div className="w-10 h-10 rounded-xl bg-purple-100 flex items-center justify-center text-purple-600 shadow-inner">
                📅
              </div>
              <span className="font-medium text-gray-800">Book an Appointment</span>
            </div>
            <svg className="w-5 h-5 text-gray-400 group-hover:text-purple-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
          </a>

          {/* Social / Contact Link */}
          <a
            href="#"
            className="group w-full p-4 rounded-2xl flex items-center justify-between shadow-sm transition-all active:scale-[0.98] hover:-translate-y-1"
            style={{
                background: 'rgba(255, 255, 255, 0.55)',
                backdropFilter: 'blur(15px) saturate(150%)',
                border: '1px solid rgba(255, 255, 255, 0.4)'
            }}
          >
            <div className="flex items-center gap-4">
              <div className="w-10 h-10 rounded-xl bg-blue-100 flex items-center justify-center text-blue-600 shadow-inner">
                ✉️
              </div>
              <span className="font-medium text-gray-800">Contact Us</span>
            </div>
            <svg className="w-5 h-5 text-gray-400 group-hover:text-blue-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
          </a>

        </div>

        {/* Social Icons row */}
        <div className="flex gap-4 mt-8">
            <a href="#" className="w-12 h-12 rounded-full bg-white/50 backdrop-blur-md shadow-sm border border-white/60 flex items-center justify-center hover:bg-white transition-colors">
                <span className="text-xl">📸</span>
            </a>
            <a href="#" className="w-12 h-12 rounded-full bg-white/50 backdrop-blur-md shadow-sm border border-white/60 flex items-center justify-center hover:bg-white transition-colors">
                <span className="text-xl">🐦</span>
            </a>
            <a href="#" className="w-12 h-12 rounded-full bg-white/50 backdrop-blur-md shadow-sm border border-white/60 flex items-center justify-center hover:bg-white transition-colors">
                <span className="text-xl">🎵</span>
            </a>
        </div>
      </main>

      {/* Sticky Viral Growth Widget */}
      <div className="fixed bottom-0 left-0 w-full p-4 bg-white/90 backdrop-blur-xl border-t border-gray-200 shadow-[0_-10px_40px_rgba(0,0,0,0.05)] z-50 flex justify-center">
         <a
            href={`ohc://join?ref=${profileData.tenantId}-profile`}
            className="group flex flex-col sm:flex-row items-center gap-2 text-gray-600 hover:text-gray-900 transition-colors py-1 px-4 max-w-[414px] w-full"
         >
            <div className="flex items-center gap-2">
                <span className="text-sm font-medium tracking-wide">Powered by</span>
                <span className="font-outfit font-black tracking-tighter text-lg text-indigo-600 group-hover:text-indigo-700">OHC</span>
            </div>
            <div className="h-1 w-1 bg-gray-300 rounded-full hidden sm:block"></div>
            <span className="text-xs font-semibold uppercase tracking-wider text-indigo-500 group-hover:text-indigo-600 bg-indigo-50 px-2 py-0.5 rounded flex items-center gap-1">
                Create your free profile
                <svg className="w-3 h-3 transition-transform group-hover:translate-x-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
            </span>
         </a>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}