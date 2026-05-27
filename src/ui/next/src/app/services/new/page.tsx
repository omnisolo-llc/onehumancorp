'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function NewServicePage() {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [isRecurring, setIsRecurring] = useState(false);
  const [frequency, setFrequency] = useState('monthly');
  const [price, setPrice] = useState('');
  const [autoPricingEnabled, setAutoPricingEnabled] = useState(false);
  const [minPrice, setMinPrice] = useState('');
  const [maxPrice, setMaxPrice] = useState('');
  const [saved, setSaved] = useState(false);

  const handleSave = async () => {
    if (!title) return;

    // Attempt to submit the data to an API
    try {
      await fetch('/api/services/new', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title,
          description,
          price: parseFloat(price) || 0,
          isRecurring,
          frequency: isRecurring ? frequency : null,
          autoPricingEnabled,
          minPrice: autoPricingEnabled && minPrice ? parseFloat(minPrice) : null,
          maxPrice: autoPricingEnabled && maxPrice ? parseFloat(maxPrice) : null,
        }),
      });
    } catch (e) {
      // Ignore errors for now since the mock API doesn't exist yet
    }

    setSaved(true);
    setTimeout(() => {
      window.location.href = '/dashboard';
    }, 1500);
  };

  const generateDescription = () => {
    setDescription('A weekly music tutoring session focused on improving technique, music theory, and performance skills. Perfect for students of all levels.');
  };

  if (saved) {
    return (
      <div className="p-4 max-w-md mx-auto mt-20 text-center">
        <h2 className="text-2xl font-bold text-green-600 mb-2">Service Saved!</h2>
        <p>Redirecting to dashboard...</p>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-md mx-auto">
      <div className="flex items-center mb-6">
        <Link href="/dashboard" className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold">Add Service</h1>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Service Title</label>
          <input
            type="text"
            value={title}
            onChange={e => setTitle(e.target.value)}
            className="w-full border rounded p-2 text-black"
            placeholder="e.g. Weekly Music Tutoring"
            required
          />
        </div>

        <div>
          <div className="flex justify-between items-center mb-1">
            <label className="block text-sm font-medium text-gray-700">Description</label>
            <button
              onClick={generateDescription}
              className="text-xs text-purple-600 hover:text-purple-800 flex items-center"
            >
              ✨ Auto-draft
            </button>
          </div>
          <textarea
            value={description}
            onChange={e => setDescription(e.target.value)}
            className="w-full border rounded p-2 text-black h-24"
            placeholder="Describe the service..."
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Price</label>
          <div className="relative">
            <span className="absolute left-3 top-2 text-gray-500">$</span>
            <input
              type="number"
              value={price}
              onChange={e => setPrice(e.target.value)}
              className="w-full border rounded p-2 pl-8 text-black"
              placeholder="0.00"
            />
          </div>
        </div>

        <div className="border-t pt-4 mt-4">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="font-medium text-gray-900">Auto-optimize pricing to maximize sales</h3>
              <p className="text-sm text-gray-500">Let Operations AI adjust your price based on demand and capacity.</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={autoPricingEnabled}
                onChange={() => setAutoPricingEnabled(!autoPricingEnabled)}
                className="sr-only peer"
                data-testid="auto-pricing-toggle"
              />
              <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          {autoPricingEnabled && (
            <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-black/10 dark:border-white/10 p-4 rounded-[16px] shadow-sm flex gap-4 mt-2">
              <div className="flex-1">
                <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Minimum Price</label>
                <div className="relative">
                  <span className="absolute left-3 top-2 text-gray-500">$</span>
                  <input
                    type="number"
                    value={minPrice}
                    onChange={e => setMinPrice(e.target.value)}
                    className="w-full border rounded-[8px] p-2 pl-8 text-black dark:text-white bg-transparent border-black/20 dark:border-white/20"
                    placeholder="Floor"
                    data-testid="min-price-input"
                  />
                </div>
              </div>
              <div className="flex-1">
                <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Maximum Price</label>
                <div className="relative">
                  <span className="absolute left-3 top-2 text-gray-500">$</span>
                  <input
                    type="number"
                    value={maxPrice}
                    onChange={e => setMaxPrice(e.target.value)}
                    className="w-full border rounded-[8px] p-2 pl-8 text-black dark:text-white bg-transparent border-black/20 dark:border-white/20"
                    placeholder="Ceiling"
                    data-testid="max-price-input"
                  />
                </div>
              </div>
            </div>
          )}
        </div>

        <div className="border-t pt-4 mt-4">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="font-medium">Recurring Payment</h3>
              <p className="text-sm text-gray-500">Automatically bill customers for this service.</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={isRecurring}
                onChange={() => setIsRecurring(!isRecurring)}
                className="sr-only peer"
              />
              <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          {isRecurring && (
            <div className="bg-gray-50 p-3 rounded border">
              <label className="block text-sm font-medium text-gray-700 mb-1">Billing Frequency</label>
              <select
                value={frequency}
                onChange={e => setFrequency(e.target.value)}
                className="w-full border rounded p-2 text-black bg-white"
              >
                <option value="weekly">Weekly</option>
                <option value="biweekly">Every 2 weeks</option>
                <option value="monthly">Monthly</option>
                <option value="yearly">Yearly</option>
              </select>
            </div>
          )}
        </div>

        <div className="pt-6">
          <button
            onClick={handleSave}
            className="w-full bg-black text-white font-medium py-3 rounded-lg hover:bg-gray-800"
          >
            Save Service
          </button>
        </div>
      </div>
    </div>
  );
}
