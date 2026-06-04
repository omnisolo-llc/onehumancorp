"use client";
import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function SettingsPage() {
  const router = useRouter();

  const [phone, setPhone] = useState('');
  const [otp, setOtp] = useState('');
  const [isVerifying, setIsVerifying] = useState(false);
  const [isVerified, setIsVerified] = useState(false);
  const [preferences, setPreferences] = useState({
    urgent_booking: false,
    failed_payment: false,
    new_order: false
  });

  const handleVerify = async () => {
    setIsVerifying(true);
    try {
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
      alert("Network error");
      setIsVerifying(false);
    }
  };

  const handleConfirm = async () => {
    try {
      const res = await fetch('/api/settings/sms-confirm', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ phone, otp })
      });
      if (res.ok) {
        setIsVerified(true);
      } else {
        alert("Invalid OTP");
      }
    } catch (e) {
      alert("Network error");
    }
  };

  const handlePreferenceChange = async (key: string, checked: boolean) => {
    const newPrefs = { ...preferences, [key]: checked };
    setPreferences(newPrefs);
    if (isVerified) {
      try {
        await fetch('/api/settings/sms-preferences', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ phone, ...newPrefs })
        });
      } catch (e) {
        console.error("Failed to save preferences", e);
      }
    }
  };

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
            </div>
          </div>
        </section>

        {/* Global SMS Notifications for Critical Alerts */}
        <section className="mb-8 border-b pb-8">
          <h2 className="text-xl font-semibold mb-2 text-gray-800">Global SMS Notifications for Critical Alerts</h2>
          <p className="text-sm text-gray-500 mb-4">Get immediate text alerts for urgent business events.</p>

          <div className="space-y-4">
            <div className="flex flex-col gap-2 max-w-sm">
              <input
                type="text"
                placeholder="Mobile Phone Number (e.g. +1234567890)"
                value={phone}
                onChange={(e) => setPhone(e.target.value)}
                disabled={isVerified}
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
            </div>
          </div>
        </section>

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
  );
}
