"use client";

import React, { useState, useEffect, useRef } from "react";
import Head from "next/head";
import { useProPlan } from '../components/useProPlan';

function safeJavaScriptString(value: string): string {
  return JSON.stringify(value)
    .replace(/</g, "\\u003c")
    .replace(/>/g, "\\u003e")
    .replace(/\u2028/g, "\\u2028")
    .replace(/\u2029/g, "\\u2029");
}

export default function ExitIntentBuilder() {
  const [headline, setHeadline] = useState("Wait! Before you go...");
  const [description, setDescription] = useState(
    "Get 10% off your first order when you sign up for our newsletter."
  );
  const [buttonText, setButtonText] = useState("Claim My 10% Off");
  const [themeColor, setThemeColor] = useState("#2563eb");
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isCopied, setIsCopied] = useState(false);
  const { hasPro } = useProPlan();

  const previewRef = useRef<HTMLDivElement>(null);

  const handleBrandingToggle = () => {
    if (!removeBranding) {
      if (hasPro) setRemoveBranding(true);
      else setShowPaywall(true);
    } else {
      setRemoveBranding(false);
    }
  };

  const handleUpgrade = () => {
    setShowPaywall(false);
    window.location.href = '/pricing';
  };

  const generatedCode = `
<!-- OHC Exit Intent Pop-up -->
<script>
  (function() {
    const headline = ${safeJavaScriptString(headline)};
    const description = ${safeJavaScriptString(description)};
    const buttonText = ${safeJavaScriptString(buttonText)};
    const themeColor = ${safeJavaScriptString(themeColor)};
    let triggered = false;
    document.addEventListener("mouseleave", function(e) {
      if (e.clientY < 0 && !triggered) {
        triggered = true;
        const overlay = document.createElement("div");
        Object.assign(overlay.style, { position: "fixed", inset: "0", width: "100vw", height: "100vh", background: "rgba(0,0,0,0.5)", display: "flex", alignItems: "center", justifyContent: "center", zIndex: "999999" });
        const popup = document.createElement("div");
        Object.assign(popup.style, { background: "white", padding: "2rem", borderRadius: "8px", textAlign: "center", maxWidth: "400px", width: "100%", position: "relative" });
        const close = document.createElement("button");
        close.type = "button";
        close.textContent = "×";
        close.setAttribute("aria-label", "Close offer");
        Object.assign(close.style, { position: "absolute", top: "10px", right: "10px", background: "none", border: "none", fontSize: "1.5rem", cursor: "pointer" });
        close.addEventListener("click", function() { overlay.remove(); });
        const heading = document.createElement("h2");
        heading.textContent = headline;
        Object.assign(heading.style, { marginTop: "0", color: "#1f2937" });
        const copy = document.createElement("p");
        copy.textContent = description;
        Object.assign(copy.style, { color: "#4b5563", marginBottom: "1.5rem" });
        const action = document.createElement("button");
        action.type = "button";
        action.textContent = buttonText;
        Object.assign(action.style, { background: themeColor, color: "white", border: "none", padding: "0.75rem 1.5rem", borderRadius: "4px", fontWeight: "bold", cursor: "pointer", width: "100%" });
        popup.append(close, heading, copy, action);
        ${!removeBranding ? `const branding = document.createElement("a");
        branding.href = "https://onehumancorp.com";
        branding.textContent = "⚡ Powered by OHC";
        Object.assign(branding.style, { display: "block", marginTop: "1rem", fontSize: "0.75rem", color: "#9ca3af", textDecoration: "none" });
        popup.appendChild(branding);` : ""}
        overlay.appendChild(popup);
        document.body.appendChild(overlay);
      }
    });
  })();
</script>
`.trim();

  const handleCopyCode = async () => {
    try {
      await navigator.clipboard.writeText(generatedCode);
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), 2000);
    } catch (err) {
      console.error("Failed to copy!", err);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8 flex flex-col items-center justify-center font-sans">
      <Head>
        <title>Exit-Intent Pop-up Builder | OHC</title>
      </Head>

      <div className="max-w-4xl w-full bg-white rounded-2xl shadow-xl overflow-hidden flex flex-col md:flex-row">
        {/* Left Column: Editor */}
        <div className="w-full md:w-1/2 p-8 border-r border-gray-100 flex flex-col space-y-6">
          <div>
            <h1 className="text-2xl font-bold text-gray-900 mb-2">
              Exit-Intent Pop-up Builder
            </h1>
            <p className="text-gray-500 text-sm">
              Recover abandoning visitors by showing them a special offer right before they leave.
            </p>
          </div>

          <div className="space-y-4">
            <div>
              <label htmlFor="exit-headline" className="block text-sm font-medium text-gray-700 mb-1">
                Headline
              </label>
              <input
                type="text"
                id="exit-headline"
                value={headline}
                onChange={(e) => setHeadline(e.target.value)}
                className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none"
                placeholder="Wait! Before you go..."
              />
            </div>

            <div>
              <label htmlFor="exit-description" className="block text-sm font-medium text-gray-700 mb-1">
                Offer Description
              </label>
              <textarea
                id="exit-description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                rows={3}
                className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none resize-none"
                placeholder="Get 10% off your first order..."
              />
            </div>

            <div>
              <label htmlFor="exit-button-text" className="block text-sm font-medium text-gray-700 mb-1">
                Button Text
              </label>
              <input
                type="text"
                id="exit-button-text"
                value={buttonText}
                onChange={(e) => setButtonText(e.target.value)}
                className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none"
                placeholder="Claim My 10% Off"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Theme Color
              </label>
              <div className="flex items-center space-x-3">
                <input
                  type="color"
                  value={themeColor}
                  onChange={(e) => setThemeColor(e.target.value)}
                  className="w-10 h-10 border-0 rounded cursor-pointer p-0"
                />
                <span className="text-gray-600 text-sm uppercase">{themeColor}</span>
              </div>
            </div>

            <div className="pt-4 border-t border-gray-100 flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-900">Remove OHC Branding</p>
                <p className="text-xs text-gray-500">Upgrade to Pro to remove the watermark.</p>
              </div>
              <button
                onClick={handleBrandingToggle}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  removeBranding ? "bg-[#0071E3]" : "bg-gray-200"
                }`}
                role="switch"
                aria-checked={removeBranding}
              >
                <span
                  className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    removeBranding ? "translate-x-6" : "translate-x-1"
                  }`}
                />
              </button>
            </div>
          </div>
        </div>

        {/* Right Column: Preview & Output */}
        <div className="w-full md:w-1/2 bg-gray-50 p-8 flex flex-col">
          <div className="mb-6 flex-grow">
            <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4">
              Live Preview
            </h2>
            <div
              className="relative w-full h-64 bg-white border border-gray-200 rounded-xl overflow-hidden flex items-center justify-center cursor-crosshair group shadow-inner"
              title="Move cursor out of window to trigger"
              ref={previewRef}
            >
              {/* Fake UI background */}
              <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'radial-gradient(#cbd5e1 1px, transparent 1px)', backgroundSize: '20px 20px' }}></div>
              <p className="text-gray-400 text-sm">Move cursor up to trigger &uarr;</p>

              {/* Live Preview Pop-up */}
              <div className="absolute z-10 w-64 bg-white rounded-lg shadow-2xl p-4 text-center transform transition-transform scale-95 group-hover:scale-100">
                <button className="absolute top-2 right-2 text-gray-400 hover:text-gray-600" disabled>&times;</button>
                <h3 className="text-lg font-bold text-gray-900 mb-1">{headline || "Your Headline"}</h3>
                <p className="text-xs text-gray-600 mb-4">{description || "Your offer description goes here."}</p>
                <button
                  style={{ backgroundColor: themeColor }}
                  className="w-full text-white text-sm font-bold py-2 rounded"
                  disabled
                >
                  {buttonText || "Button Text"}
                </button>
                {!removeBranding && (
                  <p className="mt-2 text-[10px] text-gray-400">⚡ Powered by OHC</p>
                )}
              </div>
            </div>
          </div>

          <div>
            <div className="flex items-center justify-between mb-2">
              <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider">
                Your Embed Code
              </h2>
              <button
                onClick={handleCopyCode}
                className="text-[#0071E3] text-sm font-medium hover:text-blue-800 transition-colors"
              >
                {isCopied ? "Copied!" : "Copy to Clipboard"}
              </button>
            </div>
            <pre className="bg-gray-900 text-gray-100 p-4 rounded-xl text-xs overflow-x-auto overflow-y-auto max-h-48 custom-scrollbar">
              <code>{generatedCode}</code>
            </pre>
          </div>
        </div>
      </div>

      {/* Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-[30px] saturate-[210%] p-4">
          <div className="bg-white rounded-2xl shadow-2xl max-w-sm w-full p-6 text-center animate-in fade-in zoom-in duration-200">
            <div className="w-16 h-16 bg-blue-100 text-[#0071E3] rounded-full flex items-center justify-center mx-auto mb-4 text-2xl">
              ✨
            </div>
            <h3 className="text-xl font-bold text-gray-900 mb-2">Remove OHC Branding</h3>
            <p className="text-gray-600 text-sm mb-6">
              Upgrade to the Pro tier to remove the "Powered by OHC" watermark and unlock advanced pop-up triggers.
            </p>
            <div className="flex flex-col space-y-3">
              <button
                onClick={handleUpgrade}
                className="w-full bg-[#0071E3] text-white font-bold py-3 rounded-xl hover:bg-blue-700 transition-colors shadow-lg shadow-blue-200"
              >
                Upgrade to Pro
              </button>
              <button
                onClick={() => setShowPaywall(false)}
                className="w-full text-gray-500 font-medium py-2 hover:text-gray-700 transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
