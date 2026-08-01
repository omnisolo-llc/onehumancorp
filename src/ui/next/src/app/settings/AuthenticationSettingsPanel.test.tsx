import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, it, vi } from "vitest";
import { AuthenticationSettingsPanel } from "./AuthenticationSettingsPanel";

it("controls registration and only enables configured OIDC providers", async () => {
  const changeMode = vi.fn();
  const changeProvider = vi.fn();
  render(
    <AuthenticationSettingsPanel
      onProviderChange={changeProvider}
      onRegistrationModeChange={changeMode}
      providers={[
        { key: "google", display_name: "Google", provider_kind: "google", issuer: "https://accounts.google.com", configured: true, enabled: false },
        { key: "keycloak", display_name: "Keycloak", provider_kind: "oidc", issuer: "https://id.example.test/realms/ohc", configured: false, enabled: false },
      ]}
      providerStatus="idle"
      registrationMode="closed"
      registrationStatus="idle"
    />,
  );
  const user = userEvent.setup();
  await user.selectOptions(screen.getByLabelText("Registration mode"), "open");
  expect(changeMode).toHaveBeenCalledWith("open");
  await user.click(screen.getByRole("checkbox", { name: /enable google/i }));
  expect(changeProvider).toHaveBeenCalledWith("google", true);
  expect(screen.getByRole("checkbox", { name: /enable keycloak/i })).toBeDisabled();
  expect(screen.getByText(/configure its deployment credentials/i)).toBeInTheDocument();
});
