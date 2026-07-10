import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useOnboardingStore } from './store';
import { IconLabel } from './components/IconLabel';
import { SetupIcon } from './components/SetupIcon';

export function ZeroClickOnboarding({ onSwitchMode }: { onSwitchMode: () => void }) {
  const [prompt, setPrompt] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");
  const [progress, setProgress] = useState(0);
  const { updateState } = useOnboardingStore();
  const router = useRouter();

  const handleStartZeroClick = async () => {
    if (!prompt.trim()) {
      setError("Please describe your business");
      return;
    }

    setIsLoading(true);
    setError("");
    setProgress(10);

    // Simulate initial loading
    const interval = setInterval(() => {
        setProgress(prev => Math.min(prev + 10, 85));
    }, 500);

    try {
      const tenantId =
        typeof localStorage !== "undefined"
          ? localStorage.getItem("tenant_id") ||
            localStorage.getItem("tenant") ||
            "storefront"
          : "storefront";
      let userId = "guest";
      if (typeof localStorage !== "undefined") {
        userId = localStorage.getItem("user_id") || "";
        if (!userId) {
          userId = crypto.randomUUID();
          localStorage.setItem("user_id", userId);
        }
      }

      const res = await fetch("/api/onboarding/start_zero_click", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({ prompt }),
      });

      if (!res.ok) {
        throw new Error("Failed to generate storefront");
      }
      clearInterval(interval);
      setProgress(100);

      const result = await res.json();
      updateState({ startResult: result, step: 5, businessName: prompt.split(" ")[0] || "My Business" });

      localStorage.setItem("has_onboarded", "true");
      if (result.organization_id) {
        localStorage.setItem("tenant_id", result.organization_id);
        localStorage.setItem("tenant", result.organization_id);
      }
    } catch (err: any) {
      clearInterval(interval);
      setError(err.message || "An error occurred");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col flex-1 h-full w-full justify-center p-6 animate-fade-in relative z-10 glass-panel">
      {isLoading ? (
        <div className="flex flex-col items-center justify-center space-y-6 flex-1 text-center animate-fade-in">
           <div className="w-24 h-24 relative mb-4">
                <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                Generating Your Storefront...
            </h2>
            <div className="w-full max-w-xs h-2 bg-[rgba(255,255,255,0.2)] dark:bg-[rgba(255,255,255,0.1)] rounded-full overflow-hidden mb-6">
                <div
                  className="h-full bg-[#0066FF] transition-all duration-300"
                  style={{ width: `${progress}%` }}
                ></div>
            </div>
            <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">
                Our AI agents are building your business from scratch. This usually takes about 5-10 seconds.
            </p>
        </div>
      ) : (
        <>
            <div className="mb-8">
                <div className="w-16 h-16 bg-[#0066FF]/10 rounded-2xl flex items-center justify-center mb-6">
                    <SetupIcon name="sparkles" className="w-8 h-8 text-[#0066FF]" />
                </div>
                <h1 className="text-3xl font-bold font-outfit tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Zero-Click Launch
                </h1>
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm">
                Describe your business in one sentence, and our AI agents will instantly generate your complete storefront, products, and CRM.
                </p>
            </div>

            <div className="space-y-4 flex-1">
                <div className="space-y-2">
                    <label className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] ml-1">
                        What are you building?
                    </label>
                    <textarea
                        value={prompt}
                        onChange={(e) => setPrompt(e.target.value)}
                        placeholder="e.g. I sell custom vegan cupcakes in San Francisco"
                        className="w-full bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.4)] border border-[rgba(0,0,0,0.1)] dark:border-[rgba(255,255,255,0.1)] rounded-[12px] p-4 text-[#1D1D1F] dark:text-[#F5F5F7] placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 transition-all shadow-inner h-32 resize-none"
                    />
                </div>
                {error && <p className="text-red-500 text-sm">{error}</p>}
            </div>

            <div className="mt-auto space-y-4">
                <button
                onClick={handleStartZeroClick}
                className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px]"
                >
                <IconLabel icon="launch">Generate Storefront</IconLabel>
                </button>
                <button
                onClick={onSwitchMode}
                className="w-full bg-transparent text-[#0066FF] p-4 font-bold active:scale-[0.98] transition-all duration-[250ms]"
                >
                Use step-by-step wizard instead
                </button>
            </div>
        </>
      )}
    </div>
  );
}
