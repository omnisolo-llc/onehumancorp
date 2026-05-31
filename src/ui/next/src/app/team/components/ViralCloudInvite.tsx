"use client";

import React, { useState } from 'react';

export default function ViralCloudInvite() {
  const [showModal, setShowModal] = useState(false);
  const [inviteLink, setInviteLink] = useState('');
  const [copied, setCopied] = useState(false);
  const [loading, setLoading] = useState(false);

  const handleOpenModal = async () => {
    setShowModal(true);
    setLoading(true);

    // Call the actual endpoint to generate a referral link
    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'default' : 'default';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'current-user' : 'current-user';

      const response = await fetch('/api/v1/growth/team-invites', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ team_id: tenantId, inviter_id: userId, invitee_id: 'new-collaborator' })
      });
      if (response.ok) {
        const data = await response.json();
        setInviteLink(`https://ohc.app/invite/${data.id || Date.now()}`);
      } else {
        setInviteLink(`https://ohc.app/invite/temp-${Date.now()}`);
      }
    } catch (e) {
      setInviteLink(`https://ohc.app/invite/temp-${Date.now()}`);
    } finally {
      setLoading(false);
    }
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(inviteLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="mt-8 mb-4">
      {/* Growth Card */}
      <div className="bg-white/65 backdrop-blur-[20px] saturate-[200%] border border-white/20 rounded-2xl p-6 shadow-lg shadow-indigo-100/50 flex flex-col items-center text-center relative overflow-hidden" style={{ background: 'linear-gradient(135deg, rgba(255,255,255,0.7) 0%, rgba(245,247,255,0.7) 100%)' }}>
        <div className="w-12 h-12 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-full flex items-center justify-center mb-4 shadow-md shadow-indigo-200">
          <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
        </div>
        <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Grow Your Team</h2>
        <p className="text-sm font-inter text-gray-600 mb-6 max-w-sm">
          Bring your team online easily. Share access to your workspace securely.
        </p>
        <button
          onClick={handleOpenModal}
          className="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold font-inter py-2.5 px-6 rounded-xl shadow-md transition-colors w-full sm:w-auto"
        >
          Invite Team Member
        </button>
      </div>

      {/* Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center px-4 bg-gray-900/40 backdrop-blur-sm">
          <div className="bg-white rounded-2xl shadow-2xl w-full max-w-md overflow-hidden border border-gray-100 relative">
            <button
              onClick={() => setShowModal(false)}
              className="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
              aria-label="Close Team Invite"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>

            <div className="p-6">
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Team Invite</h2>
              <p className="text-sm font-inter text-gray-600 mb-6">
                Share this secure link with your team member so they can collaborate with you online.
              </p>

              <div className="flex gap-2">
                <input
                  id="team-invite-link"
                  type="text"
                  readOnly
                  value={loading ? 'Generating link...' : inviteLink}
                  className="flex-1 bg-gray-50 border border-gray-200 rounded-xl px-4 py-2 text-sm font-inter text-gray-800 outline-none focus:border-indigo-300"
                />
                <button
                  onClick={handleCopy}
                  disabled={loading}
                  className={`px-4 py-2 rounded-xl text-sm font-semibold transition-colors flex items-center gap-1 ${
                    copied
                      ? 'bg-green-100 text-green-700 hover:bg-green-200'
                      : 'bg-indigo-50 text-indigo-700 hover:bg-indigo-100'
                  }`}
                >
                  {copied ? (
                    <>
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                      Copied!
                    </>
                  ) : (
                    <>
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                      Copy Link
                    </>
                  )}
                </button>
              </div>
            </div>

            <div className="bg-gray-50 px-6 py-4 border-t border-gray-100 flex justify-end">
              <button
                onClick={() => setShowModal(false)}
                className="text-sm font-semibold font-inter text-gray-600 hover:text-gray-800 px-4 py-2 rounded-lg"
              >
                Done
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
