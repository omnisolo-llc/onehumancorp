"use client";

import { useState, useEffect } from "react";
import { SmartBlock, DraggableBlock } from "../builder/components";
import { useWalkthrough } from "../../components/help";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function WebsiteBuilderPage() {
  const [bio, setBio] = useState("");
  const [blocks, setBlocks] = useState<any[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live">("idle");
  const [wizardStep, setWizardStep] = useState<number | string>(0);
  const [liveUrl, setLiveUrl] = useState("");

  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [selectedBlockIndex, setSelectedBlockIndex] = useState<number | null>(null);
  const [tenantId, setTenantId] = useState("storefront");

  // Wizard state bindings
  const [businessName, setBusinessName] = useState("");
  const [businessType, setBusinessType] = useState("");
  const [hasPhysicalProducts, setHasPhysicalProducts] = useState(false);
  const [hasDigitalProducts, setHasDigitalProducts] = useState(false);
  const [productName, setProductName] = useState("");
  const [productPrice, setProductPrice] = useState("");
  const [paymentMethod, setPaymentMethod] = useState("");
  const [userName, setUserName] = useState("");
  const [userEmail, setUserEmail] = useState("");
  const [userPassword, setUserPassword] = useState("");
  const [template, setTemplate] = useState("");
  const { startWalkthrough } = useWalkthrough();

  useEffect(() => {
    const savedTenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "storefront";
    setTenantId(savedTenantId);

    const savedBio = localStorage.getItem("ohc_builder_bio");
    if (savedBio) setBio(savedBio);

    const savedStatus = localStorage.getItem("ohc_builder_status") as "idle" | "generating" | "draft" | "live";
    if (savedStatus) setStatus(savedStatus);

    const savedBlocks = localStorage.getItem("ohc_builder_blocks");
    if (savedBlocks) {
      try {
        setBlocks(JSON.parse(savedBlocks));
      } catch (e) {
        console.error("Failed to parse saved blocks", e);
      }
    }

    const savedLiveUrl = localStorage.getItem("ohc_builder_liveUrl");
    if (savedLiveUrl) setLiveUrl(savedLiveUrl);
  }, []);

  useEffect(() => {
    // Only save to server if there's actual state to save that deviates from idle
    if (status !== 'idle' || bio !== '' || blocks.length > 0) {
      const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
      const userId = localStorage.getItem('user_id') || 'test-user';

      const payload = {
        builderState: { bio, blocks, status }
      };

      const timer = setTimeout(() => {
        fetch('/api/onboarding/state', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
          body: JSON.stringify(payload)
        }).catch(err => console.error('Failed to sync builder state', err));
      }, 1000); // debounce 1s

      return () => clearTimeout(timer);
    }
  }, [bio, blocks, status]);

  // Read state from server on mount
  useEffect(() => {
    const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
    const userId = localStorage.getItem('user_id') || 'test-user';
    fetch('/api/onboarding/state', {
      headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId }
    })
    .then(res => res.json())
    .then(data => {
      if (data && data.builderState) {
        if (data.builderState.bio) setBio(data.builderState.bio);
        if (data.builderState.blocks && Array.isArray(data.builderState.blocks)) setBlocks(data.builderState.blocks);
        if (data.builderState.status) setStatus(data.builderState.status);
      }
    })
    .catch(err => console.error('Failed to load builder state', err));
  }, []);

  const updateBio = (newBio: string) => {
    setBio(newBio);
    localStorage.setItem("ohc_builder_bio", newBio);
  };

  const updateStatus = (newStatus: "idle" | "generating" | "draft" | "live") => {
    setStatus(newStatus);
    localStorage.setItem("ohc_builder_status", newStatus);
  };

  const handleGenerate = async () => {
    setStatus("generating");

    try {
      const response = await fetch('/api/v1/builder/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: bio })
      });

      const data = await response.json();
      const blocks = data.pages[0].blocks.map((b: any) => ({
        type: b.block_type === 'HeroBlock' ? 'Hero' :
              b.block_type === 'ProductGridBlock' ? 'Catalog' :
              b.block_type === 'ServiceBookingBlock' ? 'Booking' :
              b.block_type === 'TestimonialBlock' ? 'Testimonials' : b.block_type,
        props: b.content
      }));
      setBlocks(blocks);
      localStorage.setItem("ohc_builder_blocks", JSON.stringify(blocks));
      updateStatus("draft");
    } catch (error) {
      console.error("Failed to generate storefront", error);
      updateStatus("idle");
    }
  };

  const moveBlock = (fromIndex: number, toIndex: number) => {
    if (toIndex < 0 || toIndex >= blocks.length || fromIndex === toIndex) return;

    setBlocks(prev => {
      const newBlocks = [...prev];
      const [moved] = newBlocks.splice(fromIndex, 1);
      newBlocks.splice(toIndex, 0, moved);
      localStorage.setItem("ohc_builder_blocks", JSON.stringify(newBlocks));
      return newBlocks;
    });

    if (selectedBlockIndex === fromIndex) {
      setSelectedBlockIndex(toIndex);
    } else if (selectedBlockIndex === toIndex) {
      setSelectedBlockIndex(fromIndex);
    }
  };

  const handleLaunch = async () => {
    try {
      const draftBlocks = blocks.map((b, i) => ({
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
                    "name": bio
                  }
              }]
          }
      };

      const response = await fetch('/api/v1/builder/publish_draft', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
      });
      if (response.ok) {
        const data = await response.json();
        updateStatus("live");
        const url = `https://${data.domain || 'myshop'}.ohc.store`;
        setLiveUrl(url);
        localStorage.setItem("ohc_builder_liveUrl", url);
      } else {
        console.error('Failed to publish');
      }
    } catch (error) {
      console.error('Error publishing:', error);
    }
  };

  if (status === "idle") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div id="setup-screen" className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden mac-glass-container">

          <div className="px-8 pb-8 pt-12 flex flex-col flex-1 justify-start overflow-y-auto">
            <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>

              {wizardStep === 0 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Your business, live in minutes.</h1>
                  <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                    Zero tech skills needed. We do the heavy lifting.
                  </p>

                  <div className="flex flex-col gap-4">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => setWizardStep(1)}
                    >
                      Start My Business Next
                    </button>

                    <button
                      className="w-full bg-white text-[#0071E3] border border-[#0071E3] p-4 font-bold rounded-[8px] shadow-sm hover:bg-blue-50 transition-all"
                      onClick={() => setWizardStep('instant-build')}
                    >
                      Instant Build
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 1 && (
                <>
                  <button onClick={() => setWizardStep(0)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">What kind of business are you building?</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-white text-gray-800 border border-gray-200 p-4 font-bold rounded-[8px] shadow-sm hover:bg-gray-50 transition-all text-left"
                      onClick={() => { setBusinessType('Online Store'); setWizardStep(2); }}
                    >
                      Online Store
                    </button>
                    <button
                      className="w-full bg-white text-gray-800 border border-gray-200 p-4 font-bold rounded-[8px] shadow-sm hover:bg-gray-50 transition-all text-left"
                      onClick={() => { setBusinessType('Restaurant'); setWizardStep(2); }}
                    >
                      Restaurant
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 2 && (
                <>
                  <button onClick={() => setWizardStep(1)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Give your business a name</h1>
                  <div className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7]"
                      style={{ borderRadius: '8px' }}
                      placeholder="What is your business called?"
                      value={businessName}
                      onChange={(e) => setBusinessName(e.target.value)}
                    />
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setWizardStep(3)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 3 && (
                <>
                  <button onClick={() => setWizardStep(2)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">What do you sell?</h1>
                  <div className="mt-6 flex flex-col gap-4">
                    <label className="flex items-center gap-3 p-4 border border-gray-200 rounded-[8px] cursor-pointer hover:bg-gray-50 bg-white">
                      <input
                        type="checkbox"
                        className="w-5 h-5 accent-[#0071E3]"
                        checked={hasPhysicalProducts}
                        onChange={(e) => setHasPhysicalProducts(e.target.checked)}
                      />
                      <span className="font-semibold text-gray-800">Physical Products</span>
                    </label>
                    <label className="flex items-center gap-3 p-4 border border-gray-200 rounded-[8px] cursor-pointer hover:bg-gray-50 bg-white">
                      <input
                        type="checkbox"
                        className="w-5 h-5 accent-[#0071E3]"
                        checked={hasDigitalProducts}
                        onChange={(e) => setHasDigitalProducts(e.target.checked)}
                      />
                      <span className="font-semibold text-gray-800">Digital Products</span>
                    </label>
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setWizardStep(4)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 4 && (
                <>
                  <button onClick={() => setWizardStep(3)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Product details</h1>
                  <div className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7]"
                      style={{ borderRadius: '8px' }}
                      placeholder="What is the name of this product?"
                      value={productName}
                      onChange={(e) => setProductName(e.target.value)}
                    />
                    <input
                      type="text"
                      className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7]"
                      style={{ borderRadius: '8px' }}
                      placeholder="0.00"
                      value={productPrice}
                      onChange={(e) => setProductPrice(e.target.value)}
                    />
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setWizardStep(5)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 5 && (
                <>
                  <button onClick={() => setWizardStep(4)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">How do you want to receive payments?</h1>
                  <div className="mt-6 flex flex-col gap-4">
                    <button
                      className="w-full bg-white text-gray-800 border border-gray-200 p-4 font-bold rounded-[8px] shadow-sm hover:bg-gray-50 transition-all text-left"
                      onClick={() => { setPaymentMethod('Online'); setWizardStep(6); }}
                    >
                      Online
                    </button>
                    <button
                      className="w-full bg-white text-gray-800 border border-gray-200 p-4 font-bold rounded-[8px] shadow-sm hover:bg-gray-50 transition-all text-left"
                      onClick={() => { setPaymentMethod('In Person'); setWizardStep(6); }}
                    >
                      In Person
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 6 && (
                <>
                  <button onClick={() => setWizardStep(5)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Create your account</h1>
                  <div className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7]"
                      style={{ borderRadius: '8px' }}
                      placeholder="e.g. Maya Smith"
                      value={userName}
                      onChange={(e) => setUserName(e.target.value)}
                    />
                    <input
                      type="email"
                      className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7]"
                      style={{ borderRadius: '8px' }}
                      placeholder="you@email.com"
                      value={userEmail}
                      onChange={(e) => setUserEmail(e.target.value)}
                    />
                    <input
                      type="password"
                      className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7]"
                      style={{ borderRadius: '8px' }}
                      placeholder="Password"
                      value={userPassword}
                      onChange={(e) => setUserPassword(e.target.value)}
                    />
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setWizardStep(7)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 7 && (
                <>
                  <button onClick={() => setWizardStep(6)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Template selection</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-white text-gray-800 border border-gray-200 p-4 font-bold rounded-[8px] shadow-sm hover:bg-gray-50 transition-all text-left"
                      onClick={() => { setTemplate('Modern'); setWizardStep('7.5'); }}
                    >
                      Modern
                    </button>
                    <button
                      className="w-full bg-white text-gray-800 border border-gray-200 p-4 font-bold rounded-[8px] shadow-sm hover:bg-gray-50 transition-all text-left"
                      onClick={() => { setTemplate('Bold'); setWizardStep('7.5'); }}
                    >
                      Bold
                    </button>
                  </div>
                </>
              )}

              {wizardStep === '7.5' && (
                <>
                  <button onClick={() => setWizardStep(7)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setWizardStep(8)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 8 && (
                <>
                  <button onClick={() => setWizardStep('7.5')} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Choose your domain</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => setWizardStep('8.5')}
                    >
                      Free OHC Domain
                    </button>
                    <button
                      className="w-full bg-white text-[#0071E3] border border-[#0071E3] p-4 font-bold rounded-[8px] shadow-sm hover:bg-blue-50 transition-all"
                      onClick={() => setWizardStep('8.5')}
                    >
                      Connect Custom Domain
                    </button>
                  </div>
                </>
              )}

              {wizardStep === '8.5' && (
                <>
                  <button onClick={() => setWizardStep(8)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setWizardStep(9)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 9 && (
                <>
                  <button onClick={() => setWizardStep('8.5')} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Review your choices</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => {
                        setStatus('generating');
                        setTimeout(() => {
                           setStatus('live');
                        }, 2000);
                      }}
                    >
                      Publish my business
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 'instant-build' && (
                <>
                  <button onClick={() => setWizardStep(0)} className="self-start text-[#0071E3] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Describe your business in a sentence</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <textarea
                      className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800 dark:text-[#f5f5f7]"
                      style={{ borderRadius: '8px' }}
                      placeholder="e.g. I run a local bakery"
                      rows={4}
                    />
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => {
                        setStatus('generating');
                        setTimeout(() => {
                           setStatus('live');
                        }, 2000);
                      }}
                    >
                      Generate Storefront
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
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden justify-center items-center mac-glass-container">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
            <p className="text-gray-500 dark:text-[#a1a1a6] font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden text-center p-8 justify-center mac-glass-container">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Success! Your business is live!</h1>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">Your automated storefront is successfully published.</p>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">You're set up! Here's what to do next:</p>

          <div className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 dark:text-[#a1a1a6] truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          <button
            className="w-full bg-[#0071E3] text-white font-bold p-4 active:scale-[0.98] transition-all hover:bg-[#005bb5]"
            style={{ borderRadius: '8px' }}
          >
            View Welcome Checklist
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden mac-glass-container">
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
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
                  moveBlock(draggedIndex, i);
                  setDraggedIndex(i);
                }
              }}
              onDragEnd={() => setDraggedIndex(null)}
              onMoveUp={i > 0 ? () => moveBlock(i, i - 1) : undefined}
              onMoveDown={i < blocks.length - 1 ? () => moveBlock(i, i + 1) : undefined}
            >
              <SmartBlock {...b} />
            </DraggableBlock>
          ))}
          {/* Default to false for premium status here. In a full implementation, we'd fetch this from the user's profile. */}
          <SmartBlock type="PoweredBy" props={{ tenantId, isPremium: false }} />
        </div>

        <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50" style={{ borderRadius: '0 0 16px 16px' }}>
          <WithTooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
            <button
              id="launch-btn"
              className="w-full bg-blue-600 text-white p-4 font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
              style={{ borderRadius: '8px' }}
              onClick={handleLaunch}
            >
              <span>1-Tap Launch</span>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </button>
          </WithTooltip>
        </div>
      </div>
    </div>
  );
}
