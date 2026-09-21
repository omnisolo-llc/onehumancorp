"use client";

import { useState,useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOmniSolo } from '../components/PoweredByOmniSolo';
import { safeBioHref } from '@/lib/bioLinks';

interface BioLink { id: string; title: string; url: string }

export default function LinkInBioGeneratorPage() {
  const router = useRouter();
  const [storeName, setStoreName] = useState('My Store');
  const [bio, setBio] = useState('Welcome to my storefront!');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [links, setLinks] = useState<BioLink[]>([{ id: 'initial-link', title: 'Shop Now', url: '' }]);
  const [tenant, setTenant] = useState('my-store');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [copied, setCopied] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [saveError, setSaveError] = useState('');
  const [copyError, setCopyError] = useState('');
  const [isCopying, setIsCopying] = useState(false);

  useEffect(() => { setSaveSuccess(false); }, [storeName, bio, theme, links, removeBranding]);

  useEffect(() => {
    const tid = typeof window !== 'undefined' ? (localStorage.getItem('business_display_name') || 'my-store') : 'my-store';
    setTenant(tid);

    // Load existing config if available
    const loadConfig = async () => {
      try {
        const res = await fetch(`/api/v1/growth/link-in-bio/${encodeURIComponent(tid)}`);
        if (res.ok) {
          const data = await res.json();
          if (data && data.store_name) {
             setStoreName(data.store_name);
             setBio(data.bio || '');
             setTheme(data.theme || 'light');
             if (Array.isArray(data.links)) {
               const ids = new Set<string>();
               setLinks(data.links.map((link: Partial<BioLink>) => {
                 const id = typeof link.id === 'string' && link.id.trim() && !ids.has(link.id)
                   ? link.id : crypto.randomUUID();
                 ids.add(id);
                 return { id, title: typeof link.title === 'string' ? link.title : '', url: typeof link.url === 'string' ? link.url : '' };
               }));
             }
             setRemoveBranding(data.remove_branding || false);
          }
        }
      } catch  {
        // ignore
      }
    };
    loadConfig();
  }, []);

  const handleAddLink = () => {
    setLinks([...links, { id: crypto.randomUUID(), title: 'New Link', url: '' }]);
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
    setSaveSuccess(false);
    setSaveError('');
    const invalidLink = links.findIndex(link => !link.title.trim() || !safeBioHref(link.url));
    if (invalidLink !== -1) {
      setSaveError(`Link ${invalidLink + 1} needs a title and a valid HTTP(S) URL or site-relative path before publishing.`);
      return;
    }
    setIsSaving(true);
    try {
      const res = await fetch('/api/v1/growth/link-in-bio', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          store_name: storeName,
          bio,
          theme,
          links: links.map(link => ({ ...link, title: link.title.trim(), url: safeBioHref(link.url)! })),
          remove_branding: removeBranding
        })
      });
      if (!res.ok) throw new Error('Publish failed');
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch {
      setSaveError('Unable to publish your page. Your edits are preserved; please try again.');
    } finally {
      setIsSaving(false);
    }
  };

  const linkUrl = `https://cloud.omnisolo.co/bio/${encodeURIComponent(tenant)}`;

  const handleCopy = async () => {
    setCopied(false);
    setCopyError('');
    setIsCopying(true);
    try {
      if (!navigator.clipboard?.writeText) throw new Error('Clipboard unavailable');
      await navigator.clipboard.writeText(linkUrl);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      setCopyError('Unable to copy the link. Check clipboard permissions and try again.');
    } finally {
      setIsCopying(false);
    }
  };

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
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">One link to rule them all. Drive social traffic to your store.</p>
          </div>
        </div>

        <div className="flex flex-col lg:flex-row gap-8">
          {/* Builder Controls */}
          <fieldset disabled={isSaving} aria-label="Link-in-bio configuration" className="flex-1 min-w-0 space-y-6">
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
                    aria-label="Store / Creator Name"
                    className="w-full px-4 py-2 bg-gray-50 dark:bg-[#1C1C1E] border border-gray-200 dark:border-white/10 rounded-xl focus:ring-2 focus:ring-indigo-500 outline-none text-gray-900 dark:text-white"
                  />
                </div>
                <div>
                  <label htmlFor="bioInput" className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Bio / Description</label>
                  <textarea
                    id="bioInput"
                    value={bio}
                    onChange={(e) => setBio(e.target.value)}
                    aria-label="Bio / Description"
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
                  <div key={link.id} className="flex flex-col gap-2 p-4 bg-gray-50 dark:bg-[#1C1C1E] rounded-xl border border-gray-100 dark:border-white/5">
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
                        placeholder="Link Title (e.g. Shop My Collection)"
                        aria-label={`Link ${index + 1} Title`}
                        className="w-full px-3 py-2 bg-white dark:bg-[#2C2C2E] border border-gray-200 dark:border-white/10 rounded-lg text-sm outline-none text-gray-900 dark:text-white"
                    />
                    <input
                        type="text"
                        value={link.url}
                        onChange={(e) => handleLinkChange(index, 'url', e.target.value)}
                        placeholder="URL (e.g. https://... or /booking)"
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
                {isSaving ? 'Saving...' : saveSuccess ? 'Saved! ✅' : 'Save & Publish'}
            </button>
            {saveError && <p role="alert" className="text-sm text-red-600">{saveError}</p>}
          </fieldset>

          {/* Live Preview */}
          <div className="w-full lg:w-[400px] flex-shrink-0">
             <div className="sticky top-8">
                <div className="flex items-center justify-between mb-4">
                    <h2 className="text-lg font-bold font-outfit text-gray-900 dark:text-white">Live Preview</h2>
                    <button onClick={handleCopy} disabled={isCopying} className="text-sm font-semibold text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-500/20 px-3 py-1 rounded-full hover:bg-indigo-100 transition-colors">
                        {isCopying ? 'Copying...' : copied ? 'Copied URL!' : 'Copy Link'}
                    </button>
                </div>

                {copyError && <p role="alert" className="mb-3 text-sm text-red-600">{copyError}</p>}

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
                            {links.map(link => {
                              const href = safeBioHref(link.url);
                              const className = `block w-full py-4 px-6 rounded-2xl text-center font-bold text-sm transition-transform ${theme === 'dark' ? 'bg-[#222222] text-white' : 'bg-white text-black shadow-md'}`;
                              return href ? (
                                <a key={link.id} href={href} target="_blank" rel="noopener noreferrer" className={`${className} hover:scale-[1.02]`}>
                                  {link.title || 'Link Title'}
                                </a>
                              ) : (
                                <div key={link.id} className={className}>
                                  <span>{link.title || 'Link Title'}</span>
                                  <p className="mt-1 text-xs font-normal">Add a valid destination to enable this link.</p>
                                </div>
                              );
                            })}
                        </div>

                        {!removeBranding && (
                          <div className="mt-auto pt-8 pb-4">
                              <PoweredByOmniSolo tenantId={tenant} />
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
