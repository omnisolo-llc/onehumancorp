import { act } from "react";
import "@testing-library/jest-dom/vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ExitIntentBuilder from "./page";
import { vi } from "vitest";

describe("ExitIntentBuilder", () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });
  });

  it('encodes hostile editor text as inert JavaScript strings without innerHTML', () => {
    act(() => { render(<ExitIntentBuilder />); });
    fireEvent.change(screen.getByPlaceholderText('Wait! Before you go...'), { target: { value: '</script><img src=x onerror=alert(1)>` ${evil}' } });

    const code = screen.getByText((_, element) => element?.tagName === 'CODE').textContent ?? '';
    expect(code).not.toContain('</script><img');
    expect(code).not.toContain('innerHTML');
    expect(code).toContain('\\u003c/script\\u003e');
    expect(code).toContain('textContent');
  });

  it("renders the builder and preview properly", () => {
    act(() => { render(<ExitIntentBuilder />); });

    expect(screen.getByText("Exit-Intent Pop-up Builder")).toBeInTheDocument();
    expect(screen.getByDisplayValue("Wait! Before you go...")).toBeInTheDocument();

    // Check preview updates
    const headlineInput = screen.getByDisplayValue("Wait! Before you go...");
    fireEvent.change(headlineInput, { target: { value: "Don't leave yet!" } });

    // The preview should reflect this change (it shows up twice: input and preview)
    const texts = screen.getAllByText("Don't leave yet!");
    expect(texts.length).toBeGreaterThan(0);
  });

  it("routes to pricing without granting branding removal locally", async () => {
    act(() => { render(<ExitIntentBuilder />); });

    const toggleButton = screen.getByRole("switch");
    await userEvent.click(toggleButton);

    expect(screen.getAllByText("Remove OHC Branding").length).toBeGreaterThan(0);
    expect(screen.getByText("Upgrade to Pro")).toBeInTheDocument();

    await userEvent.click(screen.getByText("Upgrade to Pro"));

    expect(screen.queryByText("Upgrade to Pro")).not.toBeInTheDocument();
    expect(toggleButton).toHaveAttribute("aria-checked", "false");
    expect(localStorage.getItem('has_pro')).toBeNull();
  });
});
