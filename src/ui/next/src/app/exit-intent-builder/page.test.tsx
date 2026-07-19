import { render, screen, fireEvent } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ExitIntentBuilder from "./page";
import { vi } from "vitest";

describe("ExitIntentBuilder", () => {
  beforeEach(() => {
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });
  });

  it("renders the builder and preview properly", () => {
    render(<ExitIntentBuilder />);

    expect(screen.getByText("Exit-Intent Pop-up Builder")).toBeInTheDocument();
    expect(screen.getByDisplayValue("Wait! Before you go...")).toBeInTheDocument();

    // Check preview updates
    const headlineInput = screen.getByDisplayValue("Wait! Before you go...");
    fireEvent.change(headlineInput, { target: { value: "Don't leave yet!" } });

    // The preview should reflect this change (it shows up twice: input and preview)
    const texts = screen.getAllByText("Don't leave yet!");
    expect(texts.length).toBeGreaterThan(0);
  });

  it("shows the paywall when trying to remove branding", async () => {
    render(<ExitIntentBuilder />);

    const toggleButton = screen.getByRole("switch");
    await userEvent.click(toggleButton);

    expect(screen.getAllByText("Remove OHC Branding").length).toBeGreaterThan(0);
    expect(screen.getByText("Upgrade to Pro")).toBeInTheDocument();

    // Simulate upgrade
    await userEvent.click(screen.getByText("Upgrade to Pro"));

    // Paywall closes, toggle is checked
    expect(screen.queryByText("Upgrade to Pro")).not.toBeInTheDocument();
    expect(toggleButton).toHaveAttribute("aria-checked", "true");
  });
});
