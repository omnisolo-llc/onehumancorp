"use client";

import React, { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function SettingsPage() {
  const router = useRouter();
  const [phone, setPhone] = useState("");
  const [otp, setOtp] = useState("");
  const [isVerifying, setIsVerifying] = useState(false);
  const [isVerified, setIsVerified] = useState(false);
  const [smsStatus, setSmsStatus] = useState("");
  const [preferences, setPreferences] = useState({
    urgent_booking: false,
    failed_payment: false,
    new_order: false,
  });

  const [deliverySettings, setDeliverySettings] = useState({
    delivery_enabled: false,
    delivery_radius: 5.0,
    delivery_fee: 8.50,
  });

  const [voiceSettings, setVoiceSettings] = useState({
    voice_receptionist_enabled: false,
    voice_receptionist_number: "",
    voice_receptionist_persona: "Friendly",
  });

  const [isLoading, setIsLoading] = useState(true);
  const [agentName, setAgentName] = useState("Agent One");


  useEffect(() => {
    Promise.all([
      fetch("/api/settings/delivery")
        .then(res => res.json())
        .then(data => {
           setDeliverySettings({
             delivery_enabled: data.delivery_enabled || false,
             delivery_radius: data.delivery_radius || 5.0,
             delivery_fee: data.delivery_fee || 8.50,
           });
        })
        .catch(e => console.error("Failed to load delivery settings", e)),

      fetch("/api/assistant/settings")
        .then(res => res.json())
        .then(data => {
          if (data?.settings?.agentName) {
            setAgentName(data.settings.agentName);
          }
        })
        .catch(e => console.error("Failed to load assistant settings", e)),

      fetch("/api/settings/voice")
        .then(res => res.json())
        .then(data => {
          if (data) {
            setVoiceSettings({
              voice_receptionist_enabled: data.voice_receptionist_enabled || false,
              voice_receptionist_number: data.voice_receptionist_number || "",
              voice_receptionist_persona: data.voice_receptionist_persona || "Friendly",
            });
          }
        })
        .catch(e => console.error("Failed to load voice settings", e))
    ]).finally(() => {
      setIsLoading(false);
    });
  }, []);

  const handleDeliverySettingChange = async (key: string, value: any) => {
    const newSettings = { ...deliverySettings, [key]: value };
    setDeliverySettings(newSettings);
    try {
      await fetch("/api/settings/delivery", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newSettings),
      });
    } catch (e) {
      console.error("Failed to save delivery settings", e);
    }
  };

  const handleVerify = async () => {
    setIsVerifying(true);
    try {
      const res = await fetch("/api/settings/sms-verify", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ phone }),
      });
      if (!res.ok) {
        setSmsStatus("Failed to send verification SMS.");
        setIsVerifying(false);
      } else {
        setSmsStatus("Verification code sent.");
      }
    } catch {
      setSmsStatus("Network error while sending verification SMS.");
      setIsVerifying(false);
    }
  };

  const handleConfirm = async () => {
    try {
      const res = await fetch("/api/settings/sms-confirm", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ phone, otp }),
      });
      if (res.ok) {
        setIsVerified(true);
        setSmsStatus("Phone number verified.");
      } else {
        setSmsStatus("Invalid OTP.");
      }
    } catch {
      setSmsStatus("Network error while confirming OTP.");
    }
  };

  const handlePreferenceChange = async (key: string, checked: boolean) => {
    const newPrefs = { ...preferences, [key]: checked };
    setPreferences(newPrefs);
    if (isVerified) {
      try {
        await fetch("/api/settings/sms-preferences", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ phone, ...newPrefs }),
        });
      } catch (e) {
        console.error("Failed to save preferences", e);
      }
    }
  };

  const handleVoiceSettingChange = async (key: string, value: string | boolean) => {
    const newSettings = { ...voiceSettings, [key]: value };
    setVoiceSettings(newSettings);

    try {
      await fetch("/api/settings/voice", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newSettings),
      });
    } catch (e) {
      console.error("Failed to save voice settings", e);
    }
  };

  const handleAgentNameChange = async (value: string) => {
    setAgentName(value);
    try {
      await fetch("/api/assistant/settings", {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ agentName: value }),
      });
    } catch (e) {
      console.error("Failed to save agent settings", e);
    }
  };

  const handleProvisionVoiceNumber = async () => {
    try {
      const res = await fetch("/api/settings/voice/provision", {
        method: "POST",
      });
      if (res.ok) {
        const data = await res.json();
        handleVoiceSettingChange('voice_receptionist_number', data.number);
      }
    } catch (e) {
      console.error("Failed to provision voice number", e);
    }
  };

  if (isLoading) {
    return (
      <AppShell title="Settings">
        <div className="flex h-64 items-center justify-center">
          <div className="w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin"></div>
        </div>
      </AppShell>
    );
  }

  return (
    <AppShell title="Settings">
      <div className="mx-auto max-w-4xl space-y-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold font-outfit text-gray-900">Workspace Settings</h1>
          <p className="app-list-subtitle">Manage integrations, local routing, and security.</p>
        </header>

        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">SMS Notifications & Security</div>
              <div className="app-list-subtitle">Get texts for critical events. Verify your phone number to enable.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="space-y-4">
              <label className="block text-sm font-semibold text-gray-700">Mobile Number</label>
              <input
                aria-label="Mobile Number"
                type="tel"
                placeholder="+1 (555) 000-0000"
                value={phone}
                onChange={(e) => setPhone(e.target.value)}
                disabled={isVerified}
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800"
              />
              {!isVerifying && !isVerified && (
                <WithTooltip id="settings-verify-tooltip" defaultText="Verify your number to receive critical notifications.">
                  <button onClick={handleVerify} className="app-button primary" type="button">
                    Verify Number
                  </button>
                </WithTooltip>
              )}

              {smsStatus && <p className="text-sm font-medium text-blue-700" role="status">{smsStatus}</p>}

              {isVerifying && !isVerified && (
                <div className="rounded-md border border-blue-100 bg-blue-50 p-3">
                  <p className="mb-2 text-sm text-blue-800">A 6-digit code has been sent. Enter it below:</p>
                  <div className="flex gap-2">
                    <input
                      aria-label="Verification code"
                      type="text"
                      placeholder="123456"
                      value={otp}
                      onChange={(e) => setOtp(e.target.value)}
                      className="w-28 rounded-md border border-gray-300 px-3 py-2 text-center text-sm text-gray-800"
                    />
                    <WithTooltip id="settings-otp-tooltip" defaultText="Click to confirm the code sent to your phone.">
                      <button onClick={handleConfirm} className="app-button primary" type="button">
                        Confirm OTP
                      </button>
                    </WithTooltip>
                  </div>
                </div>
              )}

              {isVerified && <span className="app-badge good">Number Verified</span>}

              <div className="space-y-2 border-t border-gray-200 pt-4">
                {[
                  ["urgent_booking", "Urgent Bookings"],
                  ["failed_payment", "Failed Payments"],
                  ["new_order", "New Orders"],
                ].map(([key, label]) => (
                  <label key={key} className="flex items-center gap-3">
                    <input
                      aria-label={label}
                      type="checkbox"
                      disabled={!isVerified}
                      checked={(preferences as any)[key]}
                      onChange={(e) => handlePreferenceChange(key, e.target.checked)}
                      className="rounded"
                    />
                    <span className={`text-sm ${isVerified ? "text-gray-800" : "text-gray-400"}`}>{label}</span>
                  </label>
                ))}
                <label className="flex items-center gap-3">
                  <input
                    aria-label="Enable Email Notifications"
                    type="checkbox"
                    checked={(preferences as any)["email_notifications"] || false}
                    onChange={(e) => handlePreferenceChange("email_notifications", e.target.checked)}
                    className="rounded"
                  />
                  <span className="text-sm text-gray-800">Enable Email Notifications</span>
                </label>
              </div>
            </div>
          </div>
        </section>

        <section className="app-grid two">
          <div className="app-panel">
            <div className="app-panel-header">
              <div>
                <div className="app-panel-title">Local Delivery Setup</div>
                <div className="app-list-subtitle">Configure delivery radius and rates.</div>
              </div>
            </div>
            <div className="app-panel-body space-y-4">
              <WithTooltip id="settings-delivery-tooltip" defaultText="Turn this on to offer local delivery to your customers.">
                <label className="flex items-center justify-between rounded-md border border-gray-200 p-3 text-sm text-gray-700 cursor-pointer">
                  <span>Enable Local Delivery</span>
                  <input
                    aria-label="Enable Local Delivery"
                    type="checkbox"
                    checked={deliverySettings.delivery_enabled}
                    onChange={(e) => handleDeliverySettingChange('delivery_enabled', e.target.checked)}
                    className="rounded cursor-pointer"
                  />
                </label>
              </WithTooltip>

              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                <label className="block">
                  <span className="app-metric-label">Delivery Radius (miles)</span>
                  <input
                    type="number"
                    step="0.1"
                    value={deliverySettings.delivery_radius}
                    onChange={(e) => handleDeliverySettingChange('delivery_radius', parseFloat(e.target.value))}
                    disabled={!deliverySettings.delivery_enabled}
                    className="mt-2 w-full rounded-md border border-gray-300 glassmorphism px-3 py-2 text-sm text-gray-800 disabled:bg-gray-100"
                  />
                </label>
                <label className="block">
                  <span className="app-metric-label">Flat Delivery Fee ($)</span>
                  <input
                    type="number"
                    step="0.01"
                    value={deliverySettings.delivery_fee}
                    onChange={(e) => handleDeliverySettingChange('delivery_fee', parseFloat(e.target.value))}
                    disabled={!deliverySettings.delivery_enabled}
                    className="mt-2 w-full rounded-md border border-gray-300 glassmorphism px-3 py-2 text-sm text-gray-800 disabled:bg-gray-100"
                  />
                </label>
              </div>
            </div>
          </div>

          <div className="app-panel">
            <div className="app-panel-header">
              <div>
                <div className="app-panel-title">AI Voice Receptionist</div>
                <div className="app-list-subtitle">Let OHC handle your business calls.</div>
              </div>
            </div>
            <div className="app-panel-body">
              <div className="space-y-4">
              <label className="flex items-center justify-between rounded-md border border-gray-200 p-3 text-sm text-gray-700">
                <span>Enable AI Voice Receptionist</span>
                <input
                  type="checkbox"
                  checked={voiceSettings.voice_receptionist_enabled}
                  onChange={(e) => handleVoiceSettingChange('voice_receptionist_enabled', e.target.checked)}
                  className="rounded"
                />
              </label>

              {voiceSettings.voice_receptionist_enabled && (
                <div className="space-y-4 border-t border-gray-200 pt-4">
                  <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                    <label className="block">
                      <span className="app-metric-label">Voice Persona</span>
                      <select
                        value={voiceSettings.voice_receptionist_persona}
                        onChange={(e) => handleVoiceSettingChange('voice_receptionist_persona', e.target.value)}
                        className="mt-2 w-full rounded-md border border-gray-300 glassmorphism px-3 py-2 text-sm text-gray-800"
                      >
                        <option value="Friendly">Friendly & Casual</option>
                        <option value="Professional">Professional & Crisp</option>
                        <option value="Efficient">Fast & Efficient</option>
                      </select>
                    </label>

                    <div className="block">
                      <span className="app-metric-label">Assigned Phone Number</span>
                      <div className="mt-2 flex gap-2">
                        <input
                          aria-label="Assigned Phone Number"
                          type="text"
                          readOnly
                          value={voiceSettings.voice_receptionist_number || "Not assigned"}
                          className="w-full rounded-md border border-gray-300 bg-gray-50 px-3 py-2 text-sm text-gray-500"
                        />
                        {!voiceSettings.voice_receptionist_number && (
                          <button onClick={handleProvisionVoiceNumber} className="app-button secondary whitespace-nowrap" type="button">
                            Get Number
                          </button>
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Agent Settings</div>
              <div className="app-list-subtitle">Configure your AI agent's identity.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <label className="block">
              <span className="app-metric-label">Agent Name</span>
              <input
                type="text"
                value={agentName}
                onChange={(e) => handleAgentNameChange(e.target.value)}
                className="mt-2 w-full rounded-md border border-gray-300 glassmorphism px-3 py-2 text-sm text-gray-800"
                placeholder="Agent One"
              />
            </label>
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Change Password</div>
            </div>
          </div>
          <div className="app-panel-body grid gap-3">
            <input aria-label="Current Password" type="password" placeholder="Current Password" className="rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
            <input aria-label="New Password" type="password" placeholder="New Password" className="rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
            <input aria-label="Confirm Password" type="password" placeholder="Confirm Password" className="rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
            <button onClick={() => router.push("/dashboard")} className="app-button primary w-fit" type="button">
              Save
            </button>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
