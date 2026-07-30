export type RegistrationMode = "closed" | "open" | "invite_only";

export type AdminOidcProvider = Readonly<{
  key: string;
  display_name: string;
  provider_kind: string;
  issuer: string;
  configured: boolean;
  enabled: boolean;
}>;

type SaveStatus = "idle" | "saving" | "saved" | "error";

export function AuthenticationSettingsPanel({
  registrationMode,
  registrationStatus,
  providers,
  providerStatus,
  onRegistrationModeChange,
  onProviderChange,
}: Readonly<{
  registrationMode: RegistrationMode;
  registrationStatus: SaveStatus;
  providers: readonly AdminOidcProvider[];
  providerStatus: SaveStatus;
  onRegistrationModeChange: (mode: RegistrationMode) => void;
  onProviderChange: (provider: string, enabled: boolean) => void;
}>) {
  return (
    <section className="app-panel glassmorphism mt-8 overflow-hidden border border-white/40 dark:border-white/10">
      <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
        <div>
          <div className="app-panel-title font-outfit text-base font-bold text-gray-900 dark:text-white">Authentication & Registration</div>
          <div className="mt-1 text-xs text-[#0f766e] dark:text-[#6ac5bd]">Registration starts closed. Email verification remains mandatory in every enabled mode.</div>
        </div>
      </div>
      <div className="app-panel-body max-w-2xl space-y-6 p-6">
        <label className="block text-sm font-semibold text-gray-800 dark:text-gray-200">
          Registration mode
          <select
            aria-label="Registration mode"
            className="mt-2 w-full rounded-xl border border-gray-200 bg-white px-4 py-3 text-sm dark:border-gray-700 dark:bg-gray-900"
            disabled={registrationStatus === "saving"}
            onChange={(event) => onRegistrationModeChange(event.target.value as RegistrationMode)}
            value={registrationMode}
          >
            <option value="closed">Closed</option>
            <option value="open">Open</option>
            <option value="invite_only">Invite only</option>
          </select>
        </label>
        <p className="text-xs text-gray-500">First-time Google or Keycloak users obey this policy. Existing linked users can continue signing in.</p>
        <div aria-live="polite" className="min-h-5 text-xs">
          {registrationStatus === "saved" && <span className="text-green-700">Registration policy saved.</span>}
          {registrationStatus === "error" && <span className="text-red-700" role="alert">Registration policy could not be saved.</span>}
        </div>

        <div className="border-t border-gray-200 pt-5 dark:border-gray-700">
          <h3 className="text-sm font-bold text-gray-900 dark:text-white">Sign-in providers</h3>
          {providers.length === 0 ? (
            <p className="mt-2 text-xs text-gray-500">No OIDC providers are configured in this deployment.</p>
          ) : (
            <div className="mt-3 space-y-3">
              {providers.map((provider) => (
                <div className="rounded-xl border border-gray-200 p-4 dark:border-gray-700" key={provider.key}>
                  <label className="flex items-center justify-between gap-4 text-sm font-semibold text-gray-900 dark:text-white">
                    <span>
                      {provider.display_name}
                      <span className="mt-1 block break-all text-xs font-normal text-gray-500">{provider.issuer}</span>
                    </span>
                    <input
                      aria-label={`Enable ${provider.display_name}`}
                      checked={provider.enabled}
                      className="h-5 w-5 rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e]"
                      disabled={!provider.configured || providerStatus === "saving"}
                      onChange={(event) => onProviderChange(provider.key, event.target.checked)}
                      type="checkbox"
                    />
                  </label>
                  {!provider.configured && (
                    <p className="mt-2 text-xs text-amber-700 dark:text-amber-300">Configure its deployment credentials before enabling this provider.</p>
                  )}
                </div>
              ))}
            </div>
          )}
          <div aria-live="polite" className="mt-2 min-h-5 text-xs">
            {providerStatus === "saved" && <span className="text-green-700">Provider setting saved.</span>}
            {providerStatus === "error" && <span className="text-red-700" role="alert">Provider setting could not be saved.</span>}
          </div>
        </div>
      </div>
    </section>
  );
}
