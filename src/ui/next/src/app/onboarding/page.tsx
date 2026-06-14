"use client";

import React, { useEffect, useState, useRef } from "react";
import { useOnboardingStore } from "./store";
type SetupIconName =
  | "dashboard"
  | "eye"
  | "launch"
  | "next"
  | "save"
  | "sparkles";

function SetupIcon({ name }: { name: SetupIconName }) {
  const paths: Record<SetupIconName, string[]> = {
    dashboard: [
      "M4 5h7v7H4z",
      "M13 5h7v4h-7z",
      "M13 11h7v8h-7z",
      "M4 14h7v5H4z",
    ],
    eye: [
      "M2.5 12s3.5-6 9.5-6 9.5 6 9.5 6-3.5 6-9.5 6-9.5-6-9.5-6z",
      "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z",
    ],
    launch: ["M13 10V3L4 14h7v7l9-11h-7z"],
    next: ["M5 12h14", "M13 6l6 6-6 6"],
    save: ["M5 4h12l2 2v16H5z", "M8 4v7h8V4", "M8 18h8"],
    sparkles: [
      "M21 12l-3-1 1-3 1 3 3 1-3 1-1 3-1-3zM8 21l-3-4-4-3 4-3 3-4 3 4 4 3-4 3z",
    ],
  };

  return (
    <svg
      className="h-4 w-4 flex-none"
      aria-hidden="true"
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={2}
      viewBox="0 0 24 24"
    >
      {paths[name].map((d) => (
        <path key={d} d={d} />
      ))}
    </svg>
  );
}

function IconLabel({
  icon,
  children,
}: {
  icon: SetupIconName;
  children: React.ReactNode;
}) {
  return (
    <span className="inline-flex items-center justify-center gap-2 flex-none">
      <span className="flex-none inline-flex items-center justify-center w-4 h-4">
        <SetupIcon name={icon} />
      </span>
      <span className="whitespace-nowrap">{children}</span>
    </span>
  );
}

function generateSubdomain(name: string): string {
  if (!name || name.trim() === "") return "my-business.ohc.app";
  const cleanName = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return cleanName ? `${cleanName}.ohc.app` : "my-business.ohc.app";
}

export default function OnboardingWizard() {
  const {
    step,
    setStep,
    businessDescription,
    setBusinessDescription,
    businessGoal,
    setBusinessGoal,
    businessName,
    setBusinessName,
    whatYouSell,
    setWhatYouSell,
    location,
    setLocation,
    targetAudience,
    setTargetAudience,
    bio,
    setBio,
    businessType,
    setBusinessType,
    categories,
    setCategories,
    websiteTemplate,
    setWebsiteTemplate,
    domainChoice,
    setDomainChoice,
    firstProductName,
    setFirstProductName,
    firstProductPrice,
    setFirstProductPrice,
    adminName,
    setAdminName,
    adminEmail,
    setAdminEmail,
    adminPassword,
    setAdminPassword,
    aiAgents,
    setAiAgents,
    aiAutoRespond,
    setAiAutoRespond,
    isLoading,
    setIsLoading,
    error,
    setError,
    startResult,
    setStartResult,
    instantImageUrl,
    setInstantImageUrl,
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const initialStateLoaded = useRef(false);
  const [chatMessages, setChatMessages] = useState<
    { role: string; content: string; image_url?: string }[]
  >([]);
  const [chatInput, setChatInput] = useState("");
  const [chatImageUrl, setChatImageUrl] = useState("");
  const chatMessagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (chatMessagesEndRef.current && typeof chatMessagesEndRef.current.scrollIntoView === 'function') {
      chatMessagesEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [chatMessages]);

  const fetchWithRetry = async (
    url: string,
    options: RequestInit,
    retries = 3,
    backoff = process.env.NODE_ENV === "test" ? 10 : 500,
  ) => {
    for (let i = 0; i < retries; i++) {
      try {
        const response = await fetch(url, options);
        if (!response.ok) {
          let errMsg = `HTTP error! status: ${response.status}`;
          try {
            const result = await response.clone().json();
            errMsg = result.error || result.message || errMsg;
          } catch (e) {}
          throw new Error(errMsg);
        }
        return response;
      } catch (err: any) {
        if (i === retries - 1) throw err;
        await new Promise((res) => setTimeout(res, backoff * Math.pow(2, i)));
      }
    }
    throw new Error("Max retries reached");
  };

  const syncStateToBackend = async (overrideState: Partial<any> = {}) => {
    const tenantId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("tenant_id") ||
          localStorage.getItem("tenant") ||
          "storefront"
        : "storefront";
    const userId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("user_id") || "test-user"
        : "test-user";

    const wizardState = {
      step,
      businessDescription,
      businessName,
      whatYouSell,
      location,
      targetAudience,
      businessType,
      categories,
      websiteTemplate,
      domainChoice,
      firstProductName,
      firstProductPrice,
      adminName,
      adminEmail,
      adminPassword,
      aiAgents,
      aiAutoRespond,
      ...overrideState,
    };

    try {
      await fetch("/api/onboarding/state", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({ wizardState }),
      });
    } catch (err) {
      console.error("Failed to sync onboarding state", err);
    }
  };
  const [validationError, setValidationError] = useState("");
  const [validationErrors, setValidationErrors] = useState<
    Record<string, string>
  >({});
  const [saveMessage, setSaveMessage] = useState("");

  const handleSaveDraft = async () => {
    setIsLoading(true);
    setError("");

    try {
      const tenantId =
        typeof localStorage !== "undefined"
          ? localStorage.getItem("tenant_id") ||
            localStorage.getItem("tenant") ||
            "storefront"
          : "storefront";
      const userId =
        typeof localStorage !== "undefined"
          ? localStorage.getItem("user_id") || "test-user"
          : "test-user";

      const wizardState = {
        step,
        businessDescription,
        businessName,
        whatYouSell,
        location,
        targetAudience,
        businessType,
        categories,
        websiteTemplate,
        domainChoice,
        firstProductName,
        firstProductPrice,
        adminName,
        adminEmail,
        adminPassword,
        aiAgents,
        aiAutoRespond,
      };

      const res = await fetchWithRetry("/api/onboarding/draft", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({ wizardState }),
      });

      setSaveMessage("Draft Saved!");
      setTimeout(() => setSaveMessage(""), 3000);
    } catch (err: any) {
      console.error(err);
      setError(err.message || "An error occurred saving draft");
    } finally {
      setIsLoading(false);
    }
  };

  // Read state from server on mount
  useEffect(() => {
    const tenantId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("tenant_id") ||
          localStorage.getItem("tenant") ||
          "storefront"
        : "storefront";
    const userId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("user_id") || "test-user"
        : "test-user";

    Promise.all([
      fetch("/api/onboarding/draft", {
        headers: { "X-Tenant-ID": tenantId, "X-User-ID": userId },
      })
        .then((res) => (res.ok ? res.json() : null))
        .catch(() => null),
      fetch("/api/onboarding/state", {
        headers: { "X-Tenant-ID": tenantId, "X-User-ID": userId },
      })
        .then((res) => (res.ok ? res.json() : null))
        .catch(() => null),
    ])
      .then(([draftData, stateData]) => {
        const data = draftData && draftData.wizardState ? draftData : stateData;
        if (data && data.wizardState) {
          if (data.wizardState.step !== undefined)
            setStep(data.wizardState.step);

          if (data.wizardState.businessDescription !== undefined)
            setBusinessDescription(data.wizardState.businessDescription);
          if (data.wizardState.businessName !== undefined)
            setBusinessName(data.wizardState.businessName);
          if (data.wizardState.whatYouSell !== undefined)
            setWhatYouSell(data.wizardState.whatYouSell);
          if (data.wizardState.location !== undefined)
            setLocation(data.wizardState.location);
          if (data.wizardState.targetAudience !== undefined)
            setTargetAudience(data.wizardState.targetAudience);
          if (data.wizardState.businessType !== undefined)
            setBusinessType(data.wizardState.businessType);
          if (data.wizardState.categories !== undefined)
            setCategories(data.wizardState.categories);
          if (data.wizardState.websiteTemplate !== undefined)
            setWebsiteTemplate(data.wizardState.websiteTemplate);
          if (data.wizardState.firstProductName !== undefined)
            setFirstProductName(data.wizardState.firstProductName);
          if (data.wizardState.firstProductPrice !== undefined)
            setFirstProductPrice(data.wizardState.firstProductPrice);
          if (data.wizardState.adminName !== undefined)
            setAdminName(data.wizardState.adminName);
          if (data.wizardState.adminEmail !== undefined)
            setAdminEmail(data.wizardState.adminEmail);
          if (data.wizardState.adminPassword !== undefined)
            setAdminPassword(data.wizardState.adminPassword);
          if (data.wizardState.domainChoice !== undefined)
            setDomainChoice(data.wizardState.domainChoice);
          if (data.wizardState.aiAgents !== undefined)
            setAiAgents(data.wizardState.aiAgents);
          if (data.wizardState.aiAutoRespond !== undefined)
            setAiAutoRespond(data.wizardState.aiAutoRespond);
          initialStateLoaded.current = true;
        }
      })
      .catch((err) => console.error("Failed to load onboarding state", err))
      .finally(() => {
        initialStateLoaded.current = true;
        setIsLoaded(true);
      });
  }, []);

  // Sync state to backend
  useEffect(() => {
    if (!isLoaded || !initialStateLoaded.current) return;

    // Only save if we are past the initial state
    if (
      step === 0 &&
      !businessName &&
      !whatYouSell &&
      !location &&
      !targetAudience
    )
      return;

    const tenantId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("tenant_id") ||
          localStorage.getItem("tenant") ||
          "storefront"
        : "storefront";
    const userId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("user_id") || "test-user"
        : "test-user";

    const wizardState = {
      step,
      businessDescription,
      businessName,
      whatYouSell,
      location,
      targetAudience,
      businessType,
      categories,
      websiteTemplate,
      domainChoice,
      firstProductName,
      firstProductPrice,
      adminName,
      adminEmail,
      adminPassword,
      aiAgents,
      aiAutoRespond,
    };

    const timer = setTimeout(() => {
      fetch("/api/onboarding/state", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({ wizardState }),
      }).catch((err) => console.error("Failed to sync onboarding state", err));
    }, 1000); // debounce 1s

    return () => clearTimeout(timer);
  }, [
    step,
    businessDescription,
    businessName,
    whatYouSell,
    location,
    targetAudience,
    businessType,
    categories,
    websiteTemplate,
    domainChoice,
    firstProductName,
    firstProductPrice,
    adminName,
    adminEmail,
    adminPassword,
    aiAgents,
    aiAutoRespond,
    isLoaded,
  ]);

  const handleIntake = async () => {
    setIsLoading(true);
    setError("");

    try {
      const tenantId =
        typeof localStorage !== "undefined"
          ? localStorage.getItem("tenant_id") ||
            localStorage.getItem("tenant") ||
            "storefront"
          : "storefront";
      const userId =
        typeof localStorage !== "undefined"
          ? localStorage.getItem("user_id") || "test-user"
          : "test-user";

      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}\nTarget Audience: ${targetAudience}`;
      setBio(combinedDescription);

      const intakeRes = await fetch("/api/onboarding/intake", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({ description: combinedDescription }),
      });

      const intakeData = await intakeRes.json();
      if (!intakeRes.ok) {
        throw new Error(
          intakeData.error ||
            intakeData.message ||
            "Failed to process business details",
        );
      }

      setBusinessType(intakeData.business_type || "Online Store");
      setBusinessName(intakeData.business_name || "My Business");
      setFirstProductName(
        intakeData.initial_products?.[0]?.name || "First Product",
      );
      setFirstProductPrice(intakeData.initial_products?.[0]?.price || "10.00");
      if (intakeData.initial_products) {
        localStorage.setItem(
          "onboarding_initial_products",
          JSON.stringify(intakeData.initial_products),
        );
      }
      const mappedCategories = intakeData.categories || ["physical"];
      setCategories(mappedCategories);

      // Auto-configure AI Departments based on inferred business context
      const newAgents = [
        "Operations",
        "Marketing",
        "Finance",
        "Legal",
        "Advisory",
      ];
      if (
        mappedCategories.includes("physical") ||
        mappedCategories.includes("digital") ||
        mappedCategories.includes("subscriptions")
      ) {
        newAgents.push("Sales");
      }
      if (
        mappedCategories.includes("services") ||
        mappedCategories.includes("food") ||
        mappedCategories.includes("physical")
      ) {
        newAgents.push("Customer Success");
      }
      setAiAgents(newAgents);

      setStep(2);
      await syncStateToBackend({ step: 2, aiAgents: newAgents }); // Go to review step
    } catch (err: any) {
      console.error(err);
      setError(err.message || "An error occurred processing details");
      setStep(1);
      setChatStep(1);
      syncStateToBackend({ step: 1 });
    } finally {
      setIsLoading(false);
    }
  };

  const handleSendChatMessage = async () => {
    if (!chatInput.trim() && !chatImageUrl.trim()) return;

    const newMessage = {
      role: 'user',
      content: chatInput,
      image_url: chatImageUrl || undefined,
    };

    const newHistory = [...chatMessages, newMessage];
    setChatMessages(newHistory);
    setChatInput('');
    setChatImageUrl('');
    setIsLoading(true);

    try {
      const backendUrl = (typeof window !== 'undefined' && (window.location.origin.includes('localhost') || window.location.protocol === 'file:')) ? 'http://127.0.0.1:18789' : '';

      const res = await fetch(`${backendUrl}/api/onboarding/chat`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ messages: newHistory })
      });

      if (!res.ok) throw new Error('Chat request failed');
      const data = await res.json();

      setChatMessages([...newHistory, { role: 'assistant', content: data.reply }]);

      if (data.is_complete && data.intake_data) {
        const intakeData = data.intake_data;

        // Pre-fill state values
        setBusinessName(intakeData.business_name || "My Business");
        setBusinessType(intakeData.business_type || "Online Store");
        setBusinessDescription(newHistory.map(m => m.content).join(" "));
        setCategories(intakeData.categories || ["physical"]);
        setFirstProductName(intakeData.initial_products?.[0]?.name || "First Product");
        setFirstProductPrice(intakeData.initial_products?.[0]?.price || "0.00");
        setLocation(intakeData.location || "");
        setTargetAudience(intakeData.target_audience || "");

        // Transition to secure account step instead of launching instantly
        setStep(4);
        syncStateToBackend({ step: 4 });
      }
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'Failed to send chat message');
    } finally {
      setIsLoading(false);
    }
  };

  const handleStartOnboarding = async () => {
    if (!adminEmail || !adminPassword) return;

    setIsLoading(true);
    setStep(4.5); // loading spinner

    try {
      const backendUrl = (typeof window !== 'undefined' && (window.location.origin.includes('localhost') || window.location.protocol === 'file:')) ? 'http://127.0.0.1:18789' : '';
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const startRes = await fetchWithRetry(`${backendUrl}/api/onboarding/start`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          business_type: businessType || "Online Store",
          company_name: businessName || "My Business",
          company_description: businessDescription || "Generated by OHC",
          selling_categories: categories || ["physical"],
          payment_pref: "online",
          admin_email: adminEmail,
          admin_name: adminName || businessName || "Admin",
          admin_password: adminPassword,
          website_template: "auto",
          first_product_name: firstProductName || "First Product",
          first_product_price: firstProductPrice || "0.00",
          domain_choice: "subdomain",
          price_type: "fixed",
          location: location || "",
          target_audience: targetAudience || "",
          ai_agents: [],
          ai_auto_respond: true,
          initial_products: [{ name: firstProductName || "First Product", price: firstProductPrice || "0.00" }],
        })
      });

      const result = await startRes.json();
      setStartResult(result);
      if (result.organization_id) {
         localStorage.setItem('tenant_id', result.organization_id);
         localStorage.setItem('tenant', result.organization_id);
      }
      setStep(5);
      fetch(`${backendUrl}/api/onboarding/launch`, { method: 'POST', headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId } }).catch(console.error);

      if (typeof window !== 'undefined' && window.location.href.includes('setup.html')) {
         window.location.href = '/success.html';
      }
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'Failed to start onboarding');
      setStep(4);
    } finally {
      setIsLoading(false);
    }
  };



  if (!isLoaded) return null;

  // Progress percentage calculation
  const getProgress = () => {
    // Zero-click flow
    if (step === 0) return 25;
    if (step === 4) return 95;
    if (step === 5) return 100;
    return 0;
  };

  return (
    <div className="min-h-screen w-full bg-[#F5F5F7] dark:bg-[#16161a] flex items-center justify-center p-4">
      <div
        id="setup-screen"
        className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[640px] sm:min-h-[812px] relative rounded-[16px] glassmorphism border border-white/20 shadow-2xl"
      >
        <div className="px-6 pt-5 text-center">
          <h1 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
            Setup
          </h1>
          <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">
            Your business, live in minutes.
          </p>
        </div>
        {/* Progress Bar */}
        <div className="h-1.5 w-full bg-gray-200 overflow-hidden">
          <div
            className="h-full bg-[#0066FF] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-[0_0_10px_rgba(0,102,255,0.5)]"
            style={{ width: `${getProgress()}%` }}
          ></div>
        </div>

        <div className="p-6 flex-1 flex flex-col overflow-y-auto custom-scrollbar">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-4 rounded-[8px] text-sm animate-shake">
              {error}
            </div>
          )}

          {step === 0 && (
            <div className="flex flex-col flex-1 h-full animate-fade-in relative">
              <div className="px-4 py-3 bg-white/50 dark:bg-white/5 border-b border-white/20 flex items-center justify-between shrink-0">
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 bg-[#0066FF]/20 rounded-full flex items-center justify-center shrink-0">
                    <svg
                      className="w-5 h-5 text-[#0066FF]"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M13 10V3L4 14h7v7l9-11h-7z"
                      />
                    </svg>
                  </div>
                  <div>
                    <h2 className="text-sm font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
                      OHC Setup Agent
                    </h2>
                    <p className="text-[11px] text-gray-500 dark:text-[#A1A1A6]">
                      Online • Typically finishes in seconds
                    </p>
                  </div>
                </div>
              </div>

              <div className="flex-1 overflow-y-auto mb-4 p-4 space-y-4 min-h-[300px]">
                {chatMessages.length === 0 && (
                  <div className="flex items-start gap-3">
                    <div className="w-8 h-8 rounded-full bg-[#0066FF] flex items-center justify-center shrink-0 shadow-sm mt-1">
                      <SetupIcon name="sparkles" />
                    </div>
                    <div className="glassmorphism p-4 rounded-2xl rounded-tl-sm text-sm text-[#1D1D1F] dark:text-[#F5F5F7] max-w-[85%] shadow-sm">
                      Hi! I'm your OHC setup agent. What kind of business are
                      you starting, and what will you sell?
                    </div>
                  </div>
                )}
                {chatMessages.map((msg, idx) => (
                  <div
                    key={idx}
                    className={`flex items-start gap-3 ${msg.role === "user" ? "flex-row-reverse" : ""}`}
                  >
                    <div
                      className={`w-8 h-8 rounded-full flex items-center justify-center shrink-0 shadow-sm mt-1 ${msg.role === "user" ? "bg-gray-200 dark:bg-gray-700" : "bg-[#0066FF]"}`}
                    >
                      {msg.role === "user" ? (
                        <svg
                          className="w-4 h-4 text-gray-500 dark:text-gray-300"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                          />
                        </svg>
                      ) : (
                        <SetupIcon name="sparkles" />
                      )}
                    </div>
                    <div
                      className={`glassmorphism p-4 rounded-2xl text-sm max-w-[85%] shadow-sm ${msg.role === "user" ? "rounded-tr-sm bg-[#0066FF]/10 text-[#1D1D1F] dark:text-[#F5F5F7]" : "rounded-tl-sm text-[#1D1D1F] dark:text-[#F5F5F7]"}`}
                    >
                      {msg.content}
                      {msg.image_url && (
                        <img
                          src={msg.image_url}
                          alt="Uploaded preview"
                          className="mt-2 rounded-[8px] max-w-full h-auto max-h-48 object-cover shadow-sm"
                        />
                      )}
                    </div>
                  </div>
                ))}
                {isLoading && (
                  <div className="flex items-start gap-3">
                    <div className="w-8 h-8 rounded-full bg-[#0066FF] flex items-center justify-center shrink-0 shadow-sm mt-1">
                      <SetupIcon name="sparkles" />
                    </div>
                    <div className="glassmorphism p-4 rounded-2xl rounded-tl-sm text-sm text-[#1D1D1F] dark:text-[#F5F5F7] max-w-[85%] shadow-sm flex items-center gap-2">
                      <span
                        className="w-2 h-2 bg-[#0066FF] rounded-full animate-bounce"
                        style={{ animationDelay: "0ms" }}
                      ></span>
                      <span
                        className="w-2 h-2 bg-[#0066FF] rounded-full animate-bounce"
                        style={{ animationDelay: "150ms" }}
                      ></span>
                      <span
                        className="w-2 h-2 bg-[#0066FF] rounded-full animate-bounce"
                        style={{ animationDelay: "300ms" }}
                      ></span>
                    </div>
                  </div>
                )}
                <div ref={chatMessagesEndRef} />
              </div>

              <div className="p-4 bg-white/50 dark:bg-white/5 border-t border-white/20 shrink-0">
                <div className="flex items-end gap-2">
                  <div className="flex-1 glassmorphism rounded-[24px] border border-white/50 dark:border-white/10 overflow-hidden focus-within:border-[#0066FF] transition-colors relative">
                    <textarea
                      value={chatInput}
                      onChange={(e) => setChatInput(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter" && !e.shiftKey) {
                          e.preventDefault();
                          handleSendChatMessage();
                        }
                      }}
                      placeholder="I sell custom wedding cakes..."
                      className="w-full max-h-[120px] bg-transparent text-sm text-[#1D1D1F] dark:text-[#F5F5F7] p-3 pr-10 outline-none resize-none hide-scrollbar placeholder-gray-400 dark:placeholder-gray-500"
                      rows={1}
                      style={{ minHeight: "44px" }}
                    />
                  </div>
                  <button
                    onClick={handleSendChatMessage}
                    disabled={
                      isLoading || (!chatInput.trim() && !chatImageUrl.trim())
                    }
                    className="w-11 h-11 rounded-full bg-[#0066FF] text-white flex items-center justify-center shrink-0 disabled:opacity-50 disabled:cursor-not-allowed shadow-md active:scale-95 transition-transform"
                  >
                    <svg
                      className="w-5 h-5 ml-1"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                      />
                    </svg>
                  </button>
                </div>
                <div className="mt-3 flex justify-center">
                  <a
                    href="/api/v1/growth/referrals/click?target=/onboarding&ref=website-builder"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-[10px] font-semibold text-gray-400 uppercase tracking-widest hover:text-gray-600 transition-colors flex items-center gap-1"
                  >
                    ⚡ Powered by OHC
                  </a>
                </div>
              </div>
            </div>
          )}

          {step === 4 && (
             <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
               <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6">
                 <svg className="w-8 h-8 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
               </div>
               <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Secure your store</h2>
               <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-6">
                 We've built your business! Let's set up an admin account so you can log in later.
               </p>

               <div className="w-full space-y-4">
                 <div>
                   <input
                     type="email"
                     value={adminEmail}
                     onChange={(e) => setAdminEmail(e.target.value)}
                     placeholder="you@example.com"
                     className="w-full p-3 sm:p-4 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none glassmorphism min-h-[44px] text-[#1D1D1F] dark:text-[#F5F5F7]"
                   />
                 </div>
                 <div>
                   <input
                     type="password"
                     value={adminPassword}
                     onChange={(e) => setAdminPassword(e.target.value)}
                     placeholder="Create a password"
                     className="w-full p-3 sm:p-4 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none glassmorphism min-h-[44px] text-[#1D1D1F] dark:text-[#F5F5F7]"
                   />
                 </div>
                 {error && <p className="text-[#FF3B30] text-sm text-center">{error}</p>}
               </div>

               <div className="mt-auto pt-6 w-full">
                 <button
                   onClick={handleStartOnboarding}
                   disabled={isLoading || !adminEmail || !adminPassword}
                   className="w-full bg-[#0066FF] text-white min-h-[44px] min-w-[44px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
                 >
                   {isLoading ? "Launching..." : "Launch Store"}
                 </button>
               </div>
             </div>
          )}

          {step === 4.5 && (
             <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in glassmorphism rounded-[16px] shadow-2xl p-8">
               <div className="w-24 h-24 relative mb-8">
                 <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                 <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
               </div>
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Building Your Business...</h2>
               <div className="space-y-2">
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Generating your product catalog</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring payment settings</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Designing your storefront</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1.5s' }}>Onboarding your AI agents</p>
               </div>
             </div>
          )}

          {step === 5 && startResult && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-20 h-20 bg-[#34C759]/20 rounded-full flex items-center justify-center mb-6">
                <svg
                  className="w-10 h-10 text-[#34C759]"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={3}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                You're Live!
              </h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8 px-4">
                {startResult.message ||
                  "Your business has been successfully launched."}
              </p>

              <div className="w-full space-y-3 mt-auto">
                <div className="p-3 glassmorphism rounded-[8px] flex flex-col items-center mb-6">
                  <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">
                    Your Shareable Link
                  </p>
                  <div className="flex items-center gap-2">
                    <span className="text-[#0066FF] font-semibold">
                      {generateSubdomain(businessName)}
                    </span>
                  </div>
                </div>

                <a
                  href="/assistant"
                  className="flex w-full items-center justify-center glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[8px] font-bold shadow-md hover:border-gray-400 dark:hover:border-gray-500 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="sparkles">Open Assistant</IconLabel>
                </a>
                <a
                  href="/website-builder"
                  className="flex w-full items-center justify-center glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[8px] font-bold shadow-sm active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="eye">Storefront Builder</IconLabel>
                </a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
