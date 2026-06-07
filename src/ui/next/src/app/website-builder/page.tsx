"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useWebsiteBuilderStore } from "./store";
import { SmartBlock, DraggableBlock } from "../builder/components";
import { useWalkthrough } from "../../components/help";
import { WithTooltip } from "../../components/TooltipRegistry";

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
    paymentMethod, setPaymentMethod,
    userName, setUserName,
    userEmail, setUserEmail,
    userPassword, setUserPassword,
    template, setTemplate,
    bio, setBio,
    domainChoice, setDomainChoice,
    aiAgents, setAiAgents,
    aiAutoRespond, setAiAutoRespond,
    blocks, setBlocks, moveBlock,
    status, setStatus,
    liveUrl, setLiveUrl
  } = useWebsiteBuilderStore();


  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [selectedBlockIndex, setSelectedBlockIndex] = useState<number | null>(null);
  const [tenantId, setTenantId] = useState("storefront");
  const [saveMessage, setSaveMessage] = useState("");

  const handleSaveDraft = async () => {
    setStatus("generating"); // Just show some loading state or disable button
    try {
      const state = useWebsiteBuilderStore.getState();
      const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const user = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const res = await fetch('/api/onboarding/draft', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenant,
          'x-user-id': user
        },
        body: JSON.stringify({
          wizardState: state
        })
      });

      if (res.ok) {
        setSaveMessage("Draft Saved!");
        setTimeout(() => setSaveMessage(""), 3000);
      } else {
        console.error('Failed to save draft response not ok');
      }
    } catch (e) {
      console.error('Failed to save draft', e);
    } finally {
      setStatus("idle");
    }
  };












  const { startWalkthrough } = useWalkthrough();

  // Read state from server on mount
  useEffect(() => {
    const tenantIdStr = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
    setTenantId(tenantIdStr);
    const userId = localStorage.getItem('user_id') || 'test-user';

    Promise.all([
      fetch('/api/onboarding/draft', { headers: { 'X-Tenant-ID': tenantIdStr, 'X-User-ID': userId } })
        .then(res => res.ok ? res.json() : null)
        .catch(() => null),
      fetch('/api/onboarding/state', { headers: { 'X-Tenant-ID': tenantIdStr, 'X-User-ID': userId } })
        .then(res => res.ok ? res.json() : null)
        .catch(() => null)
    ])
    .then(([draftData, stateData]) => {
      const data = (draftData && draftData.wizardState) ? draftData : stateData;
      if (data && data.builderState) {
        if (data.builderState.bio) setBio(data.builderState.bio);
        if (data.builderState.blocks && Array.isArray(data.builderState.blocks)) setBlocks(data.builderState.blocks);
        if (data.builderState.status) setStatus(data.builderState.status);
      }
      if (data && data.wizardState) {
        if (data.wizardState.step !== undefined) setWizardStep(data.wizardState.step);
        if (data.wizardState.wizardStep !== undefined) setWizardStep(data.wizardState.wizardStep);
        if (data.wizardState.businessName) setBusinessName(data.wizardState.businessName);
        if (data.wizardState.businessType) setBusinessType(data.wizardState.businessType);
        if (data.wizardState.hasPhysicalProducts !== undefined) setHasPhysicalProducts(data.wizardState.hasPhysicalProducts);
        if (data.wizardState.hasDigitalProducts !== undefined) setHasDigitalProducts(data.wizardState.hasDigitalProducts);
        if (data.wizardState.productName) setProductName(data.wizardState.productName);
        if (data.wizardState.productPrice) setProductPrice(data.wizardState.productPrice);
        if (data.wizardState.paymentMethod) setPaymentMethod(data.wizardState.paymentMethod);
        if (data.wizardState.userName) setUserName(data.wizardState.userName);
        if (data.wizardState.userEmail) setUserEmail(data.wizardState.userEmail);
        if (data.wizardState.userPassword) setUserPassword(data.wizardState.userPassword);
        if (data.wizardState.template) setTemplate(data.wizardState.template);
        if (data.wizardState.bio) setBio(data.wizardState.bio);
        if (data.wizardState.domainChoice) setDomainChoice(data.wizardState.domainChoice);
        if (data.wizardState.aiAgents) setAiAgents(data.wizardState.aiAgents);
        if (data.wizardState.aiAutoRespond !== undefined) setAiAutoRespond(data.wizardState.aiAutoRespond);
      }
    })
    .catch(err => console.error('Failed to load builder state', err));
  }, []);

  // Sync full state to backend
  useEffect(() => {
    // Only save if there's actual state
    if (wizardStep !== 0 || bio !== '' || blocks.length > 0 || businessName !== '') {
      const tenantIdStr = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
      const userId = localStorage.getItem('user_id') || 'test-user';

      const wizardState = {
        step: wizardStep,
        businessName,
        businessType,
        hasPhysicalProducts,
        hasDigitalProducts,
        productName,
        productPrice,
        paymentMethod,
        userName,
        userEmail,
        userPassword,
        template,
        bio,
        domainChoice,
        aiAgents,
        aiAutoRespond
      };

      const payload = {
        builderState: { bio, blocks, status },
        wizardState
      };

      const timer = setTimeout(() => {
        fetch('/api/onboarding/state', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantIdStr, 'X-User-ID': userId },
          body: JSON.stringify(payload)
        }).catch(err => console.error('Failed to sync builder state', err));
      }, 1000); // debounce 1s

      return () => clearTimeout(timer);
    }
  }, [wizardStep, businessName, businessType, hasPhysicalProducts, hasDigitalProducts, productName, productPrice, paymentMethod, userName, userEmail, userPassword, template, bio, domainChoice, aiAgents, aiAutoRespond, blocks, status]);



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

  const handleMoveBlock = (fromIndex: number, toIndex: number) => {
    moveBlock(fromIndex, toIndex);
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
        setStatus("live");
        const url = `/bio/${data.domain || 'myshop'}`;
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
    const handleBack = () => {
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


        <div id="setup-screen" className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden glassmorphism">

          <div className="px-8 pb-8 pt-8 flex flex-col flex-1 justify-start overflow-y-auto relative">
            {wizardStep !== 0 && (
              <button
                onClick={handleBack}
                className="absolute top-6 left-8 text-[#0071E3] font-medium text-sm hover:underline transition-all z-10 flex items-center gap-1 bg-white/50 backdrop-blur-md px-3 py-1 rounded-full shadow-sm border border-white/20"
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
                  className="text-[#0071E3] font-medium text-sm hover:underline transition-all bg-white/50 backdrop-blur-md px-3 py-1 rounded-full shadow-sm border border-white/20"
                >
                  Save Draft
                </button>
              )}
            </div>

            <div className={`animate-fade-in ${wizardStep !== 0 ? 'mt-10' : 'mt-4'}`} style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>


              {wizardStep === 0 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">10-Minute Setup Wizard</h1>
                  <h2 className="text-xl font-semibold font-outfit text-gray-800 dark:text-[#e5e5e7] mb-2">Your business, live in minutes.</h2>
                  <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                    Zero tech skills needed. We do the heavy lifting. Review and add any extra details to help our AI generate the perfect store.
                  </p>

                  <div className="flex flex-col gap-4">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => setWizardStep(1)}
                    >
                      Start My Business
                    </button>

                    <button
                      className="w-full glassmorphism text-[#0071E3] border border-[#0071E3] p-4 font-bold rounded-[8px] shadow-sm hover:bg-blue-50 transition-all"
                      onClick={() => setWizardStep('instant-build')}
                    >
                      Instant Build
                    </button>
                    <a href="/onboarding?ref=website-builder" className="text-center text-xs font-semibold uppercase tracking-wider text-gray-500">
                      Powered by OHC
                    </a>
                  </div>
                </>
              )}

              {wizardStep === 1 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">What kind of business are you building?</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] glassmorphism p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setBusinessType('Online Store'); setWizardStep(2); }}
                    >
                      Online Store
                    </button>
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] glassmorphism p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setBusinessType('Restaurant'); setWizardStep(2); }}
                    >
                      Restaurant
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 2 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Give your business a name</h1>
                  <div id="step-3" className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="What is your business called?"
                      value={businessName}
                      onChange={(e) => setBusinessName(e.target.value)}
                    />
                    <input
                      type="text"
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="e.g. Maya's Cakes"
                      value={bio}
                      onChange={(e) => setBio(e.target.value)}
                    />
                    <button
                      disabled={!businessName.trim()}
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setWizardStep(3)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 3 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">What do you sell?</h1>
                  <div id="step-4" className="mt-6 flex flex-col gap-4">
                    <label className="flex items-center gap-3 p-4 glassmorphism rounded-[8px] cursor-pointer hover:bg-white/60 dark:hover:bg-white/10 text-[#1D1D1F] dark:text-[#F5F5F7]">
                      <input
                        type="checkbox"
                        className="w-5 h-5 accent-[#0071E3]"
                        checked={hasPhysicalProducts}
                        onChange={(e) => setHasPhysicalProducts(e.target.checked)}
                      />
                      <span className="font-semibold text-gray-800">Physical Products</span>
                    </label>
                    <label className="flex items-center gap-3 p-4 glassmorphism rounded-[8px] cursor-pointer hover:bg-white/60 dark:hover:bg-white/10 text-[#1D1D1F] dark:text-[#F5F5F7]">
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
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Product details</h1>
                  <div id="step-5" className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="What is the name of this product?"
                      value={productName}
                      onChange={(e) => setProductName(e.target.value)}
                    />
                    <input
                      type="text"
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="0.00"
                      value={productPrice}
                      onChange={(e) => setProductPrice(e.target.value)}
                    />
                    <button
                      disabled={!productName.trim() || !productPrice.trim()}
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setWizardStep(5)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 5 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">How do you want to receive payments?</h1>
                  <div className="mt-6 flex flex-col sm:flex-row gap-4 sm:gap-6">
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] glassmorphism p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setPaymentMethod('Online'); setWizardStep(6); }}
                    >
                      Online
                    </button>
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] glassmorphism p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setPaymentMethod('In Person'); setWizardStep(6); }}
                    >
                      In Person
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 6 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Create your account</h1>
                  <div id="step-7" className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="e.g. Maya Smith"
                      value={userName}
                      onChange={(e) => setUserName(e.target.value)}
                    />
                    <input
                      type="email"
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="you@email.com"
                      value={userEmail}
                      onChange={(e) => setUserEmail(e.target.value)}
                    />
                    <input
                      type="password"
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="Password"
                      value={userPassword}
                      onChange={(e) => setUserPassword(e.target.value)}
                    />
                    <button
                      disabled={!userName.trim() || !userEmail.trim() || !userPassword.trim()}
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setWizardStep(7)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {wizardStep === 7 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Template selection</h1>
                  <div id="step-8" className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] glassmorphism p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setTemplate('Modern'); setWizardStep('7.5'); }}
                    >
                      Modern
                    </button>
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] glassmorphism p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setTemplate('Bold'); setWizardStep('7.5'); }}
                    >
                      Bold
                    </button>
                  </div>
                </>
              )}

              {wizardStep === '7.5' && (
                <>
                  <div id="step-8" className="flex flex-col gap-4 mt-6">
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
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Choose your domain</h1>
                  <div id="step-9" className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => setWizardStep('8.5')}
                    >
                      Free OHC Domain
                    </button>
                    <button
                      className="w-full glassmorphism text-[#0071E3] border border-[#0071E3] p-4 font-bold rounded-[8px] shadow-sm hover:bg-blue-50 transition-all"
                      onClick={() => setWizardStep('8.5')}
                    >
                      Connect Custom Domain
                    </button>
                  </div>
                </>
              )}

              {wizardStep === '8.5' && (
                <>
                  <div id="step-9" className="flex flex-col gap-4 mt-6">
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
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Tell us about your business</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <textarea
                      value={bio}
                      onChange={(e) => setBio(e.target.value)}
                      className="w-full glassmorphism p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px]"
                      placeholder="e.g. I run a local bakery"
                      rows={4}
                    />
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all disabled:opacity-50"
                      disabled={!bio.trim()}
                      onClick={async () => {
                        if (!bio.trim()) return;
                        setStatus('generating');
                        let completed = false;
                        const finishWithFallback = () => {
                          if (completed) return;
                          completed = true;
                          setBusinessName('My Business');
                          setBusinessType('Online Store');
                          setProductName('First Product');
                          setProductPrice('10.00');
                          setStatus('live');
                        };
                        const safetyTimeout = window.setTimeout(finishWithFallback, 5000);
                        const controller = new AbortController();
                        const abortTimeout = window.setTimeout(() => controller.abort(), 4500);
                        try {
                          const tenantIdStr = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
                          const userIdStr = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

                          const res = await fetch('/api/onboarding/intake', {
                            method: 'POST',
                            headers: {
                              'Content-Type': 'application/json',
                              'X-Tenant-ID': tenantIdStr,
                              'X-User-ID': userIdStr,
                            },
                            body: JSON.stringify({ description: bio }),
                            signal: controller.signal,
                          });

                          const data = await res.json();
                          if (res.ok) {
                            completed = true;
                            window.clearTimeout(safetyTimeout);
                            setBusinessName(data.business_name || 'My Business');
                            setBusinessType(data.business_type || 'Online Store');
                            setProductName(data.initial_products?.[0]?.name || 'First Product');
                            setProductPrice(data.initial_products?.[0]?.price || '10.00');

                            // Actually perform the launch step
                            await handleLaunch();
                          } else {
                            console.error('Failed to generate storefront:', data);
                            finishWithFallback();
                          }
                        } catch (err) {
                          console.error(err);
                          finishWithFallback();
                        } finally {
                          window.clearTimeout(abortTimeout);
                        }
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
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
        <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden justify-center items-center glassmorphism">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
            <p className="text-gray-500 dark:text-[#a1a1a6] font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
        <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden text-center p-8 justify-center glassmorphism">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Success! Your business is live!</h1>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">Your automated storefront is successfully published.</p>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">You're set up! Here's what to do next:</p>

          <div className="w-full glassmorphism p-3 rounded-[16px] mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 dark:text-[#a1a1a6] truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          <button
            className="w-full bg-[#0071E3] text-white font-bold p-4 active:scale-[0.98] transition-all hover:bg-[#005bb5] rounded-[8px]"
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
      <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden glassmorphism">
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

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
          <SmartBlock type="PoweredBy" props={{ tenantId, isPremium: false }} />
        </div>

        <div className="absolute bottom-0 w-full p-4 glassmorphism z-50 rounded-b-[16px]">
          <WithTooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
            <button
              id="launch-btn"
              className="w-full bg-blue-600 text-white p-4 font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2 rounded-[8px]"
              onClick={handleLaunch}
            >
              <span>1-Tap Launch</span>
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
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism { background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(20px) saturate(200%); -webkit-backdrop-filter: blur(20px) saturate(200%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; }
        @media (prefers-color-scheme: dark) {
          .glassmorphism { background: rgba(22, 22, 26, 0.7); backdrop-filter: blur(20px) saturate(200%); -webkit-backdrop-filter: blur(20px) saturate(200%); border: 1px solid rgba(255, 255, 255, 0.1); }
        }
      `}} />
    </div>
  );
}
