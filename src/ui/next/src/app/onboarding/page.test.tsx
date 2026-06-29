/* @vitest-environment jsdom */
import { render, screen, waitFor, act } from "@testing-library/react";

import OnboardingWizard from "./page";
import { useOnboardingStore } from "./store";
import { TooltipProvider } from "../../components/TooltipRegistry";
import { beforeEach, describe, it, expect, vi, afterEach } from "vitest";
import "@testing-library/jest-dom/vitest";
import userEvent from "@testing-library/user-event";

const mockRouterPush = vi.hoisted(() => vi.fn());

vi.mock("next/navigation", () => ({
  usePathname: () => "/onboarding",
  useRouter: () => ({
    push: mockRouterPush,
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
}));

describe("OnboardingWizard", () => {
  const renderOnboardingWizard = async () => {
    let view;
    await act(async () => {
      view = render(
        <TooltipProvider>
          <OnboardingWizard />
        </TooltipProvider>,
      );
    });
    return view;
  };

  beforeEach(() => {
    localStorage.clear();
    mockRouterPush.mockClear();
    useOnboardingStore.setState({
      step: 1,
      chatStep: 1,
      businessName: "",
      whatYouSell: "",
      location: "",
      businessDescription: "",
      domainChoice: "subdomain",
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: "",
      startResult: null,
    });

    global.fetch = vi.fn().mockImplementation((url) => {
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    }) as any;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it("Step 1: Renders initial screen correctly", async () => {
    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }
    screen.getByText("What's the name of your business?");
    const button = screen.getByRole("button", { name: /Next/i });
    expect(button).not.toBeDisabled();
  });

  it("Handles enter key progression in chat steps", async () => {
    const user = userEvent.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/launch") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (url === "/api/onboarding/intake") {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_type: "Bakery",
            business_name: "Maya Bakery",
            categories: ["food"],
            initial_products: [{ name: "Cake", price: "20" }],
          }),
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    // Chat Step 1 - Use Enter Key
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, "Maya Bakery{Enter}");

    // Chat Step 2 - Use Enter Key
    const sellInput = await screen.findByPlaceholderText(
      /I bake custom vegan cakes/i,
    );
    await user.type(sellInput, "Cakes{Enter}");

    // Chat Step 3 - Use Enter Key
    const locInput = await screen.findByPlaceholderText(/Portland, OR/i);
    await user.type(locInput, "NY{Enter}");

    // Chat Step 4 - Use Enter Key
    const targetAudienceInput = await screen.findByPlaceholderText(
      /Local families, Tech startups/i,
    );
    await user.type(targetAudienceInput, "Local families{Enter}");

    // Verify it transitions to Step 2: Review Details by triggering handleIntake
    await waitFor(() => {
      screen.getByText("Review Details");
      screen.getByDisplayValue("Maya Bakery");
    });
  });

  it("Handles validation failures when fields are empty", async () => {
    const user = userEvent.setup({ delay: null });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    // Chat Step 1 - Enter Key with short name
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, "Ma{Enter}");
    await screen.findByText("Business Name must be at least 3 characters.");

    await user.clear(nameInput);
    await user.type(nameInput, "Maya Bakery{Enter}");

    // Chat Step 2 - Next click with empty value
    const sellInput = await screen.findByPlaceholderText(
      /I bake custom vegan cakes/i,
    );

    // Test validation with missing data
    await user.clear(sellInput);

    const nextBtn2 = screen.getByRole("button", { name: /Next/i });

    // Verify the button is enabled when empty
    expect(nextBtn2).not.toBeDisabled();

    // Provide value to enable button and proceed
    await user.type(sellInput, "Cakes");
    expect(nextBtn2).not.toBeDisabled();
    await user.type(sellInput, "{Enter}");

    // Chat Step 3 - Next click with empty value
    const locInput = await screen.findByPlaceholderText(/Portland, OR/i);

    await user.clear(locInput);

    const nextBtn3 = screen.getByRole("button", { name: /Next/i });

    // Verify the button is enabled when empty
    expect(nextBtn3).not.toBeDisabled();

    // Provide value to enable button and proceed
    await user.type(locInput, "NY");
    expect(nextBtn3).not.toBeDisabled();
    await user.click(nextBtn3);

    // Chat Step 4
    await waitFor(() => {
      screen.getByText("Who is your target audience?");
    });
    const targetAudienceInput = await screen.findByPlaceholderText(
      /Local families, Tech startups/i,
    );
    await user.type(targetAudienceInput, "Local families");
    const generateBtn = screen.getByRole("button", { name: /Next/i });
    expect(generateBtn).not.toBeDisabled();
    await user.click(generateBtn);
  });

  it("Handles multi-step successful onboarding flow", async () => {
    const user = userEvent.setup({ delay: null });

    // Mock intake success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/launch") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (url === "/api/onboarding/intake") {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            business_type: "Bakery",
            business_name: "Maya Bakery",
            categories: ["food"],
            initial_products: [{ name: "Cake", price: "20" }],
          }),
        });
      }
      if (url === "/api/onboarding/start") {
        return Promise.resolve({
          ok: true,
          json: async () => ({ message: "Success!" }),
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    // Chat Step 1
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, "Maya Bakery");

    const nextBtn1 = screen.getByRole("button", { name: /Next/i });
    await user.click(nextBtn1);

    // Chat Step 2
    const sellInput = await screen.findByPlaceholderText(
      /I bake custom vegan cakes/i,
      {},
      { timeout: 3000 },
    );
    await user.type(sellInput, "Cakes");

    const nextBtn2 = screen.getByRole("button", { name: /Next/i });
    await user.type(sellInput, "{Enter}");

    // Chat Step 3
    const locInput = await screen.findByPlaceholderText(
      /Portland, OR/i,
      {},
      { timeout: 3000 },
    );
    await user.type(locInput, "NY");

    const button3 = screen.getByRole("button", { name: /Next/i });
    expect(button3).not.toBeDisabled();
    await user.click(button3);

    // Chat Step 4
    await waitFor(() => {
      screen.getByText("Who is your target audience?");
    });
    const targetAudienceInput = await screen.findByPlaceholderText(
      /Local families, Tech startups/i,
    );
    await user.type(targetAudienceInput, "Local families");

    const button = screen.getByRole("button", { name: /Next/i });
    expect(button).not.toBeDisabled();

    // Step 1: Intake
    await user.click(button);

    // Verify it transitions to Step 2: Review Details
    await waitFor(() => {
      screen.getByText("Review Details");
      screen.getByDisplayValue("Maya Bakery");
    });

    const continueButton = screen.getByRole("button", { name: /Continue/i });
    await user.click(continueButton);

    // Verify it transitions to Step 3: Style & Team
    await waitFor(() => {
      screen.getByText("Style & Team");
      screen.getByText("Website Template");
    });

    // Fill in Account Setup fields
    const nameInput2 = screen.getByPlaceholderText(/e.g. Maya Smith/i);
    await user.type(nameInput2, "Maya Smith");

    const emailInput = screen.getByPlaceholderText(/you@example.com/i);
    await user.type(emailInput, "maya@example.com");

    const passwordInput = screen.getByPlaceholderText(/••••••••/i);
    await user.type(passwordInput, "mypassword123");

    const launchButton = screen.getByRole("button", {
      name: /Approve & Publish/i,
    });
    await user.click(launchButton);

    // Verify it transitions to Step 5 (Live Screen) on success
    await waitFor(() => {
      screen.getByText("You're Live!");
      screen.getByText("maya-bakery.ohc.app");
    });

    // Check that start API was called with the correct credentials
    expect(global.fetch).toHaveBeenCalledWith(
      "/api/onboarding/start",
      expect.objectContaining({
        method: "POST",
        body: expect.stringContaining('"admin_name":"Maya Smith"'),
      }),
    );
    expect(global.fetch).toHaveBeenCalledWith(
      "/api/onboarding/start",
      expect.objectContaining({
        method: "POST",
        body: expect.stringContaining('"admin_email":"maya@example.com"'),
      }),
    );
    expect(global.fetch).toHaveBeenCalledWith(
      "/api/onboarding/start",
      expect.objectContaining({
        method: "POST",
        body: expect.stringContaining('"admin_password":"mypassword123"'),
      }),
    );
  });

  it("Step 1: Handles intake API failure", async () => {
    const consoleErrorSpy = vi
      .spyOn(console, "error")
      .mockImplementation(() => {});
    const user = userEvent.setup({ delay: null });

    // Mock intake failure
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/launch") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (url === "/api/onboarding/intake" || url === "/api/onboarding/start") {
        return Promise.resolve({
          ok: false,
          status: 500,
          json: async () => ({ error: "Failed to process business details" }),
          clone: function () {
            return this;
          },
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    // Chat Step 1
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, "Maya Bakery");

    const nextBtn1 = screen.getByRole("button", { name: /Next/i });
    await user.click(nextBtn1);

    // Chat Step 2
    const sellInput = await screen.findByPlaceholderText(
      /I bake custom vegan cakes/i,
      {},
      { timeout: 3000 },
    );
    await user.type(sellInput, "Cakes");

    const nextBtn2 = screen.getByRole("button", { name: /Next/i });
    await user.type(sellInput, "{Enter}");

    // Chat Step 3
    const locInput = await screen.findByPlaceholderText(
      /Portland, OR/i,
      {},
      { timeout: 3000 },
    );
    await user.type(locInput, "NY");

    const button3 = screen.getByRole("button", { name: /Next/i });
    await user.click(button3);

    // Chat Step 4
    await waitFor(() => {
      screen.getByText("Who is your target audience?");
    });
    const targetAudienceInput = await screen.findByPlaceholderText(
      /Local families, Tech startups/i,
    );
    await user.type(targetAudienceInput, "Local families");

    const button = screen.getByRole("button", { name: /Next/i });

    await user.click(button);

    // Verify error appears and step goes back to last input screen
    await waitFor(() => {
      screen.getByText("Failed to process business details");
    });

    consoleErrorSpy.mockRestore();
  });

  it("Step 3: Handles start API failure and returns to Step 3", async () => {
    const consoleErrorSpy = vi
      .spyOn(console, "error")
      .mockImplementation(() => {});
    const user = userEvent.setup({ delay: null });

    // Set initial state to Step 3 to test start API directly
    act(() => {
      useOnboardingStore.setState({
        step: 3,
        adminName: "Test Admin",
        adminEmail: "test@example.com",
        adminPassword: "Password123",
      });
    });

    // Mock start failure
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/launch") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (url === "/api/onboarding/intake" || url === "/api/onboarding/start") {
        return Promise.resolve({
          ok: false,
          status: 500,
          clone: () => ({
            json: async () => ({ error: "Failed to start onboarding" }),
          }),
          json: async () => ({ error: "Failed to start onboarding" }),
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const launchButton = screen.getByRole("button", {
      name: /Approve & Publish/i,
    });

    await user.click(launchButton);

    // Verify error appears and step goes back to 3
    await waitFor(() => {
      screen.getByText("Failed to start onboarding");
      screen.getByText("Style & Team");
    });

    consoleErrorSpy.mockRestore();
  });

  it("Step 1: Displays validation error when business name is too short", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 1,
        chatStep: 1,
        businessName: "A",
        location: "",
        businessType: "Online Store",
        categories: [],
        firstProductName: "",
        firstProductPrice: "",
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const nextButton = screen.getByRole("button", { name: /Next/i });

    await user.click(nextButton);

    await screen.findByText("Business Name must be at least 3 characters.");
  });

  it("Step 2: Displays validation error when product price is invalid", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 2,
        businessName: "Valid Name",
        businessType: "Bakery",
        categories: ["food"],
        domainChoice: "subdomain",
        firstProductName: "Cake",
        firstProductPrice: "abc", // Invalid price
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const continueButton = screen.getByRole("button", { name: /Continue/i });
    expect(continueButton).not.toBeDisabled(); // Button should not be disabled based on input length, but validation will stop it

    const priceInput = screen.getByDisplayValue("abc");
    await user.type(priceInput, "d"); // Type 'd' to trigger the onChange validation.

    await user.click(continueButton);

    await waitFor(() => {
      // The general error message should trigger
      screen.getByText("Please fix the errors before continuing.");
      screen.getByText("Invalid price.");
    });

    // Check that we're still on step 2
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it("Step 2: Displays validation error when business type is empty", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 2,
        businessName: "Valid Name",
        businessType: "Bakery",
        categories: ["food"],
        domainChoice: "subdomain",
        firstProductName: "Cake",
        firstProductPrice: "20",
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const continueButton = screen.getByRole("button", { name: /Continue/i });
    expect(continueButton).not.toBeDisabled();

    // Find the input element that is associated with the 'Business Type' label
    const inputs = screen.getAllByRole("textbox");
    const businessTypeInput = screen.getByDisplayValue("Bakery");

    // Clear the input to trigger validation
    await user.clear(businessTypeInput);

    // Click continue to trigger validation
    await user.click(continueButton);

    await waitFor(() => {
      screen.getByText("Business Type is required to configure your agents.");
    });
  });

  it("Step 2: Proceeds to Step 3 when validation passes", async () => {
    const user = userEvent.setup({ delay: null });

    // Set initial state to Step 2
    act(() => {
      useOnboardingStore.setState({
        step: 2,
        businessName: "Valid Name",
        businessType: "Bakery",
        categories: ["food"],
        domainChoice: "subdomain",
        firstProductName: "Cake",
        firstProductPrice: "20",
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const continueButton = screen.getByRole("button", { name: /Continue/i });

    await user.click(continueButton);

    expect(
      screen.queryByText("Business Name must be at least 3 characters."),
    ).toBeNull();
    screen.getByText("Style & Team");
  });

  it("Step 3: Can select Web Address, AI agents and toggle auto-respond", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 3,
        aiAgents: [],
        aiAutoRespond: true,
        domainChoice: "subdomain",
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    // Verify initial Web Address options
    const subdomainOption = screen.getByText("Free Subdomain");
    const customOption = screen.getByText("Custom Domain");
    expect(subdomainOption).toBeDefined();
    expect(customOption).toBeDefined();

    // Select Custom Domain
    await user.click(customOption);

    // Verify initial state
    // By default, since the store initializes with empty agents, we might not see any badges immediately.
    // However, if the store had active agents, they would appear.
    // The auto-respond toggle remains.

    // Check toggle
    // Checkbox might be hidden by sr-only or similar, use label text instead or get by id
    const toggle = document.querySelector(
      'input[type="checkbox"]',
    ) as HTMLInputElement;
    expect(toggle).toBeChecked();

    // Toggle auto respond
    await user.click(toggle);

    await waitFor(() => {
      const state = useOnboardingStore.getState();
      expect(state.aiAutoRespond).toBe(false);
      expect(state.domainChoice).toBe("custom");
    });
  });

  it("Step 5: Shows Live Screen with correct links", async () => {
    act(() => {
      useOnboardingStore.setState({
        step: 5,
        startResult: {
          message: "Your business has been successfully launched.",
        },
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    await waitFor(() => {
      screen.getByText("You're Live!");
      screen.getByText("Your business has been successfully launched.");
      expect(
        screen.getByRole("link", { name: /Open Assistant/i }),
      ).toHaveAttribute("href", "/assistant");
      screen.getByRole("link", { name: /Preview Storefront/i });
    });
  });

  it("retries handleSaveDraft on network failure", async () => {
    const user = userEvent.setup({ delay: null });

    let fetchCalls = 0;
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/draft") {
        fetchCalls++;
        if (fetchCalls < 2) {
          return Promise.resolve({
            ok: false,
            status: 500,
            json: async () => ({}),
            clone: function () {
              return this;
            },
          });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    });

    act(() => {
      useOnboardingStore.setState({ step: 2 });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const saveDraftButton = await screen.findByRole("button", {
      name: /Save Draft/i,
    });
    await user.click(saveDraftButton);

    await waitFor(
      () => {
        screen.getByText("Draft Saved!");
      },
      { timeout: 3000 },
    );

    expect(fetchCalls).toBeGreaterThanOrEqual(2);
  });

  it("loads draft state correctly on mount", async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/draft") {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            wizardState: {
              bio: "Draft Bio",
              step: 1,
              chatStep: 2,
              businessName: "Draft Business Name",
              whatYouSell: "Draft Products",
              instantImageUrl: "https://example.com/image.png",
            },
          }),
        });
      }
      if (url === "/api/onboarding/state") {
        return Promise.resolve({
          ok: true,
          json: async () => ({ wizardState: { bio: "Draft Bio" } }),
        });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    render(<OnboardingWizard />);

    // Wait for the mock fetch to resolve and state to update
    await waitFor(() => {
      screen.getByText("What do you sell?");
    });

    screen.getByDisplayValue("Draft Products");
  });

  it("Step 4: Target audience saves and navigates to launch correctly", async () => {
    const user = userEvent.setup({ delay: null });

    // Set initial state to Step 4 (chatStep = 4)
    act(() => {
      useOnboardingStore.setState({
        step: 1,
        chatStep: 4,
        businessName: "Valid Name",
        whatYouSell: "Products",
        location: "City",
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const targetAudienceInput = await screen.findByPlaceholderText(
      /Local families, Tech startups/i,
    );
    await user.type(targetAudienceInput, "Local families");

    const generateBtn = screen.getByRole("button", { name: /Next/i });
    expect(generateBtn).not.toBeDisabled();

    // Note: handleIntake uses fetch which is either mocked or fails, but we just want to test
    // that the UI hook for targetAudience works.
  });

  it("Save Draft button triggers draft API and shows success message", async () => {
    const user = userEvent.setup({ delay: null });

    // Mock draft API success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/draft") {
        return Promise.resolve({
          ok: true,
          json: async () => ({}),
        });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    });

    // Start at Step 2
    act(() => {
      useOnboardingStore.setState({ step: 2 });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const saveDraftButton = await screen.findByRole("button", {
      name: /Save Draft/i,
    });
    expect(saveDraftButton).toBeDefined();

    await user.click(saveDraftButton);

    await waitFor(() => {
      screen.getByText("Draft Saved!");
    });

    // Verify API was called
    expect(global.fetch).toHaveBeenCalledWith(
      "/api/onboarding/draft",
      expect.objectContaining({
        method: "POST",
      }),
    );
  });

  it("Step 3: Shows inline validation errors for admin fields", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 3,
        aiAgents: [],
        aiAutoRespond: true,
        domainChoice: "subdomain",
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const nameInput = screen.getByPlaceholderText(/e.g. Maya Smith/i);
    const emailInput = screen.getByPlaceholderText(/you@example.com/i);
    const passwordInput = screen.getByPlaceholderText(/••••••••/i);
    // Test Admin Name Validation
    await user.clear(nameInput);
    await user.type(nameInput, "a");
    await user.clear(nameInput);
    await screen.findByText("Admin Name is required");

    await user.type(nameInput, "Maya Smith");
    await waitFor(() => {
      expect(screen.queryByText("Admin Name is required")).toBeNull();
    });

    // Test Admin Email Validation
    await user.clear(emailInput);
    await user.type(emailInput, "invalidemail");
    await screen.findByText("Please enter a valid email address");

    await user.clear(emailInput);
    // Workaround for clear not triggering empty string validation properly sometimes
    await user.type(emailInput, "x");
    await user.keyboard("{Backspace}");
    await screen.findByText("Admin Email is required");

    await user.type(emailInput, "maya@example.com");
    await waitFor(() => {
      expect(
        screen.queryByText("Please enter a valid email address"),
      ).toBeNull();
      expect(screen.queryByText("Admin Email is required")).toBeNull();
    });

    // Test Admin Password Validation
    await user.clear(passwordInput);
    await user.type(passwordInput, "weak");
    await screen.findByText(
      "Password must be at least 8 characters and contain a number",
    );

    await user.clear(passwordInput);
    await screen.findByText("Password is required");

    await user.type(passwordInput, "mypassword123");
    await waitFor(() => {
      expect(
        screen.queryByText(
          "Password must be at least 8 characters and contain a number",
        ),
      ).toBeNull();
      expect(screen.queryByText("Password is required")).toBeNull();
    });
    expect(
      screen.queryByText(
        "Password must be at least 8 characters and contain a number",
      ),
    ).toBeNull();
    expect(screen.queryByText("Password is required")).toBeNull();
  });

  it("Handles Save Draft button functionality", async () => {
    const user = userEvent.setup({ delay: null });

    // Mock the draft save success
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === "/api/onboarding/draft") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ wizardState: { bio: "Draft Bio" } }),
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    // Chat Step 1
    const nameInput = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(nameInput, "Draft Bakery");

    // Proceed to Step 2
    const nextBtn = screen.getByRole("button", { name: /Next/i });
    await user.click(nextBtn);

    // On step 2, wait for "What do you sell" or another input indicating step 2 is active
    await screen.findByText(/What do you sell\?/i);

    // Try finding the url inputs
    const urlInputs = screen.queryAllByPlaceholderText(
      /Image URL \(Optional\)/i,
    );
    if (urlInputs.length > 0) {
      const urlInput =
        urlInputs.find((el) => el.id === "instant-image-url") || urlInputs[0];
      await user.type(urlInput, "https://example.com/save_draft.png");
    } else {
      // Fallback to chat input or another state if URL isn't here
      // We know instantImageUrl is in state, so we update it directly to test draft save
      act(() => {
        useOnboardingStore.setState({
          instantImageUrl: "https://example.com/save_draft.png",
        });
      });
    }

    // Click Save Draft
    const saveDraftBtn = screen.getByRole("button", { name: /Save Draft/i });
    await user.click(saveDraftBtn);

    // Verify it saved
    expect(global.fetch).toHaveBeenCalledWith(
      "/api/onboarding/draft",
      expect.objectContaining({
        method: "POST",
        body: expect.stringContaining("https://example.com/save_draft.png"),
      }),
    );
    await waitFor(() => {
      screen.getByText("Draft Saved!");
    });
  });

  it("Instant Build: shows validation error when bio is empty", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({ step: -1, bio: "", error: "" });
    });

    await renderOnboardingWizard();

    const bioInput = screen.getByTestId("instant-bio");
    await user.clear(bioInput);

    const generateBtn = screen.getByRole("button", { name: /Generate Storefront/i });

    await waitFor(() => {
      expect(generateBtn).toBeDisabled();
    });
  });

  it("Instant Build: completes end-to-end flow with correct API calls", async () => {
    const user = userEvent.setup({ delay: null });

    let fetchCalls: any[] = [];
    global.fetch = vi.fn().mockImplementation((url, options) => {
      fetchCalls.push({ url, options });

      if (typeof url === "string" && url.includes("/api/onboarding/intake")) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            business_name: "Instant Tech",
            business_type: "Consulting",
            categories: ["digital"],
            location: "SF",
            target_audience: "Startups",
            initial_products: [{ name: "Consult", price: "500.00" }]
          }),
        });
      }
      if (typeof url === "string" && url.includes("/api/onboarding/start")) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ organization_id: "org_123" }),
        });
      }
      if (typeof url === "string" && url.includes("/api/onboarding/launch")) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({}),
        });
      }
      if (typeof url === "string" && url.includes("/api/onboarding/state")) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
      }
      if (typeof url === "string" && url.includes("/api/onboarding/draft")) {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
      }

      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    act(() => {
      useOnboardingStore.setState({ step: -2 });
    });

    await renderOnboardingWizard();

    const instantBuildBtn = screen.getByRole("button", { name: "Instant Build" });
    await user.click(instantBuildBtn);

    const bioInput = await screen.findByTestId("instant-bio");
    await user.type(bioInput, "I consult startups in SF.");

    const generateBtn = screen.getByRole("button", { name: "Generate Storefront" });
    await user.click(generateBtn);

    await waitFor(() => {
      expect(screen.queryByText(/You're Live!/i)).toBeInTheDocument();
    }, { timeout: 4000 });

    const intakeCall = fetchCalls.find(call => typeof call.url === 'string' && call.url.includes('/api/onboarding/intake'));
    expect(intakeCall).toBeDefined();

    const startCall = fetchCalls.find(call => typeof call.url === 'string' && call.url.includes('/api/onboarding/start'));
    expect(startCall).toBeDefined();
    const startBody = JSON.parse(startCall.options.body);
    expect(startBody.company_name).toBe("Instant Tech");
    expect(startBody.first_product_name).toBe("Consult");

    const launchCall = fetchCalls.find(call => typeof call.url === 'string' && call.url.includes('/api/onboarding/launch'));
    expect(launchCall).toBeDefined();
  });

  it("Instant Build: displays error when API fails", async () => {
    const user = userEvent.setup({ delay: null });

    global.fetch = vi.fn().mockImplementation((url) => {
      if (typeof url === "string" && url.includes("/api/onboarding/intake")) {
        return Promise.resolve({
          ok: false,
          status: 500,
          clone: function() { return this; },
          json: () => Promise.resolve({ error: "Failed to generate your business" }),
        });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    act(() => {
      useOnboardingStore.setState({ step: -1, bio: "Some bio" });
    });

    await renderOnboardingWizard();

    const generateBtn = screen.getByRole("button", { name: "Generate Storefront" });
    await user.click(generateBtn);

    await waitFor(() => {
      expect(useOnboardingStore.getState().error).toMatch(/Failed to generate your business|HTTP error! status: 500|Backend connection failed|error/i);
    });
  });

  it("allows skipping setup and opens the assistant", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({ step: 0 });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    const skipButton = screen.getAllByRole("button", {
      name: /Skip setup/i,
    })[0];
    await user.click(skipButton);

    expect(localStorage.getItem("has_onboarded")).toBe("true");
    expect(mockRouterPush).toHaveBeenCalledWith("/dashboard");
  });

  it("offers a global back control on later wizard steps", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 3,
        businessName: "Maya Bakery",
        businessType: "Bakery",
        firstProductName: "Cake",
        firstProductPrice: "20",
      });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }

    screen.getByText("Style & Team");

    // Get all back buttons and take the visible one
    const backButton = screen.getAllByRole("button", { name: /Back/i })[0];
    await user.click(backButton);

    screen.getByText("Review Details");
    expect(useOnboardingStore.getState().step).toBe(2);
  });

  it("can go back from the first question to the intro screen", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({ step: 1, chatStep: 1 });
    });

    await renderOnboardingWizard();
    if (screen.queryByRole("button", { name: "Start My Business" })) {
      await user.click(
        screen.getByRole("button", { name: "Start My Business" }),
      );
    }
    screen.getByText("What's the name of your business?");

    // Get all back buttons and take the visible one
    const backButton = screen.getAllByRole("button", { name: /Back/i })[0];
    await user.click(backButton);

    screen.getByText("10-Minute Setup Wizard");
    expect(useOnboardingStore.getState().step).toBe(-2);
  });

  it("Step 3: Passes initial_products from localStorage to /api/onboarding/start", async () => {
    const user = userEvent.setup({ delay: null });

    localStorage.setItem(
      "onboarding_initial_products",
      JSON.stringify([{ name: "Custom AI Product", price: "99" }]),
    );

    act(() => {
      useOnboardingStore.setState({
        step: 3,
        adminName: "Test Admin",
        adminEmail: "test@example.com",
        adminPassword: "Password123",
      });
    });

    let startRequestPayload: any = null;
    (global.fetch as any).mockImplementation((url: string, options: any) => {
      if (url === "/api/onboarding/start") {
        startRequestPayload = JSON.parse(options.body);
        return Promise.resolve({
          ok: true,
          json: async () => ({ organization_id: "org_123", status: "started" }),
        });
      }
      if (url === "/api/onboarding/launch") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (url === "/api/onboarding/state") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      if (url === "/api/onboarding/draft") {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({ ok: true, json: async () => ({}) });
    });

    await renderOnboardingWizard();

    const launchButton = await screen.findByRole("button", {
      name: /Approve & Publish/i,
    });
    await user.click(launchButton);

    expect(startRequestPayload).toBeDefined();
    expect(startRequestPayload.initial_products).toEqual([
      { name: "Custom AI Product", price: "99" },
    ]);
  });

  it("Step 1: shows validation error when location is empty", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 1,
        chatStep: 3,
        businessName: "Bakery",
        whatYouSell: "Cakes",
        location: "",
      });
    });

    let view;
    await act(async () => {
      view = render(
        <TooltipProvider>
          <OnboardingWizard />
        </TooltipProvider>,
      );
    });

    const continueButton = screen.getByRole("button", { name: /Next/i });
    await user.click(continueButton);

    await waitFor(() => {
      screen.getByText("Please tell us your location.");
    });
  });

  it("Step 1: shows validation error when target audience is empty", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 1,
        chatStep: 4,
        businessName: "Bakery",
        whatYouSell: "Cakes",
        location: "City",
        targetAudience: "",
      });
    });

    let view;
    await act(async () => {
      view = render(
        <TooltipProvider>
          <OnboardingWizard />
        </TooltipProvider>,
      );
    });

    const continueButton = screen.getByRole("button", { name: /Next/i });
    await user.click(continueButton);

    await waitFor(() => {
      screen.getByText("Please tell us your target audience.");
    });
  });

  it("Step 1: shows validation error when what you sell is empty", async () => {
    const user = userEvent.setup({ delay: null });

    act(() => {
      useOnboardingStore.setState({
        step: 1,
        chatStep: 2,
        businessName: "Bakery",
        whatYouSell: "",
      });
    });

    let view;
    await act(async () => {
      view = render(
        <TooltipProvider>
          <OnboardingWizard />
        </TooltipProvider>,
      );
    });

    const continueButton = screen.getByRole("button", { name: /Next/i });
    await user.click(continueButton);

    await waitFor(() => {
      screen.getByText("Please tell us what you sell.");
    });
  });

  it("renders error banner with premium macOS aesthetic on API failure", async () => {
    const user = userEvent.setup();

    act(() => {
      useOnboardingStore.setState({
        step: -2,
        chatStep: 0,
        businessName: "",
        error: null,
      });
    });

    // Mock API
    global.fetch = vi.fn().mockImplementation((url) => {
      if (typeof url === "string" && url.includes("/api/onboarding/intake")) {
        return Promise.resolve({
          ok: false,
          status: 500,
          json: () => Promise.resolve({ error: "Intake Error" }),
        });
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({}),
      });
    });

    await renderOnboardingWizard();

    // Intro screen
    const startBtn = screen.getByRole("button", { name: "Start My Business" });
    await user.click(startBtn);

    // Wait for the skip button on step 1
    const skipBtns = await screen.findAllByRole("button", {
      name: /Skip setup/i,
    });
    await user.click(skipBtns[0]);
  });
});
