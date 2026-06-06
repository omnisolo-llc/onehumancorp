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

  const handleSave = async () => {
    if (!title) return;
    setSaved(true);
    try {
      const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'anon' : 'anon';
      await fetch('/api/onboarding/state', {
         method: 'POST',
         headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenant, 'X-User-ID': userId },
         body: JSON.stringify({ services: [{ title, description, price }] })
      });
    } catch (e) { console.error(e); }
    router.push('/dashboard');
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
    <div className="p-6 max-w-md mx-auto min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F]">
      <div className="flex items-center mb-6">
        <Link href="/services" className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Add Service</h1>
      </div>

      <div className="space-y-4 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] p-6 shadow-sm">
        <div>
          <label className="block text-sm font-semibold text-gray-700 dark:text-[#F5F5F7] mb-1">Service Title</label>
          <input
            type="text"
            value={title}
            onChange={e => setTitle(e.target.value)}
            className="w-full border border-white/40 dark:border-white/10 rounded-[8px] p-3 text-[#1D1D1F] dark:text-white bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] outline-none focus:border-[#0066FF]"
            placeholder="e.g. Weekly Music Tutoring"
            required
          />
        </div>

        <div>
          <div className="flex justify-between items-center mb-1">
            <label className="block text-sm font-semibold text-gray-700 dark:text-[#F5F5F7]">Description</label>
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
            className="w-full border border-white/40 dark:border-white/10 rounded-[8px] p-3 text-[#1D1D1F] dark:text-white bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] outline-none focus:border-[#0066FF] h-24"
            placeholder="Describe the service..."
          />
        </div>

        <div>
          <label className="block text-sm font-semibold text-gray-700 dark:text-[#F5F5F7] mb-1">Price</label>
          <div className="relative">
            <span className="absolute left-3 top-3 text-gray-500">$</span>
            <input
              type="number"
              value={price}
              onChange={e => setPrice(e.target.value)}
              className="w-full border border-white/40 dark:border-white/10 rounded-[8px] p-3 pl-8 text-[#1D1D1F] dark:text-white bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] outline-none focus:border-[#0066FF]"
              placeholder="0.00"
            />
          </div>
        </div>

        <div className="border-t border-white/40 dark:border-white/10 pt-4 mt-4">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Recurring Payment</h3>
              <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Automatically bill customers for this service.</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={isRecurring}
                onChange={() => setIsRecurring(!isRecurring)}
                className="sr-only peer"
              />
              <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-[#0066FF]"></div>
            </label>
          </div>

          {isRecurring && (
            <div className="bg-white/40 dark:bg-black/20 p-3 rounded-[8px] border border-white/40 dark:border-white/10">
              <label className="block text-sm font-semibold text-gray-700 dark:text-[#F5F5F7] mb-1">Billing Frequency</label>
              <select
                value={frequency}
                onChange={e => setFrequency(e.target.value)}
                className="w-full border border-white/40 dark:border-white/10 rounded-[8px] p-2 text-[#1D1D1F] dark:text-white bg-white dark:bg-black outline-none focus:border-[#0066FF]"
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
            className="w-full bg-[#0066FF] text-white font-bold py-3 rounded-[8px] hover:bg-[#0052cc] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
          >
            Save Service
          </button>
        </div>
      </div>
    </div>
  );
}
