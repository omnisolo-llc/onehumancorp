"use client";


import { errorMessage } from '@/lib/errors';
import { useEffect,useState,useRef } from "react";
import { useRouter } from "next/navigation";
import { sendOnboardingDraft } from './draftWrites';
import { onboardingDraftWriteProblem } from './draftWriteGate';
import { useOnboardingStore, initializeOnboardingDraft, onboardingStorageFailed, onboardingDraftPending, markOnboardingDraftFromServer, subscribeOnboardingPersistence } from "./store";
import { fetchForOnboardingOwner, hasHeldOnboardingDraft, readOwnedOnboardingItem, writeOwnedOnboardingItem, subscribeOnboardingInvalidation, onboardingOwner, captureOnboardingRestoreSnapshot, assertOnboardingRestoreSnapshot, type DraftOwner } from "./draftSession";
import { canonicalRequest, normalizeReviewedProducts, observedWebsite, readDraftAcknowledgement, readLaunchResult, readPreparation, readPreparedResult, resultForPreparation, type Preparation } from "./contracts";
import { SetupIcon } from "./components/SetupIcon";
import { IconLabel } from "./components/IconLabel";


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
    aiAgents,
    aiAutoRespond,
    isLoading,
    error,
    startResult,
    instantImageUrl,
    skipped,
    updateState,
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const [viewOwner, setViewOwner] = useState<DraftOwner | null>(null);
  const [identityError, setIdentityError] = useState('');
  const [heldDraft, setHeldDraft] = useState(false);
  const [draftPending, setDraftPending] = useState(false);
  const [draftWriteProblem, setDraftWriteProblem] = useState<string | null>(null);
  const [manualInput, setManualInput] = useState<string | null>(null);
  useEffect(() => subscribeOnboardingPersistence(() => { setDraftPending(onboardingDraftPending()); setDraftWriteProblem(onboardingDraftWriteProblem(onboardingOwner())); }), []);
  const initialStateLoaded = useRef(false);
  const [chatMessages, setChatMessages] = useState<
    { role: string; content: string; image_url?: string }[]
  >([]);
  const [chatInput, setChatInput] = useState("");
  const [chatImageUrl, setChatImageUrl] = useState("");
  const chatMessagesEndRef = useRef<HTMLDivElement>(null);
  const operationEpoch = useRef(0);
  const operationPending = useRef(false);
  const draftSavePending = useRef(false);
  const prepared = useRef<Preparation | null>(null);
  const preparedDraft = useRef<string | null>(null);
  const needsRecovery = useRef(false);
  useEffect(() => () => { operationEpoch.current += 1; operationPending.current = false; draftSavePending.current = false; updateState({ isLoading: false }); }, []);
  const cancelOperation = () => { if (operationPending.current) needsRecovery.current = true; operationEpoch.current += 1; operationPending.current = false; draftSavePending.current = false; updateState({ isLoading: false }); };


  useEffect(() => {
    chatMessagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [chatMessages]);

  const fetchWithRetry = async (
    url: string,
    options: RequestInit,
    retries = 3,
    backoff = process.env.NODE_ENV === "test" ? 10 : 500,
  ) => {
    const method = (options.method ?? 'GET').toUpperCase();
    const safeReplay = method === 'GET' || method === 'HEAD';
    const attempts = safeReplay ? retries : 1;
    const isDraftWrite = method === 'POST' && ['/api/v1/onboarding/state', '/api/v1/onboarding/draft'].includes(url);
    for (let i = 0; i < attempts; i++) {
      try {
        const response = await (isDraftWrite ? sendOnboardingDraft(url, options, viewOwner) : fetchForOnboardingOwner(url, options, viewOwner));
        if (!response.ok) {
          let errMsg = `HTTP error! status: ${response.status}`;
          let code: string | undefined;
          try {
            const result = await response.clone().json();
            errMsg = result.error || result.message || errMsg;
            code = typeof result.error === 'string' ? result.error : undefined;
          } catch  { /* Optional local state or response decoding failed; retain the existing fallback. */ }
          throw Object.assign(new Error(errMsg), { status: response.status, code });
        }
        return response;
      } catch (err) {
        if (i === attempts - 1 || options.signal?.aborted) throw err;
        await new Promise((res) => setTimeout(res, backoff * Math.pow(2, i)));
      }
    }
    throw new Error("Max retries reached");
  };

  const syncStateToBackend = async (overrideState: Partial<import("./store").OnboardingState> & { skipped?: boolean } = {}) => {
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
      aiAgents,
      aiAutoRespond,
      instantImageUrl,
      skipped,
      ...overrideState,
    };

    try {
      await fetchWithRetry("/api/v1/onboarding/state", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(wizardState),
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
    cancelOperation();
    updateState({ error: "" });
    setValidationError("");
    localStorage.setItem("has_onboarded", "true");
    syncStateToBackend({ skipped: true });
    router.push("/dashboard");
  };

  const handleBackToIntro = () => {
    cancelOperation();
    updateState({ error: "" });
    setValidationError("");
    setValidationErrors({});
    updateState({ step: -2 });
    syncStateToBackend({ step: -2 });
  };

  const handleSaveDraft = async () => {
    if (draftSavePending.current || operationPending.current) return;
    draftSavePending.current = true;
    const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current;
    updateState({ isLoading: true });
    updateState({ error: "" });

    try {
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
        aiAgents,
        aiAutoRespond,
        instantImageUrl,
        skipped,
      };

      const response = await fetchWithRetry("/api/v1/onboarding/draft", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ step, ...wizardState }),
      });

      await readDraftAcknowledgement(response);
      if (!active()) return;
      setSaveMessage(onboardingDraftPending() ? "Earlier draft saved; local edits are pending save." : "Draft Saved!");
      setTimeout(() => { if (active()) setSaveMessage(""); }, 3000);
    } catch (err) {
      if (!active()) return;
      console.error(err);
      updateState({ error: errorMessage(err, '') || "An error occurred saving draft" });
    } finally {
      if (active()) { draftSavePending.current = false; updateState({ isLoading: false }); }
    }
  };

  const adoptPreparation = (receipt: Preparation, editableDraft?: Record<string, unknown>) => {
    const request = receipt.reviewed_request;
    const primary = receipt.catalog.find(product => product.product_id === receipt.primary_product_id)!;
    const fields: Record<string, string> = { business_type: 'businessType', company_name: 'businessName', company_description: 'businessDescription', selling_categories: 'categories', website_template: 'websiteTemplate', domain_choice: 'domainChoice', location: 'location', target_audience: 'targetAudience', ai_agents: 'aiAgents', ai_auto_respond: 'aiAutoRespond' };
    const updates: Record<string, unknown> = {};
    for (const [key, target] of Object.entries(fields)) if (request[key] !== undefined) updates[target] = request[key];
    writeOwnedOnboardingItem('products', JSON.stringify(receipt.catalog));
    updateState({ ...updates, firstProductName: primary.name, firstProductPrice: primary.price, startResult: resultForPreparation(receipt), step: receipt.status === 'launched' ? 5 : 3, isLoading: false });
    prepared.current = receipt;
    preparedDraft.current = canonicalRequest(draftRequest());
    if (receipt.status === 'prepared' && editableDraft) {
      const editable: Record<string, unknown> = {};
      for (const key of [...Object.values(fields), 'firstProductName', 'firstProductPrice', 'whatYouSell', 'bio', 'businessGoal', 'instantImageUrl']) {
        if (editableDraft[key] !== undefined) editable[key] = editableDraft[key];
      }
      updateState(editable);
    }
  };

  // Only the server's protected receipt can restore completion after reload.
  useEffect(() => {
    let cancelled = false;
    let loadVersion = 0;
    const load = async () => {
      const version = ++loadVersion;
      const active = () => !cancelled && version === loadVersion;
      setIsLoaded(false); setIdentityError('');
      try {
        const owner = await initializeOnboardingDraft();
        if (!active()) return;
        setViewOwner(owner); setHeldDraft(hasHeldOnboardingDraft());
        const restoreSnapshot = captureOnboardingRestoreSnapshot();
        const pendingLocal = onboardingDraftPending();
        const localSnapshot = pendingLocal ? { ...useOnboardingStore.getState() } : undefined;
        const localProducts = pendingLocal ? readOwnedOnboardingItem('products') : null;
        setDraftPending(pendingLocal); setDraftWriteProblem(onboardingDraftWriteProblem(owner));
        const values = await Promise.all([
      fetchForOnboardingOwner("/api/v1/onboarding/draft", {}, owner)
        .then((res) => (res.ok ? res.json() : null))
        .catch(() => null),
      fetchForOnboardingOwner("/api/v1/onboarding/state", {}, owner)
        .then((res) => (res.ok ? res.json() : null))
        .catch(() => null),
        ]);
        if (!active()) return;
        assertOnboardingRestoreSnapshot(restoreSnapshot);
        const [draftData, stateData] = values;
        const isValid = (d: unknown) => typeof d === 'object' && d !== null && Object.keys(d).length > 0;
        let data = pendingLocal ? null : isValid(draftData) ? draftData : stateData;
        if (isValid(data)) {
          if (data.wizardState) data = data.wizardState;
          if (data.step !== undefined)
            updateState({ step: data.step === 4 || data.step === 5 ? 3 : data.step });
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
          if (data.domainChoice !== undefined)
            updateState({ domainChoice: data.domainChoice });
          if (data.aiAgents !== undefined)
            updateState({ aiAgents: data.aiAgents });
          if (data.aiAutoRespond !== undefined)
            updateState({ aiAutoRespond: data.aiAutoRespond });
          if (data.instantImageUrl !== undefined)
            updateState({ instantImageUrl: data.instantImageUrl });
          if (data.skipped !== undefined)
            updateState({ skipped: data.skipped });
          initialStateLoaded.current = true;
        }
        if (stateData?.preparation) {
          adoptPreparation(readPreparation(stateData.preparation), pendingLocal ? localSnapshot : isValid(draftData) ? (draftData.wizardState || draftData) : undefined);
          if (pendingLocal && localProducts !== null) writeOwnedOnboardingItem('products', localProducts);
        }
        else if (useOnboardingStore.getState().step >= 4) updateState({ step: 3, startResult: null, isLoading: false });
        if (!pendingLocal) markOnboardingDraftFromServer(restoreSnapshot.writeVersion);
        initialStateLoaded.current = true; setIsLoaded(true);
      } catch (cause) {
        if (active()) { setViewOwner(null); setIdentityError(errorMessage(cause, 'Sign in to restore your setup draft.')); }
      }
    };
    const unsubscribe = subscribeOnboardingInvalidation(restart => {
      loadVersion += 1; operationEpoch.current += 1; operationPending.current = false; draftSavePending.current = false;
      prepared.current = null; preparedDraft.current = null; needsRecovery.current = false;
      setChatMessages([]); setChatInput(''); setChatImageUrl(''); setManualInput(null); setSaveMessage(''); setValidationError(''); setValidationErrors({}); setDraftPending(false); setDraftWriteProblem(null);
      initialStateLoaded.current = false; setViewOwner(null); setIsLoaded(false);
      if (restart) void load(); else setIdentityError('Your session could not be verified. Sign in again to continue.');
    });
    void load();
    return () => { cancelled = true; loadVersion += 1; unsubscribe(); };
  }, []);

  useEffect(() => {
    if (isLoaded && skipped) {
      router.push("/dashboard");
    }
  }, [isLoaded, skipped, router]);

  // Sync state to backend
  useEffect(() => {
    if (!isLoaded || !initialStateLoaded.current || !onboardingDraftPending()) return;

    // Only save if we are past the initial state
    if (
      step === 1 &&
      !businessName &&
      !whatYouSell &&
      !location &&
      !targetAudience
    )
      return;

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
      aiAgents,
      aiAutoRespond,
      instantImageUrl,
      skipped,
    };

    const timer = setTimeout(() => {
      fetchWithRetry("/api/v1/onboarding/state", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
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
    aiAgents,
    aiAutoRespond,
    isLoaded,
    draftPending,
    instantImageUrl,
    skipped,
  ]);

  const offerManualReview = (cause: unknown, input: string): boolean => {
    if (!cause || typeof cause !== 'object' || !('status' in cause) || cause.status !== 503
      || !('code' in cause) || cause.code !== 'onboarding_ai_unconfigured') return false;
    setManualInput(input);
    updateState({ error: 'AI-assisted setup is unavailable because no model provider is configured. Review and enter your business details manually.' });
    return true;
  };
  const continueManually = () => {
    if (manualInput === null || !viewOwner || operationPending.current || draftSavePending.current) return;
    updateState({ step: 2, bio: manualInput, businessDescription: businessDescription || whatYouSell || manualInput, error: '', isLoading: false });
    setManualInput(null);
  };

  const handleIntake = async () => {
    if (operationPending.current || draftSavePending.current) return;
    operationPending.current = true;
    setManualInput(null);
    const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current;
    updateState({ isLoading: true });
    updateState({ error: "" });

    try {
      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}\nTarget Audience: ${targetAudience}`;
      updateState({ bio: combinedDescription });

      const intakeRes = await fetchWithRetry("/api/v1/onboarding/intake", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ description: combinedDescription }),
      });

      const intakeData = await intakeRes.json();
      if (!active()) return;
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
        writeOwnedOnboardingItem(
          "products",
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
    } catch (err) {
      if (!active()) return;
      if (offerManualReview(err, whatYouSell || bio)) return;
      console.error(err);
      updateState({
        error: errorMessage(err, '') || "Backend connection failed. Please try again.",
      });
      updateState({ step: 1 });
      syncStateToBackend({ step: 1 });
      updateState({ chatStep: 3 });
      syncStateToBackend({ chatStep: 3 });
    } finally {
      if (active()) { operationPending.current = false; updateState({ isLoading: false }); }
    }
  };

  const handleSendChatMessage = async () => {
    if ((!chatInput.trim() && !chatImageUrl.trim()) || operationPending.current || draftSavePending.current) return;
    operationPending.current = true;
    setManualInput(null);
    const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current;
    const newHistory = [...chatMessages, { role: 'user', content: chatInput, image_url: chatImageUrl || undefined }];
    setChatMessages(newHistory); setChatInput(''); setChatImageUrl('');
    updateState({ isLoading: true, error: '' });
    try {
      const res = await fetchWithRetry('/api/v1/onboarding/chat', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ messages: newHistory }) });
      if (!res.ok) throw new Error('Chat request failed');
      const data = await res.json();
      if (!active()) return;
      if (typeof data.reply !== 'string') throw new Error('The setup reply is incomplete');
      setChatMessages([...newHistory, { role: 'assistant', content: data.reply }]);
      if (data.is_complete) {
        const intake = data.intake_data;
        if (!intake?.business_name || !Array.isArray(intake.initial_products) || !intake.initial_products.length) throw new Error('The setup draft is incomplete. Please add your business and product details.');
        writeOwnedOnboardingItem('products', JSON.stringify(intake.initial_products));
        updateState({ step: 2, businessName: intake.business_name, businessType: intake.business_type || 'Online Store', businessDescription: newHistory.map(message => message.content).join(' '), categories: intake.categories || [], firstProductName: intake.initial_products[0].name || '', firstProductPrice: String(intake.initial_products[0].price ?? ''), location: intake.location || '', targetAudience: intake.target_audience || '' });
      }
    } catch (cause) {
      if (active() && !offerManualReview(cause, newHistory.filter(message => message.role === 'user').map(message => message.content).join('\n'))) updateState({ error: errorMessage(cause, 'Failed to send chat message') });
    } finally {
      if (active()) { operationPending.current = false; updateState({ isLoading: false }); }
    }
  };

  const handleInstantBuild = async () => {
    if (operationPending.current || draftSavePending.current) return;
    if (!bio.trim()) { updateState({ error: 'Please tell us about your business.' }); return; }
    operationPending.current = true;
    setManualInput(null);
    const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current;
    updateState({ isLoading: true, error: '', step: 4 });
    try {
      if (needsRecovery.current) {
        const state = await fetchForOnboardingOwner('/api/v1/onboarding/state', {}, viewOwner);
        if (!state.ok) throw new Error('Could not check the previous setup. Please reload before retrying.');
        const value = await state.json();
        if (!active()) return;
        if (value.preparation) { adoptPreparation(readPreparation(value.preparation)); needsRecovery.current = false; return; }
        needsRecovery.current = false;
      }
      const response = await fetchWithRetry('/api/v1/onboarding/start_zero_click', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ prompt: bio, image_url: instantImageUrl || undefined }) });
      if (!response.ok) throw new Error('Setup preparation failed. Check its status before retrying.');
      const result = readPreparedResult(await response.json());
      if (!active()) return;
      adoptPreparation(result.preparation);
    } catch (cause) {
      if (active()) {
        if (offerManualReview(cause, bio)) { needsRecovery.current = false; updateState({ step: -1 }); }
        else { needsRecovery.current = true; updateState({ step: -1, error: errorMessage(cause, 'Setup could not be confirmed. Check its status before retrying.') }); }
      }
    } finally {
      if (active()) { operationPending.current = false; updateState({ isLoading: false }); }
    }
  };

  const draftRequest = () => {
    const { businessType, businessName, businessDescription, whatYouSell, categories, websiteTemplate, firstProductName, firstProductPrice, domainChoice, location, targetAudience, aiAgents, aiAutoRespond } = useOnboardingStore.getState();
    const initial: unknown = JSON.parse(readOwnedOnboardingItem('products') || '[]');
    const initialProducts = normalizeReviewedProducts(initial).map((product, index) =>
      (prepared.current ? product.product_id === prepared.current.primary_product_id : index === 0) && firstProductName
        ? { ...product, name: firstProductName, price: firstProductPrice } : product);
    return {
      business_type: businessType, company_name: businessName, company_description: businessDescription || whatYouSell,
      selling_categories: categories, payment_pref: 'online', website_template: websiteTemplate,
      first_product_name: firstProductName, first_product_price: firstProductPrice, domain_choice: domainChoice || 'subdomain',
      price_type: 'fixed', location: location || '', target_audience: targetAudience || '', ai_agents: aiAgents,
      ai_auto_respond: aiAutoRespond, initial_products: initialProducts,
    };
  };

  const handleStartOnboarding = async () => {
    if (operationPending.current || draftSavePending.current) return;
    operationPending.current = true;
    const epoch = ++operationEpoch.current;
    const active = () => epoch === operationEpoch.current;
    setValidationErrors({});
    updateState({ isLoading: true, error: '', step: 4 });
    try {
      if (needsRecovery.current) {
        const state = await fetchForOnboardingOwner('/api/v1/onboarding/state', {}, viewOwner);
        if (!state.ok) throw new Error('Could not check the previous setup. Please reload before retrying.');
        const value = await state.json();
        if (!active()) return;
        if (value.preparation) {
          const recovered = readPreparation(value.preparation);
          if (prepared.current && (recovered.preparation_id !== prepared.current.preparation_id || recovered.organization_id !== prepared.current.organization_id || recovered.user_id !== prepared.current.user_id)) throw new Error('Setup changed. Reload to review its current state.');
          if (!prepared.current) { adoptPreparation(recovered); needsRecovery.current = false; return; }
          prepared.current = recovered;
          if (recovered.status === 'launched') { adoptPreparation(recovered); needsRecovery.current = false; return; }
        }
        needsRecovery.current = false;
      }
      const request = draftRequest();
      const signature = canonicalRequest(request);
      let receipt = prepared.current;
      if (!receipt || preparedDraft.current !== signature) {
        const payload = receipt ? {
          ...request, replaces_preparation_id: receipt.preparation_id,
          initial_products: receipt.catalog.map(product => ({
            ...product,
            ...(product.product_id === receipt!.primary_product_id ? { name: firstProductName, price: firstProductPrice } : {}),
            variants: product.variants.map(variant => ({ ...variant })),
          })),
        } : request;
        const response = await fetchWithRetry('/api/v1/onboarding/start', {
          method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload),
        });
        if (!active()) return;
        if (!response.ok) throw new Error('Setup preparation was rejected');
        const result = readPreparedResult(await response.json(), receipt);
        if (!active()) return;
        receipt = result.preparation;
        prepared.current = receipt; preparedDraft.current = signature;
        updateState({ startResult: result });
      }
      if (!active()) return;
      const response = await fetchWithRetry('/api/v1/onboarding/launch', {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ preparation_id: receipt.preparation_id }),
      });
      if (!active()) return;
      if (!response.ok) throw new Error('Setup completion could not be confirmed');
      const launched = readLaunchResult(await response.json(), receipt);
      if (!active()) return;
      prepared.current = launched;
      updateState({ startResult: resultForPreparation(launched), step: 5 });
      localStorage.setItem('has_onboarded', 'true');
      void syncStateToBackend({ step: 5 });
    } catch (cause) {
      if (active()) { needsRecovery.current = true; updateState({ error: errorMessage(cause, 'Setup could not be confirmed. Check its status before retrying.'), step: 3 }); }
    } finally {
      if (active()) { operationPending.current = false; updateState({ isLoading: false }); }
    }
  };

  if (identityError) return <div role="alert">{identityError} Your saved drafts remain held on this device.</div>;
  if (!isLoaded || !viewOwner) {
    return (
      <div
        role="status"
        aria-label="Loading onboarding"
        className="flex min-h-[50vh] items-center justify-center px-6"
      >
        <div className="flex items-center gap-3 rounded-[16px] border border-slate-200 bg-white px-5 py-4 text-slate-700 shadow-sm dark:border-white/10 dark:bg-white/5 dark:text-slate-200">
          <span
            aria-hidden="true"
            className="h-5 w-5 animate-spin rounded-full border-2 border-slate-300 border-t-[#0f766e]"
          />
          <span className="text-sm font-medium">Preparing your setup…</span>
        </div>
      </div>
    );
  }

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
        data-voice-assistant-surface="glass"
        className="w-full max-w-[375px] sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[100dvh] sm:min-h-[812px] relative border-0 sm:border shadow-none sm:shadow-[0_18px_44px_rgba(15,23,42,0.12)] translucent-glass-light dark:translucent-glass-dark"
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
                Your business workspace, ready to review.
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

        {error && manualInput === null && (
          <div className="absolute top-4 left-4 right-4 z-[9999] border border-[#FF3B30]/50 text-[#FF3B30] p-3 rounded-[8px] text-sm font-semibold shadow-lg flex items-center gap-2 animate-shake glass-control">
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

        {manualInput !== null && <section className="mx-6 mt-4 rounded-lg border p-4" aria-label="Manual setup available">
          <p role="status">{error}</p>
          <button type="button" className="app-button mt-3" disabled={isLoading} onClick={continueManually}>Review Details Manually</button>
        </section>}

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
              <div className="text-xs font-semibold uppercase tracking-wider text-blue-600 mb-1">
                Welcome
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
                  aria-label="Start My Business"
                  className="w-full bg-[#0066FF] text-white p-4 font-bold min-h-[44px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px] min-h-[44px]"
                  onClick={() => {
                    updateState({ step: 1 });
                    syncStateToBackend({ step: 1 });
                  }}
                >
                  Start My Business
                  <span className="sr-only">Start Onboarding</span>
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
            <div data-voice-assistant-surface="glass" className="flex flex-col flex-1 animate-fade-in w-full h-full max-h-full translucent-glass-light dark:translucent-glass-dark  p-4">
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
                  className="glass-control translucent-glass-light dark:translucent-glass-dark flex-1 overflow-y-auto p-4 text-[#1D1D1F] dark:text-[#F5F5F7] text-left space-y-4"
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
                    className="glass-control rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] border   focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 min-h-[44px]"
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
                      className="glass-control rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none flex-1 transition-all duration-[250ms] border   focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 min-h-[44px]"
                      placeholder="Type a message..."
                      enterKeyHint="send"
                    />
                    <button
                      id="chat-send-btn"
                      onClick={handleSendChatMessage}
                      disabled={isLoading || (!chatInput.trim() && !chatImageUrl.trim())}
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
                  className={`glass-control rounded-[8px] w-full p-4 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] ${error === "Please tell us about your business." || error ? "border border-[#FF3B30]" : "border   focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"}`}
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
                      type="button"
                      onClick={() => handleSaveDraft()}
                      disabled={isLoading}
                      className="setup-nav-button min-h-[44px]"
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
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]  ${validationError === "Business Name must be at least 3 characters." ? "border-[#FF3B30]" : "  focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} min-h-[44px]`}
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
                      type="button"
                      onClick={() => handleSaveDraft()}
                      disabled={isLoading}
                      className="setup-nav-button min-h-[44px]"
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
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]  ${validationError === "Please tell us what you sell." ? "border-[#FF3B30]" : "  focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 focus:ring-2 focus:ring-[#0066FF]/30"}`}
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
                      type="button"
                      onClick={() => handleSaveDraft()}
                      disabled={isLoading}
                      className="setup-nav-button min-h-[44px]"
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
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]  ${validationError === "Please tell us your location." ? "border-[#FF3B30]" : "  focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} min-h-[44px]`}
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
                      type="button"
                      onClick={() => handleSaveDraft()}
                      disabled={isLoading}
                      className="setup-nav-button min-h-[44px]"
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
                        className={`w-full p-3 sm:p-4 border outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]  ${validationError === "Please tell us your target audience." ? "border-[#FF3B30]" : "  focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} min-h-[44px]`}
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
                  Review your business details before preparing your workspace.
                </p>
                <button
                  type="button"
                      onClick={() => handleSaveDraft()}
                      disabled={isLoading}
                  className="setup-nav-button min-h-[44px]"
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
                  <label htmlFor="review-business-name" className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                    Business Name
                  </label>
                  <input
                    id="review-business-name"
                    type="text"
                    autoFocus
                    autoCapitalize="words"
                    value={businessName}
                    onChange={(e) => {
                      updateState({ businessName: e.target.value });
                      setValidationErrors((prev) => {
                        const rest = { ...prev }; delete rest.businessName;
                        return rest;
                      });
                    }}
                    className={`w-full p-3 sm:p-4 border ${validationErrors.businessName ? "border-[#FF3B30]" : "  focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                  />
                  {validationErrors.businessName && (
                    <p className="text-[#FF3B30] text-xs mt-1">
                      {validationErrors.businessName}
                    </p>
                  )}
                </div>
                <div>
                  <label htmlFor="review-business-type" className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                    Business Type
                  </label>
                  <input
                    id="review-business-type"
                    type="text"
                    autoCapitalize="words"
                    value={businessType}
                    onChange={(e) => {
                      updateState({ businessType: e.target.value });
                      setValidationErrors((prev) => {
                        const rest = { ...prev }; delete rest.businessType;
                        return rest;
                      });
                    }}
                    className={`w-full p-3 sm:p-4 border ${validationErrors.businessType ? "border-[#FF3B30]" : "  focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
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
                    className="w-full p-3 sm:p-4 border   focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]"
                  />
                </div>
                <div className="grid grid-cols-2 gap-2">
                  <div>
                    <label htmlFor="review-first-product-name" className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                      First Product
                    </label>
                    <input
                      type="text"
                      autoCapitalize="words"
                      id="review-first-product-name"
                      value={firstProductName}
                      onChange={(e) =>
                        updateState({ firstProductName: e.target.value })
                      }
                      className="w-full p-3 sm:p-4 border   focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]"
                    />
                  </div>
                  <div>
                    <label htmlFor="review-first-product-price" className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-1">
                      Price
                    </label>
                    <input
                      type="text"
                      inputMode="decimal"
                      id="review-first-product-price"
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
                            const rest = { ...prev }; delete rest.firstProductPrice;
                            return rest;
                          });
                        }
                      }}
                      className={`w-full p-3 sm:p-4 border ${validationErrors.firstProductPrice ? "border-[#FF3B30]" : "  focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"} outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
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
                        "A price is needed for your first product or service.";
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
                  Choose your storefront style and preferred assistants for review.
                </p>
                <button
                  type="button"
                      onClick={() => handleSaveDraft()}
                      disabled={isLoading}
                  className="setup-nav-button min-h-[44px]"
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
                          className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px] ${websiteTemplate === template ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]" : "  glass-control hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white"}`}
                        >
                          <div className="font-semibold text-sm">
                            {template}
                          </div>
                        </div>
                      ),
                    )}
                  </div>
                </div>

                <div className="pt-2 border-t  ">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Web Address
                  </label>
                  <div className="grid grid-cols-2 gap-3 mb-2">
                    <div
                      onClick={() => updateState({ domainChoice: "subdomain" })}
                      className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px] flex flex-col items-center justify-center text-center ${domainChoice === "subdomain" ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]" : "  glass-control text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500"}`}
                    >
                      <span className="font-semibold text-sm mb-1">
                        Free Subdomain
                      </span>
                      <span className="text-[10px] opacity-70">
                        your-name.cloud.omnisolo.co
                      </span>
                    </div>
                    <div
                      onClick={() => updateState({ domainChoice: "custom" })}
                      className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px] flex flex-col items-center justify-center text-center ${domainChoice === "custom" ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]" : "  glass-control text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500"}`}
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

                <div className="pt-2 border-t  ">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Preferred AI Departments
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
                          className={`cursor-pointer p-3 flex items-start gap-3 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]  border ${isActive ? "border-[#0066FF] bg-[#0066FF]/5 dark:bg-[#0066FF]/10 shadow-[0_2px_8px_rgba(0,102,255,0.15)]" : "glass-control rounded-[8px] hover:border-gray-400 dark:hover:border-gray-500"}`}
                        >
                          <div
                            className={`flex items-center justify-center w-10 h-10 rounded-full text-lg ${isActive ? "bg-[#0066FF]/20" : "bg-black/10 dark:bg-white/10"}`}
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
                      Completing setup...
                    </span>
                  ) : (
                    <IconLabel icon="launch">Approve & Complete Setup</IconLabel>
                  )}
                </button>
              </div>
            </div>
          )}

          {step === 4 && (
            <div
              aria-live="polite"
              data-voice-assistant-surface="glass"
              className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in translucent-glass-light dark:translucent-glass-dark  shadow-2xl p-4 sm:p-8"
            >
              <div className="w-24 h-24 relative mb-8">
                <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
              </div>
              <h2
                id="loading-title"
                className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4"
              >
                Preparing your workspace...
              </h2>

              <p role="status">Waiting for confirmed setup status. You can leave this screen and recover the saved preparation later.</p>
            </div>
          )}

          {draftPending && <p role="status">Local edits are pending save.</p>}
          {draftWriteProblem && <p role="alert">{draftWriteProblem}</p>}
          {onboardingStorageFailed() && <p role="alert">This device couldn’t save your latest edits. Your existing saved draft is unchanged.</p>}
          {heldDraft && <p role="status">An older or different-account draft is held on this device and was not loaded.</p>}
          {step === 5 && startResult && prepared.current?.status === 'launched' && (
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
                Setup complete
              </h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8 px-4">
                {startResult.message ||
                  "Your local workspace setup has been recorded. Review your storefront before sharing it."}
              </p>

              <div className="w-full space-y-3 mt-auto">
                {observedWebsite(startResult) && <div className="p-3 translucent-glass-light dark:translucent-glass-dark flex flex-col items-center mb-6">
                  <p className="text-xs text-gray-500 uppercase font-bold">Recorded storefront link</p>
                  <a href={observedWebsite(startResult)!}>{observedWebsite(startResult)}</a>
                </div>}

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
