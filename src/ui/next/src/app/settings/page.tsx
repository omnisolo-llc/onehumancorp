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

  useEffect(() => {
    fetch("/api/settings/delivery")
      .then(res => res.json())
      .then(data => {
         setDeliverySettings({
           delivery_enabled: data.delivery_enabled || false,
           delivery_radius: data.delivery_radius || 5.0,
           delivery_fee: data.delivery_fee || 8.50,
         });
      })
      .catch(e => console.error("Failed to load delivery settings", e));

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
      .catch(e => console.error("Failed to load voice settings", e));
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

  return (
    <AppShell
      title="Settings"
      subtitle="Application preferences, notification channels, and account controls."
      statusItems={[
        { label: "SMS", value: isVerified ? "Verified" : "Not verified", tone: isVerified ? "good" : "neutral" },
        { label: "Alerts", value: String(Object.values(preferences).filter(Boolean).length), tone: "neutral" },
      ]}
      actions={[{ label: "Dashboard", href: "/dashboard" }]}
    >
      <div id="settings-screen" className="app-grid two">
        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">General Notifications</div>
              <div className="app-list-subtitle">Baseline notification preferences.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="space-y-4">
              <label className="flex items-center gap-2 text-sm text-gray-700">
                <input type="checkbox" className="rounded" /> Enable Email Notifications
              </label>
              <label className="flex items-center gap-2 text-sm text-gray-700">
                <input type="checkbox" className="rounded" /> Enable Push Notifications
              </label>
              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                <label className="block">
                  <span className="app-metric-label">Timezone</span>
                  <select className="mt-2 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-800">
                    <option>UTC</option>
                    <option>EST</option>
                    <option>PST</option>
                  </select>
                </label>
                <label className="block">
                  <span className="app-metric-label">Language</span>
                  <select className="mt-2 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-800">
                    <option>English</option>
                    <option>Spanish</option>
                  </select>
                </label>
              </div>
            </div>
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <WithTooltip id="settings-sms-alerts" defaultText="Get text messages for important events like new orders or failed payments."><div className="app-panel-title">Critical SMS Alerts</div></WithTooltip>
              <div className="app-list-subtitle">Immediate text alerts for urgent events.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="space-y-4">
              <input
                aria-label="Mobile Phone Number"
                type="text"
                placeholder="Mobile Phone Number (e.g. +1234567890)"
                value={phone}
                onChange={(e) => setPhone(e.target.value)}
                disabled={isVerified}
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800"
              />
              {!isVerifying && !isVerified && (
                <button onClick={handleVerify} className="app-button primary" type="button">
                  Verify Number
                </button>
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
                    <button onClick={handleConfirm} className="app-button primary" type="button">
                      Confirm OTP
                    </button>
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
                  <label key={key} className="flex items-center justify-between rounded-md border border-gray-200 p-3 text-sm text-gray-700">
                    <span>{label}</span>
                    <input
                      aria-label={label}
                      type="checkbox"
                      checked={preferences[key as keyof typeof preferences]}
                      onChange={(e) => handlePreferenceChange(key, e.target.checked)}
                      disabled={!isVerified}
                      className="rounded"
                    />
                  </label>
                ))}
              </div>
            </div>
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <WithTooltip id="settings-doordash" defaultText="Offer local delivery powered by DoorDash drivers without a DoorDash storefront."><div className="app-panel-title">Local Delivery (DoorDash Drive)</div></WithTooltip>
              <div className="app-list-subtitle">Configure white-label delivery powered by DoorDash.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="space-y-4">
              <label className="flex items-center justify-between rounded-md border border-gray-200 p-3 text-sm text-gray-700">
                <span>Enable Local Delivery</span>
                <input
                  aria-label="Enable Local Delivery"
                  type="checkbox"
                  checked={deliverySettings.delivery_enabled}
                  onChange={(e) => handleDeliverySettingChange('delivery_enabled', e.target.checked)}
                  className="rounded"
                />
              </label>

              <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                <label className="block">
                  <span className="app-metric-label">Delivery Radius (miles)</span>
                  <input
                    type="number"
                    step="0.1"
                    value={deliverySettings.delivery_radius}
                    onChange={(e) => handleDeliverySettingChange('delivery_radius', parseFloat(e.target.value))}
                    disabled={!deliverySettings.delivery_enabled}
                    className="mt-2 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-800 disabled:bg-gray-100"
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
                    className="mt-2 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-800 disabled:bg-gray-100"
                  />
                </label>
              </div>
            </div>
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <WithTooltip id="settings-voice-receptionist" defaultText="An AI phone agent that answers calls and books appointments for you."><div className="app-panel-title">Autonomous Voice Receptionist</div></WithTooltip>
              <div className="app-list-subtitle">Never miss a call. Let AI answer, book appointments, and take orders.</div>
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
                        className="mt-2 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-800"
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
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div className="app-panel-title">Profile</div>
          </div>
          <div className="app-panel-body grid gap-3">
            <input type="text" placeholder="Display Name" className="rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
            <textarea placeholder="Bio" className="h-24 rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div className="app-panel-title">Security</div>
          </div>
          <div className="app-panel-body grid gap-3">
            <input type="password" placeholder="Current Password" className="rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
            <input type="password" placeholder="New Password" className="rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
            <input type="password" placeholder="Confirm Password" className="rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-800" />
            <button onClick={() => router.push("/dashboard")} className="app-button primary w-fit" type="button">
              Save
            </button>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
