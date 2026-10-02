"use client";

import { useState, useEffect, useRef } from "react";
import { SmartBlock, SkeletonBlock, ActionSheet, DraggableBlock } from "./components";
import { useWalkthrough } from "../../components/help";
import { WalkthroughTarget, InteractiveWalkthrough } from "../../components/Walkthrough";
import { WithTooltip } from "../../components/TooltipRegistry";
import { useBuilderStore, initializeBuilderDraft, builderDraftError, subscribeBuilderPersistence, LEGACY_BUILDER_DRAFT_KEY, isBuilderBlocks, type BuilderState } from "./store";
import { assertBuilderEditor, builderScopeActive, type BuilderScope } from './ownedDraft';
import { fetchForOwnedBusinessAction, fetchForOwnedBusinessRead, subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { canonicalRequest } from '../onboarding/contracts';
import { sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';
import { PublicationPanel } from './PublicationPanel';
import { layoutPublicationSnapshot } from './layoutPublicationSnapshot';
import { prepareSiteSnapshot } from './publicationContracts';
import type { BuilderBlock } from '@/lib/builder-types';

export default function BuilderPage() {
  const state = useBuilderStore();
  const { bio, businessName, businessCategory, vibe, wizardStep, blocks, drafts, status, seoMetadata } = state;
  const [isLoaded, setIsLoaded] = useState(false);
  const [sessionError, setSessionError] = useState('');
  const [heldLegacy, setHeldLegacy] = useState(false);
  const [viewScope, setViewScope] = useState<BuilderScope | null>(null);
  const currentScope = useRef<BuilderScope | null>(null);
  const releaseEditor = useRef<() => void>(() => {});
  const epoch = useRef(0);
  const busy = useRef(false);
  const [selectedDraftIndex, setSelectedDraftIndex] = useState(0);
  const [selectedBlockIndex, setSelectedBlockIndex] = useState<number | null>(null);
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);
  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [startY, setStartY] = useState(0);
  const [saveMessage, setSaveMessage] = useState("");
  const [isWalkthroughOpen, setIsWalkthroughOpen] = useState(false);
  const [walkthroughSteps, setWalkthroughSteps] = useState<React.ComponentProps<typeof InteractiveWalkthrough>["steps"]>([]);
  const [wizardStep1Error, setWizardStep1Error] = useState("");
  const [geoScore, setGeoScore] = useState<number | null>(null);
  const [geoRecs, setGeoRecs] = useState<string[]>([]);
  useWalkthrough();

  const edit = (action: () => void) => {
    if (!builderScopeActive(viewScope)) return;
    try { assertBuilderEditor(viewScope, LEGACY_BUILDER_DRAFT_KEY); action(); }
    catch (error) { setSaveMessage(error instanceof Error ? error.message : 'Your editor changed. Reopen this draft.'); }
  };
  const setBio = (value: string) => edit(() => state.setBio(value));
  const setBusinessName = (value: string) => edit(() => state.setBusinessName(value));
  const setBusinessCategory = (value: string) => edit(() => state.setBusinessCategory(value));
  const setVibe = (value: string) => edit(() => state.setVibe(value));
  const setWizardStep = (value: number) => edit(() => state.setWizardStep(value));
  const setBlocks = (value: BuilderBlock[]) => edit(() => state.setBlocks(value));
  const setStatus = (value: BuilderState['status']) => edit(() => state.setStatus(value));
  const setBusinessGoal = (value: BuilderState['businessGoal']) => edit(() => state.setBusinessGoal(value));

  useEffect(() => subscribeBuilderPersistence(() => { if (builderDraftError()) setSaveMessage(builderDraftError()); }), []);
  useEffect(() => {
    let disposed = false; let load = 0;
    const open = async () => {
      const attempt = ++load; setIsLoaded(false); setSessionError('');
      try {
        const restored = await initializeBuilderDraft();
        if (disposed || attempt !== load) { restored.release(); return; }
        releaseEditor.current(); releaseEditor.current = restored.release;
        currentScope.current = restored.scope; setViewScope(restored.scope);
        setHeldLegacy(localStorage.getItem('builder-storage') !== null); setIsLoaded(true);
        void fetchForOwnedBusinessRead('/api/v1/walkthrough/store-setup', restored.scope.owner).then(async response => {
          const steps: unknown = await response.json();
          if (!disposed && attempt === load && builderScopeActive(restored.scope) && response.status === 200 && Array.isArray(steps)) setWalkthroughSteps(steps);
        }).catch(() => {});
      } catch (error) { if (!disposed && attempt === load) setSessionError(error instanceof Error ? error.message : 'Verify your builder session.'); }
    };
    const unsubscribe = subscribeOnboardingInvalidation(restart => {
      ++load; ++epoch.current; busy.current = false; releaseEditor.current(); releaseEditor.current = () => {};
      currentScope.current = null; setViewScope(null); setIsLoaded(false); setSaveMessage(''); setGeoScore(null); setGeoRecs([]); setWalkthroughSteps([]);
      setSelectedBlockIndex(null); setSelectedDraftIndex(0); setIsActionSheetOpen(false); setDraggedIndex(null); setWizardStep1Error('');
      if (restart) void open(); else setSessionError('Your session could not be verified. Your owned draft remains held.');
    });
    void open();
    return () => { disposed = true; ++load; ++epoch.current; busy.current = false; releaseEditor.current(); releaseEditor.current = () => {}; currentScope.current = null; unsubscribe(); };
  }, []);
  const retireEditor = (owner: QueueOwner, reason: string) => {
    if (!currentScope.current || !sameOwner(currentScope.current.owner, owner)) return;
    ++epoch.current; busy.current = false; releaseEditor.current(); releaseEditor.current = () => {};
    currentScope.current = null; setViewScope(null); setIsLoaded(false); setSessionError(reason);
    setIsActionSheetOpen(false); setSelectedBlockIndex(null); setSaveMessage(''); setGeoScore(null); setGeoRecs([]);
  };
  const handleStep1Next = () => {
    if (businessName.trim().length < 3) { setWizardStep1Error("Business name must be at least 3 characters."); return; }
    if (businessCategory.trim().length < 5) { setWizardStep1Error("Category must be at least 5 characters."); return; }
    setWizardStep1Error(""); setWizardStep(2);
  };
  const publicationSnapshot = () => {
    if (!viewScope) throw new Error('Verify your editor owner.');
    assertBuilderEditor(viewScope, LEGACY_BUILDER_DRAFT_KEY);
    const value = useBuilderStore.getState();
    const snapshot = layoutPublicationSnapshot({ title: value.businessName || 'Home', bio: value.bio, blocks: value.blocks });
    const seo_metadata = Object.keys(value.seoMetadata).length ? value.seoMetadata : snapshot.pages[0].seo_metadata;
    return { ...snapshot, pages: [{ ...snapshot.pages[0], seo_metadata }] };
  };
  const runDraftAction = async (url: string, action: (data: unknown) => void) => {
    const scope = viewScope;
    if (!builderScopeActive(scope) || busy.current || builderDraftError()) return;
    const attempt = ++epoch.current; busy.current = true;
    const fingerprint = canonicalRequest({ bio: state.bio, blocks: state.blocks });
    const active = () => attempt === epoch.current && builderScopeActive(scope);
    try {
      assertBuilderEditor(scope, LEGACY_BUILDER_DRAFT_KEY);
      const response = await fetchForOwnedBusinessAction(url, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ content: state.bio }) }, scope.owner, () => assertBuilderEditor(scope, LEGACY_BUILDER_DRAFT_KEY));
      const data: unknown = await response.json();
      if (!active()) return;
      const latest = useBuilderStore.getState();
      if (canonicalRequest({ bio: latest.bio, blocks: latest.blocks }) !== fingerprint) throw new Error('Your draft changed; the earlier result was not applied.');
      if (response.status !== 200 || !data || typeof data !== 'object' || Array.isArray(data) || 'error' in data || ('success' in data && data.success === false)) throw new Error('Draft content suggestions could not be confirmed.');
      action(data);
    } catch (error) { if (active()) setSaveMessage(error instanceof Error ? error.message : 'Draft content suggestions could not be confirmed.'); }
    finally { if (active()) busy.current = false; }
  };
  const handleGeoAnalysis = () => void runDraftAction('/api/v1/builder/geo_score', data => {
    const result = data as Record<string, unknown>;
    if (!Number.isInteger(result.generative_score) || Number(result.generative_score) < 0 || Number(result.generative_score) > 100 || !Array.isArray(result.recommendations) || !result.recommendations.every(item => typeof item === 'string')) throw new Error('The content analysis response was incomplete.');
    setGeoScore(Number(result.generative_score)); setGeoRecs(result.recommendations as string[]);
    setSaveMessage('Draft analysis returned suggestions; this is not a measured search ranking.');
  });
  const handleAutoSeo = () => void runDraftAction('/api/v1/builder/auto_seo', data => {
    state.setSeoMetadata(data as Record<string, unknown>);
    setSaveMessage('SEO metadata updated in this private draft. Review it before publishing.');
  });
  const handleGenerate = async () => {
    const scope = viewScope;
    if (!builderScopeActive(scope) || busy.current || builderDraftError()) return;
    const attempt = ++epoch.current; busy.current = true;
    const input = () => { const value = useBuilderStore.getState(); return { businessName: value.businessName, businessCategory: value.businessCategory, vibe: value.vibe, bio: value.bio, blocks: value.blocks, seoMetadata: value.seoMetadata }; };
    const submitted = input(); const fingerprint = canonicalRequest(submitted);
    const active = () => attempt === epoch.current && builderScopeActive(scope);
    setStatus('generating');
    try {
      const description = [submitted.businessName, submitted.businessCategory, submitted.bio, submitted.vibe ? `Requested style: ${submitted.vibe}` : ''].filter(Boolean).join('. ');
      const response = await fetchForOwnedBusinessAction('/api/v1/builder/generate', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ description }) }, scope.owner, () => { assertBuilderEditor(scope, LEGACY_BUILDER_DRAFT_KEY); if (canonicalRequest(input()) !== fingerprint) throw new Error('Your draft changed before generation.'); });
      const data = await response.json();
      if (!active()) return;
      if (canonicalRequest(input()) !== fingerprint) throw new Error('Your draft changed; the earlier generated result was not applied.');
      if (response.status !== 200 || !data || data.success === false || data.error != null || !Array.isArray(data.pages) || !Array.isArray(data.pages[0]?.blocks)) throw new Error('The generated draft could not be confirmed.');
      const names: Record<string, string> = { HeroBlock: 'Hero', ProductGridBlock: 'Catalog', ServiceBookingBlock: 'Booking', TestimonialBlock: 'Testimonials', ReferralBlock: 'Referral' };
      const generated: unknown = data.pages[0].blocks.map((block: Record<string, unknown>) => ({ type: typeof block.block_type === 'string' ? names[block.block_type] || block.block_type : '', props: block.content }));
      if (!isBuilderBlocks(generated)) throw new Error('The generated draft has unsupported content.');
      await prepareSiteSnapshot(layoutPublicationSnapshot({ title: submitted.businessName || 'Home', bio: submitted.bio, blocks: generated }));
      if (!active()) return;
      if (canonicalRequest(input()) !== fingerprint) throw new Error('Your draft changed; the earlier generated result was not applied.');
      state.setDrafts([generated]); state.setBlocks(generated); state.setSeoMetadata({}); state.setStatus('selection'); setSaveMessage('');
    } catch (error) {
      if (active()) { state.setStatus(useBuilderStore.getState().blocks.length ? 'draft' : 'idle'); setSaveMessage(error instanceof Error ? error.message : 'Generation could not be confirmed.'); }
    } finally { if (active()) busy.current = false; }
  };

  if (sessionError) return <div role="alert">{sessionError}</div>;
  if (!isLoaded || !builderScopeActive(viewScope)) return <div role="status">Verifying your builder session…</div>;
  const draftNotice = <div className="p-3 text-sm">
    {heldLegacy && <p>Older unowned builder draft is held separately. It has not been opened or assigned to this account.</p>}
    {saveMessage && <p role="status">{saveMessage}</p>}
  </div>;

  if (status === "selection") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter overflow-hidden">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:glassmorphism shadow-2xl">
          {draftNotice}
           <div className="px-8 pt-12 pb-6 text-center">
              <h1 className="text-2xl font-extrabold font-outfit text-gray-900 mb-2">Pick your draft</h1>
              <p className="text-sm text-gray-500">Review the returned proposal, including its content and prices.</p>
           </div>

           <div className="flex-1 overflow-y-auto px-6 space-y-6 pb-24">
              {drafts.map((d, idx) => (
                <button
                  key={idx}
                  onClick={() => {
                    setBlocks(d);
                    setSelectedDraftIndex(idx);
                  }}
                  className={`w-full text-left glassmorphism backdrop-blur-[30px] saturate-[210%] rounded-[8px] border-2 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] overflow-hidden ${selectedDraftIndex === idx ? 'border-[#0066FF] ring-2 ring-[#0066FF]/20 shadow-lg' : 'border-white/50 dark:border-white/10 opacity-70 hover:opacity-100 hover:border-white/80'}`}
                >
                   <div className="h-32 bg-white/50 dark:bg-black/30 flex items-center justify-center relative backdrop-blur-[30px] saturate-[210%] border-b border-white/40 dark:border-white/10">
                      <span className="font-outfit font-bold text-gray-400 dark:text-gray-500">Draft {idx + 1}</span>
                      {selectedDraftIndex === idx && (
                        <div className="absolute top-2 right-2 bg-[#0066FF] text-white rounded-full p-1">
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                        </div>
                      )}
                   </div>
                   <div className="p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%]">
                      <p className="text-xs font-bold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wider mb-1">Preview</p>
                      <p className="text-sm text-[#1D1D1F] dark:text-[#F5F5F7] line-clamp-1 font-inter">{d[0]?.props?.headline || "Storefront Preview"}</p>
                   </div>
                </button>
              ))}
           </div>

           <div className="absolute bottom-0 w-full p-6 glassmorphism border-t border-white/40 dark:border-white/10 z-50">
              <button
                onClick={() => setStatus("draft")}
                className="w-full bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[8px] font-bold font-outfit shadow-md hover:shadow-lg active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
              >
                Customize Selected Draft
              </button>
           </div>
        </div>
      </div>
    );
  }

  if (status === "onboarding") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter overflow-hidden">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:glassmorphism shadow-2xl">
          {draftNotice}
          {/* Abstract Background Blur */}
          <div className="absolute inset-0 -z-10">
            <div className="absolute top-[-10%] left-[-10%] w-[120%] h-[120%] bg-gradient-to-br from-blue-400 via-purple-400 to-pink-400 blur-[80px] opacity-30 animate-pulse" />
          </div>

          <div className="flex-1 flex flex-col items-center justify-center px-6 text-center">
            <div className="glassmorphism backdrop-blur-[30px] saturate-[210%] border border-white/50 dark:border-white/10 shadow-sm p-8 w-full animate-fade-in" style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
              <h1 className="text-3xl font-extrabold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-6 leading-tight">
                What are you building today?
              </h1>

              <div className="space-y-4">
                {[
                  { id: 'products', label: 'Selling Products', icon: '🛍️' },
                  { id: 'services', label: 'Offering Services', icon: '🛠️' },
                  { id: 'work', label: 'Showcasing Work', icon: '✨' },
                ].map((option) => (
                  <button
                    key={option.id}
                    onClick={() => {
                      setBusinessGoal(option.id as Parameters<typeof setBusinessGoal>[0]);
                      setStatus("idle");
                    }}
                    className="w-full p-6 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] rounded-[8px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col items-center gap-2 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] group hover:bg-white/80 dark:hover:bg-black/50"
                  >
                    <span className="text-3xl group-hover:scale-110 transition-transform">{option.icon}</span>
                    <span className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">{option.label}</span>
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (status === "idle") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:glassmorphism shadow-2xl">
          {draftNotice}

          <div className="px-8 pt-12 pb-4 relative">
             <div className="flex justify-between mb-8">
               {[1, 2, 3].map(step => (
                 <div key={step} className={`h-1.5 flex-1 mx-1 rounded-full ${step <= wizardStep ? 'bg-[#0066FF]' : 'bg-gray-200 dark:bg-gray-700'}`} style={{ transition: 'all 250ms cubic-bezier(0.4, 0, 0.2, 1)' }} />
               ))}
             </div>
             {(businessName || businessCategory || vibe || bio) && (
               <div className="absolute top-4 right-8 flex items-center gap-1 text-xs text-green-600 font-medium animate-fade-in bg-green-50 px-2 py-1 rounded-full border border-green-200 shadow-sm">
                 <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                 {builderDraftError() ? 'Local save needs attention' : 'Draft saved on this device'}
               </div>
             )}
          </div>

          <div className="px-8 pb-8 flex flex-col flex-1 justify-start overflow-y-auto">
            <InteractiveWalkthrough
              steps={walkthroughSteps}
              isOpen={isWalkthroughOpen}
              onClose={() => setIsWalkthroughOpen(false)}
            />
            {wizardStep === 1 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <WalkthroughTarget id="dashboard-title">
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Let's build your store</h1>
                </WalkthroughTarget>
                <div className="mb-4 flex flex-wrap gap-2">
                  <button
                    type="button"
                    onClick={() => setIsWalkthroughOpen(true)}
                    id="dashboard-walkthrough-btn"
                    className="app-button min-h-[44px]"
                  >
                    Start Tour
                  </button>
                </div>
                <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                  Start with the basics. What's your business called, and what do you do?
                </p>

                <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 flex items-center justify-between">
                  <span>Business Name</span>
                  {businessName.trim().length > 0 && (
                    <span className={`text-xs ${businessName.trim().length >= 3 ? "text-green-500" : "text-orange-500"}`}>
                      {businessName.trim().length >= 3 ? "✓ Looks good" : "Needs 3+ characters"}
                    </span>
                  )}
                </label>
                <input
                  type="text"
                  className="w-full border border-white/50 dark:border-white/10 glassmorphism backdrop-blur-[30px] saturate-[210%] p-4 mb-6 focus:ring-2 focus:ring-[#0066FF]/50 focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-[#1D1D1F] dark:text-[#f5f5f7] shadow-inner"
                  style={{ borderRadius: '8px' }}
                  value={businessName}
                  onChange={(e) => setBusinessName(e.target.value)}
                  placeholder="e.g. Acme Corp"
                />

                <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 flex items-center justify-between">
                  <span>Category</span>
                  {businessCategory.trim().length > 0 && (
                    <span className={`text-xs ${businessCategory.trim().length >= 5 ? "text-green-500" : "text-orange-500"}`}>
                      {businessCategory.trim().length >= 5 ? "✓ Sounds great" : "Needs 5+ characters"}
                    </span>
                  )}
                </label>
                <input
                  type="text"
                  className="w-full border border-white/50 dark:border-white/10 glassmorphism backdrop-blur-[30px] saturate-[210%] p-4 mb-6 focus:ring-2 focus:ring-[#0066FF]/50 focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-[#1D1D1F] dark:text-[#f5f5f7] shadow-inner"
                  style={{ borderRadius: '8px' }}
                  value={businessCategory}
                  onChange={(e) => setBusinessCategory(e.target.value)}
                  placeholder="e.g. Retail, Consulting, Tech"
                />

                {wizardStep1Error && (
                   <div className="bg-[#FF3B30]/10 border border-[#FF3B30]/20 p-3 rounded-[8px] mb-6 flex items-center gap-2 text-left">
                     <svg className="w-5 h-5 text-[#FF3B30]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                     <p className="text-[#FF3B30] text-sm font-semibold m-0">{wizardStep1Error}</p>
                   </div>
                )}

                <button
                  className={`w-full p-4 font-bold font-outfit text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] ${
                    businessName.trim().length >= 3 && businessCategory.trim().length >= 5
                      ? "text-white shadow-md active:scale-[0.98] bg-gradient-to-r from-[#0066FF] to-[#0052cc]"
                      : "glassmorphism text-gray-400 dark:text-gray-500 cursor-not-allowed border border-white/50 dark:border-white/10"
                  }`}
                  style={{ borderRadius: '8px' }}
                  onClick={handleStep1Next}
                >
                  Next: Choose Vibe
                </button>
              </div>
            )}

            {wizardStep === 2 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Select Your Vibe</h1>
                <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                  How should your store feel? Our AI agents will match this tone.
                </p>

                <div className="grid gap-4 mb-8">
                  {[
                    { id: 'Professional', label: 'Professional', icon: '👔' },
                    { id: 'Friendly', label: 'Friendly', icon: '👋' },
                    { id: 'Energetic', label: 'Energetic', icon: '⚡' },
                    { id: 'Minimalist', label: 'Minimalist', icon: '🌿' }
                  ].map((v) => (
                    <button
                      key={v.id}
                      onClick={() => setVibe(v.id)}
                      className={`p-4 border text-left flex items-center gap-3 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] font-semibold backdrop-blur-[30px] saturate-[210%] ${
                        vibe === v.id ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] shadow-sm" : "border-white/50 dark:border-white/10 text-gray-700 dark:text-gray-300 hover:border-white/80 dark:hover:border-white/20 glassmorphism"
                      }`}
                      style={{ borderRadius: '8px' }}
                    >
                      <span className="text-2xl">{v.icon}</span>
                      <span>{v.label}</span>
                    </button>
                  ))}
                </div>

                <div className="flex gap-4">
                  <button
                    className="flex-1 p-4 glassmorphism text-gray-700 dark:text-gray-300 font-bold font-outfit text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] hover:bg-white/60 dark:hover:bg-black/40 active:scale-[0.98] border border-white/50 dark:border-white/10 backdrop-blur-[30px] saturate-[210%]"
                    style={{ borderRadius: '8px' }}
                    onClick={() => setWizardStep(1)}
                  >
                    Back
                  </button>
                  <button
                    className={`flex-1 p-4 font-bold font-outfit text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] ${
                      vibe
                        ? "text-white shadow-md active:scale-[0.98] bg-gradient-to-r from-[#0066FF] to-[#0052cc]"
                        : "glassmorphism text-gray-400 dark:text-gray-500 cursor-not-allowed border border-white/50 dark:border-white/10 backdrop-blur-[30px] saturate-[210%]"
                    }`}
                    style={{ borderRadius: '8px' }}
                    onClick={() => {
                       if (!bio.trim()) {
                         setBio(`I run a ${businessCategory} business called ${businessName}. We want a ${vibe.toLowerCase()} vibe.`);
                       }
                       setWizardStep(3);
                    }}
                    disabled={!vibe}
                  >
                    Next: Details
                  </button>
                </div>
              </div>
            )}

            {wizardStep === 3 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Final Details</h1>
                <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                  Review and add any extra details to help our AI generate the perfect store.
                </p>

                <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 block text-left">Your Business Details</label>
                <WalkthroughTarget id="bio-input-tooltip">
                <WithTooltip id="bio-input-tooltip" defaultText="Describe what you sell, your target audience, and the vibe of your brand.">
                  <textarea
                    id="bio-input"
                    className="w-full border border-white/50 dark:border-white/10 glassmorphism backdrop-blur-[30px] saturate-[210%] p-4 mb-8 focus:ring-2 focus:ring-[#0066FF]/50 focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] resize-none text-[#1D1D1F] dark:text-[#f5f5f7] shadow-inner"
                    style={{ borderRadius: '8px' }}
                    value={bio}
                    onChange={(e) => setBio(e.target.value)}
                    placeholder="e.g. I run a mobile dog grooming service in Portland"
                    rows={6}
                  />
                </WithTooltip>
                </WalkthroughTarget>

                <div className="flex gap-4">
                  <button
                    className="flex-1 p-4 glassmorphism text-gray-700 dark:text-gray-300 font-bold font-outfit text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] hover:bg-white/60 dark:hover:bg-black/40 active:scale-[0.98] border border-white/50 dark:border-white/10 backdrop-blur-[30px] saturate-[210%]"
                    style={{ borderRadius: '8px' }}
                    onClick={() => setWizardStep(2)}
                  >
                    Back
                  </button>
                  <WalkthroughTarget id="generate-btn-tooltip">
                  <WithTooltip id="generate-btn-tooltip" defaultText="Our AI agents will analyze your description and build a ready-to-launch store for you.">
                    <button
                      id="generate-btn"
                      className={`flex-[2] p-4 font-bold font-outfit text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] ${
                        bio.trim().length > 5
                          ? "text-white shadow-md active:scale-[0.98] bg-gradient-to-r from-[#0066FF] to-[#0052cc]"
                          : "glassmorphism text-gray-400 dark:text-gray-500 cursor-not-allowed border border-white/50 dark:border-white/10 backdrop-blur-[30px] saturate-[210%]"
                      }`}
                      style={{ borderRadius: '8px' }}
                      onClick={handleGenerate}
                      disabled={bio.trim().length <= 5}
                    >
                      Build Store
                    </button>
                  </WithTooltip>
                  </WalkthroughTarget>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    );
  }

  if (status === "generating") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:glassmorphism shadow-2xl">
          {draftNotice}
           <div className="px-8 pt-20 pb-4 text-center">
              <h1 className="text-2xl font-extrabold font-outfit text-gray-900 mb-2">Draft builder</h1>
              <p className="text-sm text-gray-500 animate-pulse">Preparing your storefront draft...</p>
           </div>
           <div className="flex-1 overflow-y-auto px-4">
              <SkeletonBlock />
              <SkeletonBlock />
              <SkeletonBlock />
           </div>
           {/* Abstract pulse overlay */}
           <div className="absolute inset-0 bg-[#0066FF]/5 animate-pulse pointer-events-none" />
        </div>
      </div>
    );
  }


  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter">
      <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:glassmorphism shadow-2xl">
          {draftNotice}

        {/* Draft Preview Header */}
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-[30px] saturate-[210%] text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Mobile Editor</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto pb-32 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
            <DraggableBlock
              key={i}
              isSelected={selectedBlockIndex === i}
              onClick={() => {
                setSelectedBlockIndex(i);
                setIsActionSheetOpen(true);
              }}
              onDragStart={(e) => {
                setDraggedIndex(i);
                if ('touches' in e) {
                  setStartY(e.touches[0].clientY);
                } else if ('clientY' in e) {
                  setStartY((e as React.DragEvent).clientY);
                }
                setSelectedBlockIndex(i);
              }}
              onDragOver={(e) => {
                if (draggedIndex === null) return;
                let currentY: number;
                if ('touches' in e) {
                  currentY = e.touches[0].clientY;
                } else if ('clientY' in e) {
                  currentY = (e as React.DragEvent).clientY;
                } else {
                  return;
                }
                const diff = currentY - startY;
                if (Math.abs(diff) > 50) {
                  const newIndex = diff > 0 ? i + 1 : i - 1;
                  if (newIndex >= 0 && newIndex < blocks.length && newIndex !== draggedIndex) {
                    const newBlocks = [...blocks];
                    const [removed] = newBlocks.splice(draggedIndex, 1);
                    newBlocks.splice(newIndex, 0, removed);
                    setBlocks(newBlocks);
                    setDraggedIndex(newIndex);
                    setStartY(currentY);
                  }
                }
              }}
              onDragEnd={() => {
                setDraggedIndex(null);
              }}
            >
              <SmartBlock {...b} />
            </DraggableBlock>
          ))}
          <SmartBlock type="PoweredBy" props={{ tenantId: viewScope.owner.tenantId }} />
          <section aria-label="Private draft metadata" className="p-4 space-y-2">
            <h2>Draft content suggestions</h2>
            <p>Content analysis and metadata changes apply only to this private draft.</p>
            <button type="button" onClick={handleGeoAnalysis}>Analyze draft content</button>
            {geoScore !== null && <p>Content review score: {geoScore}/100</p>}
            {geoRecs.map((text, index) => <p key={index}>{text}</p>)}
            <button type="button" onClick={handleAutoSeo}>Prepare draft SEO metadata</button>
            {Object.keys(seoMetadata).length > 0 && <pre aria-label="Private SEO metadata">{JSON.stringify(seoMetadata, null, 2)}</pre>}
          </section>
          <PublicationPanel channel="builder" expectedOwner={viewScope.owner} getSnapshot={publicationSnapshot}
            isEditorCurrent={() => { try { assertBuilderEditor(viewScope, LEGACY_BUILDER_DRAFT_KEY); return !builderDraftError(); } catch { return false; } }}
            onRetired={retireEditor} />
        </div>

        {/* Action Sheet for Editing Blocks */}
        <ActionSheet
          isOpen={isActionSheetOpen}
          onClose={() => setIsActionSheetOpen(false)}
          title={`Edit ${blocks[selectedBlockIndex || 0]?.type} Block`}
        >
          <div className="space-y-4 font-inter">
            {blocks[selectedBlockIndex || 0]?.type === 'Hero' && (
              <>
                <label className="text-xs font-bold text-gray-400 uppercase">Headline</label>
                <input
                  type="text"
                  className="w-full p-4 glassmorphism backdrop-blur-[30px] saturate-[210%] rounded-[8px] border border-white/50 dark:border-white/10 focus:ring-2 focus:ring-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-[#1D1D1F] dark:text-[#F5F5F7] shadow-inner"
                  value={blocks[selectedBlockIndex || 0]?.props.headline}
                  onChange={(e) => {
                    const newBlocks = [...blocks];
                    newBlocks[selectedBlockIndex || 0].props.headline = e.target.value;
                    setBlocks(newBlocks);
                  }}
                />
                <div className="grid grid-cols-2 gap-3 mt-4">
                  <button disabled title="This editor cannot upload or generate images yet" className="p-4 glassmorphism backdrop-blur-[30px] saturate-[210%] rounded-[8px] border border-white/50 dark:border-white/10 text-sm font-bold flex flex-col items-center gap-2 hover:bg-white/60 dark:hover:bg-black/40">
                    <span>🖼️</span>
                    <span>Upload Photo</span>
                  </button>
                  <button disabled title="This editor cannot upload or generate images yet" className="p-4 glassmorphism backdrop-blur-[30px] saturate-[210%] rounded-[8px] border border-white/50 dark:border-white/10 text-sm font-bold flex flex-col items-center gap-2 hover:bg-white/60 dark:hover:bg-black/40">
                    <span>✨</span>
                    <span>AI Generate</span>
                  </button>
                </div>
              </>
            )}
            {blocks[selectedBlockIndex || 0]?.type !== 'Hero' && (
              <p className="text-sm text-gray-500 italic">Context-aware editing for {blocks[selectedBlockIndex || 0]?.type} coming soon...</p>
            )}
            <button
              onClick={() => setIsActionSheetOpen(false)}
              className="w-full bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[8px] font-bold mt-4 shadow-md hover:shadow-lg active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
            >
              Save Changes
            </button>
          </div>
        </ActionSheet>

        <div className="border-t p-4 flex gap-4">
          <button type="button" onClick={() => { setWizardStep(2); setStatus('idle'); }}>Review style and details</button>
          <a href="/plan">Review plan options</a>
        </div>

      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes slideUp {
          from { transform: translateY(100%); opacity: 0; }
          to { transform: translateY(0); opacity: 1; }
        }
        .animate-slide-up { animation: slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1); }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism { background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 8px; }
        @media (prefers-color-scheme: dark) {
          .glassmorphism { background: rgba(22, 22, 26, 0.7); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 8px; }
        }
      `}} />
    </div>
  );
}
