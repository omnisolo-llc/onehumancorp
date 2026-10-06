'use client';
import { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function NewServicePage() {
  const router = useRouter();
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [isRecurring, setIsRecurring] = useState(false);
  const [frequency, setFrequency] = useState('monthly');
  const [price, setPrice] = useState('');
  const [saved, setSaved] = useState(false);
  const [statusMessage, setStatusMessage] = useState('');

  const handleSave = async () => {
    if (!title.trim()) {
      setStatusMessage('Enter a service title before saving.');
      return;
    }
    try {
      const response = await fetch('/api/v1/onboarding/state', {
         method: 'POST',
         headers: { 'Content-Type': 'application/json' },
         body: JSON.stringify({ services: [{ title, description, price, isRecurring, frequency: isRecurring ? frequency : undefined }] })
      });
      if (!response.ok) {
        setStatusMessage('Failed to save service. Please try again.');
        return;
      }
      setSaved(true);
      setTimeout(() => {
        router.push('/dashboard');
      }, 1000);
    } catch (e) {
      console.error(e);
      setStatusMessage('Failed to save service. Please try again.');
      return;
    }
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
    <div className="p-6 max-w-md mx-auto mt-8 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] shadow-lg">
      <div className="flex items-center mb-6">

        <Link href="/dashboard" className="mr-4 text-[#0066FF] hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Add Service</h1>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">Service Title</label>
          <input
            type="text"
            value={title}
            onChange={e => setTitle(e.target.value)}
            className="w-full rounded-[8px] glass-control min-h-[44px] p-3 text-[#1D1D1F] dark:text-[#F5F5F7] font-inter placeholder-gray-500 bg-transparent outline-none focus:ring-2 focus:ring-[#0066FF]/50 transition-all"
            placeholder="e.g. Weekly Music Tutoring"
            required
          />
        </div>

        <div>
          <div className="flex justify-between items-center mb-1">
            <label className="block text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">Description</label>
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
            className="w-full rounded-[8px] glass-control min-h-[44px] p-3 text-[#1D1D1F] dark:text-[#F5F5F7] font-inter placeholder-gray-500 bg-transparent outline-none focus:ring-2 focus:ring-[#0066FF]/50 transition-all h-24 resize-y"
            placeholder="Describe the service..."
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">Price</label>
          <div className="relative">
            <span className="absolute left-3 top-2 text-[#86868B] dark:text-[#A1A1A6]">$</span>
            <input
              type="number"
              value={price}
              onChange={e => setPrice(e.target.value)}
              className="w-full rounded-[8px] glass-control min-h-[44px] p-3 pl-8 text-[#1D1D1F] dark:text-[#F5F5F7] font-inter placeholder-gray-500 bg-transparent outline-none focus:ring-2 focus:ring-[#0066FF]/50 transition-all"
              placeholder="0.00"
            />
          </div>
        </div>

        <div className="border-t border-white/40 dark:border-white/10 pt-4 mt-4">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">Recurring Payment</h3>
              <p className="text-sm text-[#86868B] dark:text-[#A1A1A6]">Automatically bill customers for this service.</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                aria-label="Recurring payment"
                type="checkbox"
                checked={isRecurring}
                onChange={() => setIsRecurring(!isRecurring)}
                className="sr-only peer"
              />
              <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-[#34C759]"></div>
            </label>
          </div>

          {isRecurring && (
            <div className="bg-gray-50 dark:bg-black/20 p-3 rounded-[8px] border border-white/40 dark:border-white/10">
              <label className="block text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">Billing Frequency</label>
              <select
                value={frequency}
                onChange={e => setFrequency(e.target.value)}
                className="w-full rounded-[8px] glass-control min-h-[44px] p-3 text-[#1D1D1F] dark:text-[#F5F5F7] font-inter bg-white/50 dark:bg-black/20 outline-none focus:ring-2 focus:ring-[#0066FF]/50 transition-all"
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
          {statusMessage && <p className="mb-3 text-sm font-medium text-red-600" role="status">{statusMessage}</p>}
          <button
            onClick={handleSave}
            className="w-full bg-[#0066FF] hover:bg-blue-700 text-white font-medium min-h-[44px] py-3 rounded-[8px] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] active:scale-[0.98] shadow-sm"
          >
            Save Service
          </button>
        </div>
      </div>
    </div>
  );
}
