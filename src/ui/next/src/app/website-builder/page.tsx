"use client";

import { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";
import { useWebsiteBuilderStore, initializeWebsiteDraft, websiteDraftError, subscribeWebsitePersistence, WEBSITE_DRAFT_KEY } from "./store";
import { builderScopeActive, assertBuilderEditor, captureBuilderRestore, assertBuilderRestore, hasHeldBuilderLegacy, publishOwnedLayout, type BuilderScope } from '../builder/ownedDraft';
import { fetchForOnboardingOwner, subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { initializeOnboardingDraft, useOnboardingStore } from '../onboarding/store';
import { canonicalRequest } from '../onboarding/contracts';
import { onboardingWriteVersion, subscribeOnboardingWriteState } from '../onboarding/draftWriteGate';
import { SmartBlock, DraggableBlock } from "../builder/components";
import { useWalkthrough } from "../../components/help";
import { WithTooltip } from "../../components/TooltipRegistry";
import { PoweredByOmniSolo } from "../components/PoweredByOmniSolo";

export default function WebsiteBuilderPage() {
  const router = useRouter();

  const {
    wizardStep, setWizardStep,
    businessName, setBusinessName,
    businessType, setBusinessType,
    hasPhysicalProducts, setHasPhysicalProducts,
    hasDigitalProducts, setHasDigitalProducts,
    productName, setProductName,
    productPrice, setProductPrice,
    setPaymentMethod,
    template, setTemplate,
    bio, setBio,
    domainChoice,
    aiAgents,
    aiAutoRespond,
    blocks, moveBlock,
    status,
    liveUrl
  } = useWebsiteBuilderStore();


  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [selectedBlockIndex, setSelectedBlockIndex] = useState<number | null>(null);
  const [saveMessage, setSaveMessage] = useState("");
  const [isLoaded, setIsLoaded] = useState(false);
  const [sessionError, setSessionError] = useState('');
  const [heldLegacy, setHeldLegacy] = useState(false);
  const [saving, setSaving] = useState(false);
  const scope = useRef<BuilderScope | null>(null);
  const [viewScope, setViewScope] = useState<BuilderScope | null>(null);
  const operationEpoch = useRef(0);
  const draftEpoch = useRef(0);
  const lastSavedDetails = useRef<string | null>(null);
  const savedWriteVersion = useRef<string | null>(null);
  const savingDraft = useRef(false);
  const actionBusy = useRef(false);
  const setupFields = () => {
    const value = useWebsiteBuilderStore.getState();
    return { businessName: value.businessName, businessType: value.businessType, firstProductName: value.productName, firstProductPrice: value.productPrice, websiteTemplate: value.template, bio: value.bio, domainChoice: value.domainChoice, categories: [...(value.hasPhysicalProducts ? ['physical'] : []), ...(value.hasDigitalProducts ? ['digital'] : [])], aiAgents: value.aiAgents, aiAutoRespond: value.aiAutoRespond };
  };
  useEffect(() => subscribeWebsitePersistence(() => { if (websiteDraftError()) setSaveMessage(websiteDraftError()); }), []);
  useEffect(() => subscribeOnboardingWriteState(() => {
    if (builderScopeActive(scope.current) && savedWriteVersion.current !== null && savedWriteVersion.current !== onboardingWriteVersion(scope.current.owner)) {
      lastSavedDetails.current = null; setSaveMessage('Local setup details need review after another save.');
    }
  }), []);

  const saveSetupDetails = async (path: 'draft' | 'state') => {
    const current = viewScope; const epoch = draftEpoch.current;
    if (!builderScopeActive(current)) throw new Error('Verify your session before saving setup details.');
    assertBuilderEditor(current, WEBSITE_DRAFT_KEY);
    if (websiteDraftError()) throw new Error(websiteDraftError());
    const payload = setupFields(); const fingerprint = canonicalRequest(payload);
    const response = await fetchForOnboardingOwner('/api/v1/onboarding/' + path, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ wizardState: payload }) }, current.owner, version => {
      if (epoch !== draftEpoch.current || !builderScopeActive(current)) return;
      if (canonicalRequest(setupFields()) !== fingerprint) { setSaveMessage('Earlier setup details saved; newer local edits are pending.'); return; }
      lastSavedDetails.current = fingerprint; savedWriteVersion.current = version;
      setSaveMessage('Setup details saved. Layout drafts remain on this device.');
    });
    if (!response.ok) throw new Error('Setup details were not saved. Your local draft remains held.');
  };
  const handleSaveDraft = async () => {
    if (savingDraft.current) return;
    const epoch = draftEpoch.current; savingDraft.current = true; setSaving(true); setSaveMessage('');
    try { await saveSetupDetails('draft'); }
    catch (error) { if (epoch === draftEpoch.current) setSaveMessage(error instanceof Error ? error.message : 'Draft save could not be confirmed.'); }
    finally { if (epoch === draftEpoch.current) { savingDraft.current = false; setSaving(false); } }
  };

  useWalkthrough();
  useEffect(() => {
    let disposed = false; let loadVersion = 0; let releaseEditor = () => {};
    const load = async () => {
      const version = ++loadVersion; setIsLoaded(false); setSessionError('');
      try {
        const restored = await initializeWebsiteDraft();
        if (disposed || version !== loadVersion) { restored.release(); return; }
        releaseEditor(); releaseEditor = restored.release;
        scope.current = restored.scope; setViewScope(restored.scope);
        const snapshot = captureBuilderRestore(WEBSITE_DRAFT_KEY, restored.scope);
        const responses = await Promise.all(['draft','state'].map(async path => {
          try {
            const response = await fetchForOnboardingOwner('/api/v1/onboarding/' + path, {}, restored.scope.owner);
            if (!response.ok) return { ok: false, data: null };
            const data = await response.json();
            const ok = data !== null && typeof data === 'object' && !Array.isArray(data) && data.success !== false && data.error == null && (data.wizardState === undefined || data.wizardState !== null && typeof data.wizardState === 'object' && !Array.isArray(data.wizardState));
            return { ok, data };
          } catch { return { ok: false, data: null }; }
        }));
        if (disposed || version !== loadVersion) return;
        assertBuilderRestore(snapshot);
        if (!restored.hasLocal) {
          if (responses.some(result => !result.ok)) throw new Error('Your saved setup details could not be restored. Reopen the builder when the service is available.');
          const source = responses.map(result => result.data).find(value => Object.keys(value).length > 0);
          const data = source?.wizardState || source;
          if (data) {
            const changes: Partial<import('./store').WebsiteBuilderState> = {};
            const strings = { businessName:'businessName', businessType:'businessType', firstProductName:'productName', firstProductPrice:'productPrice', websiteTemplate:'template', bio:'bio', domainChoice:'domainChoice' } as const;
            for (const [from,to] of Object.entries(strings)) if (typeof data[from] === 'string') changes[to] = data[from];
            if (Array.isArray(data.aiAgents) && data.aiAgents.every((value: unknown) => typeof value === 'string')) changes.aiAgents = data.aiAgents;
            if (typeof data.aiAutoRespond === 'boolean') changes.aiAutoRespond = data.aiAutoRespond;
            if (Array.isArray(data.categories)) { changes.hasPhysicalProducts = data.categories.includes('physical'); changes.hasDigitalProducts = data.categories.includes('digital'); }
            useWebsiteBuilderStore.getState().loadState?.(changes);
          }
          lastSavedDetails.current = canonicalRequest(setupFields());
        } else lastSavedDetails.current = null;
        setHeldLegacy(hasHeldBuilderLegacy()); setIsLoaded(true);
      } catch (error) { if (!disposed && version === loadVersion) setSessionError(error instanceof Error ? error.message : 'Verify your session before using the builder.'); }
    };
    const unsubscribe = subscribeOnboardingInvalidation(restart => {
      loadVersion += 1; operationEpoch.current += 1; draftEpoch.current += 1; releaseEditor(); scope.current = null; setViewScope(null); savingDraft.current = false; actionBusy.current = false;
      lastSavedDetails.current = null; savedWriteVersion.current = null;
      setSelectedBlockIndex(null); setDraggedIndex(null); setSaveMessage(''); setSaving(false); setIsLoaded(false);
      if (restart) void load(); else setSessionError('Your session could not be verified. Your saved draft remains held.');
    });
    void load();
    return () => { disposed = true; loadVersion += 1; operationEpoch.current += 1; draftEpoch.current += 1; releaseEditor(); unsubscribe(); };
  }, []);

  useEffect(() => {
    if (!isLoaded || !builderScopeActive(scope.current) || canonicalRequest(setupFields()) === lastSavedDetails.current) return;
    const epoch = draftEpoch.current;
    const timer = setTimeout(() => { void saveSetupDetails('state').catch(error => { if (epoch === draftEpoch.current) setSaveMessage(error instanceof Error ? error.message : 'Setup save could not be confirmed.'); }); }, 1000);
    return () => clearTimeout(timer);
  }, [isLoaded, businessName, businessType, productName, productPrice, template, bio, domainChoice, aiAgents, aiAutoRespond, hasPhysicalProducts, hasDigitalProducts]);

  const handleMoveBlock = (fromIndex: number, toIndex: number) => {
    if (!builderScopeActive(viewScope)) return;
    moveBlock(fromIndex, toIndex);
    if (selectedBlockIndex === fromIndex) {
      setSelectedBlockIndex(toIndex);
    } else if (selectedBlockIndex === toIndex) {
      setSelectedBlockIndex(fromIndex);
    }
  };

  const handleLaunch = async () => {
    const current = viewScope;
    if (!builderScopeActive(current) || actionBusy.current) return;
    actionBusy.current = true; const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current && builderScopeActive(current);
    const submitted = useWebsiteBuilderStore.getState();
    const layoutFingerprint = canonicalRequest({ blocks: submitted.blocks, bio: submitted.bio });
    try {
      const draftBlocks = submitted.blocks.map((b, i) => ({
        block_type: b.type === 'Hero' ? 'HeroBlock' :
                    b.type === 'Catalog' ? 'ProductGridBlock' :
                    b.type === 'Booking' ? 'ServiceBookingBlock' :
                    b.type === 'Testimonials' ? 'TestimonialBlock' : b.type,
        content: b.props,
        sort_order: i
      }));

      const payload = {
          domain: null,
          draft: {
              domain: null,
              pages: [{
                  path: '/',
                  title: 'Home',
                  blocks: draftBlocks,
                  seo_metadata: {
                    "@context": "https://schema.org",
                    "@type": "LocalBusiness",
                    "name": submitted.bio
                  }
              }]
          }
      };

      const site = await publishOwnedLayout(current, payload);
      if (active()) {
        const latest = useWebsiteBuilderStore.getState();
        setSaveMessage(canonicalRequest({ blocks: latest.blocks, bio: latest.bio }) === layoutFingerprint ? `Site saved (${site.id}); publishing has not been verified.` : `Earlier layout saved (${site.id}); newer edits remain local. Publishing has not been verified.`);
      }
    } catch (error) {
      if (active()) setSaveMessage(error instanceof Error ? error.message : 'Site save could not be confirmed.');
    } finally { if (active()) actionBusy.current = false; }
  };

  const reviewWorkspaceSetup = async (instant: boolean) => {
    const current = viewScope;
    if (!builderScopeActive(current) || actionBusy.current) return;
    const fields = setupFields(); const fingerprint = canonicalRequest(fields);
    actionBusy.current = true; const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current && builderScopeActive(current);
    try {
      await initializeOnboardingDraft();
      if (!active()) return;
      if (canonicalRequest(setupFields()) !== fingerprint) throw new Error('Your draft changed. Review the latest details before continuing.');
      useOnboardingStore.getState().updateState({ ...fields, step: instant ? -1 : 3, businessDescription: fields.bio, skipped: false, isLoading: false, error: '', startResult: null });
      router.push('/onboarding');
    } catch (error) { if (active()) setSaveMessage(error instanceof Error ? error.message : 'Your setup draft could not be opened.'); }
    finally { if (active()) actionBusy.current = false; }
  };

  if (sessionError) return <div role="alert">{sessionError}</div>;
  if (!isLoaded) return <div role="status">Verifying your builder session…</div>;

  if (status === "idle") {
    const handleBack = () => {
      operationEpoch.current += 1; draftEpoch.current += 1; actionBusy.current = false; savingDraft.current = false; setSaving(false); setSaveMessage('');
      if (wizardStep === 1) setWizardStep(0);
      else if (wizardStep === 2) setWizardStep(1);
      else if (wizardStep === 3) setWizardStep(2);
      else if (wizardStep === 4) setWizardStep(3);
      else if (wizardStep === 5) setWizardStep(4);
      else if (wizardStep === 6) setWizardStep(5);
      else if (wizardStep === 7) setWizardStep(6);
      else if (wizardStep === '7.5') setWizardStep(7);
      else if (wizardStep === 8) setWizardStep('7.5');
      else if (wizardStep === '8.5') setWizardStep(8);
      else if (wizardStep === 9) setWizardStep('8.5');
      else if (wizardStep === 'instant-build') setWizardStep(0);
    };

    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      {/* Background Glows for Premium Aesthetic */}
      <div className="fixed top-[-10%] left-[-10%] w-[40%] h-[40%] bg-[#0066FF]/10 blur-[120px] rounded-full pointer-events-none"></div>
      <div className="fixed bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-[#34C759]/10 blur-[120px] rounded-full pointer-events-none"></div>


        <div id="setup-screen" className="w-full max-w-[375px] sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden translucent-glass-light dark:translucent-glass-dark">

          <div className="px-8 pb-8 pt-8 flex flex-col flex-1 justify-start overflow-y-auto relative">
            {wizardStep !== 0 && (
              <button
                onClick={handleBack}
                className="absolute top-6 left-8 text-[#0066FF] font-medium text-sm hover:underline transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] z-10 flex items-center gap-1 bg-white/50 backdrop-blur-[30px] saturate-[210%] px-3 py-1 rounded-[8px] shadow-sm border border-white/20 min-h-[44px]"
              >
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                Back
              </button>
            )}


            <div className="absolute top-6 right-8 flex items-center gap-4 z-10">
              {saveMessage && <span className="text-[#34C759] text-sm font-semibold animate-fade-in">{saveMessage}</span>}
              {wizardStep !== 0 && wizardStep !== 'instant-build' && (
                <button
                  onClick={handleSaveDraft}
                  disabled={saving}
                  className="text-[#0066FF] font-medium text-sm hover:underline transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] bg-white/50 backdrop-blur-[30px] saturate-[210%] px-3 py-1 rounded-[8px] shadow-sm border border-white/20 min-h-[44px]"
                >
                  Save Draft
                </button>
              )}
            </div>

            <div className={`animate-fade-in ${wizardStep !== 0 ? 'mt-10' : 'mt-4'}`} style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>


              {heldLegacy && <p role="status">An older builder draft remains held on this device.</p>}
              {wizardStep === 0 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">
                    10-Minute Setup Wizard <span className="sr-only">Setup Assistant</span>
                  </h1>
                  <h2 className="text-xl font-semibold font-outfit text-gray-800 dark:text-[#e5e5e7] mb-2">Your business, live in minutes.</h2>
                  <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                    Zero tech skills needed. We do the heavy lifting. Review and add any extra details to help our AI generate the perfect store.
                  </p>

                  <div className="flex flex-col sm:flex-row gap-4 sm:gap-6">
                    <button
                      className="w-full min-h-[54px] bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                      onClick={() => setWizardStep(1)}
                    >
                      Start My Business
                    </button>

                    <button
                      className="w-full min-h-[54px] glass-control text-[#0066FF] border border-[#0066FF] p-4 font-bold shadow-sm hover:bg-[#0066FF]/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                      onClick={() => { setBio(''); setWizardStep('instant-build'); }}
                    >
                      Instant Build
                    </button>
                    <PoweredByOmniSolo tenantId="omnisolo" />
                  </div>
                </>
              )}

              {wizardStep === 1 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">What kind of business are you building?</h1>
                  <div className="flex flex-col sm:flex-row gap-4 sm:gap-6 mt-6">
                    <button
                      className="w-full min-h-[54px] text-[#1D1D1F] dark:text-[#F5F5F7] glass-control p-4 font-bold shadow-sm hover: dark:hover:bg-white/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-left"
                      onClick={() => { setBusinessType('Online Store'); setWizardStep(2); }}
                    >
                      Online Store
                    </button>
                    <button
                      className="w-full min-h-[54px] text-[#1D1D1F] dark:text-[#F5F5F7] glass-control p-4 font-bold shadow-sm hover: dark:hover:bg-white/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-left"
                      onClick={() => { setBusinessType('Restaurant'); setWizardStep(2); }}
                    >
                      Restaurant
                    </button>
                    <button
                      className="w-full min-h-[54px] text-[#1D1D1F] dark:text-[#F5F5F7] glass-control p-4 font-bold shadow-sm hover: dark:hover:bg-white/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-left"
                      onClick={() => { setBusinessType('Real Estate'); setWizardStep(2); }}
                    >
                      Real Estate
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 2 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Give your business a name</h1>
                  <div id="step-3" className="mt-6 flex flex-col sm:flex-row gap-4 sm:gap-6">
                    <input
                      type="text"
                      className="w-full min-h-[54px] glass-control p-4 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-[#1D1D1F] dark:text-[#F5F5F7]    border  "
                      placeholder="What is your business called?"
                      value={businessName}
                      onChange={(e) => setBusinessName(e.target.value)}
                    />
                    <input
                      type="text"
                      className="w-full min-h-[54px] glass-control p-4 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-[#1D1D1F] dark:text-[#F5F5F7]    border  "
                      placeholder="e.g. Maya's Cakes"
                      value={bio}
                      onChange={(e) => setBio(e.target.value)}
                    />
                    <button
                      disabled={!businessName.trim()}
                      className="w-full min-h-[54px] bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setWizardStep(3)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 3 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">What do you sell?</h1>
                  <div id="step-4" className="mt-6 flex flex-col sm:flex-row gap-4 sm:gap-6">
                    <label className="flex items-center gap-3 p-4 glass-control cursor-pointer hover: dark:hover:bg-white/10 text-[#1D1D1F] dark:text-[#F5F5F7]">
                      <input
                        type="checkbox"
                        className="w-5 h-5 accent-[#0066FF]"
                        checked={hasPhysicalProducts}
                        onChange={(e) => setHasPhysicalProducts(e.target.checked)}
                      />
                      <span className="font-semibold text-gray-800">Physical Products</span>
                    </label>
                    <label className="flex items-center gap-3 p-4 glass-control cursor-pointer hover: dark:hover:bg-white/10 text-[#1D1D1F] dark:text-[#F5F5F7]">
                      <input
                        type="checkbox"
                        className="w-5 h-5 accent-[#0066FF]"
                        checked={hasDigitalProducts}
                        onChange={(e) => setHasDigitalProducts(e.target.checked)}
                      />
                      <span className="font-semibold text-gray-800">Digital Products</span>
                    </label>
                    <button
                      className="w-full bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] mt-4"
                      onClick={() => setWizardStep(4)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 4 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Product details</h1>
                  <div id="step-5" className="mt-6 flex flex-col sm:flex-row gap-4 sm:gap-6">
                    <input
                      type="text"
                      className="w-full min-h-[54px] glass-control p-4 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-[#1D1D1F] dark:text-[#F5F5F7]    border  "
                      placeholder="What is the name of this product?"
                      value={productName}
                      onChange={(e) => setProductName(e.target.value)}
                    />
                    <input
                      type="text"
                      className="w-full min-h-[54px] glass-control p-4 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-[#1D1D1F] dark:text-[#F5F5F7]    border  "
                      placeholder="0.00"
                      value={productPrice}
                      onChange={(e) => setProductPrice(e.target.value)}
                    />
                    <button
                      disabled={!productName.trim() || !productPrice.trim()}
                      className="w-full min-h-[54px] bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setWizardStep(5)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 5 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">How do you want to receive payments?</h1>
                  <div className="mt-6 flex flex-col sm:flex-row gap-4 sm:gap-6">
                    <button
                      className="w-full min-h-[54px] text-[#1D1D1F] dark:text-[#F5F5F7] glass-control p-4 font-bold shadow-sm hover: dark:hover:bg-white/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-left"
                      onClick={() => { setPaymentMethod('Online'); setWizardStep(6); }}
                    >
                      Online
                    </button>
                    <button
                      className="w-full min-h-[54px] text-[#1D1D1F] dark:text-[#F5F5F7] glass-control p-4 font-bold shadow-sm hover: dark:hover:bg-white/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-left"
                      onClick={() => { setPaymentMethod('In Person'); setWizardStep(6); }}
                    >
                      In Person
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 6 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Your secure account is ready</h1>
                  <p className="text-sm text-gray-500 dark:text-[#a1a1a6]">We’ll publish this business under your signed-in workspace.</p>
                  <div id="step-7" className="mt-6 flex flex-col sm:flex-row gap-4 sm:gap-6">
                    <button
                      className="w-full min-h-[54px] bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] mt-4"
                      onClick={() => setWizardStep(7)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 7 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Template selection</h1>
                  <div id="step-8" className="flex flex-col sm:flex-row gap-4 sm:gap-6 mt-6">
                    <button
                      className="w-full min-h-[54px] text-[#1D1D1F] dark:text-[#F5F5F7] glass-control p-4 font-bold shadow-sm hover: dark:hover:bg-white/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-left"
                      onClick={() => { setTemplate('Modern'); setWizardStep('7.5'); }}
                    >
                      Modern
                    </button>
                    <button
                      className="w-full min-h-[54px] text-[#1D1D1F] dark:text-[#F5F5F7] glass-control p-4 font-bold shadow-sm hover: dark:hover:bg-white/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] text-left"
                      onClick={() => { setTemplate('Bold'); setWizardStep('7.5'); }}
                    >
                      Bold
                    </button>
                  </div>
                </>
              )}

              {wizardStep === '7.5' && (
                <>
                  <div id="step-8" className="flex flex-col sm:flex-row gap-4 sm:gap-6 mt-6">
                    <button
                      className="w-full bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] mt-4"
                      onClick={() => setWizardStep(8)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 8 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Choose your domain</h1>
                  <div id="step-9" className="flex flex-col sm:flex-row gap-4 sm:gap-6 mt-6">
                    <button
                      className="w-full min-h-[54px] bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                      onClick={() => setWizardStep('8.5')}
                    >
                      Free OmniSolo Domain
                    </button>
                    <button
                      className="w-full min-h-[54px] glass-control text-[#0066FF] border border-[#0066FF] p-4 font-bold shadow-sm hover:bg-[#0066FF]/10 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                      onClick={() => setWizardStep('8.5')}
                    >
                      Connect Custom Domain
                    </button>
                  </div>
                </>
              )}

              {wizardStep === '8.5' && (
                <>
                  <div id="step-9" className="flex flex-col sm:flex-row gap-4 sm:gap-6 mt-6">
                    <button
                      className="w-full bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] mt-4"
                      onClick={() => setWizardStep(9)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 9 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Review your choices</h1>
                  <div className="flex flex-col sm:flex-row gap-4 sm:gap-6 mt-6">
                    <button
                      className="w-full min-h-[54px] bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                      onClick={() => { void reviewWorkspaceSetup(false); }}
                    >
                      Review workspace setup
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 'instant-build' && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">10-Minute Setup Wizard</h1>
<h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Tell us about your business</h2>
                  <div className="flex flex-col sm:flex-row gap-4 sm:gap-6 mt-6">
                    <textarea
                      value={bio}
                      onChange={(e) => setBio(e.target.value)}
                      className="w-full min-h-[54px] glass-control p-4 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] resize-none text-[#1D1D1F] dark:text-[#F5F5F7]    border    rounded-[8px]"
                      placeholder="e.g. I run a local bakery"
                      rows={4}
                    />
                    <button
                      className="w-full min-h-[54px] bg-[#0066FF] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50"
                      disabled={!bio.trim()}
                      onClick={() => { void reviewWorkspaceSetup(true); }}
                    >
                      Review setup options
                    </button>
                  </div>
                </>
              )}

            </div>
          </div>
        </div>
      </div>
    );
  }

  if (status === "generating") {
    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
        <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden justify-center items-center translucent-glass-light dark:translucent-glass-dark">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-[#0066FF] mb-4"></div>
            <p className="text-gray-500 dark:text-[#a1a1a6] font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
        <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden text-center p-8 justify-center translucent-glass-light dark:translucent-glass-dark">
          <div className="w-16 h-16 bg-[#34C759]/10 text-[#34C759] rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Site save recorded</h1>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">Publishing has not been verified.</p>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">You're set up! Here's what to do next:</p>

          <div className="w-full translucent-glass-light dark:translucent-glass-dark p-3 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 dark:text-[#a1a1a6] truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-[#0071E3] font-semibold text-sm hover:underline shrink-0 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]">Copy</button>
          </div>

          <button
            className="w-full bg-[#0066FF] text-white font-bold p-4 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] hover:bg-[#005bb5] rounded-[8px]"
            onClick={() => router.push('/dashboard')}
          >
            View Welcome Checklist
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative overflow-hidden translucent-glass-light dark:translucent-glass-dark">
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-[30px] saturate-[210%] text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        {saveMessage && <p role="status" className="mt-8 px-4 py-2 text-sm">{saveMessage}</p>}
        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {Array.isArray(blocks) && blocks.map((b, i) => (
            <DraggableBlock
              key={b.type + i}
              isSelected={selectedBlockIndex === i}
              onClick={() => setSelectedBlockIndex(i === selectedBlockIndex ? null : i)}
              onDragStart={(e) => {
                if (e.type.includes('drag') && (e as React.DragEvent).dataTransfer) {
                  (e as React.DragEvent).dataTransfer.effectAllowed = 'move';
                  (e as React.DragEvent).dataTransfer.setData('text/plain', i.toString());
                }
                setDraggedIndex(i);
                setSelectedBlockIndex(i);
              }}
              onDragOver={(e) => {
                if (e.type.includes('drag') && (e as React.DragEvent).dataTransfer) {
                  (e as React.DragEvent).dataTransfer.dropEffect = 'move';
                }
              }}
              onDragEnter={() => {
                if (draggedIndex !== null && draggedIndex !== i) {
                  handleMoveBlock(draggedIndex, i);
                  setDraggedIndex(i);
                }
              }}
              onDragEnd={() => setDraggedIndex(null)}
              onMoveUp={i > 0 ? () => handleMoveBlock(i, i - 1) : undefined}
              onMoveDown={i < blocks.length - 1 ? () => handleMoveBlock(i, i + 1) : undefined}
            >
              <SmartBlock {...b} />
            </DraggableBlock>
          ))}
          {/* Default to false for premium status here. In a full implementation, we'd fetch this from the user's profile. */}
          <SmartBlock type="PoweredBy" props={{ tenantId: "omnisolo", isPremium: false }} />
          <div className="text-center mt-4 mb-8">
            <a href="/onboarding?ref=storefront" target="_blank" className="text-xs font-semibold text-gray-500 hover:text-gray-700">⚡ Powered by OmniSolo</a>
          </div>
        </div>

        <div className="absolute bottom-0 w-full p-4 translucent-glass-light dark:translucent-glass-dark z-50 rounded-b-[16px]">
          <WithTooltip id="launch-btn-tooltip" defaultText="Save your reviewed site and request publishing. Publishing completion is checked separately.">
            <button
              id="launch-btn"
              className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-lg hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] flex justify-center items-center gap-2 rounded-[8px]"
              onClick={handleLaunch}
            >
              <span>Save site for publishing</span>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </button>
          </WithTooltip>
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
        .translucent-glass-light dark:translucent-glass-dark { background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 8px; }
        @media (prefers-color-scheme: dark) {
          .translucent-glass-light dark:translucent-glass-dark { background: rgba(22, 22, 26, 0.7); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 8px; }
        }
      `}} />
    </div>
  );
}
