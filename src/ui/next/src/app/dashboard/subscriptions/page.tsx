"use client";

import React, { useState } from 'react';

export default function SubscriptionsDashboard() {
  const [plans, setPlans] = useState([]);
  const [showAdd, setShowAdd] = useState(false);

  const [formData, setFormData] = useState({
    name: '',
    price: '',
    billing_interval: 'month'
  });

  const handleSave = () => {
    setPlans([...plans, { ...formData, id: `plan-${Date.now()}` }]);
    setShowAdd(false);
  };

  return (
    <div className="p-4 sm:p-8 max-w-[1440px] mx-auto min-h-screen">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-8 gap-4">
        <h1 className="text-3xl font-bold tracking-tight">Subscriptions</h1>
        <button onClick={() => setShowAdd(true)} className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium">Add Product</button>
      </div>

      {showAdd ? (
        <div className="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-xl p-6 shadow-sm mb-8">
            <h2 className="text-xl font-semibold mb-4">Add Subscription Box</h2>
            <div className="space-y-4">
                <div className="space-y-2">
                    <label htmlFor="name" className="block text-sm font-medium">Plan Name</label>
                    <input id="name" name="name" value={formData.name} onChange={(e) => setFormData({...formData, name: e.target.value})} placeholder="e.g. Monthly Coffee Bean" className="flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500" />
                </div>
                <div className="space-y-2">
                    <label htmlFor="price" className="block text-sm font-medium">Price (USD)</label>
                    <input id="price" name="price" value={formData.price} onChange={(e) => setFormData({...formData, price: e.target.value})} type="number" placeholder="25" className="flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500" />
                </div>
                <div className="space-y-2">
                    <label htmlFor="billing_interval" className="block text-sm font-medium">Billing Interval</label>
                    <select id="billing_interval" name="billing_interval" value={formData.billing_interval} onChange={(e) => setFormData({...formData, billing_interval: e.target.value})} className="flex h-10 w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500">
                        <option value="week">Weekly</option>
                        <option value="month">Monthly</option>
                        <option value="year">Yearly</option>
                    </select>
                </div>
                <button onClick={handleSave} className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium">Save</button>
            </div>
        </div>
      ) : (
          <>
            {plans.length > 0 ? (
                <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative mb-4">
                    <span className="block sm:inline">Subscription saved successfully</span>
                </div>
            ) : null}
            <div className="space-y-4">
              {plans.map((plan) => (
                <div key={plan.id} className="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-xl p-4 shadow-sm flex justify-between items-center">
                    <div>
                      <h3 className="font-bold text-gray-900 dark:text-white">{plan.name}</h3>
                      <p className="text-sm text-gray-500">${plan.price} / {plan.billing_interval}</p>
                    </div>
                </div>
              ))}
              {plans.length === 0 && (
                <div className="text-center py-12 text-gray-500">
                  No subscriptions yet. Click Add Product to create one.
                </div>
              )}
            </div>
          </>
      )}
    </div>
  );
}
