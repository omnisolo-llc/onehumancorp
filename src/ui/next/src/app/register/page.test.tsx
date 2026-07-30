import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import RegisterPage from "./page";

const push = vi.fn();
vi.mock("next/navigation", () => ({ useRouter: () => ({ push }) }));

describe("registration entry", () => {
  beforeEach(() => {
    push.mockReset();
    sessionStorage.clear();
    vi.mocked(fetch).mockReset();
  });

  it("shows the persisted closed policy without collecting credentials", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(Response.json({
      registration_mode: "closed",
      registration_available: false,
      email_verification_required: true,
    }));
    render(<RegisterPage />);

    expect(await screen.findByText(/registration is currently closed/i)).toBeDefined();
    expect(screen.queryByLabelText(/email address/i)).toBeNull();
    expect(screen.queryByLabelText(/username/i)).toBeNull();
    expect(screen.queryByLabelText(/^password$/i)).toBeNull();
  });

  it("collects only email before verification and stores a bounded challenge", async () => {
    vi.mocked(fetch)
      .mockResolvedValueOnce(Response.json({
        registration_mode: "open",
        registration_available: true,
        email_verification_required: true,
      }))
      .mockResolvedValueOnce(Response.json({ challenge_id: "challenge-7", expires_in_seconds: 900 }, { status: 202 }));
    render(<RegisterPage />);
    const user = userEvent.setup();
    await user.type(await screen.findByLabelText(/email address/i), "Alice@example.test");
    expect(screen.queryByLabelText(/username/i)).toBeNull();
    expect(screen.queryByLabelText(/^password$/i)).toBeNull();
    await user.click(screen.getByRole("button", { name: /verify email/i }));

    await waitFor(() => expect(push).toHaveBeenCalledWith("/verify-email"));
    expect(JSON.parse(sessionStorage.getItem("ohc-registration-challenge") ?? "null")).toEqual({
      challengeId: "challenge-7",
      email: "alice@example.test",
    });
  });
});
