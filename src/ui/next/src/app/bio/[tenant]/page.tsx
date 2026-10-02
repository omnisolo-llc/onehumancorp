"use client";

import { useState,useEffect } from 'react';
import { isSupportedBioUrl } from '@/lib/bioLinks';
import { useParams } from 'next/navigation';

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
  const tenant = typeof params.tenant === 'string' ? params.tenant : '';
  const [error, setError] = useState('');
  const [config, setConfig] = useState<BioConfig | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let active = true; let expiry: ReturnType<typeof setTimeout> | undefined;
    const controller = new AbortController();
    const retired = () => {active=false;controller.abort();clearTimeout(expiry);setConfig(null);setLoading(false);setError('Your session changed. Reload to verify private preview access.');};
    const storage = (event: StorageEvent) => {if(event.key === null || event.key === 'omnisolo_queue_identity_epoch_v2') retired();};
    window.addEventListener('omnisolo_auth_changed',retired);window.addEventListener('storage',storage);window.addEventListener('pagehide',retired);
    setConfig(null);setLoading(true);setError('');
    void(async()=>{
      try {
        const identityResponse = await fetch('/api/v1/auth/session-identity',{credentials:'same-origin',cache:'no-store',redirect:'error',signal:controller.signal});
        const owner = await identityResponse.json();
        if(!active)return;
        if(identityResponse.status !== 200 || owner?.error != null || owner?.success === false || typeof owner?.userId !== 'string' || !owner.userId || owner.tenantId !== tenant || !tenant || !Number.isFinite(owner.expiresAt) || owner.expiresAt <= Date.now()) throw new Error('Sign in to the owning business to view this private profile.');
        const headers = new Headers({'x-ohc-expected-user':owner.userId,'x-ohc-expected-tenant':owner.tenantId});
        if(headers.get('x-ohc-expected-user')!==owner.userId || headers.get('x-ohc-expected-tenant')!==owner.tenantId)throw new Error('Private profile identity is unavailable.');
        expiry=setTimeout(retired,Math.min(owner.expiresAt-Date.now(),2147483647));
        const response = await fetch(`/api/v1/growth/link-in-bio/${encodeURIComponent(tenant)}`,{headers,credentials:'same-origin',cache:'no-store',redirect:'error',signal:controller.signal});
        if(response.status === 404)throw new Error('No saved private profile exists for this business.');
        if(response.status === 401 || response.status === 403)throw new Error('Sign in to the owning business to view this private profile.');
        if(response.status !== 200)throw new Error('The private profile is unavailable. Try loading it again later.');
        const data = await response.json();
        if(!active)return;
        if(data?.error != null || data?.success === false || typeof data?.store_name !== 'string' || typeof data.bio !== 'string' || typeof data.theme !== 'string' || !Array.isArray(data.links) || data.links.some((link:Link|null)=>!link || typeof link.title !== 'string' || typeof link.url !== 'string'))throw new Error('The private profile is unavailable.');
        setConfig(data);
      } catch (cause) {if(active){setConfig(null);setError(cause instanceof Error ? cause.message : 'The private profile is unavailable.');}}
      finally{if(active)setLoading(false);}
    })();
    return()=>{active=false;controller.abort();clearTimeout(expiry);window.removeEventListener('omnisolo_auth_changed',retired);window.removeEventListener('storage',storage);window.removeEventListener('pagehide',retired);};
  }, [tenant]);

  if(error)return <div role="alert" className="min-h-screen flex items-center justify-center">{error}</div>;

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
        <p className={`leading-relaxed text-center mb-10 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-600'}`}>{bio}</p>

        <div className="w-full space-y-4 flex-1">
          {links && links.map((link, i) => (
            isSupportedBioUrl(link.url) ? <a
              key={i}
              href={link.url}
              target="_blank"
              rel="noopener noreferrer"
              className={`block w-full py-4 px-6 rounded-2xl text-center font-bold text-lg transition-transform hover:scale-[1.02] ${theme === 'dark' ? 'bg-[#222222] text-white hover:bg-[#333333]' : 'bg-white text-gray-900 shadow-md hover:shadow-lg'}`}
            >
              {link.title}
            </a> : <span key={i} aria-disabled="true" className="block py-4 px-6 text-center">{link.title} (unavailable)</span>
          ))}
        </div>

        {/* Viral Loop / Soft Paywall */}
        {!config.remove_branding && (
          <div className="mt-12 pt-8">
            <a
              href={`/onboarding?ref=${encodeURIComponent(`linkinbio_${tenant}`)}`}
              className={`text-sm font-semibold flex items-center justify-center gap-1 hover:underline ${theme === 'dark' ? 'text-gray-500 hover:text-gray-300' : 'text-gray-400 hover:text-gray-600'}`}
            >
              ⚡ OmniSolo
            </a>
          </div>
        )}
      </div>
    </div>
  );
}
