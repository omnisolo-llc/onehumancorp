"use client";
<<<<<<< HEAD

import { useState } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";

export default function SettingsPage() {
  const router = useRouter();
  const [phone, setPhone] = useState("");
  const [otp, setOtp] = useState("");
=======
import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function SettingsPage() {
  const router = useRouter();

  const [phone, setPhone] = useState('');
  const [otp, setOtp] = useState('');
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
  const [isVerifying, setIsVerifying] = useState(false);
  const [isVerified, setIsVerified] = useState(false);
  const [preferences, setPreferences] = useState({
    urgent_booking: false,
    failed_payment: false,
<<<<<<< HEAD
    new_order: false,
=======
    new_order: false
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
  });

  const handleVerify = async () => {
    setIsVerifying(true);
    try {
<<<<<<< HEAD
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
=======
      const res = await fetch('/api/settings/sms-verify', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ phone })
      });
      if (res.ok) {
        // Show OTP field
      } else {
        alert("Failed to send verification SMS");
        setIsVerifying(false);
      }
    } catch (e) {
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
      alert("Network error");
      setIsVerifying(false);
    }
  };

  const handleConfirm = async () => {
    try {
<<<<<<< HEAD
      const res = await fetch("/api/settings/sms-confirm", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ phone, otp }),
=======
      const res = await fetch('/api/settings/sms-confirm', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ phone, otp })
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
      });
      if (res.ok) {
        setIsVerified(true);
      } else {
        alert("Invalid OTP");
      }
<<<<<<< HEAD
    } catch {
=======
    } catch (e) {
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
      alert("Network error");
    }
  };

  const handlePreferenceChange = async (key: string, checked: boolean) => {
    const newPrefs = { ...preferences, [key]: checked };
    setPreferences(newPrefs);
    if (isVerified) {
      try {
<<<<<<< HEAD
        await fetch("/api/settings/sms-preferences", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ phone, ...newPrefs }),
=======
        await fetch('/api/settings/sms-preferences', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ phone, ...newPrefs })
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
        });
      } catch (e) {
        console.error("Failed to save preferences", e);
      }
    }
  };

<<<<<<< HEAD
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
=======
  const handleSave = () => {
    // Return to dashboard on save
    router.push('/dashboard');
  };

  return (
    <div id="settings-screen" className="min-h-screen bg-gray-50 p-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white rounded-xl shadow p-8">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold font-outfit text-gray-900">Settings</h1>
          <Link href="/dashboard" className="text-blue-600 hover:text-blue-800">Back to Dashboard</Link>
        </div>

        {/* General Notifications */}
        <section className="mb-8 border-b pb-8">
          <h2 className="text-xl font-semibold mb-4 text-gray-800">General Notifications</h2>
          <div className="space-y-4">
            <label className="flex items-center gap-2 text-gray-700">
              <input type="checkbox" className="rounded" /> Enable Email Notifications
            </label>
            <label className="flex items-center gap-2 text-gray-700">
              <input type="checkbox" className="rounded" /> Enable Push Notifications
            </label>

            <div>
              <p className="text-sm font-medium text-gray-700 mb-1">Timezone</p>
              <select className="border rounded px-3 py-2 w-full max-w-xs text-gray-700 bg-white">
                <option>UTC</option>
                <option>EST</option>
                <option>PST</option>
              </select>
            </div>

            <div>
              <p className="text-sm font-medium text-gray-700 mb-1">Language</p>
              <select className="border rounded px-3 py-2 w-full max-w-xs text-gray-700 bg-white">
                <option>English</option>
                <option>Spanish</option>
              </select>
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
            </div>
          </div>
        </section>

<<<<<<< HEAD
        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Critical SMS Alerts</div>
              <div className="app-list-subtitle">Immediate text alerts for urgent events.</div>
            </div>
          </div>
          <div className="app-panel-body">
            <div className="space-y-4">
=======
        {/* Global SMS Notifications for Critical Alerts */}
        <section className="mb-8 border-b pb-8">
          <h2 className="text-xl font-semibold mb-2 text-gray-800">Global SMS Notifications for Critical Alerts</h2>
          <p className="text-sm text-gray-500 mb-4">Get immediate text alerts for urgent business events.</p>

          <div className="space-y-4">
            <div className="flex flex-col gap-2 max-w-sm">
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
              <input
                type="text"
                placeholder="Mobile Phone Number (e.g. +1234567890)"
                value={phone}
                onChange={(e) => setPhone(e.target.value)}
                disabled={isVerified}
<<<<<<< HEAD
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
=======
                className="border rounded px-3 py-2 w-full text-gray-700"
              />
              {!isVerifying && !isVerified && (
                <button
                  onClick={handleVerify}
                  className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition-colors w-fit"
                >
                  Verify Number
                </button>
              )}
            </div>

            {isVerifying && !isVerified && (
              <div className="bg-blue-50 p-4 rounded-lg border border-blue-100 max-w-sm">
                <p className="text-sm text-blue-800 mb-2">A 6-digit code has been sent. Enter it below:</p>
                <div className="flex gap-2">
                  <input
                    type="text"
                    placeholder="123456"
                    value={otp}
                    onChange={(e) => setOtp(e.target.value)}
                    className="border rounded px-3 py-2 w-24 text-center text-gray-700"
                  />
                  <button
                    onClick={handleConfirm}
                    className="bg-green-600 hover:bg-green-700 text-white font-medium py-2 px-4 rounded transition-colors"
                  >
                    Confirm OTP
                  </button>
                </div>
              </div>
            )}

            {isVerified && (
              <div className="text-green-600 font-semibold flex items-center gap-2">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                Number Verified
              </div>
            )}

            <div className="space-y-2 mt-4">
              <label className="flex items-center gap-2 text-gray-700">
                <input
                  type="checkbox"
                  checked={preferences.urgent_booking}
                  onChange={(e) => handlePreferenceChange('urgent_booking', e.target.checked)}
                  disabled={!isVerified}
                  className="rounded"
                />
                Urgent Bookings
              </label>
              <label className="flex items-center gap-2 text-gray-700">
                <input
                  type="checkbox"
                  checked={preferences.failed_payment}
                  onChange={(e) => handlePreferenceChange('failed_payment', e.target.checked)}
                  disabled={!isVerified}
                  className="rounded"
                />
                Failed Payments
              </label>
              <label className="flex items-center gap-2 text-gray-700">
                <input
                  type="checkbox"
                  checked={preferences.new_order}
                  onChange={(e) => handlePreferenceChange('new_order', e.target.checked)}
                  disabled={!isVerified}
                  className="rounded"
                />
                New Orders
              </label>
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
            </div>
          </div>
        </section>

<<<<<<< HEAD
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
=======
        {/* Profile */}
        <section className="mb-8 border-b pb-8">
          <h2 className="text-xl font-semibold mb-4 text-gray-800">Profile</h2>
          <div className="space-y-4 max-w-sm">
            <input type="text" placeholder="Display Name" className="border rounded px-3 py-2 w-full text-gray-700" />
            <textarea placeholder="Bio" className="border rounded px-3 py-2 w-full h-24 text-gray-700" />
          </div>
        </section>

        {/* Security */}
        <section className="mb-8">
          <h2 className="text-xl font-semibold mb-4 text-gray-800">Security</h2>
          <div className="space-y-4 max-w-sm">
            <input type="password" placeholder="Current Password" className="border rounded px-3 py-2 w-full text-gray-700" />
            <input type="password" placeholder="New Password" className="border rounded px-3 py-2 w-full text-gray-700" />
            <input type="password" placeholder="Confirm Password" className="border rounded px-3 py-2 w-full text-gray-700" />
          </div>
        </section>

        <div className="flex gap-4 pt-4 border-t">
          <button
            onClick={handleSave}
            className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-6 rounded transition-colors"
          >
            Save
          </button>
        </div>
      </div>
    </div>
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
  );
}
