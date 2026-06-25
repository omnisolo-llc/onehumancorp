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
    const userId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("user_id") || "test-user"
        : "test-user";

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
    updateState({ step: 0 });
    syncStateToBackend({ step: 0 });
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
      const userId =
        typeof localStorage !== "undefined"
          ? localStorage.getItem("user_id") || "test-user"
          : "test-user";

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
    const userId =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("user_id") || "test-user"
        : "test-user";

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
      fetch("/api/onboarding/state", {
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
      const userId =
        typeof localStorage !== "undefined"
          ? localStorage.getItem("user_id") || "test-user"
          : "test-user";

      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}\nTarget Audience: ${targetAudience}`;
      updateState({ bio: combinedDescription });

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

      updateState({ businessType: intakeData.business_type || "Online Store" });
      updateState({ businessName: intakeData.business_name || "My Business" });
      updateState({
        firstProductName:
          intakeData.initial_products?.[0]?.name || "First Product",
      });
      updateState({
        firstProductPrice: intakeData.initial_products?.[0]?.price || "10.00",
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
      updateState({ aiAgents: newAgents });

      updateState({ step: 2 });
      await syncStateToBackend({
        step: 2,
        aiAgents: newAgents,
        firstProductName:
          intakeData.initial_products?.[0]?.name || "First Product",
        firstProductPrice: intakeData.initial_products?.[0]?.price || "10.00",
      }); // Go to review step
    } catch (err: any) {
      console.error(err);
      updateState({
        error: err.message || "An error occurred processing details",
      });
      updateState({ step: 1 });
      syncStateToBackend({ step: 1 });
      updateState({ chatStep: 3 });
      syncStateToBackend({ chatStep: 3 });
    } finally {
      updateState({ isLoading: false });
    }
  };

  const handleSendChatMessage = async (overrideMessage?: string) => {
    const msgToSend = overrideMessage || chatInput;
    if (!msgToSend.trim() && !chatImageUrl.trim()) return;

    const newMessage = {
      role: "user",
      content: msgToSend,
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

      const res = await fetch(`${backendUrl}/api/onboarding/chat`, {
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
        const userId =
          typeof localStorage !== "undefined"
            ? localStorage.getItem("user_id") || "test-user"
            : "test-user";

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
          firstProductPrice: intakeData.initial_products?.[0]?.price || "10.00",
        });
        if (intakeData.initial_products) {
          localStorage.setItem(
            "onboarding_initial_products",
            JSON.stringify(intakeData.initial_products),
          );
        }

        await new Promise((resolve) => setTimeout(resolve, 500));

        const startRes = await fetchWithRetry("/api/onboarding/start", {
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
            admin_email: adminEmail || "admin@example.com",
            admin_name: adminName || "Admin",
            admin_password: adminPassword || "password123",
            website_template: websiteTemplate,
            first_product_name:
              intakeData.initial_products?.[0]?.name || "First Product",
            first_product_price:
              intakeData.initial_products?.[0]?.price || "10.00",
            domain_choice: domainChoice,
            location: intakeData.location || "",
            target_audience: intakeData.target_audience || "",
            ai_agents: [],
            ai_auto_respond: true,
            initial_products: intakeData.initial_products || [],
          }),
        });

        const result = await startRes.json().catch(() => ({}));
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
      }
    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || "Failed to send chat message" });
    } finally {
      updateState({ isLoading: false });
    }
  };
}
