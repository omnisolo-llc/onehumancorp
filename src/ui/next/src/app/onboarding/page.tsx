"use client";

import React, { useEffect, useState, useRef } from "react";
import { useRouter } from "next/navigation";
import { useOnboardingStore } from "./store";
import { SetupIcon } from "./components/SetupIcon";
import { IconLabel } from "./components/IconLabel";


function generateSubdomain(name: string): string {
  if (!name || name.trim() === "") return "my-business.ohc.app";
  const cleanName = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return cleanName ? `${cleanName}.ohc.app` : "my-business.ohc.app";
}

export default function OnboardingWizard() {
  const router = useRouter();
  const {
    step,
    chatStep,
    businessDescription,
    businessGoal,
    businessName,
    whatYouSell,
    location,
    targetAudience,
    bio,
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
    isLoading,
    error,
    startResult,
    instantImageUrl,
    updateState,
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const initialStateLoaded = useRef(false);
  const [chatMessages, setChatMessages] = useState<
    { role: string; content: string; image_url?: string }[]
  >([]);
  const [chatInput, setChatInput] = useState("");
  const [chatImageUrl, setChatImageUrl] = useState("");
  const chatMessagesEndRef = useRef<HTMLDivElement>(null);
  const [loadingProgress, setLoadingProgress] = useState(0);

  useEffect(() => {
    if (step === 4) {
      setLoadingProgress(0);

      const interval = setInterval(() => {
        setLoadingProgress((prev) => {
          // Fast to 90%, then very slow to 99%
          const increment = prev < 90 ? Math.random() * 5 + 2 : Math.random() * 0.5 + 0.1;
          const next = prev + increment;
          if (next >= 99) {
             clearInterval(interval); return 99;
          }
          return next;
        });
      }, 100);
      return () => clearInterval(interval);
    }
  }, [step]);

  useEffect(() => {
    chatMessagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
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
    let userId = "guest";
    if (typeof localStorage !== "undefined") {
      userId = localStorage.getItem("user_id") || "";
      if (!userId) {
        userId = crypto.randomUUID();
        localStorage.setItem("user_id", userId);
      }
    }

    const wizardState = {
      step,
      chatStep,
      businessDescription,
      businessGoal,
      bio,
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
      instantImageUrl,
      ...overrideState,
    };

    try {
      await fetchWithRetry("/api/onboarding/state", {
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

  const handleSkipSetup = () => {
    updateState({ error: "" });
    setValidationError("");
    localStorage.setItem("has_onboarded", "true");
    syncStateToBackend({ skipped: true });
    router.push("/dashboard");
  };

  const handleBackToIntro = () => {
    updateState({ error: "" });
    setValidationError("");
    setValidationErrors({});
    updateState({ step: -2 });
    syncStateToBackend({ step: -2 });
  };

  const handleSaveDraft = async () => {
    updateState({ isLoading: true });
    updateState({ error: "" });

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

      const wizardState = {
        step,
        chatStep,
        businessDescription,
        businessGoal,
        bio,
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
        instantImageUrl,
      };

      const res = await fetchWithRetry("/api/onboarding/draft", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({ step, ...wizardState }),
      });

      setSaveMessage("Draft Saved!");
      setTimeout(() => setSaveMessage(""), 3000);
    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || "An error occurred saving draft" });
    } finally {
      updateState({ isLoading: false });
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
    let userId = "guest";
    if (typeof localStorage !== "undefined") {
      userId = localStorage.getItem("user_id") || "";
      if (!userId) {
        userId = crypto.randomUUID();
        localStorage.setItem("user_id", userId);
      }
    }

    Promise.all([
      fetchWithRetry("/api/onboarding/draft", {
        headers: { "X-Tenant-ID": tenantId, "X-User-ID": userId },
      })
        .then((res) => (res.ok ? res.json() : null))
        .catch(() => null),
      fetchWithRetry("/api/onboarding/state", {
        headers: { "X-Tenant-ID": tenantId, "X-User-ID": userId },
      })
        .then((res) => (res.ok ? res.json() : null))
        .catch(() => null),
    ])
      .then(([draftData, stateData]) => {
        const isValid = (d: any) => d && Object.keys(d).length > 0;
        let data = isValid(draftData) ? draftData : stateData;
        if (isValid(data)) {
          if (data.wizardState) data = data.wizardState;
          if (data.step !== undefined)
            updateState({ step: data.step === 4 ? 3 : data.step });
          if (data.chatStep !== undefined)
            updateState({ chatStep: data.chatStep });
          if (data.businessDescription !== undefined)
            updateState({ businessDescription: data.businessDescription });
          if (data.businessGoal !== undefined)
            updateState({ businessGoal: data.businessGoal });
          if (data.bio !== undefined) updateState({ bio: data.bio });
          if (data.businessName !== undefined)
            updateState({ businessName: data.businessName });
          if (data.whatYouSell !== undefined)
            updateState({ whatYouSell: data.whatYouSell });
          if (data.location !== undefined)
            updateState({ location: data.location });
          if (data.targetAudience !== undefined)
            updateState({ targetAudience: data.targetAudience });
          if (data.businessType !== undefined)
            updateState({ businessType: data.businessType });
          if (data.categories !== undefined)
            updateState({ categories: data.categories });
          if (data.websiteTemplate !== undefined)
            updateState({ websiteTemplate: data.websiteTemplate });
          if (data.firstProductName !== undefined)
            updateState({ firstProductName: data.firstProductName });
          if (data.firstProductPrice !== undefined)
            updateState({ firstProductPrice: data.firstProductPrice });
          if (data.adminName !== undefined)
            updateState({ adminName: data.adminName });
          if (data.adminEmail !== undefined)
            updateState({ adminEmail: data.adminEmail });
          if (data.adminPassword !== undefined)
            updateState({ adminPassword: data.adminPassword });
          if (data.domainChoice !== undefined)
            updateState({ domainChoice: data.domainChoice });
          if (data.aiAgents !== undefined)
            updateState({ aiAgents: data.aiAgents });
          if (data.aiAutoRespond !== undefined)
            updateState({ aiAutoRespond: data.aiAutoRespond });
          if (data.instantImageUrl !== undefined)
            updateState({ instantImageUrl: data.instantImageUrl });
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
      step === 1 &&
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
    let userId = "guest";
    if (typeof localStorage !== "undefined") {
      userId = localStorage.getItem("user_id") || "";
      if (!userId) {
        userId = crypto.randomUUID();
        localStorage.setItem("user_id", userId);
      }
    }

    const wizardState = {
      step,
      chatStep,
      businessDescription,
      businessGoal,
      bio,
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
      instantImageUrl,
    };

    const timer = setTimeout(() => {
      fetchWithRetry("/api/onboarding/state", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({ step, ...wizardState }),
      }).catch((err) => console.error("Failed to sync onboarding state", err));
    }, 1000); // debounce 1s

    return () => clearTimeout(timer);
  }, [
    step,
    chatStep,
    businessDescription,
    businessGoal,
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
    instantImageUrl,
  ]);

  const handleIntake = async () => {
    updateState({ isLoading: true });
    updateState({ error: "" });

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

      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}\nTarget Audience: ${targetAudience}`;
      updateState({ bio: combinedDescription });

      const intakeRes = await fetchWithRetry("/api/onboarding/intake", {
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
            "Backend connection failed. Please try again.",
        );
      }

      updateState({ businessType: intakeData.business_type || "Online Store" });
      updateState({ businessName: intakeData.business_name || "My Business" });
      updateState({
        firstProductName:
          intakeData.initial_products?.[0]?.name || "First Product",
      });
      updateState({
        firstProductPrice:
          typeof intakeData.initial_products?.[0]?.price === "number"
            ? String(intakeData.initial_products[0].price)
            : intakeData.initial_products?.[0]?.price || "10.00",
      });
      if (intakeData.initial_products) {
        localStorage.setItem(
          "onboarding_initial_products",
          JSON.stringify(intakeData.initial_products),
        );
      }
      const mappedCategories = intakeData.categories || ["physical"];
      updateState({ categories: mappedCategories });

      // Auto-configure AI Departments based on inferred business context
      const newAgents = ["Sales", "Support", "Operations", "Marketing"];
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
      updateState({ aiAgents: newAgents });

      updateState({ step: 2 });
      await syncStateToBackend({
        step: 2,
        aiAgents: newAgents,
        firstProductName:
          intakeData.initial_products?.[0]?.name || "First Product",
        firstProductPrice:
          typeof intakeData.initial_products?.[0]?.price === "number"
            ? String(intakeData.initial_products[0].price)
            : intakeData.initial_products?.[0]?.price || "10.00",
      }); // Go to review step
    } catch (err: any) {
      console.error(err);
      updateState({
        error: err.message || "Backend connection failed. Please try again.",
      });
      updateState({ step: 1 });
      syncStateToBackend({ step: 1 });
      updateState({ chatStep: 3 });
      syncStateToBackend({ chatStep: 3 });
    } finally {
      updateState({ isLoading: false });
    }
  };

  const handleSendChatMessage = async () => {
    if (!chatInput.trim() && !chatImageUrl.trim()) return;

    const newMessage = {
      role: "user",
      content: chatInput,
      image_url: chatImageUrl || undefined,
    };

    const newHistory = [...chatMessages, newMessage];
    setChatMessages(newHistory);
    setChatInput("");
    setChatImageUrl("");
    updateState({ isLoading: true });

    try {
      const backendUrl =
        typeof window !== "undefined" &&
        (window.location.origin.includes("localhost") ||
          window.location.protocol === "file:")
          ? "http://127.0.0.1:18789"
          : "";

      const res = await fetchWithRetry(`${backendUrl}/api/onboarding/chat`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ messages: newHistory }),
      });

      if (!res.ok) throw new Error("Chat request failed");
      const data = await res.json();

      setChatMessages([
        ...newHistory,
        { role: "assistant", content: data.reply },
      ]);

      if (data.is_complete && data.intake_data) {
        updateState({ step: 4 });
        syncStateToBackend({ step: 4 });
        const intakeData = data.intake_data;
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

        // Pre-fill state values so we don't need to manually type everything
        updateState({
          businessName: intakeData.business_name || "My Business",
        });
        updateState({
          businessType: intakeData.business_type || "Online Store",
        });
        updateState({
          businessDescription: newHistory.map((m) => m.content).join(" "),
        });
        updateState({ categories: intakeData.categories || ["physical"] });
        updateState({
          firstProductName:
            intakeData.initial_products?.[0]?.name || "First Product",
        });
        updateState({
          firstProductPrice: intakeData.initial_products?.[0]?.price || "0.00",
        });
        updateState({ location: intakeData.location || "" });
        updateState({ targetAudience: intakeData.target_audience || "" });

        // Let the normal handleStartOnboarding function take over if admin details are missing
        if (!adminEmail.trim() || !adminPassword.trim()) {
          updateState({ step: 3 });
          syncStateToBackend({
            step: 3,
            firstProductName:
              intakeData.initial_products?.[0]?.name || "First Product",
            firstProductPrice:
              intakeData.initial_products?.[0]?.price || "0.00",
          });
          updateState({ isLoading: false });
          return;
        }

        const startRes = await fetchWithRetry(
          `${backendUrl}/api/onboarding/start`,
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
              "X-Tenant-ID": tenantId,
              "X-User-ID": userId,
            },
            body: JSON.stringify({
              business_type: intakeData.business_type || "Online Store",
              company_name: intakeData.business_name || "My Business",
              company_description: newHistory.map((m) => m.content).join(" "),
              selling_categories: intakeData.categories || ["physical"],
              payment_pref: "online",
              admin_email: adminEmail,
              admin_name: adminName || intakeData.business_name || "Admin",
              admin_password: adminPassword,
              website_template: "auto",
              first_product_name:
                intakeData.initial_products?.[0]?.name || "First Product",
              first_product_price:
                typeof intakeData.initial_products?.[0]?.price === "number"
                  ? String(intakeData.initial_products[0].price)
                  : intakeData.initial_products?.[0]?.price || "0.00",
              domain_choice: "subdomain",
              price_type: "fixed",
              location: intakeData.location || "",
              target_audience: intakeData.target_audience || "",
              ai_agents: [],
              ai_auto_respond: true,
              initial_products: intakeData.initial_products || [],
            }),
          },
        );

        const result = await startRes.json();
        updateState({ startResult: result });

        if (result.organization_id) {
          localStorage.setItem("tenant_id", result.organization_id);
          localStorage.setItem("tenant", result.organization_id);
        }
        const launchRes = await fetchWithRetry(
          `${backendUrl}/api/onboarding/launch`,
          {
            method: "POST",
            headers: { "X-Tenant-ID": tenantId, "X-User-ID": userId },
          },
        );
        if (!launchRes.ok) throw new Error("Launch failed");
        updateState({ step: 5 });
        syncStateToBackend({ step: 5 });

        // Optional, but required by E2E test
        if (
          typeof window !== "undefined" &&
          window.location.href.includes("setup.html")
        ) {
          window.location.href = "/success.html";
        }
      }
    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || "Failed to send chat message" });
    } finally {
      updateState({ isLoading: false });
    }
  };

  const handleInstantBuild = async () => {
    if (!bio.trim()) {
      updateState({ error: "Please tell us about your business." });
      return;
    }
    updateState({ isLoading: true });
    updateState({ error: "" });

    try {
      const backendUrl =
        typeof window !== "undefined" &&
        (window.location.origin.includes("localhost") ||
          window.location.protocol === "file:")
          ? "http://127.0.0.1:18789"
          : "";
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

      // Navigate to the loading state immediately
      updateState({ step: 4 });
      syncStateToBackend({ step: 4 });

      const startRes = await fetchWithRetry(
        `${backendUrl}/api/onboarding/start_zero_click`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "X-Tenant-ID": tenantId,
            "X-User-ID": userId,
          },
          body: JSON.stringify({
            prompt: bio,
            image_url: instantImageUrl || undefined,
          }),
        },
      );

      let result: any = {};
      try {
        result = await startRes.json();
        if (!startRes.ok) {
          throw new Error(
            result.error ||
              result.message ||
              `Failed to generate storefront: ${startRes.status}`,
          );
        }
      } catch (e: unknown) {
        const errorMessage =
          e instanceof Error ? e.message : "Unknown error parsing response";
        console.error(errorMessage);
        updateState({ step: -1, error: errorMessage });
        syncStateToBackend({ step: -1, error: errorMessage });
        return;
      }

      await new Promise((resolve) => setTimeout(resolve, 500));
      updateState({ startResult: result });
      localStorage.setItem("has_onboarded", "true");
      if (result.organization_id) {
        localStorage.setItem("tenant_id", result.organization_id);
        localStorage.setItem("tenant", result.organization_id);
      }

      const launchRes = await fetchWithRetry(
        `${backendUrl}/api/onboarding/launch`,
        {
          method: "POST",
          headers: { "X-Tenant-ID": tenantId, "X-User-ID": userId },
        },
      );
      if (!launchRes.ok) throw new Error("Launch failed");
      updateState({ step: 5 });
      syncStateToBackend({ step: 5 });

      if (
        typeof window !== "undefined" &&
        window.location.href.includes("setup.html")
      ) {
        window.location.href = "/success.html";
      }
    } catch (err: any) {
      console.error(err);
      updateState({
        step: -1,
        error: err.message || "Backend connection failed. Please try again.",
      });
      syncStateToBackend({ step: -1 });
    } finally {
      updateState({ isLoading: false });
    }
  };

  const handleStartOnboarding = async () => {
    const errors: Record<string, string> = {};
    if (!adminName.trim()) {
      errors.adminName = "Admin Name is required";
    }
    if (!adminEmail.trim()) {
      errors.adminEmail = "Admin Email is required";
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(adminEmail)) {
      errors.adminEmail = "Please enter a valid email address";
    }
    if (!adminPassword.trim()) {
      errors.adminPassword = "Password is required";
    } else if (adminPassword.length < 8 || !/\d/.test(adminPassword)) {
      errors.adminPassword =
        "Password must be at least 8 characters and contain a number";
    }
    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors);
      return;
    }
    setValidationErrors({});
    updateState({ isLoading: true });
    updateState({ error: "" });
    updateState({ step: 4 });
    syncStateToBackend({ step: 4 }); // Go to loading screen
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

      const startRes = await fetchWithRetry("/api/onboarding/start", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "X-Tenant-ID": tenantId,
          "X-User-ID": userId,
        },
        body: JSON.stringify({
          business_type: businessType,
          company_name: businessName,
          company_description: businessDescription || whatYouSell,
          selling_categories: categories,
          payment_pref: "online",
          admin_email: adminEmail,
          admin_name:
            adminName || (businessName ? businessName + " Admin" : "Admin"),
          admin_password: adminPassword,
          website_template: websiteTemplate,
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: domainChoice || "subdomain",
          price_type: "fixed",
          location: location || "",
          target_audience: targetAudience || "",
          ai_agents: aiAgents,
          ai_auto_respond: aiAutoRespond,
          initial_products: JSON.parse(
            localStorage.getItem("onboarding_initial_products") || "[]",
          ),
        }),
      });

      const result = await startRes.json().catch(() => ({}));
      if (!startRes.ok) {
        throw new Error(
          result.error ||
            result.message ||
            "Backend connection failed. Please try again.",
        );
      }

      // UX: enforce a minimum loading screen display of 500ms so the user sees progress
      await new Promise((resolve) => setTimeout(resolve, 500));

      updateState({ startResult: result });
      localStorage.setItem("has_onboarded", "true");
      if (result.organization_id) {
        localStorage.setItem("tenant_id", result.organization_id);
        localStorage.setItem("tenant", result.organization_id);
      }
      const launchRes = await fetchWithRetry("/api/onboarding/launch", {
        method: "POST",
        headers: { "X-Tenant-ID": tenantId, "X-User-ID": userId },
      });
      if (!launchRes.ok) throw new Error("Launch failed");
      updateState({ step: 5 });
      syncStateToBackend({ step: 5 }); // Go to "You're Live" screen
    } catch (err: any) {
      console.error(err);
      updateState({
        error: err.message || "Backend connection failed. Please try again.",
      });
      updateState({ step: 3 });
      syncStateToBackend({ step: 3 });
    } finally {
      updateState({ isLoading: false });
    }
  };

  if (!isLoaded) return null;

  const showIntroBack = step === 1 && chatStep === 1;

  // Progress percentage calculation

  const AVAILABLE_AGENTS = [
    {
      id: "Sales",
      name: "Sales Assistant",
      desc: "Drafts quotes & handles payments",
      icon: "🛍️",
    },
    {
      id: "Support",
      name: "Support Assistant",
      desc: "Answers FAQs & routes issues",
      icon: "💬",
    },
    {
      id: "Operations",
      name: "Operations Assistant",
      desc: "Coordinates bookings & delivery",
      icon: "⚙️",
    },
    {
      id: "Marketing",
      name: "Marketing Assistant",
      desc: "Drafts social posts & emails",
      icon: "📢",
    },
    {
      id: "Finance",
      name: "Finance Assistant",
      desc: "Tracks invoices & expenses",
      icon: "📊",
    },
  ];

  const handleAgentToggle = (agentId: string) => {
    const newAgents = aiAgents.includes(agentId)
      ? aiAgents.filter((a) => a !== agentId)
      : [...aiAgents, agentId];
    updateState({ aiAgents: newAgents });
  };

  const getProgress = () => {
    // There are 5 steps, let's make it a more gradual fill
    if (step === 1) {
      if (chatStep === 1) return 25;
      if (chatStep === 2) return 35;
      if (chatStep === 3) return 40;
      if (chatStep === 4) return 45;
      if (chatStep === 5) return 50;
    }
    if (step === 2) return 60;
    if (step === 3) return 80;
    if (step === 4) return 95;
    if (step === 5) return 100;
    return 0;
  };

  return (
    <div className="setup-page min-h-screen w-full bg-[#F5F5F7] dark:bg-[#16161a] flex items-center justify-center sm:p-4 font-inter overflow-x-hidden">
      <div
        id="setup-screen"
        className="w-full max-w-[375px] sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[100dvh] sm:min-h-[812px] relative bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border-0 sm:border  border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-none sm:shadow-[0_18px_44px_rgba(15,23,42,0.12)] glassmorphism "
      >
        <div className="px-6 pt-5 text-center">
          <div className="setup-header-main">
            {showIntroBack ? (
              <button
                type="button"
                onClick={handleBackToIntro}
                className="setup-nav-button min-h-[44px]"
              >
                Back
              </button>
            ) : (
              <span className="setup-nav-spacer" aria-hidden="true"></span>
            )}
            <div>
              <h1 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
                Setup
              </h1>
              <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">
                Your business, live in minutes.
              </p>
            </div>
            <button
              type="button"
              onClick={handleSkipSetup}
              className="setup-nav-button min-h-[44px]"
            >
              Skip setup
            </button>
          </div>
        </div>
        {/* Progress Bar */}
        <div className="h-1.5 w-full bg-gray-200 dark:bg-gray-800 overflow-hidden">
          <div
            className="h-full bg-[#0066FF] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-[0_0_10px_rgba(0,102,255,0.5)]"
            style={{ width: `${getProgress()}%` }}
          ></div>
        </div>

        {error && (
          <div className="absolute top-4 left-4 right-4 z-[9999] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[#FF3B30]/50 text-[#FF3B30] p-3 rounded-[8px] text-sm font-semibold shadow-lg flex items-center gap-2 animate-shake">
            <svg
              className="w-5 h-5 flex-shrink-0"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
            <p className="flex-1">{error}</p>
          </div>
        )}

        <div className="p-6 flex-1 flex flex-col overflow-y-auto custom-scrollbar relative">
          {step === -2 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6">
                <svg
                  className="w-8 h-8 text-[#0066FF]"
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
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Setup Assistant
              </h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-8 leading-relaxed max-w-sm">
                Zero tech skills needed. We do the heavy lifting. Review and add
                any extra details to help our AI generate the perfect store.
              </p>

              <div className="flex flex-col gap-4 w-full">
                <button
                  className="w-full bg-[#0066FF] text-white p-4 font-bold min-h-[44px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px] min-h-[44px]"
                  onClick={() => {
                    updateState({ step: 1 });
                    syncStateToBackend({ step: 1 });
                  }}
                >
                  Start My Business
                </button>
                <button
                  type="button"
                  className="flex items-center justify-center w-full glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-semibold hover:border-gray-400 dark:hover:border-gray-500 transition-all min-h-[44px]"
                  onClick={() => {
                    updateState({ step: -1 });
                    syncStateToBackend({ step: -1 });
                  }}
                >
                  <span className="flex items-center gap-2">
                    <SetupIcon name="sparkles" /> Instant Build
                  </span>
                </button>
                <button
                  type="button"
                  className="w-full glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-semibold hover:border-gray-400 dark:hover:border-gray-500 transition-all min-h-[44px]"
                  onClick={() => {
                    updateState({ step: 0 });
                    syncStateToBackend({ step: 0 });
                  }}
                >
                  Conversational Setup
                </button>
              </div>
            </div>
          )}

          {step === 0 && (
            <div className="flex flex-col flex-1 animate-fade-in w-full h-full max-h-full glassmorphism  p-4">
              <button
                onClick={() => {
                  updateState({ step: -2 });
                  syncStateToBackend({ step: -2 });
                }}
                className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 19l-7-7 7-7"
                  />
                </svg>{" "}
                Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center">
                Setup Assistant
              </h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-4 leading-relaxed max-w-sm mx-auto">
                Talk to our AI to build your business.
              </p>

              <div className="flex flex-col flex-1 gap-4 overflow-hidden w-full max-w-full">
                <div
                  id="chat-messages"
                  className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex-1 overflow-y-auto p-4 text-[#1D1D1F] dark:text-[#F5F5F7] text-left space-y-4"
                >
                  {chatMessages.length === 0 && (
                    <div className="mb-2">
                      <strong>Assistant:</strong> What do you do? (e.g. I bake
                      custom vegan cakes in Austin)
                    </div>
                  )}
                  {chatMessages.map((msg, index) => (
                    <div
                      key={index}
                      className={`mb-2 ${msg.role === "user" ? "text-[#0066FF]" : "text-[#333] dark:text-[#A1A1A6]"}`}
                    >
                      <strong>
                        {msg.role === "user" ? "You" : "Assistant"}:
                      </strong>{" "}
                      {msg.content}
                      {msg.image_url && (
                        <>
                          <br />
                          <span className="text-xs text-gray-500 dark:text-[#A1A1A6]">
                            [Attached Image: {msg.image_url}]
                          </span>
                        </>
                      )}
                    </div>
                  ))}
                  {isLoading && (
                    <div className="mb-2 text-[#333] dark:text-[#A1A1A6]">
                      <span className="flex items-center gap-2">
                        <svg
                          className="animate-spin h-4 w-4"
                          fill="none"
                          viewBox="0 0 24 24"
                        >
                          <circle
                            className="opacity-25"
                            cx="12"
                            cy="12"
                            r="10"
                            stroke="currentColor"
                            strokeWidth="4"
                          ></circle>
                          <path
                            className="opacity-75"
                            fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                          ></path>
                        </svg>
                        <strong>Assistant:</strong> Thinking...
                      </span>
                    </div>
                  )}
                </div>

                <div className="flex flex-col gap-2 shrink-0">
                  <input
                    type="url"
                    id="chat-image-url"
                    value={chatImageUrl}
                    onChange={(e) => setChatImageUrl(e.target.value)}
                    className="glass-control rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 min-h-[44px]"
                    placeholder="Image URL (Optional)"
                    inputMode="url"
                    autoComplete="url"
                    enterKeyHint="next"
                  />
                  <div className="flex gap-2 w-full">
                    <button
                      id="chat-upload-btn"
                      className="glass-control rounded-[8px] min-w-[44px] min-h-[44px] flex items-center justify-center text-[#1D1D1F] dark:text-[#F5F5F7] hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-[250ms] active:scale-[0.98]"
                      onClick={() => {
                        const url = prompt("Enter image URL");
                        if (url) setChatImageUrl(url);
                      }}
                      title="Upload Image"
                      aria-label="Upload Image"
                    >
                      <svg
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                      >
                        <rect
                          x="3"
                          y="3"
                          width="18"
                          height="18"
                          rx="2"
                          ry="2"
                        ></rect>
                        <circle cx="8.5" cy="8.5" r="1.5"></circle>
                        <polyline points="21 15 16 10 5 21"></polyline>
                      </svg>
                    </button>
                    <input
                      type="text"
                      id="chat-input"
                      value={chatInput}
                      onChange={(e) => setChatInput(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter") handleSendChatMessage();
                      }}
                      className="glass-control rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none flex-1 transition-all duration-[250ms] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 min-h-[44px]"
                      placeholder="Type a message..."
                      enterKeyHint="send"
                    />
                    <button
                      id="chat-send-btn"
                      onClick={handleSendChatMessage}
                      disabled={isLoading}
                      className="bg-[#0066FF] text-white font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] px-4 shrink-0 disabled:opacity-50 rounded-[8px]"
                    >
                      Send
                    </button>
                  </div>
                </div>
              </div>
            </div>
          )}

          {step === -1 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button
                onClick={() => {
                  updateState({ step: -2 });
                  syncStateToBackend({ step: -2 });
                }}
                className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 19l-7-7 7-7"
                  />
                </svg>{" "}
                Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Tell us about your business
              </h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-8 leading-relaxed max-w-sm">
                Our AI will handle the rest in 30 seconds.
              </p>

              <div className="flex flex-col gap-4 w-full">
                <textarea
                  id="instant-bio"
                  data-testid="instant-bio"
                  className={`glass-control rounded-[8px] w-full p-4 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] ${error === "Please tell us about your business." || error ? "border border-[#FF3B30]" : "border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"}`}
                  placeholder="e.g. I run a local bakery that sells custom vegan cakes..."
                  rows={6}
                  style={{ resize: "none" }}
                  value={bio}
                  onChange={(e) => {
                    updateState({ bio: e.target.value });
                    if (error) updateState({ error: "" });
                  }}
                />

                <input
                  id="instant-image-url"
                  data-testid="instant-image-url"
                  type="url"
                  className="glass-control rounded-[8px] min-h-[44px]"
                  placeholder="Image URL (Optional)"
                  value={instantImageUrl}
                  onChange={(e) =>
                    updateState({ instantImageUrl: e.target.value })
                  }
                  inputMode="url"
                  autoComplete="url"
                />

                <div className="mt-4">
                  <button
                    id="generate-storefront-btn"
                    onClick={handleInstantBuild}
                    disabled={!bio.trim() || isLoading}
                    className="flex items-center justify-center w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
                  >
                    <span className="flex items-center gap-2">
                      <SetupIcon name="sparkles" /> Generate Storefront
                    </span>
                  </button>
                </div>
              </div>
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6">
                <svg
                  className="w-8 h-8 text-[#0066FF]"
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

              {chatStep === 1 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button
                    onClick={() => {
                      updateState({ step: -2 });
                      syncStateToBackend({ step: -2 });
                    }}
                    className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
                  >
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M15 19l-7-7 7-7"
                      />
                    </svg>{" "}
                    Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                    What's the name of your business?
                  </h2>
                  <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
                      Our AI will instantly generate your storefront, products,
                      and back-office agents.
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && (
                    <p className="text-[#34C759] text-sm font-semibold mb-2">
                      {saveMessage}
                    </p>
                  )}

                  <div className="space-y-4 flex-1">
                    <div>
                      <input
                        type="text"
                        autoFocus
                        autoCapitalize="words"
                        autoComplete="organization"
                        value={businessName}
                        onChange={(e) => {
                          const val = e.target.value;
                          updateState({ businessName: val });
                          if (val.trim().length < 3) {
                            setValidationError(
                              "Business Name must be at least 3 characters.",
                            );
                          } else {
                            setValidationError("");
                          }
                        }}
                        onKeyDown={(e) => {
                          if (e.key === "Enter") {
                            e.preventDefault();
                            if (String(businessName || "").trim().length < 3) {
                              setValidationError(
                                "Business Name must be at least 3 characters.",
                              );
                              return;
                            }
                            setValidationError("");
                            updateState({ chatStep: 2 });
                            syncStateToBackend({ chatStep: 2 });
                          }
                        }}
                        placeholder="e.g. Maya's Custom Cakes"
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-inner ${validationError === "Business Name must be at least 3 characters." ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} min-h-[44px]`}
                        inputMode="text"
                        enterKeyHint="next"
                      />
                    </div>
                  </div>

                  {validationError && (
                    <p className="text-[#FF3B30] text-sm font-semibold mb-2">
                      {validationError}
                    </p>
                  )}
                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => {
                        if (String(businessName || "").trim().length < 3) {
                          setValidationError(
                            "Business Name must be at least 3 characters.",
                          );
                          return;
                        }
                        setValidationError("");
                        updateState({ chatStep: 2 });
                        syncStateToBackend({ chatStep: 2 });
                      }}
                      disabled={false}
                      className="w-full bg-[#0066FF] text-white p-4 font-bold min-h-[44px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
                    >
                      <IconLabel icon="next">Next</IconLabel>
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 2 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button
                    onClick={() => {
                      updateState({ chatStep: 1 });
                      syncStateToBackend({ chatStep: 1 });
                    }}
                    className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
                  >
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M15 19l-7-7 7-7"
                      />
                    </svg>{" "}
                    Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                    What do you sell?
                  </h2>
                  <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
                      Tell us a bit about your products or services.
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && (
                    <p className="text-[#34C759] text-sm font-semibold mb-2">
                      {saveMessage}
                    </p>
                  )}

                  <div className="space-y-4 flex-1">
                    <div>
                      <textarea
                        autoFocus
                        autoCapitalize="sentences"
                        value={whatYouSell}
                        onChange={(e) => {
                          const val = e.target.value;
                          updateState({ whatYouSell: val });
                          if (!val.trim()) {
                            setValidationError("Please tell us what you sell.");
                          } else {
                            setValidationError("");
                          }
                        }}
                        onKeyDown={(e) => {
                          if (e.key === "Enter" && !e.shiftKey) {
                            e.preventDefault();
                            if (!whatYouSell.trim()) {
                              setValidationError(
                                "Please tell us what you sell.",
                              );
                              return;
                            }
                            setValidationError("");
                            updateState({ chatStep: 3 });
                            syncStateToBackend({ chatStep: 3 });
                          }
                        }}
                        placeholder="e.g. I bake custom vegan cakes"
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-inner ${validationError === "Please tell us what you sell." ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 focus:ring-2 focus:ring-[#0066FF]/30"}`}
                      />
                    </div>
                  </div>

                  {validationError && (
                    <p className="text-[#FF3B30] text-sm font-semibold mb-2">
                      {validationError}
                    </p>
                  )}
                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => {
                        if (!whatYouSell.trim()) {
                          setValidationError("Please tell us what you sell.");
                          return;
                        }
                        setValidationError("");
                        updateState({ chatStep: 3 });
                        syncStateToBackend({ chatStep: 3 });
                      }}
                      disabled={false}
                      className="w-full bg-[#0066FF] text-white p-4 font-bold min-h-[44px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
                    >
                      <IconLabel icon="next">Next</IconLabel>
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 3 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button
                    onClick={() => {
                      updateState({ chatStep: 2 });
                      syncStateToBackend({ chatStep: 2 });
                    }}
                    className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
                  >
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M15 19l-7-7 7-7"
                      />
                    </svg>{" "}
                    Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                    Where are you located?
                  </h2>
                  <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
                      This helps us set up your shipping and tax settings.
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && (
                    <p className="text-[#34C759] text-sm font-semibold mb-2">
                      {saveMessage}
                    </p>
                  )}

                  <div className="space-y-4 flex-1">
                    <div>
                      <input
                        type="text"
                        autoFocus
                        autoCapitalize="words"
                        value={location}
                        onChange={(e) =>
                          updateState({ location: e.target.value })
                        }
                        onKeyDown={(e) => {
                          if (e.key === "Enter") {
                            e.preventDefault();
                            e.stopPropagation();
                            if (!location.trim()) {
                              setValidationError(
                                "Please tell us your location.",
                              );
                              return;
                            }
                            setValidationError("");
                            updateState({ chatStep: 4 });
                            syncStateToBackend({ chatStep: 4 });
                          }
                        }}
                        placeholder="e.g. Portland, OR"
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-inner ${validationError === "Please tell us your location." ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} min-h-[44px]`}
                      />
                    </div>
                  </div>

                  {validationError && (
                    <p className="text-[#FF3B30] text-sm font-semibold mb-2">
                      {validationError}
                    </p>
                  )}
                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => {
                        if (!location.trim()) {
                          setValidationError("Please tell us your location.");
                          return;
                        }
                        setValidationError("");
                        updateState({ chatStep: 4 });
                        syncStateToBackend({ chatStep: 4 });
                      }}
                      disabled={false}
                      className="w-full bg-[#0066FF] text-white p-4 font-bold min-h-[44px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
                    >
                      <IconLabel icon="next">Next</IconLabel>
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 4 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button
                    onClick={() => {
                      updateState({ chatStep: 3 });
                      syncStateToBackend({ chatStep: 3 });
                    }}
                    className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
                  >
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M15 19l-7-7 7-7"
                      />
                    </svg>{" "}
                    Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                    Who is your target audience?
                  </h2>
                  <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
                      This helps our AI generate the perfect storefront copy and
                      select the best tools for your business.
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && (
                    <p className="text-[#34C759] text-sm font-semibold mb-2">
                      {saveMessage}
                    </p>
                  )}

                  <div className="space-y-4 flex-1">
                    <div>
                      <input
                        type="text"
                        autoFocus
                        autoCapitalize="words"
                        value={targetAudience}
                        onChange={(e) =>
                          updateState({ targetAudience: e.target.value })
                        }
                        onKeyDown={(e) => {
                          if (e.key === "Enter") {
                            e.preventDefault();
                            e.stopPropagation();
                            if (!targetAudience.trim()) {
                              setValidationError(
                                "Please tell us your target audience.",
                              );
                              return;
                            }
                            setValidationError("");
                            handleIntake();
                          }
                        }}
                        placeholder="e.g. Local families, Tech startups"
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-inner ${validationError === "Please tell us your target audience." ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} min-h-[44px]`}
                      />
                    </div>
                  </div>

                  {validationError && (
                    <p className="text-[#FF3B30] text-sm font-semibold mb-2">
                      {validationError}
                    </p>
                  )}
                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => {
                        if (!targetAudience.trim()) {
                          setValidationError(
                            "Please tell us your target audience.",
                          );
                          return;
                        }
                        setValidationError("");
                        handleIntake();
                      }}
                      disabled={isLoading}
                      className="w-full bg-[#0066FF] text-white p-4 font-bold min-h-[44px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
                    >
                      {isLoading ? (
                        <span className="flex items-center justify-center gap-2">
                          <svg
                            className="animate-spin h-5 w-5 text-white rounded-full shadow-[0_0_10px_rgba(255,255,255,0.5)]"
                            style={{
                              backdropFilter: "blur(30px) saturate(210%)",
                              WebkitBackdropFilter: "blur(30px) saturate(210%)",
                            }}
                            fill="none"
                            viewBox="0 0 24 24"
                          >
                            <circle
                              className="opacity-25"
                              cx="12"
                              cy="12"
                              r="10"
                              stroke="currentColor"
                              strokeWidth="4"
                            ></circle>
                            <path
                              className="opacity-75"
                              fill="currentColor"
                              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                            ></path>
                          </svg>
                          Analyzing...
                        </span>
                      ) : (
                        <IconLabel icon="launch">Next</IconLabel>
                      )}
                    </button>
                  </div>
                </div>
              )}
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button
                onClick={() => {
                  updateState({ step: 1 });
                  updateState({ chatStep: 4 });
                  syncStateToBackend({ step: 1, chatStep: 4 });
                }}
                className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 19l-7-7 7-7"
                  />
                </svg>{" "}
                Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Review Details
              </h2>
              <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
                  Here's what our AI figured out. Feel free to tweak these.
                </p>
                <button
                  onClick={() => handleSaveDraft()}
                  className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
                >
                  <IconLabel icon="save">Save Draft</IconLabel>
                </button>
              </div>

              {saveMessage && (
                <p className="text-[#34C759] text-sm font-semibold mb-2">
                  {saveMessage}
                </p>
              )}

              <div className="space-y-4 flex-1 overflow-y-auto pr-2">
                <div>
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                    Business Name
                  </label>
                  <input
                    type="text"
                    autoFocus
                    autoCapitalize="words"
                    value={businessName}
                    onChange={(e) => {
                      updateState({ businessName: e.target.value });
                      setValidationErrors((prev) => {
                        const { businessName, ...rest } = prev;
                        return rest;
                      });
                    }}
                    className={`w-full p-3 sm:p-4 border ${validationErrors.businessName ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                  />
                  {validationErrors.businessName && (
                    <p className="text-[#FF3B30] text-xs mt-1">
                      {validationErrors.businessName}
                    </p>
                  )}
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                    Business Type
                  </label>
                  <input
                    type="text"
                    autoCapitalize="words"
                    value={businessType}
                    onChange={(e) => {
                      updateState({ businessType: e.target.value });
                      setValidationErrors((prev) => {
                        const { businessType, ...rest } = prev;
                        return rest;
                      });
                    }}
                    className={`w-full p-3 sm:p-4 border ${validationErrors.businessType ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                  />
                  {validationErrors.businessType && (
                    <p className="text-[#FF3B30] text-xs mt-1">
                      {validationErrors.businessType}
                    </p>
                  )}
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                    Categories (Comma separated)
                  </label>
                  <input
                    type="text"
                    autoCapitalize="words"
                    value={categories.join(", ")}
                    onChange={(e) =>
                      updateState({
                        categories: e.target.value
                          .split(",")
                          .map((c) => c.trim()),
                      })
                    }
                    className="w-full p-3 sm:p-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]"
                  />
                </div>
                <div className="grid grid-cols-2 gap-2">
                  <div>
                    <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                      First Product
                    </label>
                    <input
                      type="text"
                      autoCapitalize="words"
                      value={firstProductName}
                      onChange={(e) =>
                        updateState({ firstProductName: e.target.value })
                      }
                      className="w-full p-3 sm:p-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]"
                    />
                  </div>
                  <div>
                    <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                      Price
                    </label>
                    <input
                      type="text"
                      inputMode="decimal"
                      value={firstProductPrice}
                      onChange={(e) => {
                        updateState({ firstProductPrice: e.target.value });
                        if (
                          e.target.value.trim().length > 0 &&
                          !/^\d+(\.\d{1,2})?$/.test(e.target.value)
                        ) {
                          setValidationErrors((prev) => ({
                            ...prev,
                            firstProductPrice: "Invalid price.",
                          }));
                        } else {
                          setValidationErrors((prev) => {
                            const { firstProductPrice, ...rest } = prev;
                            return rest;
                          });
                        }
                      }}
                      className={`w-full p-3 sm:p-4 border ${validationErrors.firstProductPrice ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                    />
                    {validationErrors.firstProductPrice && (
                      <p className="text-[#FF3B30] text-xs mt-1">
                        {validationErrors.firstProductPrice}
                      </p>
                    )}
                  </div>
                </div>
              </div>

              {validationError && (
                <p className="text-[#FF3B30] text-sm font-semibold mb-2">
                  {validationError}
                </p>
              )}
              <div className="mt-auto pt-6">
                <button
                  onClick={() => {
                    let hasError = false;
                    const newErrors: Record<string, string> = {
                      ...validationErrors,
                    };
                    if (String(businessName || "").trim().length < 3) {
                      newErrors.businessName = "Must be at least 3 characters.";
                      hasError = true;
                    }
                    if (String(businessType || "").trim().length === 0) {
                      newErrors.businessType =
                        "Business Type is required to configure your agents.";
                      hasError = true;
                    }
                    if (String(firstProductPrice || "").trim().length === 0) {
                      newErrors.firstProductPrice =
                        "A price is needed to set up your Stripe catalog.";
                      hasError = true;
                    }

                    if (hasError || Object.keys(newErrors).length > 0) {
                      setValidationErrors(newErrors);
                      setValidationError(
                        "Please fix the errors before continuing.",
                      );
                      return;
                    }

                    setValidationError("");
                    updateState({ step: 3 });
                    syncStateToBackend({ step: 3 });
                  }}
                  className="w-full bg-[#0066FF] text-white p-4 font-bold min-h-[44px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
                >
                  <IconLabel icon="next">Continue</IconLabel>
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button
                onClick={() => {
                  updateState({ step: 2 });
                  syncStateToBackend({ step: 2 });
                }}
                className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2"
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 19l-7-7 7-7"
                  />
                </svg>{" "}
                Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Style & Team
              </h2>
              <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
                  Pick your storefront vibe. We'll automatically assign the best
                  AI agents to manage it.
                </p>
                <button
                  onClick={() => handleSaveDraft()}
                  className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
                >
                  <IconLabel icon="save">Save Draft</IconLabel>
                </button>
              </div>

              {saveMessage && (
                <p className="text-[#34C759] text-sm font-semibold mb-2">
                  {saveMessage}
                </p>
              )}

              <div className="space-y-4 flex-1 overflow-y-auto pr-2 hide-scrollbar">
                <div>
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Website Template
                  </label>
                  <div className="grid grid-cols-2 gap-3">
                    {["Modern", "Minimal", "Bold", "Classic"].map(
                      (template) => (
                        <div
                          key={template}
                          onClick={() =>
                            updateState({ websiteTemplate: template })
                          }
                          className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[16px] ${websiteTemplate === template ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] glass-control hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white"}`}
                        >
                          <div className="font-semibold text-sm">
                            {template}
                          </div>
                        </div>
                      ),
                    )}
                  </div>
                </div>

                <div className="pt-2 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Web Address
                  </label>
                  <div className="grid grid-cols-2 gap-3 mb-2">
                    <div
                      onClick={() => updateState({ domainChoice: "subdomain" })}
                      className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[16px] flex flex-col items-center justify-center text-center ${domainChoice === "subdomain" ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] glass-control text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500"}`}
                    >
                      <span className="font-semibold text-sm mb-1">
                        Free Subdomain
                      </span>
                      <span className="text-[10px] opacity-70">
                        your-name.ohc.app
                      </span>
                    </div>
                    <div
                      onClick={() => updateState({ domainChoice: "custom" })}
                      className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[16px] flex flex-col items-center justify-center text-center ${domainChoice === "custom" ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] glass-control text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500"}`}
                    >
                      <span className="font-semibold text-sm mb-1">
                        Custom Domain
                      </span>
                      <span className="text-[10px] opacity-70">
                        your-name.com
                      </span>
                    </div>
                  </div>
                </div>

                <div className="pt-2 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Account Setup
                  </label>
                  <div className="space-y-3 mb-4">
                    <div>
                      <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] mb-1">
                        Admin Name
                      </label>
                      <input
                        type="text"
                        autoCapitalize="words"
                        autoComplete="name"
                        value={adminName}
                        onChange={(e) => {
                          const val = e.target.value;
                          updateState({ adminName: val });
                          if (!val.trim()) {
                            setValidationErrors((prev) => ({
                              ...prev,
                              adminName: "Admin Name is required",
                            }));
                          } else {
                            setValidationErrors((prev) => {
                              const { adminName, ...rest } = prev;
                              return rest;
                            });
                          }
                        }}
                        placeholder="e.g. Maya Smith"
                        className={`w-full p-3 sm:p-4 border ${validationErrors.adminName ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                        inputMode="text"
                        enterKeyHint="next"
                      />
                      {validationErrors.adminName && (
                        <p className="text-[#FF3B30] text-xs mt-1">
                          {validationErrors.adminName}
                        </p>
                      )}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] mb-1">
                        Admin Email
                      </label>
                      <input
                        type="email"
                        autoCapitalize="none"
                        autoComplete="email"
                        inputMode="email"
                        enterKeyHint="next"
                        value={adminEmail}
                        onChange={(e) => {
                          const val = e.target.value;
                          updateState({ adminEmail: val });
                          if (!val.trim()) {
                            setValidationErrors((prev) => ({
                              ...prev,
                              adminEmail: "Admin Email is required",
                            }));
                          } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val)) {
                            setValidationErrors((prev) => ({
                              ...prev,
                              adminEmail: "Please enter a valid email address",
                            }));
                          } else {
                            setValidationErrors((prev) => {
                              const { adminEmail, ...rest } = prev;
                              return rest;
                            });
                          }
                        }}
                        placeholder="you@example.com"
                        className={`w-full p-3 sm:p-4 border ${validationErrors.adminEmail ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                      />
                      {validationErrors.adminEmail && (
                        <p className="text-[#FF3B30] text-xs mt-1">
                          {validationErrors.adminEmail}
                        </p>
                      )}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] mb-1">
                        Admin Password
                      </label>
                      <input
                        type="password"
                        enterKeyHint="done"
                        autoComplete="new-password"
                        value={adminPassword}
                        onChange={(e) => {
                          const val = e.target.value;
                          updateState({ adminPassword: val });
                          if (!val.trim()) {
                            setValidationErrors((prev) => ({
                              ...prev,
                              adminPassword: "Password is required",
                            }));
                          } else if (val.length < 8 || !/\d/.test(val)) {
                            setValidationErrors((prev) => ({
                              ...prev,
                              adminPassword:
                                "Password must be at least 8 characters and contain a number",
                            }));
                          } else {
                            setValidationErrors((prev) => {
                              const { adminPassword, ...rest } = prev;
                              return rest;
                            });
                          }
                        }}
                        placeholder="••••••••"
                        className={`w-full p-3 sm:p-4 border ${validationErrors.adminPassword ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                      />
                      {validationErrors.adminPassword && (
                        <p className="text-[#FF3B30] text-xs mt-1">
                          {validationErrors.adminPassword}
                        </p>
                      )}
                    </div>
                  </div>
                </div>

                <div className="pt-2 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Auto-Configured AI Departments
                  </label>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-xs mb-2">
                    Here are the AI departments we've configured for you.
                  </p>

                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 mt-2">
                    {AVAILABLE_AGENTS.map((agent) => {
                      const isActive = aiAgents.includes(agent.id);
                      return (
                        <div
                          key={agent.id}
                          onClick={() => handleAgentToggle(agent.id)}
                          className={`cursor-pointer p-3 flex items-start gap-3 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]  border ${isActive ? "border-[#0066FF] bg-[#0066FF]/5 dark:bg-[#0066FF]/10 shadow-[0_2px_8px_rgba(0,102,255,0.15)]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] glass-control rounded-[8px] hover:border-gray-400 dark:hover:border-gray-500"}`}
                        >
                          <div
                            className={`flex items-center justify-center w-10 h-10 rounded-full text-lg ${isActive ? "bg-[#0066FF]/20" : "bg-[rgba(255,255,255,0.2)] dark:bg-[rgba(255,255,255,0.05)]"}`}
                          >
                            {agent.icon}
                          </div>
                          <div className="flex-1 min-w-0">
                            <div className="flex justify-between items-center mb-1">
                              <p
                                className={`text-sm font-bold truncate ${isActive ? "text-[#0066FF]" : "text-[#1D1D1F] dark:text-[#F5F5F7]"}`}
                              >
                                {agent.name}
                              </p>
                              <div
                                className={`w-4 h-4 rounded-full border flex items-center justify-center transition-colors ${isActive ? "bg-[#0066FF] border-[#0066FF]" : "border-gray-300 dark:border-gray-600"}`}
                              >
                                {isActive && (
                                  <svg
                                    className="w-2.5 h-2.5 text-white"
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
                                )}
                              </div>
                            </div>
                            <p className="text-xs text-gray-500 dark:text-[#A1A1A6] leading-tight">
                              {agent.desc}
                            </p>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </div>

                <div className="pt-2">
                  <label className="flex items-center justify-between cursor-pointer p-3 glass-control rounded-[8px] text-[#1D1D1F] dark:text-white">
                    <span className="font-semibold text-sm">
                      Allow AI to Auto-Respond
                    </span>
                    <input
                      type="checkbox"
                      className="glass-control sr-only"
                      checked={aiAutoRespond}
                      onChange={(e) =>
                        updateState({ aiAutoRespond: e.target.checked })
                      }
                    />
                    <div
                      className={`w-10 h-6 rounded-full transition-colors ${aiAutoRespond ? "bg-[#34C759]" : "bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(255,255,255,0.1)]"} relative`}
                    >
                      <div
                        className={`w-4 h-4 rounded-full bg-white absolute top-1 transition-transform ${aiAutoRespond ? "translate-x-5" : "translate-x-1"}`}
                      ></div>
                    </div>
                  </label>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => handleStartOnboarding()}
                  disabled={isLoading}
                className="w-full bg-[#0066FF] text-white p-4 min-h-[44px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
                >
                  {isLoading ? (
                    <span className="flex items-center justify-center gap-2">
                      <svg
                        className="animate-spin h-5 w-5 text-white"
                        fill="none"
                        viewBox="0 0 24 24"
                      >
                        <circle
                          className="opacity-25"
                          cx="12"
                          cy="12"
                          r="10"
                          stroke="currentColor"
                          strokeWidth="4"
                        ></circle>
                        <path
                          className="opacity-75"
                          fill="currentColor"
                          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                        ></path>
                      </svg>
                      Launching...
                    </span>
                  ) : (
                    <IconLabel icon="launch">Approve & Publish</IconLabel>
                  )}
                </button>
              </div>
            </div>
          )}

          {step === 4 && (
            <div
              aria-live="polite"
              className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in glass-card  shadow-2xl p-4 sm:p-8"
            >
              <div className="w-24 h-24 relative mb-8">
                <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
              </div>
              <h2
                id="loading-title"
                className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4"
              >
                Building Your Business...
              </h2>

              <div className="w-full max-w-xs h-2 bg-[rgba(255,255,255,0.2)] dark:bg-[rgba(255,255,255,0.1)] rounded-full overflow-hidden mb-6">
                <div
                  className="h-full bg-[#0066FF] transition-all duration-300"
                  style={{ width: `${loadingProgress}%` }}
                ></div>
              </div>

              <div className="space-y-3 w-full max-w-xs text-left">
                <div className="flex items-center gap-3">
                  <svg
                    className={`w-5 h-5 transition-colors ${loadingProgress > 25 ? "text-[#34C759]" : "text-[rgba(255,255,255,0.4)] dark:text-[rgba(255,255,255,0.2)]"}`}
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
                  <span
                    className={`text-sm ${loadingProgress > 25 ? "text-[#1D1D1F] dark:text-[#F5F5F7] font-semibold" : "text-gray-500"}`}
                  >
                    Generating your product catalog
                  </span>
                </div>
                <div className="flex items-center gap-3">
                  <svg
                    className={`w-5 h-5 transition-colors ${loadingProgress > 50 ? "text-[#34C759]" : "text-[rgba(255,255,255,0.4)] dark:text-[rgba(255,255,255,0.2)]"}`}
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
                  <span
                    className={`text-sm ${loadingProgress > 50 ? "text-[#1D1D1F] dark:text-[#F5F5F7] font-semibold" : "text-gray-500"}`}
                  >
                    Configuring payment settings
                  </span>
                </div>
                <div className="flex items-center gap-3">
                  <svg
                    className={`w-5 h-5 transition-colors ${loadingProgress > 75 ? "text-[#34C759]" : "text-[rgba(255,255,255,0.4)] dark:text-[rgba(255,255,255,0.2)]"}`}
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
                  <span
                    className={`text-sm ${loadingProgress > 75 ? "text-[#1D1D1F] dark:text-[#F5F5F7] font-semibold" : "text-gray-500"}`}
                  >
                    Designing your storefront
                  </span>
                </div>
                <div className="flex items-center gap-3">
                  <svg
                    className={`w-5 h-5 transition-colors ${loadingProgress > 90 ? "text-[#34C759]" : "text-[rgba(255,255,255,0.4)] dark:text-[rgba(255,255,255,0.2)]"}`}
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
                  <span
                    className={`text-sm ${loadingProgress > 90 ? "text-[#1D1D1F] dark:text-[#F5F5F7] font-semibold" : "text-gray-500"}`}
                  >
                    Onboarding your AI agents
                  </span>
                </div>
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
                <div className="p-3 glass-card  flex flex-col items-center mb-6">
                  <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">
                    Your Shareable Link
                  </p>
                  <div className="flex items-center gap-2">
                    <span className="text-[#0066FF] font-semibold">
                      {domainChoice === "subdomain" ? generateSubdomain(businessName) : "Custom Domain Configured"}
                    </span>
                  </div>
                </div>

                <a
                  href="/assistant"
                  className="flex w-full items-center justify-center glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-bold shadow-md hover:border-gray-400 dark:hover:border-gray-500 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="sparkles">Open Assistant</IconLabel>
                </a>
                <a
                  href="/builder"
                  className="flex w-full items-center justify-center glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-bold shadow-sm active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="eye">Preview Storefront</IconLabel>
                </a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
