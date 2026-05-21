"use client";

import { useState, useEffect } from "react";
import { SmartBlock } from "../builder/components";
import { Tooltip, useWalkthrough } from "../../components/help";

export default function WebsiteBuilderPage() {
  const [bio, setBio] = useState("");
  const [blocks, setBlocks] = useState<any[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live">("idle");
  const [liveUrl, setLiveUrl] = useState("");
  const [tenantId, setTenantId] = useState("storefront");
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
        <div className="w-[375px] h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200"
             style={{ background: 'rgba(255, 255, 255, 0.45)', backdropFilter: 'blur(40px) saturate(250%)', border: '1px solid rgba(255, 255, 255, 0.5)', borderRadius: '24px', boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.07)' }}>

          <div className="px-8 pb-8 pt-12 flex flex-col flex-1 justify-start overflow-y-auto">
            <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
              <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Welcome to OHC Smart Builder</h1>
              <p className="text-gray-500 text-sm mb-8 leading-relaxed">
                Review and add any extra details to help our AI generate the perfect store.
              </p>

              <label className="text-sm font-semibold text-gray-700 mb-2 block">Your Business Details</label>
              <Tooltip id="bio-input-tooltip" defaultText="Describe what you sell, your target audience, and the vibe of your brand.">
                <textarea
                  id="bio-input"
                  enterKeyHint="done"
                  autoCapitalize="sentences"
                  className="w-full border border-gray-200 bg-white/70 backdrop-blur-sm p-4 mb-8 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800"
                  style={{ borderRadius: '12px' }}
                  value={bio}
                  onChange={(e) => updateBio(e.target.value)}
                  placeholder="e.g. I run a mobile dog grooming service in Portland"
                  rows={6}
                />
              </Tooltip>

              <div className="flex gap-4">
                <Tooltip id="generate-btn-tooltip" defaultText="Our AI agents will analyze your description and build a ready-to-launch store for you.">
                  <button
                    id="generate-btn"
                    className={`flex-[2] p-4 font-bold font-outfit text-lg transition-all ${
                      bio.trim().length > 5
                        ? "text-white shadow-md active:scale-[0.98]"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                    }`}
                    style={{ borderRadius: '12px', background: (bio.trim().length > 5) ? '#0071E3' : '' }}
                    onClick={handleGenerate}
                    disabled={bio.trim().length <= 5}
                  >
                    Build My Storefront
                  </button>
                </Tooltip>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (status === "generating") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] shadow-2xl flex flex-col relative border-x border-gray-200 justify-center items-center"
             style={{ background: 'rgba(255, 255, 255, 0.45)', backdropFilter: 'blur(40px) saturate(250%)', border: '1px solid rgba(255, 255, 255, 0.5)', borderRadius: '24px', boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.07)' }}>
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
            <p className="text-gray-500 font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    const shareUrl = `${liveUrl}?ref=${tenantId}`;
    const shareText = encodeURIComponent(`I just launched my new store! Check it out and start your own business today: ${shareUrl}`);
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden text-center justify-start p-6 pt-12"
             style={{ background: 'rgba(255, 255, 255, 0.45)', backdropFilter: 'blur(40px) saturate(250%)', border: '1px solid rgba(255, 255, 255, 0.5)', borderRadius: '24px', boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.07)' }}>
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm shrink-0">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h1>
          <p className="text-gray-500 mb-6 text-sm">Your automated storefront is successfully published.</p>

          <div className="w-full bg-white p-4 rounded-xl border border-gray-200 mb-6 text-left shadow-sm">
            <div className="flex items-center gap-3 mb-2">
                <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center text-blue-600 font-bold font-outfit">S</div>
                <div>
                    <h3 className="font-semibold text-gray-900 leading-tight">My Store</h3>
                    <p className="text-xs text-gray-500">Just Launched</p>
                </div>
            </div>
            <p className="text-sm text-gray-700 mb-3">{bio ? (bio.length > 60 ? bio.substring(0, 60) + '...' : bio) : 'Check out my new business!'}</p>
            <div className="w-full bg-gray-50 p-3 rounded-lg border border-gray-100 flex items-center justify-between">
              <span className="text-sm text-gray-700 truncate mr-2 font-medium">{liveUrl}</span>
              <button onClick={() => navigator.clipboard.writeText(shareUrl)} className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy Link</button>
            </div>
          </div>

          <div className="bg-gradient-to-br from-indigo-50 to-purple-50 p-4 rounded-xl border border-indigo-100 mb-6 text-left">
              <div className="flex items-start gap-3">
                  <span className="text-xl">🎁</span>
                  <div>
                      <h4 className="font-semibold text-indigo-900 text-sm mb-1">Viral Loop Bonus</h4>
                      <p className="text-xs text-indigo-700">Share your launch! If another business owner signs up via your link, you get 1 month of Pro free.</p>
                  </div>
              </div>
          </div>

          <div className="flex gap-3 w-full mb-auto">
            <a
              href={`https://twitter.com/intent/tweet?text=${shareText}`}
              target="_blank"
              rel="noopener noreferrer"
              onClick={() => {
                fetch('/api/v1/growth/referrals/click', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ id: tenantId })
                }).catch(console.error);
              }}
              className="flex-1 bg-black text-white flex items-center justify-center gap-2 p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share
            </a>
            <a
              href={`https://wa.me/?text=${shareText}`}
              target="_blank"
              rel="noopener noreferrer"
              onClick={() => {
                fetch('/api/v1/growth/referrals/click', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ id: tenantId })
                }).catch(console.error);
              }}
              className="flex-1 bg-[#25D366] text-white flex items-center justify-center gap-2 p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
              WhatsApp
            </a>
          </div>

          <button
            className="w-full mt-4 bg-gray-100 text-gray-800 font-bold p-4 active:scale-[0.98] transition-all hover:bg-gray-200"
            style={{ borderRadius: '12px' }}
            onClick={() => updateStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-[375px] h-[812px] shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden"
           style={{ background: 'rgba(255, 255, 255, 0.45)', backdropFilter: 'blur(40px) saturate(250%)', border: '1px solid rgba(255, 255, 255, 0.5)', borderRadius: '24px', boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.07)' }}>
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
            <SmartBlock key={i} {...b} />
          ))}
          <SmartBlock type="PoweredBy" props={{ tenantId }} />
        </div>

        <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50" style={{ borderRadius: '0 0 16px 16px' }}>
          <Tooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
            <button
              id="launch-btn"
              className="w-full bg-blue-600 text-white p-4 font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
              style={{ borderRadius: '12px' }}
              onClick={handleLaunch}
            >
              <span>1-Tap Launch</span>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </button>
          </Tooltip>
        </div>
      </div>
    </div>
  );
}
