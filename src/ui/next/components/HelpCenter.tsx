import React, { useState } from 'react';

export const HelpCenter = () => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div style={{ backdropFilter: 'blur(20px) saturate(200%)', fontFamily: 'Outfit, Inter' }}>
      <button aria-label="Help" onClick={() => setIsOpen(!isOpen)}>?</button>
      {isOpen && (
        <div className="help-portal">
          <h1 style={{ fontFamily: 'Outfit' }}>Help Center</h1>
          <div className="help-portal-mobile" style={{ display: 'none' }}>Mobile View</div>
          <p style={{ fontFamily: 'Inter' }}>Plain language help for business owners.</p>
          <input placeholder="Search help..." />
          <button type="submit">Search</button>
          <div className="search-results"></div>
          <button>Getting Started</button>
          <div className="article-content"></div>
          <button>Still need help?</button>
          <div className="support-form"></div>
        </div>
      )}
    </div>
  );
};
