"use client";

import { useState, useEffect, useRef, useCallback } from 'react';
import { isSupportedBioUrl } from '@/lib/bioLinks';
import { useClipboardFeedback } from '@/hooks/useClipboardFeedback';
import { useRouter } from 'next/navigation';
import { PoweredByOmniSolo } from '../components/PoweredByOmniSolo';

type BioOwner = {userId: string; tenantId: string; expiresAt: number};
async function verifiedIdentity(): Promise<BioOwner> {
  const response = await fetch('/api/v1/auth/session-identity', {credentials:'same-origin',cache:'no-store',redirect:'error'});
  const data = await response.json();
  if (response.status !== 200 || data?.error != null || data?.success === false || typeof data?.userId !== 'string' || !data.userId || typeof data?.tenantId !== 'string' || !data.tenantId || !Number.isFinite(data.expiresAt) || data.expiresAt <= Date.now()) throw new Error('Verified profile identity unavailable');
  return data;
}
function ownerHeaders(owner: BioOwner, jsonBody = false): Headers {
  const headers = new Headers({'x-ohc-expected-user':owner.userId,'x-ohc-expected-tenant':owner.tenantId});
  if (jsonBody) headers.set('Content-Type','application/json');
  if (headers.get('x-ohc-expected-user') !== owner.userId || headers.get('x-ohc-expected-tenant') !== owner.tenantId) throw new Error('Profile identity cannot be represented');
  return headers;
}

export default function LinkInBioGeneratorPage() {
  const router = useRouter();
  const [storeName, setStoreName] = useState('');
  const [bio, setBio] = useState('');
  const [theme, setTheme] = useState('light');
  const [links, setLinks] = useState<{title:string;url:string}[]>([]);
  const [owner, setOwner] = useState<BioOwner | null>(null);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [phase, setPhase] = useState<'loading'|'ready'|'saving'|'held'>('loading');
  const [status, setStatus] = useState('Verifying private profile access…');
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [hasSaved, setHasSaved] = useState(false);
  const epoch = useRef(0);
  const active = useRef(false);
  const saving = useRef(false);
  const isSaving = phase === 'saving';
  const linkUrl = owner && typeof window !== 'undefined' ? `${window.location.origin}/bio/${encodeURIComponent(owner.tenantId)}` : '';
  const clipboard = useClipboardFeedback(linkUrl + JSON.stringify([storeName,bio,theme,links,removeBranding]));
  const retire = useCallback(() => {
    ++epoch.current; saving.current=false; setOwner(null); setStoreName(''); setBio(''); setLinks([]); setTheme('light'); setRemoveBranding(false); setSaveSuccess(false); setHasSaved(false); setPhase('held');
    setStatus('Your session changed. Reload to verify private profile access.');
  }, []);
  useEffect(() => {
    active.current = true; const current = ++epoch.current; let timer: ReturnType<typeof setTimeout> | undefined;
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === 'omnisolo_queue_identity_epoch_v2') retire(); };
    window.addEventListener('omnisolo_auth_changed',retire); window.addEventListener('storage',storage); window.addEventListener('pagehide',retire);
    void (async () => {
      try {
        const identity = await verifiedIdentity();
        if (!active.current || current !== epoch.current) return;
        const headers = ownerHeaders(identity); setOwner(identity);
        timer = setTimeout(retire, Math.min(identity.expiresAt-Date.now(),2_147_483_647));
        const response = await fetch(`/api/v1/growth/link-in-bio/${encodeURIComponent(identity.tenantId)}`, {headers,credentials:'same-origin',cache:'no-store',redirect:'error'});
        if (response.status === 404) {
          await response.text();
          if (!active.current || current !== epoch.current) return;
          setPhase('ready'); setStatus('No saved private profile. Enter your details to save one.'); return;
        }
        if (response.status !== 200) throw new Error('Private profile unavailable');
        const data = await response.json();
        if (!active.current || current !== epoch.current) return;
        if (data?.error != null || data?.success === false || typeof data?.store_name !== 'string' || typeof data.bio !== 'string' || typeof data.theme !== 'string' || !Array.isArray(data.links) || data.links.some((link: {title?:unknown;url?:unknown}|null) => !link || typeof link.title !== 'string' || typeof link.url !== 'string') || (data.remove_branding !== undefined && typeof data.remove_branding !== 'boolean')) throw new Error('Private profile is incomplete');
        setStoreName(data.store_name); setBio(data.bio); setTheme(data.theme); setLinks(data.links); setRemoveBranding(data.remove_branding ?? false); setHasSaved(true); setPhase('ready'); setStatus('Saved private profile loaded. Public publication is not available.');
      } catch {
        if (active.current && current === epoch.current) {setPhase('held'); setStatus('Private profile could not be loaded. Reload before editing existing configuration.');}
      }
    })();
    return () => {active.current=false; ++epoch.current; clearTimeout(timer); window.removeEventListener('omnisolo_auth_changed',retire); window.removeEventListener('storage',storage); window.removeEventListener('pagehide',retire);};
  }, [retire]);

  const handleAddLink = () => {
    setLinks([...links, { title: 'New Link', url: 'https://' }]);
  };

  const handleLinkChange = (index: number, field: 'title' | 'url', value: string) => {
    const newLinks = [...links];
    newLinks[index] = { ...newLinks[index], [field]: value };
    setLinks(newLinks);
  };

  const handleRemoveLink = (index: number) => {
    const newLinks = links.filter((_, i) => i !== index);
    setLinks(newLinks);
  };

  const handleSave = async () => {
    if (phase !== 'ready' || !owner || saving.current) return;
    if (links.some(link => !isSupportedBioUrl(link.url))) {setStatus('Each link needs an absolute HTTP or HTTPS URL without whitespace or control characters.'); return;}
    saving.current=true;
    const current = epoch.current; const intended = {...owner};
    const payload = {tenant_id:intended.tenantId,store_name:storeName,bio,theme,links:links.map((link,index)=>({id:String(index+1),title:link.title,url:link.url})),remove_branding:removeBranding};
    setPhase('saving'); setSaveSuccess(false); setStatus('Saving private configuration…');
    try {
      let identity: BioOwner;
      try {identity = await verifiedIdentity();}
      catch {if (active.current && current === epoch.current) retire(); return;}
      if (!active.current || current !== epoch.current) return;
      if (identity.userId !== intended.userId || identity.tenantId !== intended.tenantId) {retire(); return;}
      const response = await fetch('/api/v1/growth/link-in-bio',{method:'POST',headers:ownerHeaders(intended,true),body:JSON.stringify(payload),credentials:'same-origin',cache:'no-store',redirect:'error'});
      if (!active.current || current !== epoch.current) return;
      if (response.status === 401 || response.status === 403) {retire(); return;}
      const body = await response.text();
      if (!active.current || current !== epoch.current) return;
      if (response.status === 409) {
        let error: unknown;
        try {error = JSON.parse(body)?.error;} catch { /* An unreadable conflict is an unknown save outcome. */ }
        if (error === 'queued owner does not match the current session' || error === 'session_identity_changed') {retire(); return;}
      }
      if (response.status !== 200 || body !== '') throw new Error('Unconfirmed profile save');
      setSaveSuccess(true); setHasSaved(true); setPhase('ready'); setStatus('Saved private configuration. Public publication is not available.');
    } catch {
      if (active.current && current === epoch.current) {setPhase('held');setStatus('The private save could not be confirmed. Reload and review stored data before trying again.');}
    } finally {if (current === epoch.current) saving.current=false;}
  };

  const handleCopy = () => {if (phase === 'ready' && hasSaved && linkUrl) void clipboard.copy(linkUrl);};

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] p-4 md:p-8 font-inter">
      <div className="max-w-6xl mx-auto">
        <button onClick={() => router.back()} className="mb-6 flex items-center text-sm font-semibold text-gray-500 hover:text-gray-900 dark:hover:text-white transition-colors">
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Dashboard
        </button>

        <div className="flex items-center gap-3 mb-8">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-indigo-400 to-purple-600 flex items-center justify-center text-white text-2xl shadow-lg">
            🔗
          </div>
          <div>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white tracking-tight">Link in Bio Generator</h1>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">Edit your private business profile. Public publication is not available.</p>
          </div>
        </div>

        <div className="flex flex-col lg:flex-row gap-8">
          {/* Builder Controls */}
          <fieldset disabled={phase !== 'ready'} className="flex-1 space-y-6 border-0 p-0 m-0 min-w-0">
            <div className="glassmorphism rounded-2xl p-6 bg-white border border-gray-100 shadow-sm dark:bg-[#2C2C2E] dark:border-white/10">
              <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-4">Profile Info</h2>

              <div className="space-y-4">
                <div>
                  <label htmlFor="storeNameInput" className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Store / Creator Name</label>
                  <input
                    id="storeNameInput"
                    type="text"
                    value={storeName}
                    onChange={(e) => setStoreName(e.target.value)}
                    aria-label="Store / Creator Name Business name"
                    className="w-full px-4 py-2 bg-gray-50 dark:bg-[#1C1C1E] border border-gray-200 dark:border-white/10 rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none text-gray-900 dark:text-white"
                  />
                </div>
                <div>
                  <label htmlFor="bioInput" className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Bio / Description</label>
                  <textarea
                    id="bioInput"
                    value={bio}
                    onChange={(e) => setBio(e.target.value)}
                    aria-label="Bio / Description Bio tagline"
                    className="w-full px-4 py-2 bg-gray-50 dark:bg-[#1C1C1E] border border-gray-200 dark:border-white/10 rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none text-gray-900 dark:text-white h-24"
                  />
                </div>
              </div>
            </div>

            <div className="glassmorphism rounded-2xl p-6 bg-white border border-gray-100 shadow-sm dark:bg-[#2C2C2E] dark:border-white/10">
              <div className="flex items-center justify-between mb-4">
                 <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white">Your Links</h2>
                 <button onClick={handleAddLink} className="text-sm font-semibold text-indigo-600 dark:text-indigo-400 hover:underline">+ Add Link</button>
              </div>

              <div className="space-y-4">
                {links.map((link, index) => (
                  <div key={index} className="flex flex-col gap-2 p-4 bg-gray-50 dark:bg-[#1C1C1E] rounded-xl border border-gray-100 dark:border-white/5">
                    <div className="flex justify-between items-center">
                        <span className="text-xs font-bold text-gray-400 uppercase tracking-wider">Link {index + 1}</span>
                        {links.length > 1 && (
                            <button onClick={() => handleRemoveLink(index)} className="text-[#FF3B30] hover:text-red-700 text-sm">Remove</button>
                        )}
                    </div>
                    <input
                        type="text"
                        value={link.title}
                        onChange={(e) => handleLinkChange(index, 'title', e.target.value)}
                        placeholder={index === 0 ? "Link Title (e.g. Shop My Collection)" : "Additional Link Title"}
                        aria-label={`Link ${index + 1} Title`}
                        className="w-full px-3 py-2 bg-white dark:bg-[#2C2C2E] border border-gray-200 dark:border-white/10 rounded-lg text-sm outline-none text-gray-900 dark:text-white"
                    />
                    <input
                        type="text"
                        value={link.url}
                        onChange={(e) => handleLinkChange(index, 'url', e.target.value)}
                        placeholder={index === 0 ? "URL (e.g. https://...)" : "Additional URL"}
                        aria-label={`Link ${index + 1} URL`}
                        className="w-full px-3 py-2 bg-white dark:bg-[#2C2C2E] border border-gray-200 dark:border-white/10 rounded-lg text-sm outline-none text-gray-900 dark:text-white"
                    />
                  </div>
                ))}
              </div>
            </div>

            <div className="glassmorphism rounded-2xl p-6 bg-white border border-gray-100 shadow-sm dark:bg-[#2C2C2E] dark:border-white/10">
              <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-4">Branding</h2>
              <div className="flex items-center gap-3">
                <input
                  type="checkbox"
                  id="removeBrandingCheckbox"
                  aria-label="Remove branding"
                  checked={removeBranding}
                  onChange={(e) => setRemoveBranding(e.target.checked)}
                  className="w-5 h-5 text-indigo-600 rounded border-gray-300 focus:ring-indigo-500"
                />
                <label htmlFor="removeBrandingCheckbox" className="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Remove "Powered by OmniSolo" branding
                </label>
              </div>
            </div>

            <div className="glassmorphism rounded-2xl p-6 bg-white border border-gray-100 shadow-sm dark:bg-[#2C2C2E] dark:border-white/10">
              <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-4">Theme</h2>
              <div className="flex gap-4">
                <button
                  onClick={() => setTheme('light')}
                  className={`flex-1 py-3 rounded-xl border-2 font-semibold transition-all ${theme === 'light' ? 'border-indigo-500 bg-indigo-50 text-indigo-700 dark:bg-indigo-500/20 dark:text-indigo-300' : 'border-gray-200 text-gray-600 dark:border-white/10 dark:text-gray-400'}`}
                >
                  Light
                </button>
                <button
                  onClick={() => setTheme('dark')}
                  className={`flex-1 py-3 rounded-xl border-2 font-semibold transition-all ${theme === 'dark' ? 'border-indigo-500 bg-indigo-50 text-indigo-700 dark:bg-indigo-500/20 dark:text-indigo-300' : 'border-gray-200 text-gray-600 dark:border-white/10 dark:text-gray-400'}`}
                >
                  Dark
                </button>
              </div>
            </div>

            <button
                onClick={handleSave}
                disabled={isSaving}
                className="w-full py-4 rounded-xl bg-indigo-600 hover:bg-indigo-700 text-white font-bold text-lg shadow-lg transition-all flex justify-center items-center gap-2"
            >
                {isSaving ? 'Saving...' : saveSuccess ? 'Saved private configuration' : 'Save private configuration'}
            </button>
            <p role="status" aria-label="Private profile status">{status}</p>
          </fieldset>

          {/* Live Preview */}
          <div className="w-full lg:w-[400px] flex-shrink-0">
             <div className="sticky top-8">
                <div className="flex items-center justify-between mb-4">
                    <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white">Private Preview</h2>
                    <button onClick={handleCopy} disabled={phase !== 'ready' || !hasSaved || clipboard.state === 'pending'} className="text-sm font-semibold text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-500/20 px-3 py-1 rounded-full hover:bg-indigo-100 transition-colors">
                        {clipboard.state === 'copied' ? 'Copied private preview link' : 'Copy saved private preview link'}
                    </button>
                </div>

                {clipboard.message && <p role={clipboard.state === 'error' ? 'alert' : 'status'} aria-label="Private profile clipboard">{clipboard.message}</p>}
                {/* Mobile Device Mockup */}
                <div className="relative w-[340px] h-[680px] mx-auto border-[12px] border-black rounded-[40px] shadow-2xl overflow-hidden bg-white">
                    <div className="absolute top-0 inset-x-0 h-6 bg-black z-20 rounded-b-3xl"></div> {/* Notch */}

                    <div className={`w-full h-full overflow-y-auto ${theme === 'dark' ? 'bg-[#111111] text-white' : 'bg-[#fafafa] text-black'} flex flex-col items-center pt-16 pb-8 px-6`}>
                        <div className="w-24 h-24 rounded-full bg-gradient-to-br from-indigo-400 to-purple-500 mb-4 shadow-lg flex items-center justify-center text-4xl text-white">
                            {storeName.charAt(0).toUpperCase()}
                        </div>
                        <h1 className="text-2xl font-bold font-outfit text-center mb-2">{storeName || 'Store Name'}</h1>
                        <p className={`text-center text-sm mb-8 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-600'}`}>{bio}</p>

                        <div className="w-full space-y-4">
                            {links.map((link, i) => (
                                isSupportedBioUrl(link.url) ? <a
                                    key={i}
                                    href={link.url}
                                    onClick={(e) => e.preventDefault()}
                                    className={`block w-full py-4 px-6 rounded-2xl text-center font-bold text-sm transition-transform hover:scale-[1.02] ${theme === 'dark' ? 'bg-[#222222] text-white hover:bg-[#333333]' : 'bg-white text-black shadow-md hover:shadow-lg'}`}
                                >
                                    {link.title || 'Link Title'}
                                </a> : <span key={i} aria-disabled="true" className="block py-4 px-6 text-center">{link.title || 'Link Title'} (unavailable)</span>
                            ))}
                        </div>

                        {owner && !removeBranding && (
                          <div className="mt-auto pt-8 pb-4">
                              <PoweredByOmniSolo tenantId={owner.tenantId} />
                          </div>
                        )}
                    </div>
                </div>
             </div>
          </div>
        </div>
      </div>
    </div>
  );
}
