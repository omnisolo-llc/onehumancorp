import React, { useState } from "react";
import { useRouter } from "next/navigation";

interface AgentUpsellPaywallProps {
  onClose: () => void;
  onSuccess: () => void;
}

export function AgentUpsellPaywall({ onClose, onSuccess }: AgentUpsellPaywallProps) {
  const router = useRouter();
  const [referralLink, setReferralLink] = useState("");
  const [copied, setCopied] = useState(false);
  const [isGenerating, setIsGenerating] = useState(false);

  const fallbackReferralLink = () => {
    let tenant = "default";
    if (typeof localStorage !== "undefined") {
      tenant = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
    }
    return `https://ohc.app/join?ref=${encodeURIComponent(tenant)}&source=agent_paywall`;
  };

  const handleGenerateLink = async () => {
    setIsGenerating(true);
    try {
      const response = await fetch("/api/v1/growth/referrals/generate", {
        method: "POST",
      });
      const data = await response.json();
      if (data && data.referral_link) {
        setReferralLink(data.referral_link);
      } else {
        setReferralLink(fallbackReferralLink());
      }
    } catch (e) {
      console.error("Failed to generate referral link", e);
      setReferralLink(fallbackReferralLink());
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 font-inter">
      <div className="w-full max-w-md overflow-hidden relative shadow-2xl p-6 rounded-[20px]"
           style={{
             background: "rgba(255, 255, 255, 0.95)",
             backdropFilter: "blur(30px) saturate(210%)",
             border: "1px solid rgba(255, 255, 255, 0.6)",
           }}>

        {/* Background embellishment */}
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

        <div className="flex justify-between items-start mb-4">
          <div className="w-12 h-12 bg-indigo-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-indigo-600">
            🤖
          </div>
        </div>

        <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
          Unlock Pro Mode
        </h2>
        <p className="text-gray-600 mb-6 text-sm leading-relaxed">
          Advanced model routing, connector automation, and custom agent skills are available in <strong>Pro Mode</strong>.
        </p>

        <div className="bg-indigo-50 border border-indigo-100 rounded-xl p-4 mb-6">
          <div className="flex items-center gap-3 mb-2">
            <span className="text-xl">🎁</span>
            <h3 className="font-bold text-indigo-900 font-outfit text-sm">
              Viral Growth Offer
            </h3>
          </div>
          <p className="text-indigo-800 text-xs font-medium">
            Invite a friend and get <strong>14 Days of Pro for Free!</strong> They get 10% off their first year. ⚡ Powered by OHC
          </p>
        </div>

        {!referralLink ? (
          <div className="flex flex-col gap-3">
             <button
              onClick={handleGenerateLink}
              disabled={isGenerating}
              className="w-full px-4 py-3 bg-gray-900 text-white rounded-xl font-bold shadow-md hover:bg-black transition-colors flex items-center justify-center gap-2"
            >
              {isGenerating ? "Generating link..." : "Get 14 Days Free via Invite"}
            </button>
            <button
              onClick={() => router.push("/pricing")}
              className="w-full px-4 py-3 bg-indigo-600 text-white rounded-xl font-bold shadow-md hover:bg-indigo-700 transition-colors"
            >
              Upgrade to Pro Now
            </button>
            <button
              type="button"
              onClick={onClose}
              className="mt-2 w-full text-sm font-semibold text-gray-500 hover:text-gray-700"
            >
              Maybe Later
            </button>
          </div>
        ) : (
          <div className="space-y-4 animate-fade-in-up">
            <div>
              <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">
                Your Unique Link
              </label>
              <div className="flex gap-2">
                <input
                  type="text"
                  readOnly
                  value={referralLink}
                  className="flex-1 bg-white border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none"
                />
                <button
                  onClick={() => {
                    navigator.clipboard.writeText(referralLink);
                    setCopied(true);
                    onSuccess(); // Grant them access optimistically
                    setTimeout(() => setCopied(false), 2000);
                  }}
                  className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${
                    copied ? "bg-green-100 text-green-700" : "bg-indigo-600 text-white hover:bg-indigo-700"
                  }`}
                >
                  {copied ? "Copied!" : "Copy & Claim"}
                </button>
              </div>
            </div>

             <div className="flex flex-col gap-3">
                <a
                  href={`https://wa.me/?text=${encodeURIComponent(`I'm building an AI workforce on OHC! Use my link to get 10% off your Pro plan: ${referralLink} ⚡ Powered by OHC`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                  onClick={() => onSuccess()}
                >
                  Share on WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`I'm building an AI workforce on OHC! Use my link to get 10% off your Pro plan: ${referralLink} ⚡ Powered by OHC`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-black text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                  onClick={() => onSuccess()}
                >
                  Share on X (Twitter)
                </a>
              </div>
          </div>
        )}
      </div>
    </div>
  );
}
