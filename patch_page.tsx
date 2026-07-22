<<<<<<< SEARCH
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
    updateState,
  } = useOnboardingStore();
=======
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
    adminName,
    adminEmail,
    adminPassword,
    firstProductName,
    firstProductPrice,
    aiAgents,
    aiAutoRespond,
    isLoading,
    error,
    startResult,
    instantImageUrl,
    updateState,
  } = useOnboardingStore();
>>>>>>> REPLACE
<<<<<<< SEARCH
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
      ...overrideState,
    };
=======
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
      adminName,
      adminEmail,
      adminPassword,
      firstProductName,
      firstProductPrice,
      aiAgents,
      aiAutoRespond,
      instantImageUrl,
      ...overrideState,
    };
>>>>>>> REPLACE
<<<<<<< SEARCH
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
      };
=======
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
        adminName,
        adminEmail,
        adminPassword,
        firstProductName,
        firstProductPrice,
        aiAgents,
        aiAutoRespond,
        instantImageUrl,
      };
>>>>>>> REPLACE
<<<<<<< SEARCH
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
    };
=======
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
      adminName,
      adminEmail,
      adminPassword,
      firstProductName,
      firstProductPrice,
      aiAgents,
      aiAutoRespond,
      instantImageUrl,
    };
>>>>>>> REPLACE
<<<<<<< SEARCH
                <div className="pt-2 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Web Address
                  </label>
=======
                <div className="pt-2 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Admin Name
                  </label>
                  <input
                    type="text"
                    value={adminName}
                    onChange={(e) => updateState({ adminName: e.target.value })}
                    placeholder="e.g. Maya Smith"
                    className="w-full p-3 mb-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]"
                  />
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Admin Email
                  </label>
                  <input
                    type="email"
                    value={adminEmail}
                    onChange={(e) => updateState({ adminEmail: e.target.value })}
                    placeholder="you@example.com"
                    className="w-full p-3 mb-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]"
                  />
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Admin Password
                  </label>
                  <input
                    type="password"
                    value={adminPassword}
                    onChange={(e) => updateState({ adminPassword: e.target.value })}
                    placeholder="••••••••"
                    className="w-full p-3 mb-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20 outline-none glass-control rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]"
                  />
                </div>

                <div className="pt-2 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
                  <label className="block text-xs font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wide mb-2">
                    Web Address
                  </label>
>>>>>>> REPLACE
<<<<<<< SEARCH
  const handleStartOnboarding = async () => {
    updateState({ isLoading: true });
    updateState({ error: "" });
=======
  const handleStartOnboarding = async () => {
    if (!adminEmail || !/^\S+@\S+\.\S+$/.test(adminEmail)) {
      setValidationError("Please enter a valid email address");
      return;
    }
    if (!adminPassword || adminPassword.length < 8 || !/\d/.test(adminPassword)) {
      setValidationError("Password must be at least 8 characters and contain a number");
      return;
    }

    updateState({ isLoading: true });
    updateState({ error: "" });
>>>>>>> REPLACE
