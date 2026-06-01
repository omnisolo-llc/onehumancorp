"use client";

import { useEffect, useState } from "react";
import { SmartBlock, DraggableBlock } from "../builder/components";
import { useWalkthrough } from "../../components/help";
import { WithTooltip } from "../../components/TooltipRegistry";
import { useStorefrontBuilderStore } from "./store";

export default function StorefrontBuilderPage() {
  const {
    bio, setBio,
    blocks, setBlocks,
    status, setStatus,
    liveUrl, setLiveUrl,
    draggedIndex, setDraggedIndex,
    selectedBlockIndex, setSelectedBlockIndex,
    tenantId, setTenantId,
    moveBlock
  } = useStorefrontBuilderStore();
  const { startWalkthrough } = useWalkthrough();
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    setIsLoaded(true);
    const savedTenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "storefront";
    setTenantId(savedTenantId);
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
      setStatus("draft");
    } catch (error) {
      console.error("Failed to generate storefront", error);
      setStatus("idle");
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
        const url = `https://${data.domain || 'myshop'}.ohc.store`;
        setLiveUrl(url);
      } else {
        console.error('Failed to publish');
      }
    } catch (error) {
      console.error('Error publishing:', error);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter">
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden mac-glass-container transition-all duration-300 ease-in-out">
        {status === "idle" && (
          <div id="setup-screen" className="px-8 pb-8 pt-12 flex flex-col flex-1 justify-start overflow-y-auto animate-fade-in">
            <h1 className="text-3xl font-extrabold tracking-tight font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Welcome to OHC</h1>
            <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
              Review and add any extra details to help our AI generate the perfect store.
            </p>

            <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 block">Your Business Details</label>
            <WithTooltip id="bio-input-tooltip" defaultText="Describe what you sell, your target audience, and the vibe of your brand.">
              <textarea
                id="bio-input"
                enterKeyHint="done"
                autoCapitalize="sentences"
                className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 mb-8 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800 dark:text-[#f5f5f7]"
                style={{ borderRadius: '8px' }}
                value={bio}
                onChange={(e) => setBio(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    if (bio.trim().length > 5) {
                      handleGenerate();
                    }
                  }
                }}
                placeholder="e.g. I run a mobile dog grooming service in Portland"
                rows={6}
              />
            </WithTooltip>

            <div className="flex gap-4">
              <WithTooltip id="generate-btn-tooltip" defaultText="Our AI agents will analyze your description and build a ready-to-launch store for you.">
                <button
                  id="generate-btn"
                  className={`flex-[2] p-4 font-bold font-outfit text-lg transition-all ${
                    bio.trim().length > 5
                      ? "text-white shadow-md active:scale-[0.98]"
                      : "bg-gray-100 text-gray-400 cursor-not-allowed"
                  }`}
                  style={{ borderRadius: '8px', background: (bio.trim().length > 5) ? '#0071E3' : '' }}
                  onClick={handleGenerate}
                  disabled={bio.trim().length <= 5}
                >
                  Build My Storefront
                </button>
              </WithTooltip>
            </div>
          </div>
        )}

        {status === "generating" && (
          <div className="flex flex-col flex-1 justify-center items-center animate-fade-in p-8 text-center">
            <div className="w-24 h-24 relative mb-8">
              <div className="absolute inset-0 border-4 border-[#0071E3]/20 rounded-full"></div>
              <div className="absolute inset-0 border-4 border-[#0071E3] rounded-full border-t-transparent animate-spin"></div>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-4">Building Your Store...</h2>
            <div className="space-y-2">
              <p className="text-gray-500 dark:text-[#a1a1a6] text-sm animate-pulse">Generating your product catalog</p>
              <p className="text-gray-500 dark:text-[#a1a1a6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Designing your storefront</p>
              <p className="text-gray-500 dark:text-[#a1a1a6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Onboarding your AI agents</p>
            </div>
          </div>
        )}

        {status === "draft" && (
          <>
            <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center animate-fade-in">
              <span>Preview Mode</span>
              <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
            </div>

            <div className="flex-1 overflow-y-auto pb-24 pt-12 hide-scrollbar animate-fade-in">
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
              <SmartBlock type="PoweredBy" props={{ tenantId, isPremium: false }} />
            </div>

            <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50 animate-fade-in" style={{ borderRadius: '0 0 16px 16px' }}>
              <WithTooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
                <button
                  id="launch-btn"
                  className="w-full bg-[#0071E3] text-white p-4 font-bold shadow-lg hover:bg-[#005bb5] active:scale-[0.98] transition-all flex justify-center items-center gap-2"
                  style={{ borderRadius: '8px' }}
                  onClick={handleLaunch}
                >
                  <span>1-Tap Launch</span>
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                </button>
              </WithTooltip>
            </div>
          </>
        )}

        {status === "live" && (
          <div className="flex flex-col flex-1 justify-center items-center text-center p-8 animate-fade-in">
            <div className="w-20 h-20 bg-[#34C759]/20 rounded-full flex items-center justify-center mb-6">
              <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h2 className="text-3xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">You're Live!</h2>
            <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 px-4">
              Your automated storefront is successfully published.
            </p>

            <div className="w-full space-y-3 mt-auto">
              <div className="p-3 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[8px] border border-white/50 dark:border-white/10 flex flex-col items-center mb-6">
                <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
                <div className="flex items-center gap-2">
                  <span className="text-[#0071E3] font-semibold">{liveUrl || "myshop.ohc.store"}</span>
                </div>
              </div>

              <button
                className="w-full bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] p-4 rounded-[8px] font-bold shadow-md hover:bg-black dark:hover:bg-gray-200 active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
                onClick={() => setStatus("idle")}
              >
                Go to Dashboard
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
