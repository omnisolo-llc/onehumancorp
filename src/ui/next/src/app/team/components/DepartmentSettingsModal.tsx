"use client";

import React, { useState, useEffect } from 'react';

type DepartmentSettingsModalProps = {
  departmentId: string;
  departmentName: string;
  onClose: () => void;
};

export default function DepartmentSettingsModal({ departmentId, departmentName, onClose }: DepartmentSettingsModalProps) {
  const [toneOfVoice, setToneOfVoice] = useState('professional');
  const [approvalMode, setApprovalMode] = useState('draft'); // 'draft' | 'auto'
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    const fetchConfig = async () => {
      try {
        const res = await fetch(`/api/agents/settings/${departmentId}`);
        if (res.ok) {
          const data = await res.json();
          setToneOfVoice(data.tone_of_voice || 'professional');
          setApprovalMode(data.auto_approve_limits > 0 ? 'auto' : 'draft');
        }
      } catch (e) {
        console.error("Failed to load settings", e);
      } finally {
        setLoading(false);
      }
    };
    fetchConfig();
  }, [departmentId]);

  const handleSave = async () => {
    setSaving(true);
    try {
      await fetch(`/api/agents/settings/${departmentId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          tone_of_voice: toneOfVoice,
          auto_approve_limits: approvalMode === 'auto' ? 100 : 0
        })
      });
      onClose();
    } catch (e) {
      console.error("Failed to save settings", e);
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
      <div className="bg-white rounded-2xl shadow-xl w-full max-w-md p-6 relative">
        <button
          onClick={onClose}
          className="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
        >
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
        </button>

        <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900">{departmentName} Settings</h2>

        {loading ? (
          <div className="flex justify-center py-8">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
          </div>
        ) : (
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Approval Mode</label>
              <div className="grid grid-cols-2 gap-3">
                <button
                  className={`p-3 border rounded-xl flex flex-col items-center justify-center transition-all ${approvalMode === 'draft' ? 'border-blue-500 bg-blue-50 text-blue-700 shadow-sm' : 'border-gray-200 text-gray-600 hover:bg-gray-50'}`}
                  onClick={() => setApprovalMode('draft')}
                >
                  <span className="font-semibold text-sm">Draft for Review</span>
                  <span className="text-xs opacity-80 mt-1 text-center">Approve every action</span>
                </button>
                <button
                  className={`p-3 border rounded-xl flex flex-col items-center justify-center transition-all ${approvalMode === 'auto' ? 'border-blue-500 bg-blue-50 text-blue-700 shadow-sm' : 'border-gray-200 text-gray-600 hover:bg-gray-50'}`}
                  onClick={() => setApprovalMode('auto')}
                >
                  <span className="font-semibold text-sm">Auto-pilot</span>
                  <span className="text-xs opacity-80 mt-1 text-center">Act autonomously</span>
                </button>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Tone of Voice</label>
              <select
                value={toneOfVoice}
                onChange={(e) => setToneOfVoice(e.target.value)}
                className="w-full bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5"
              >
                <option value="professional">Professional & Polite</option>
                <option value="friendly">Friendly & Casual</option>
                <option value="enthusiastic">Enthusiastic & Upbeat</option>
                <option value="formal">Formal & Direct</option>
              </select>
            </div>

            <button
              onClick={handleSave}
              disabled={saving}
              className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg text-sm px-5 py-3 text-center flex justify-center items-center"
            >
              {saving ? (
                <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
              ) : (
                "Save Preferences"
              )}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
