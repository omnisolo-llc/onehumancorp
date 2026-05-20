"use client";

import { useState } from "react";
import { SmartBlock } from "../builder/components";

export default function WebsiteBuilderPage() {
  const [bio, setBio] = useState("");
  const [step, setStep] = useState(0);
  const [businessName, setBusinessName] = useState("");
  const [businessType, setBusinessType] = useState("");
  const [businessDetail, setBusinessDetail] = useState("");
  const [blocks, setBlocks] = useState<any[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live">("idle");
  const [liveUrl, setLiveUrl] = useState("");

  const handleGenerate = async (instantBio?: string) => {
    setStatus("generating");

    let payloadBio = bio;
    if (instantBio) {
      payloadBio = instantBio;
    } else if (businessType && businessName && businessDetail) {
      payloadBio = `I run a ${businessType} called ${businessName}. ${businessDetail}`;
      setBio(payloadBio);
    }

    try {
      const response = await fetch('/api/v1/builder/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: payloadBio })
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
        setLiveUrl(`https://${data.domain || 'myshop'}.ohc.store`);
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
             style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>

          <div className="px-8 pb-8 pt-12 flex flex-col flex-1 justify-start overflow-y-auto">
            {step === 0 && (
              <div className="animate-fade-in flex flex-col h-full justify-center text-center" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-4">Welcome to OHC Smart Builder</h1>
                <p className="text-gray-500 text-base mb-12 leading-relaxed">
                  Your business, live in minutes.
                </p>
                <div className="mt-auto flex flex-col gap-4">
                    <button
                        className="w-full p-4 font-bold font-outfit text-lg transition-all text-white shadow-md active:scale-[0.98]"
                        style={{ borderRadius: '8px', background: '#0071E3' }}
                        onClick={() => setStep(1)}
                    >
                        Start My Business Next
                    </button>
                    <button
                        className="w-full p-4 font-bold font-outfit text-lg transition-all text-gray-700 bg-gray-100 shadow-sm active:scale-[0.98]"
                        style={{ borderRadius: '8px' }}
                        onClick={() => setStep(4)}
                    >
                        Instant Build
                    </button>
                </div>
              </div>
            )}

            {step === 1 && (
              <div className="animate-fade-in flex flex-col h-full" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <button onClick={() => setStep(0)} className="text-sm text-gray-500 hover:text-gray-800 mb-6 flex items-center gap-1 self-start">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                    Back
                </button>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">What kind of business are you building?</h2>
                <p className="text-gray-500 text-sm mb-8 leading-relaxed">Select the category that best describes your business.</p>

                <div className="flex flex-col gap-3">
                  {['Online Store', 'Services & Bookings', 'Food & Beverage', 'Tutoring & Lessons'].map(type => (
                    <button
                      key={type}
                      className={`w-full p-4 text-left font-medium text-lg transition-all border ${businessType === type ? 'border-[#0071E3] bg-blue-50' : 'border-gray-200 bg-white hover:bg-gray-50'}`}
                      style={{ borderRadius: '8px' }}
                      onClick={() => setBusinessType(type)}
                    >
                      {type}
                    </button>
                  ))}
                </div>

                <div className="mt-auto pt-8">
                  <button
                    className={`w-full p-4 font-bold font-outfit text-lg transition-all ${
                      businessType !== ""
                        ? "text-white shadow-md active:scale-[0.98]"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                    }`}
                    style={{ borderRadius: '8px', background: (businessType !== "") ? '#0071E3' : '' }}
                    onClick={() => setStep(2)}
                    disabled={businessType === ""}
                  >
                    Next
                  </button>
                </div>
              </div>
            )}

            {step === 2 && (
              <div className="animate-fade-in flex flex-col h-full" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <button onClick={() => setStep(1)} className="text-sm text-gray-500 hover:text-gray-800 mb-6 flex items-center gap-1 self-start">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                    Back
                </button>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Give your business a name</h2>
                <p className="text-gray-500 text-sm mb-8 leading-relaxed">You can always change this later.</p>

                <input
                  type="text"
                  className="w-full border border-gray-300 p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800"
                  style={{ borderRadius: '8px' }}
                  value={businessName}
                  onChange={(e) => setBusinessName(e.target.value)}
                  placeholder="What is your business called?"
                />

                <div className="mt-auto pt-8">
                  <button
                    className={`w-full p-4 font-bold font-outfit text-lg transition-all ${
                      businessName.trim().length > 1
                        ? "text-white shadow-md active:scale-[0.98]"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                    }`}
                    style={{ borderRadius: '8px', background: (businessName.trim().length > 1) ? '#0071E3' : '' }}
                    onClick={() => setStep(3)}
                    disabled={businessName.trim().length <= 1}
                  >
                    Next
                  </button>
                </div>
              </div>
            )}

            {step === 3 && (
              <div className="animate-fade-in flex flex-col h-full" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <button onClick={() => setStep(2)} className="text-sm text-gray-500 hover:text-gray-800 mb-6 flex items-center gap-1 self-start">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                    Back
                </button>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
                    {businessType === "Online Store" || businessType === "Food & Beverage" ? "What do you sell?" : "What services do you offer?"}
                </h2>
                <p className="text-gray-500 text-sm mb-8 leading-relaxed">Tell us a bit about your products or services.</p>

                <textarea
                  className="w-full border border-gray-300 p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800"
                  style={{ borderRadius: '8px' }}
                  value={businessDetail}
                  onChange={(e) => setBusinessDetail(e.target.value)}
                  placeholder={businessType === "Online Store" ? "e.g. I sell custom vegan cakes" : "e.g. I fix leaky pipes"}
                  rows={4}
                />

                <div className="mt-auto pt-8">
                  <button
                    className={`w-full p-4 font-bold font-outfit text-lg transition-all ${
                      businessDetail.trim().length > 5
                        ? "text-white shadow-md active:scale-[0.98]"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                    }`}
                    style={{ borderRadius: '8px', background: (businessDetail.trim().length > 5) ? '#0071E3' : '' }}
                    onClick={() => handleGenerate()}
                    disabled={businessDetail.trim().length <= 5}
                  >
                    Build My Storefront
                  </button>
                </div>
              </div>
            )}

            {step === 4 && (
              <div className="animate-fade-in flex flex-col h-full" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <button onClick={() => setStep(0)} className="text-sm text-gray-500 hover:text-gray-800 mb-6 flex items-center gap-1 self-start">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                    Back
                </button>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Describe your business in a sentence</h2>
                <p className="text-gray-500 text-sm mb-8 leading-relaxed">Our AI will do the rest.</p>

                <textarea
                  className="w-full border border-gray-300 p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800"
                  style={{ borderRadius: '8px' }}
                  value={bio}
                  onChange={(e) => {
                      setBio(e.target.value);
                      setBusinessDetail(e.target.value);
                  }}
                  placeholder="e.g. I run a local bakery called Maya Cakes."
                  rows={4}
                />

                <div className="mt-auto pt-8">
                  <button
                    className={`w-full p-4 font-bold font-outfit text-lg transition-all ${
                      bio.trim().length > 5
                        ? "text-white shadow-md active:scale-[0.98]"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                    }`}
                    style={{ borderRadius: '8px', background: (bio.trim().length > 5) ? '#0071E3' : '' }}
                    onClick={() => handleGenerate(bio)}
                    disabled={bio.trim().length <= 5}
                  >
                    Generate Storefront
                  </button>
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
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 justify-center items-center">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
            <p className="text-gray-500 font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden text-center p-8 justify-center">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h1>
          <p className="text-gray-500 mb-6 text-sm">Your automated storefront is successfully published.</p>

          <div className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          <button
            className="w-full bg-gray-100 text-gray-800 font-bold p-4 rounded-xl active:scale-[0.98] transition-all hover:bg-gray-200"
            onClick={() => setStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden">
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
            <SmartBlock key={i} {...b} />
          ))}
          <SmartBlock type="PoweredBy" props={{}} />
        </div>

        <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50">
          <Tooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
            <button
              id="launch-btn"
              className="w-full bg-blue-600 text-white p-4 rounded-xl font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
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
