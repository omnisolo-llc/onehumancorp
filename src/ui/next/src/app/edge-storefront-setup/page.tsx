'use client';

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

export default function EdgeStorefrontSetupPage() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState(false);

  const [domain, setDomain] = useState('');
  const [theme, setTheme] = useState('light');
  const [products, setProducts] = useState('');

  const session = { tenant_id: 'default' }; // Dummy auth since AuthProvider is missing

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    // Simulate API call
    setTimeout(() => {
      setLoading(false);
      setSuccess(true);
      setTimeout(() => router.push('/dashboard'), 2000);
    }, 1000);
  };

  return (
    <AppShell title="Edge Storefront Setup">
      <div className="max-w-md mx-auto p-6 bg-white dark:bg-gray-800 rounded-xl shadow-sm">
        <h2 className="text-xl font-semibold mb-4 text-gray-900 dark:text-white">Configure your Edge Storefront</h2>

        {success ? (
          <div className="p-4 bg-green-50 text-green-700 rounded-md">
            Storefront configured successfully! Redirecting...
          </div>
        ) : (
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Custom Domain (Optional)
              </label>
              <input
                type="text"
                value={domain}
                onChange={(e) => setDomain(e.target.value)}
                placeholder="shop.yourbusiness.com"
                className="w-full px-3 py-2 border rounded-md"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Theme
              </label>
              <select
                value={theme}
                onChange={(e) => setTheme(e.target.value)}
                className="w-full px-3 py-2 border rounded-md"
              >
                <option value="light">Light (Default)</option>
                <option value="dark">Dark</option>
                <option value="system">System Preference</option>
              </select>
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full py-2 px-4 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors"
            >
              {loading ? 'Configuring...' : 'Deploy Storefront'}
            </button>
          </form>
        )}
      </div>
    </AppShell>
  );
}
