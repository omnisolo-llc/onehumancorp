"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";

export default function SettingsPage() {
  const router = useRouter();
  const [phone, setPhone] = useState("");
  const [otp, setOtp] = useState("");
  const [isVerifying, setIsVerifying] = useState(false);
  const [isVerified, setIsVerified] = useState(false);
  const [preferences, setPreferences] = useState({
    urgent_booking: false,
    failed_payment: false,
    new_order: false,
  });

  const [deliverySettings, setDeliverySettings] = useState({
    enabled: false,
    radius_miles: 5.0,
    flat_fee_cents: 850,
  });

  const handleVerify = async () => {
    setIsVerifying(true);
    try {
      const res = await fetch("/api/settings/sms-verify", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ phone }),
      });
      if (!res.ok) {
        alert("Failed to send verification SMS");
        setIsVerifying(false);
      }
    } catch {
      alert("Network error");
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
      } else {
        alert("Invalid OTP");
      }
    } catch {
      alert("Network error");
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
              <div className="app-panel-title">Critical SMS Alerts</div>
              <div className="app-list-subtitle">Immediate text alerts for urgent events.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="space-y-4">
              <input
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

              {isVerifying && !isVerified && (
                <div className="rounded-md border border-blue-100 bg-blue-50 p-3">
                  <p className="mb-2 text-sm text-blue-800">A 6-digit code has been sent. Enter it below:</p>
                  <div className="flex gap-2">
                    <input
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

        <section className="app-panel" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)' }}>
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title font-outfit">Local Delivery (DoorDash Drive)</div>
              <div className="app-list-subtitle">Offer white-label delivery to your customers.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="space-y-4">
              <label className="flex items-center justify-between rounded-xl border border-gray-100 p-4 text-sm text-gray-700 bg-white/40">
                <div className="flex flex-col">
                  <span className="font-semibold">Enable DoorDash Drive</span>
                  <span className="text-xs text-gray-500">Dashers will fulfill your local orders.</span>
                </div>
                <input
                  type="checkbox"
                  checked={deliverySettings.enabled}
                  onChange={(e) => setDeliverySettings({ ...deliverySettings, enabled: e.target.checked })}
                  className="w-5 h-5 rounded-md text-indigo-600 focus:ring-indigo-500"
                />
              </label>

              <div className="grid grid-cols-2 gap-4">
                <label className="block">
                  <span className="app-metric-label">Delivery Radius (miles)</span>
                  <input
                    type="number"
                    value={deliverySettings.radius_miles}
                    onChange={(e) => setDeliverySettings({ ...deliverySettings, radius_miles: parseFloat(e.target.value) })}
                    className="mt-2 w-full rounded-lg border border-gray-200 bg-white/80 px-3 py-2 text-sm text-gray-800 focus:ring-2 focus:ring-indigo-500 focus:outline-none"
                  />
                </label>
                <label className="block">
                  <span className="app-metric-label">Flat Fee (USD)</span>
                  <input
                    type="number"
                    value={deliverySettings.flat_fee_cents / 100}
                    onChange={(e) => setDeliverySettings({ ...deliverySettings, flat_fee_cents: Math.round(parseFloat(e.target.value) * 100) })}
                    className="mt-2 w-full rounded-lg border border-gray-200 bg-white/80 px-3 py-2 text-sm text-gray-800 focus:ring-2 focus:ring-indigo-500 focus:outline-none"
                  />
                </label>
              </div>

              <div className="p-3 bg-blue-50/50 rounded-lg border border-blue-100/50 flex items-start gap-3">
                 <div className="text-blue-500 mt-0.5">ℹ️</div>
                 <p className="text-xs text-blue-700 leading-relaxed">
                   DoorDash Drive charges a flat rate per delivery. You can choose to pass this fee to your customers or subsidize it.
                 </p>
              </div>

              <button
                onClick={async () => {
                  try {
                    const res = await fetch("/api/delivery/settings/org-1", {
                      method: "POST",
                      headers: { "Content-Type": "application/json" },
                      body: JSON.stringify(deliverySettings),
                    });
                    if (res.ok) {
                      alert('Delivery settings saved!');
                    } else {
                      alert('Failed to save settings');
                    }
                  } catch (e) {
                    alert('Network error');
                  }
                }}
                className="app-button primary w-full font-bold py-3 rounded-xl shadow-lg shadow-indigo-500/20"
                type="button"
              >
                Save Delivery Config
              </button>
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
