"use client";

import React, { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
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
    voice_receptionist_instructions: "",
  });

  const [isLoading, setIsLoading] = useState(true);
  const [agentName, setAgentName] = useState("Agent One");
  const [seoReports, setSeoReports] = useState<any[]>([]);
  const [hitRate, setHitRate] = useState<string>("");
  const [enableLazyToolLoading, setEnableLazyToolLoading] = useState(false);
  const [productTelemetryEnabled, setProductTelemetryEnabled] = useState(false);
  const [twilioAccountSid, setTwilioAccountSid] = useState("");
  const [twilioAuthToken, setTwilioAuthToken] = useState("");
  const [twilioPhoneNumber, setTwilioPhoneNumber] = useState("");
  const [twilioStatus, setTwilioStatus] = useState<"idle" | "loading" | "success" | "error">("idle");

  const handleConnectWhatsApp = async () => {
    try {
      setTwilioStatus("loading");
      const res = await fetch("/api/v1/settings/integrations/whatsapp", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          api_token: twilioAccountSid,
          bot_token: twilioAuthToken,
          from_phone: twilioPhoneNumber,
        }),
      });
      if (res.ok) {
        setTwilioStatus("success");
      } else {
        setTwilioStatus("error");
      }
    } catch (e) {
      setTwilioStatus("error");
    }
  };



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
              voice_receptionist_instructions: data.voice_receptionist_instructions || "",
            });
          }
        })
        .catch(e => console.error("Failed to load voice settings", e)),

      fetch("/api/settings/telemetry")
        .then(res => res.json())
        .then(data => {
          if (data && data.product_telemetry_enabled !== undefined) {
            setProductTelemetryEnabled(data.product_telemetry_enabled);
          }
        })
        .catch(e => console.error("Failed to load telemetry settings", e)),

      fetch("/api/local_seo/discovery_report")
        .then(res => res.json())
        .then(data => {
          if (Array.isArray(data)) {
            setSeoReports(data);
            if (data.length > 0 && data[0].metrics && data[0].metrics.edge_cache_hit_rate !== undefined) {
              setHitRate(`${(data[0].metrics.edge_cache_hit_rate * 100).toFixed(1)}% Hit Rate`);
            } else {
              setHitRate(""); // No data available
            }
          }
        })
        .catch(e => console.error("Failed to load seo reports", e))
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

  const handleTelemetryChange = async (checked: boolean) => {
    setProductTelemetryEnabled(checked);
    try {
      await fetch("/api/settings/telemetry", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ product_telemetry_enabled: checked }),
      });
    } catch (e) {
      console.error("Failed to save telemetry settings", e);
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
      <div className="mx-auto max-w-4xl space-y-8 font-inter">
        <header className="mb-8 p-6 glassmorphism border border-white/40 dark:border-white/10 shadow-sm">
          <h1 className="text-3xl font-extrabold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Workspace Settings</h1>
          <p className="mt-2 text-sm text-gray-650 dark:text-gray-400">Manage integrations, local routing, communication rules, and advanced system security.</p>
        </header>


        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden mt-8">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Global Sales</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Multi-currency handling for international orders.</div>
            </div>
          </div>
          <div className="app-panel-body p-6">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-sm font-bold text-gray-900 dark:text-white">Automatically handle multi-currency payments and localize invoices.</h3>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" className="sr-only peer" defaultChecked />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
              </label>
            </div>
          </div>
        </section>


        {/* SMS Notifications Card */}
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">SMS Notifications & Security</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Get texts for critical events. Verify your phone number to enable.</div>
            </div>
          </div>
          <div className="app-panel-body p-6 space-y-6">
            <div className="space-y-4">
              <label className="block text-xs font-bold uppercase tracking-wider text-gray-500">Mobile Number</label>
              <div className="flex gap-3 max-w-md">
                <input
                  aria-label="Mobile Number"
                  type="tel"
                  placeholder="+1 (555) 000-0000"
                  value={phone}
                  onChange={(e) => setPhone(e.target.value)}
                  disabled={isVerified}
                  className="flex-1 rounded-xl border border-gray-200 px-4 py-3 text-sm text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none disabled:bg-gray-50 disabled:text-gray-400"
                />
                {!isVerifying && !isVerified && (
                  <WithTooltip id="settings-verify-tooltip" defaultText="Verify your number to receive critical notifications.">
                    <button onClick={handleVerify} className="px-5 py-3 bg-[#0f766e] hover:bg-[#0d645d] text-white font-bold rounded-xl shadow-md transition-all active:scale-95 text-xs whitespace-nowrap" type="button">
                      Verify Number
                    </button>
                  </WithTooltip>
                )}
              </div>

              {smsStatus && <p className="text-sm font-semibold text-[#0f766e] dark:text-[#6ac5bd]" role="status">⚡ {smsStatus}</p>}

              {isVerifying && !isVerified && (
                <div className="rounded-xl border border-teal-100 bg-teal-50/30 p-4 max-w-md animate-fade-in">
                  <p className="mb-3 text-xs font-semibold text-teal-800 dark:text-teal-200">A 6-digit code has been sent. Enter it below:</p>
                  <div className="flex gap-3">
                    <input
                      aria-label="Verification code"
                      type="text"
                      placeholder="123456"
                      value={otp}
                      onChange={(e) => setOtp(e.target.value)}
                      className="w-28 rounded-xl border border-gray-200 px-4 py-3 text-center text-sm font-mono text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none"
                    />
                    <WithTooltip id="settings-otp-tooltip" defaultText="Click to confirm the code sent to your phone.">
                      <button onClick={handleConfirm} className="px-5 py-3 bg-[#0f766e] hover:bg-[#0d645d] text-white font-bold rounded-xl shadow-md transition-all active:scale-95 text-xs" type="button">
                        Confirm OTP
                      </button>
                    </WithTooltip>
                  </div>
                </div>
              )}

              {isVerified && <span className="inline-flex items-center px-3 py-1 bg-green-50 text-green-700 rounded-full text-xs font-bold border border-green-200">✓ Number Verified</span>}

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 border-t border-gray-100 pt-6">
                {[
                  ["urgent_booking", "Urgent Bookings"],
                  ["failed_payment", "Failed Payments"],
                  ["new_order", "New Orders"],
                ].map(([key, label]) => (
                  <label key={key} className={`flex items-center gap-3 p-3 rounded-xl border transition-all cursor-pointer ${isVerified ? 'border-gray-100 hover:border-teal-200 bg-gray-50/30' : 'border-gray-100 opacity-60 cursor-not-allowed'}`}>
                    <input
                      aria-label={label}
                      type="checkbox"
                      disabled={!isVerified}
                      checked={(preferences as any)[key]}
                      onChange={(e) => handlePreferenceChange(key, e.target.checked)}
                      className="rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e] w-4 h-4 cursor-pointer disabled:cursor-not-allowed"
                    />
                    <span className={`text-sm font-medium ${isVerified ? "text-gray-800" : "text-gray-400"}`}>{label}</span>
                  </label>
                ))}
                <label className="flex items-center gap-3 p-3 rounded-xl border border-gray-100 hover:border-teal-200 bg-gray-50/30 cursor-pointer">
                  <input
                    aria-label="Enable Email Notifications"
                    type="checkbox"
                    checked={(preferences as any)["email_notifications"] || false}
                    onChange={(e) => handlePreferenceChange("email_notifications", e.target.checked)}
                    className="rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e] w-4 h-4 cursor-pointer"
                  />
                  <span className="text-sm font-medium text-gray-800">Email Notifications</span>
                </label>
                <label className="flex items-center gap-3 p-3 rounded-xl border border-gray-100 hover:border-teal-200 bg-gray-50/30 cursor-pointer">
                  <input
                    aria-label="Enable Push Notifications"
                    type="checkbox"
                    checked={(preferences as any)["push_notifications"] || false}
                    onChange={(e) => handlePreferenceChange("push_notifications", e.target.checked)}
                    className="rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e] w-4 h-4 cursor-pointer"
                  />
                  <span className="text-sm font-medium text-gray-800">Push Notifications</span>
                </label>
              </div>
            </div>
          </div>
        </section>

        {/* Local Delivery and Voice Receptionist side-by-side */}
        <section className="app-grid two gap-6">
          <div className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden flex flex-col justify-between">
            <div>
              <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
                <div>
                  <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Local Delivery Setup</div>
                  <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Configure delivery radius and rates.</div>
                </div>
              </div>
              <div className="app-panel-body p-6 space-y-4">
                <WithTooltip id="settings-delivery-tooltip" defaultText="Turn this on to offer local delivery to your customers.">
                  <label className="flex items-center justify-between rounded-xl border border-teal-55/60 p-4 text-sm font-medium text-gray-900 dark:text-white cursor-pointer bg-teal-50/10 hover:bg-teal-50/20 transition-colors">
                    <span>Enable Local Delivery</span>
                    <input
                      aria-label="Enable Local Delivery"
                      type="checkbox"
                      checked={deliverySettings.delivery_enabled}
                      onChange={(e) => handleDeliverySettingChange('delivery_enabled', e.target.checked)}
                      className="rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e] w-5 h-5 cursor-pointer"
                    />
                  </label>
                </WithTooltip>

                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <label className="block">
                    <span className="text-xs font-bold uppercase tracking-wider text-gray-400">Radius (miles)</span>
                    <input
                      type="number"
                      step="0.1"
                      value={deliverySettings.delivery_radius}
                      onChange={(e) => handleDeliverySettingChange('delivery_radius', parseFloat(e.target.value))}
                      disabled={!deliverySettings.delivery_enabled}
                      className="mt-2 w-full rounded-xl border border-gray-200 bg-white px-4 py-2.5 text-sm text-gray-800 disabled:bg-gray-50 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none"
                    />
                  </label>
                  <label className="block">
                    <span className="text-xs font-bold uppercase tracking-wider text-gray-400">Flat Fee ($)</span>
                    <input
                      type="number"
                      step="0.01"
                      value={deliverySettings.delivery_fee}
                      onChange={(e) => handleDeliverySettingChange('delivery_fee', parseFloat(e.target.value))}
                      disabled={!deliverySettings.delivery_enabled}
                      className="mt-2 w-full rounded-xl border border-gray-200 bg-white px-4 py-2.5 text-sm text-gray-800 disabled:bg-gray-50 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none"
                    />
                  </label>
                </div>
              </div>
            </div>
          </div>

          <div className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden flex flex-col justify-between">
            <div>
              <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
                <div>
                  <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">AI Voice Receptionist</div>
                  <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Let OHC handle your business calls.</div>
                </div>
              </div>
              <div className="app-panel-body p-6 space-y-4">
                <label className="flex items-center justify-between rounded-xl border border-teal-55/60 p-4 text-sm font-medium text-gray-900 dark:text-white cursor-pointer bg-teal-50/10 hover:bg-teal-50/20 transition-colors">
                  <span>Enable AI Voice Receptionist</span>
                  <input
                    type="checkbox"
                    checked={voiceSettings.voice_receptionist_enabled}
                    onChange={(e) => handleVoiceSettingChange('voice_receptionist_enabled', e.target.checked)}
                    className="rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e] w-5 h-5 cursor-pointer"
                  />
                </label>

                {voiceSettings.voice_receptionist_enabled && (
                  <div className="space-y-4 border-t border-gray-100 pt-4 animate-fade-in">
                    <div className="grid grid-cols-1 gap-4">
                                            <label className="block">
                        <span className="text-xs font-bold uppercase tracking-wider text-gray-400">Voice Persona</span>
                        <select
                          value={voiceSettings.voice_receptionist_persona}
                          onChange={(e) => handleVoiceSettingChange('voice_receptionist_persona', e.target.value)}
                          className="mt-2 w-full rounded-xl border border-gray-200 bg-white px-4 py-2.5 text-sm text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none"
                        >
                          <option value="Friendly">Friendly & Casual</option>
                          <option value="Professional">Professional & Crisp</option>
                          <option value="Efficient">Fast & Efficient</option>
                        </select>
                      </label>

                      <label className="block">
                        <span className="text-xs font-bold uppercase tracking-wider text-gray-400">Custom Instructions</span>
                        <textarea
                          value={voiceSettings.voice_receptionist_instructions || ""}
                          onChange={(e) => handleVoiceSettingChange('voice_receptionist_instructions', e.target.value)}
                          placeholder="e.g. Always mention today's special: Vegan Chocolate Cake"
                          rows={3}
                          className="mt-2 w-full rounded-xl border border-gray-200 bg-white px-4 py-2.5 text-sm text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none resize-none"
                        />
                      </label>

                      <div className="block">
                        <span className="text-xs font-bold uppercase tracking-wider text-gray-400">Assigned Number</span>
                        <div className="mt-2 flex gap-2">
                          <input
                            aria-label="Assigned Phone Number"
                            type="text"
                            readOnly
                            value={voiceSettings.voice_receptionist_number || "Not assigned"}
                            className="w-full rounded-xl border border-gray-200 bg-gray-55 px-4 py-2.5 text-sm text-gray-500 outline-none"
                          />
                          {!voiceSettings.voice_receptionist_number && (
                            <button onClick={handleProvisionVoiceNumber} className="px-4 py-2.5 bg-[#0f766e] hover:bg-[#0d645d] text-white font-bold rounded-xl shadow-md transition-all active:scale-95 text-xs whitespace-nowrap" type="button">
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

        {/* Agent Settings Card */}
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Agent Settings</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Configure your AI agent's identity.</div>
            </div>
          </div>
          <div className="app-panel-body p-6">
            <label className="block max-w-md">
              <span className="text-xs font-bold uppercase tracking-wider text-gray-400">Agent Name</span>
              <input
                type="text"
                value={agentName}
                onChange={(e) => handleAgentNameChange(e.target.value)}
                className="mt-2 w-full rounded-xl border border-gray-200 bg-white px-4 py-2.5 text-sm text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none"
                placeholder="Agent One"
              />
            </label>
          </div>
        </section>

        {/* Change Password Card */}
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Security & Credentials</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Update your account credentials regularly to keep data safe.</div>
            </div>
          </div>
          <div className="app-panel-body p-6 grid gap-4 max-w-md">
            <input aria-label="Current Password" type="password" placeholder="Current Password" className="rounded-xl border border-gray-200 px-4 py-3 text-sm text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none" />
            <input aria-label="New Password" type="password" placeholder="New Password" className="rounded-xl border border-gray-200 px-4 py-3 text-sm text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none" />
            <input aria-label="Confirm Password" type="password" placeholder="Confirm Password" className="rounded-xl border border-gray-200 px-4 py-3 text-sm text-gray-800 focus:border-[#0f766e] focus:ring-2 focus:ring-teal-100 transition-all outline-none" />
            <button onClick={() => router.push("/dashboard")} className="px-6 py-3 bg-[#0f766e] hover:bg-[#0d645d] text-white font-bold rounded-xl shadow-md transition-all active:scale-95 text-xs w-fit" type="button">
              Save New Password
            </button>
          </div>
        </section>


        {/* Agent Harness Settings Section */}
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden mt-8">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Agent Harness Settings</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Configure advanced agent harness mechanics like lazy tool loading.</div>
            </div>
          </div>
          <div className="app-panel-body p-6 space-y-4">
            <label className="flex items-center justify-between rounded-xl border border-gray-200 dark:border-gray-800 p-4 text-sm font-medium text-gray-900 dark:text-white cursor-pointer bg-white dark:bg-gray-900 transition-colors">
              <div>
                <span>Enable Lazy Tool Loading (Harness Thickness)</span>
                <p className="text-xs text-gray-500 font-normal mt-1">Reduces initial context size by lazy-loading tools only when needed. Achieves up to 95% context reduction.</p>
              </div>
              <input
                type="checkbox"
                aria-label="Enable Lazy Tool Loading"
                checked={enableLazyToolLoading}
                  onChange={(e) => setEnableLazyToolLoading(e.target.checked)}
                className="rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e] w-5 h-5 cursor-pointer"
              />
            </label>
          </div>
        </section>

        {/* Local Sovereignty Section */}
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden mt-8">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Local Sovereignty & Data Sharing</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Control your privacy and telemetry in Standalone Mode.</div>
            </div>
          </div>
          <div className="app-panel-body p-6 space-y-4">
            <label className="flex items-center justify-between rounded-xl border border-gray-200 dark:border-gray-800 p-4 text-sm font-medium text-gray-900 dark:text-white cursor-pointer bg-white dark:bg-gray-900 transition-colors">
              <div>
                <span>Enable Product Telemetry (Standalone Mode)</span>
                <p className="text-xs text-gray-500 font-normal mt-1">Shares anonymous usage data to help us improve OHC. Explicit opt-in required for Standalone Mode.</p>
              </div>
              <input
                type="checkbox"
                aria-label="Enable Product Telemetry (Standalone Mode)"
                checked={productTelemetryEnabled}
                onChange={(e) => handleTelemetryChange(e.target.checked)}
                className="rounded border-gray-300 text-[#0f766e] focus:ring-[#0f766e] w-5 h-5 cursor-pointer"
              />
            </label>
          </div>
        </section>

                {/* Integrations Section */}
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden mt-8">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Integrations</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">Connect third-party tools and channels.</div>
            </div>
          </div>
          <div className="app-panel-body p-6 space-y-4">
            <div>
              <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200 mb-2">WhatsApp Setup (Twilio)</h3>
              <p className="text-xs text-gray-500 mb-4">Connect your Twilio WhatsApp Business API to receive orders and inquiries directly in your Work Triage feed.</p>

              <div className="space-y-3">
                <input
                  type="text"
                  placeholder="Twilio Account SID"
                  value={twilioAccountSid}
                  onChange={(e) => setTwilioAccountSid(e.target.value)}
                  className="w-full text-sm bg-white/50 dark:bg-gray-800/50 border border-gray-200 dark:border-gray-700 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#0f766e]/50 transition-all placeholder-gray-400"
                />
                <input
                  type="password"
                  placeholder="Twilio Auth Token"
                  value={twilioAuthToken}
                  onChange={(e) => setTwilioAuthToken(e.target.value)}
                  className="w-full text-sm bg-white/50 dark:bg-gray-800/50 border border-gray-200 dark:border-gray-700 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#0f766e]/50 transition-all placeholder-gray-400"
                />
                <input
                  type="text"
                  placeholder="WhatsApp Phone Number (e.g., +1234567890)"
                  value={twilioPhoneNumber}
                  onChange={(e) => setTwilioPhoneNumber(e.target.value)}
                  className="w-full text-sm bg-white/50 dark:bg-gray-800/50 border border-gray-200 dark:border-gray-700 rounded-lg px-4 py-2.5 focus:outline-none focus:ring-2 focus:ring-[#0f766e]/50 transition-all placeholder-gray-400"
                />

                <button
                  onClick={handleConnectWhatsApp}
                  disabled={twilioStatus === 'loading'}
                  className="px-4 py-2 bg-[#0f766e] hover:bg-[#0f766e]/90 text-white font-bold rounded-lg shadow-sm transition-all active:scale-95 text-sm disabled:opacity-50"
                >
                  {twilioStatus === 'loading' ? 'Connecting...' : 'Connect WhatsApp'}
                </button>
                {twilioStatus === 'success' && <p className="text-xs text-green-600 mt-2">Connected successfully!</p>}
                {twilioStatus === 'error' && <p className="text-xs text-red-600 mt-2">Failed to connect. Please check credentials.</p>}
              </div>
            </div>
          </div>
        </section>

        {/* Advanced Section */}
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10 hover:shadow-md transition-all duration-300 overflow-hidden mt-8">
          <div className="app-panel-header border-b border-gray-100/50 bg-white/30 px-6 py-4">
            <div>
              <div className="app-panel-title text-base font-bold font-outfit text-gray-900 dark:text-white">Advanced Settings</div>
              <div className="text-xs text-[#0f766e] dark:text-[#6ac5bd] mt-1">For custom integrations and developers.</div>
            </div>
          </div>
          <div className="app-panel-body p-6 space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200">API Documentation</h3>
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">Interactive API reference for connecting external services to your workspace.</p>
              </div>
              <Link href="/api-docs" className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700 dark:text-gray-300 font-bold rounded-lg shadow-sm transition-all active:scale-95 text-xs">
                View API Docs
              </Link>
            </div>

            <div className="flex items-center justify-between pt-4 border-t border-gray-100 dark:border-gray-800">
              <div>
                <h3 className="text-sm font-semibold text-gray-800 dark:text-gray-200">Edge Cache & SEO Optimization</h3>
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">Autonomous edge caching and agentic SEO pre-rendering are actively managing your storefront.</p>
                <p className="text-xs text-gray-400 dark:text-gray-500 mt-1" id="seo-recent-updates">{seoReports.length > 0 ? seoReports[0].plain_language_summary : ""}</p>
              </div>
              {hitRate && (
                <div className="flex items-center space-x-2">
                  <span className="text-xs font-mono text-green-600 bg-green-50 dark:bg-green-900/30 px-2 py-1 rounded" id="edge-cache-hit-rate">{hitRate}</span>
                </div>
              )}
            </div>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
