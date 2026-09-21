"use client";

import { useState,useEffect } from 'react';
import { useParams } from 'next/navigation';
import { safeBioHref } from '@/lib/bioLinks';

interface Link {
  title: string;
  url: string;
}

interface BioConfig {
  store_name: string;
  bio: string;
  theme: 'light' | 'dark';
  links: Link[];
  remove_branding?: boolean;
}

export default function PublicBioPage() {
  const params = useParams();
  const tenant = params.tenant as string;
  const [config, setConfig] = useState<BioConfig | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchConfig = async () => {
      try {
        const res = await fetch(`/api/v1/growth/link-in-bio/${tenant}`);
        if (res.ok) {
          const data = await res.json();
          setConfig(data);
        } else {
          // Defaults if not found
          setConfig({
            store_name: tenant,
            bio: 'Welcome to my storefront!',
            theme: 'light',
            links: [{ title: 'Visit Store', url: '/' }]
          });
        }
      } catch  {
         setConfig({
            store_name: tenant,
            bio: 'Welcome to my storefront!',
            theme: 'light',
            links: [{ title: 'Visit Store', url: '/' }]
          });
      } finally {
        setLoading(false);
      }
    };
    if (tenant) {
      fetchConfig();
    }
  }, [tenant]);

  if (loading || !config) {
    return <div className="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-black text-gray-500">Loading...</div>;
  }

  const { store_name, bio, theme, links } = config;

  return (
    <div className={`min-h-screen w-full flex justify-center ${theme === 'dark' ? 'bg-[#111111] text-white' : 'bg-[#F5F5F7] text-gray-900'} font-inter`}>
      <div className="w-full max-w-md px-6 py-12 flex flex-col items-center">

        {/* Avatar Placeholder */}
        <div className="w-24 h-24 rounded-full bg-gradient-to-br from-indigo-400 to-purple-500 mb-6 shadow-xl flex items-center justify-center text-4xl text-white font-bold">
          {store_name.charAt(0).toUpperCase()}
        </div>

        <h1 className="text-3xl font-bold font-outfit text-center mb-3 tracking-tight">{store_name}</h1>
        <p className={`text-center mb-10 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-600'}`}>{bio}</p>

        <div className="w-full space-y-4 flex-1">
          {links && links.map((link, i) => {
            const href = safeBioHref(link.url);
            const className = `block w-full py-4 px-6 rounded-2xl text-center font-bold text-lg ${theme === 'dark' ? 'bg-[#222222] text-white' : 'bg-white text-gray-900 shadow-md'}`;
            return href ? (
              <a key={i} href={href} target="_blank" rel="noopener noreferrer" className={`${className} transition-transform hover:scale-[1.02]`}>
                {link.title}
              </a>
            ) : (
              <div key={i} className={className}>
                <span>{link.title}</span>
                <p className="mt-1 text-xs font-normal">Link unavailable.</p>
              </div>
            );
          })}
        </div>

        {/* Viral Loop / Soft Paywall */}
        {!config.remove_branding && (
          <div className="mt-12 pt-8">
            <a
              href={`/onboarding?ref=${encodeURIComponent(tenant)}&source=bio_footer`}
              className={`text-sm font-semibold flex items-center justify-center gap-1 hover:underline ${theme === 'dark' ? 'text-gray-500 hover:text-gray-300' : 'text-gray-400 hover:text-gray-600'}`}
            >
              ⚡ Powered by OmniSolo
            </a>
          </div>
        )}
      </div>
    </div>
  );
}
