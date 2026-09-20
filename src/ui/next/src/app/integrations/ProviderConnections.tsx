"use client";

import { FormEvent, useCallback, useEffect, useRef, useState } from "react";

type ProviderId = "openai_api" | "stripe";
type Connection = { id: ProviderId; status: string; usable: boolean };
const PROVIDERS: { id: ProviderId; title: string; description: string }[] = [
  { id: "openai_api", title: "OpenAI API", description: "Use your own API account for supported AI requests. API usage is billed by your provider; a ChatGPT subscription is not an API key." },
  { id: "stripe", title: "Stripe payments", description: "Create real payment requests in your business account. Connecting does not charge a customer or confirm that an invoice is paid." },
];
const confirmed = (value: unknown): value is { success: true; usable: true; status: "verified" } => {
  if (!value || typeof value !== "object") return false;
  const item = value as Record<string, unknown>;
  return item.success === true && item.usable === true && item.status === "verified";
};

export default function ProviderConnections() {
  const [connections, setConnections] = useState<Partial<Record<ProviderId, Connection>>>({});
  const [editing, setEditing] = useState<ProviderId | null>(null);
  const [disconnecting, setDisconnecting] = useState<ProviderId | null>(null);
  const [secret, setSecret] = useState("");
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState("");
  const generation = useRef(0);

  const load = useCallback(async () => {
    const expected = ++generation.current;
    try {
      const response = await fetch("/api/v1/integrations");
      const data: unknown = await response.json();
      if (!response.ok || !data || typeof data !== "object" || !Array.isArray((data as { integrations?: unknown }).integrations)) {
        throw new Error("Connection status unavailable");
      }
      const next: Partial<Record<ProviderId, Connection>> = {};
      for (const value of (data as { integrations: unknown[] }).integrations) {
        if (!value || typeof value !== "object") continue;
        const row = value as Record<string, unknown>;
        if ((row.id === "openai_api" || row.id === "stripe") && typeof row.status === "string") {
          next[row.id] = { id: row.id, status: row.status, usable: row.usable === true && row.status === "verified" };
        }
      }
      if (generation.current === expected) setConnections(next);
    } catch {
      if (generation.current === expected) setMessage("Provider connection status is unavailable. No connection has been assumed.");
    }
  }, []);
  useEffect(() => { void load(); return () => { generation.current++; }; }, [load]);

  async function save(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!editing || !secret.trim() || busy) return;
    const provider = editing;
    generation.current++;
    setBusy(true); setMessage("");
    try {
      const response = await fetch(`/api/v1/integrations/${provider}/connect`, {
        method: "POST", headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ api_token: secret.trim() }),
      });
      const result: unknown = await response.json();
      if (!response.ok || !confirmed(result)) throw new Error("Provider verification failed");
      setEditing(null);
      setMessage("The provider verified your key. It is stored encrypted for this business.");
      await load();
    } catch {
      // Never display an upstream body that could echo credentials.
      setMessage("The key could not be verified. Owner access and a valid provider API key are required; no new connection was confirmed.");
    } finally { setSecret(""); setBusy(false); }
  }
  async function recheck(provider: ProviderId) {
    if (busy) return;
    generation.current++;
    setBusy(true); setMessage("");
    try {
      const response = await fetch(`/api/v1/integrations/${provider}/verify`, { method: "POST" });
      const result: unknown = await response.json();
      if (!response.ok || !confirmed(result)) throw new Error("Verification unavailable");
      setMessage("Connection verified again with the provider.");
    } catch { setMessage("Reverification did not succeed. New requests may remain paused until the connection is verified."); }
    finally { await load(); setBusy(false); }
  }
  async function disconnect(provider: ProviderId) {
    if (busy) return;
    generation.current++;
    setBusy(true); setMessage("");
    try {
      const response = await fetch(`/api/v1/integrations/${provider}`, { method: "DELETE" });
      const result: unknown = await response.json();
      if (!response.ok || !result || typeof result !== "object"
          || (result as { success?: unknown }).success !== true || (result as { status?: unknown }).status !== "revoked") {
        throw new Error("Revocation unavailable");
      }
      setDisconnecting(null);
      setMessage("Disconnected for new requests. Requests already accepted by the provider may still finish.");
      await load();
    } catch { setMessage("The connection could not be revoked. It has not been marked disconnected; try again."); }
    finally { setBusy(false); }
  }

  return <section className="space-y-4 mb-8" aria-labelledby="verified-provider-title">
    <div><h2 id="verified-provider-title" className="text-xl font-semibold">Verified business connections</h2>
      <p className="text-sm opacity-80">Only an owner can connect, replace, verify or revoke these keys. Keys are never saved in browser storage.</p></div>
    {message && <p role="status" className="app-card p-3 text-sm">{message}</p>}
    <div className="grid gap-4 md:grid-cols-2">
      {PROVIDERS.map(provider => {
        const state = connections[provider.id];
        const active = state?.usable === true;
        return <article key={provider.id} aria-label={provider.title} className="app-card p-4 rounded-2xl space-y-3">
          <h3 className="font-semibold">{provider.title}</h3><p className="text-sm">{provider.description}</p>
          <p className="text-sm font-medium">{active ? "Verified connection" : state?.status === "revoked" ? "Disconnected" : state?.status === "reauthentication_required" || state?.status === "verification_required" ? "Verification required" : "Not connected"}</p>
          <div className="flex flex-wrap gap-2">
            <button type="button" className="glass-control px-3 min-h-[44px] rounded-lg" disabled={busy}
              onClick={() => { setSecret(""); setEditing(provider.id); setDisconnecting(null); setMessage(""); }}>
              {active ? `Replace ${provider.title} key` : `Configure ${provider.title}`}</button>
            {state && state.status !== "revoked" && <>
              <button type="button" className="glass-control px-3 min-h-[44px] rounded-lg" disabled={busy} onClick={() => void recheck(provider.id)}>Recheck {provider.title}</button>
              <button type="button" className="glass-control px-3 min-h-[44px] rounded-lg" disabled={busy} onClick={() => { setEditing(null); setSecret(""); setDisconnecting(provider.id); }}>Disconnect {provider.title}</button>
            </>}
          </div>
          {editing === provider.id && <form onSubmit={save} className="space-y-3">
            <label className="block text-sm" htmlFor={`provider-key-${provider.id}`}>{provider.title} API key</label>
            <input id={`provider-key-${provider.id}`} className="glass-control block w-full px-3 min-h-[44px] rounded-lg"
              type="password" autoComplete="new-password" spellCheck={false} maxLength={4096} value={secret}
              onChange={event => setSecret(event.target.value)} disabled={busy} required />
            <div className="flex gap-2 flex-wrap"><button type="submit" className="glass-control px-3 min-h-[44px] rounded-lg" disabled={busy || !secret.trim()}>Verify and save key</button>
              <button type="button" className="glass-control px-3 min-h-[44px] rounded-lg" disabled={busy} onClick={() => { setEditing(null); setSecret(""); }}>Cancel key change</button></div>
          </form>}
          {disconnecting === provider.id && <div role="group" aria-label={`Confirm disconnect ${provider.title}`} className="space-y-2">
            <p className="text-sm">Stop new requests through this connection? This cannot cancel an action the provider already accepted.</p>
            <button type="button" className="glass-control px-3 min-h-[44px] rounded-lg" disabled={busy} onClick={() => void disconnect(provider.id)}>Confirm disconnect</button>
            <button type="button" className="glass-control px-3 min-h-[44px] rounded-lg" disabled={busy} onClick={() => setDisconnecting(null)}>Keep connection</button>
          </div>}
        </article>;
      })}
    </div>
  </section>;
}
