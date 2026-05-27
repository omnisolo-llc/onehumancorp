'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function NewServicePage() {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [isRecurring, setIsRecurring] = useState(false);
  const [frequency, setFrequency] = useState('monthly');
  const [price, setPrice] = useState('');
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    if (!title) return;
    setSaved(true);
    window.location.href = '/dashboard';
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
