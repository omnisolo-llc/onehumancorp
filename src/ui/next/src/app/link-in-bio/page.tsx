"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function LinkInBioPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('Store');
  const [bio, setBio] = useState('Welcome to my business! Check out my offerings below.');
  const [links, setLinks] = useState<{ id: string, title: string, url: string }[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchBioData() {
      try {
        const urlParams = new URLSearchParams(window.location.search);
        let t = urlParams.get('tenant');
        if (!t && typeof localStorage !== 'undefined') {
          t = localStorage.getItem('tenant');
        }
        if (!t) t = 'my-store';

        setTenant(t);

        const res = await fetch(`/api/v1/growth/storefront/link-in-bio?tenant=${encodeURIComponent(t)}`);
        if (res.ok) {
          const data = await res.json();
          if (data.bio) setBio(data.bio);
          if (data.links && data.links.length > 0) {
              setLinks(data.links);
          } else {
               setLinks([
                  { id: '1', title: 'Book a Consultation', url: `/booking?tenant=${encodeURIComponent(t)}` },
                  { id: '2', title: 'Shop Products', url: `/checkout?tenant=${encodeURIComponent(t)}` },
                  { id: '3', title: 'Contact Me', url: `/inbox?tenant=${encodeURIComponent(t)}` }
               ]);
          }
        } else {
          // Fallback
          setLinks([
              { id: '1', title: 'Book a Consultation', url: `/booking?tenant=${encodeURIComponent(t)}` },
              { id: '2', title: 'Shop Products', url: `/checkout?tenant=${encodeURIComponent(t)}` },
              { id: '3', title: 'Contact Me', url: `/inbox?tenant=${encodeURIComponent(t)}` }
          ]);
        }
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    }
    fetchBioData();
  }, []);

  if (loading) {
      return (
          <div className="flex min-h-screen items-center justify-center bg-gray-50 font-inter">
              <div className="text-gray-500">Loading...</div>
          </div>
      );
  }

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <main className="flex-1 w-full max-w-md mx-auto py-12 px-6 flex flex-col items-center">
        {/* Profile Avatar */}
        <div className="w-24 h-24 rounded-full bg-gradient-to-tr from-indigo-500 to-purple-500 flex items-center justify-center text-3xl font-bold text-white shadow-lg mb-6">
          {tenant.substring(0, 2).toUpperCase()}
        </div>

        {/* Title & Bio */}
        <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2 text-center">
          {tenant}
        </h1>
        <p className="text-gray-600 text-center mb-8 px-4">
          {bio}
        </p>

        {/* Links */}
        <div className="w-full flex flex-col gap-4 mb-12">
          {links.map((link) => (
            <button
              key={link.id}
              onClick={() => router.push(link.url)}
              className="w-full py-4 px-6 bg-white/60 backdrop-blur-xl border border-white/40 shadow-sm rounded-2xl hover:bg-white/80 hover:scale-[1.02] hover:shadow-md transition-all duration-200 flex items-center justify-center font-semibold text-gray-800"
            >
              {link.title}
            </button>
          ))}
        </div>

        {/* Footer Badge (Viral Loop) */}
        <div className="mt-auto pt-8">
            <button
                onClick={() => router.push(`/join?ref=${encodeURIComponent(tenant)}`)}
                className="px-4 py-2 bg-gray-900 text-white rounded-full text-xs font-bold hover:bg-black transition-colors flex items-center gap-2"
            >
                ⚡ Powered by OHC
            </button>
        </div>
      </main>
    </div>
  );
}
