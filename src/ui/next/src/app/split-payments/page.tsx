"use client";

import React, { useState, useEffect } from 'react';

export default function SplitPaymentsPage() {
  const [rules, setRules] = useState([]);
  const [loading, setLoading] = useState(true);
  const [productId, setProductId] = useState('');
  const [partnerPhone, setPartnerPhone] = useState('');
  const [splitType, setSplitType] = useState('percentage');
  const [splitValue, setSplitValue] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  useEffect(() => {
    fetchRules();
  }, []);

  const fetchRules = async () => {
    try {
      const res = await fetch('/api/v1/split-rules');
      if (res.ok) {
        const data = await res.json();
        setRules(data);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const addRule = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSuccess('');

    try {
      const res = await fetch('/api/v1/split-rules', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          id: "",
          product_id: productId,
          partner_id: `partner_${Date.now()}`,
          partner_phone_or_email: partnerPhone,
          split_type: splitType,
          split_value: splitValue
        })
      });

      if (!res.ok) {
        throw new Error('Failed to add split rule');
      }

      setSuccess('Split partner added successfully!');
      setPartnerPhone('');
      setSplitValue('');
      fetchRules();
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <div className="p-4 w-full max-w-[375px] mx-auto min-h-screen bg-[rgba(22,22,26,0.7)] text-[#F5F5F7] font-inter backdrop-blur-[30px]">
      <h1 className="text-2xl font-bold font-outfit mb-6 text-white">Add Partner Split</h1>

      {error && <div className="text-red-500 mb-4">{error}</div>}
      {success && <div className="text-green-500 mb-4">{success}</div>}

      <form onSubmit={addRule} className="space-y-4 bg-[rgba(255,255,255,0.05)] p-4 rounded-xl border border-[rgba(255,255,255,0.1)]">
        <div>
          <label className="block mb-1 text-sm text-[rgba(255,255,255,0.7)]">Product/Service ID</label>
          <input
            type="text"
            value={productId}
            onChange={(e) => setProductId(e.target.value)}
            className="w-full p-2 bg-[rgba(0,0,0,0.3)] border border-[rgba(255,255,255,0.1)] rounded-lg text-white"
            required
          />
        </div>

        <div>
          <label className="block mb-1 text-sm text-[rgba(255,255,255,0.7)]">Partner Phone/Email</label>
          <input
            type="text"
            value={partnerPhone}
            onChange={(e) => setPartnerPhone(e.target.value)}
            className="w-full p-2 bg-[rgba(0,0,0,0.3)] border border-[rgba(255,255,255,0.1)] rounded-lg text-white"
            required
          />
        </div>

        <div>
          <label className="block mb-1 text-sm text-[rgba(255,255,255,0.7)]">Split Type</label>
          <select
            value={splitType}
            onChange={(e) => setSplitType(e.target.value)}
            className="w-full p-2 bg-[rgba(0,0,0,0.3)] border border-[rgba(255,255,255,0.1)] rounded-lg text-white"
          >
            <option value="percentage">Percentage (%)</option>
            <option value="flat">Flat Amount ($)</option>
          </select>
        </div>

        <div>
          <label className="block mb-1 text-sm text-[rgba(255,255,255,0.7)]">Split Value</label>
          <input
            type="number"
            value={splitValue}
            onChange={(e) => setSplitValue(e.target.value)}
            className="w-full p-2 bg-[rgba(0,0,0,0.3)] border border-[rgba(255,255,255,0.1)] rounded-lg text-white"
            required
            min="0"
            step="0.01"
          />
        </div>

        <button
          type="submit"
          className="w-full py-3 mt-4 bg-[#0071E3] text-white rounded-lg font-medium hover:bg-[#005bb5]"
        >
          Add Partner Split
        </button>
      </form>

      <div className="mt-8">
        <h2 className="text-xl font-bold font-outfit mb-4">Current Splits</h2>
        {loading ? (
          <p>Loading...</p>
        ) : (
          <ul className="space-y-2">
            {rules.map((rule: any) => (
              <li key={rule.id} className="p-3 bg-[rgba(255,255,255,0.05)] rounded-lg flex justify-between">
                <span>{rule.partner_phone_or_email}</span>
                <span className="font-bold text-[#0071E3]">
                  {rule.split_type === 'percentage' ? `${rule.split_value}%` : `$${rule.split_value}`}
                </span>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}
