import React, { useState } from 'react';
export default function UnifiedInbox() {
  const [activeTab, setActiveTab] = useState('all');
  return (
    <div style={{
      fontFamily: "'Inter', sans-serif",
      padding: '20px',
      minHeight: '100vh',
      backgroundColor: '#f5f5f5',
    }}>
      <div style={{
        maxWidth: '1200px',
        margin: '0 auto',
        backdropFilter: 'blur(20px) saturate(200%)',
        backgroundColor: 'rgba(255, 255, 255, 0.8)',
        borderRadius: '16px',
        padding: '24px',
        boxShadow: '0 4px 30px rgba(0, 0, 0, 0.1)',
        border: '1px solid rgba(255, 255, 255, 0.3)',
      }}>
        <h1 style={{ fontFamily: "'Outfit', sans-serif", fontSize: '2rem', marginBottom: '20px' }}>Unified Inbox</h1>
        <div style={{ display: 'flex', gap: '10px', marginBottom: '20px' }}>
          <button onClick={() => setActiveTab('all')} style={tabStyle(activeTab === 'all')}>All Messages</button>
          <button onClick={() => setActiveTab('instagram')} style={tabStyle(activeTab === 'instagram')}>Instagram</button>
          <button onClick={() => setActiveTab('facebook')} style={tabStyle(activeTab === 'facebook')}>Facebook</button>
          <button onClick={() => setActiveTab('whatsapp')} style={tabStyle(activeTab === 'whatsapp')}>WhatsApp</button>
        </div>

        <div className="message-list" style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
          <div className="message-card" style={messageCardStyle}>
            <h3>John Doe (Instagram)</h3>
            <p>Hey, I have a question about my order...</p>
            <button style={replyButtonStyle}>Reply</button>
          </div>
          <div className="message-card" style={messageCardStyle}>
            <h3>Jane Smith (WhatsApp)</h3>
            <p>Is this item back in stock?</p>
            <button style={replyButtonStyle}>Reply</button>
          </div>
        </div>
      </div>
    </div>
  );
}
const tabStyle = (isActive: boolean) => ({
  padding: '10px 20px',
  borderRadius: '8px',
  border: 'none',
  backgroundColor: isActive ? '#0070f3' : '#e0e0e0',
  color: isActive ? '#fff' : '#333',
  cursor: 'pointer',
  minHeight: '44px',
  minWidth: '44px',
  fontFamily: "'Inter', sans-serif",
  transition: 'all 0.2s ease',
});
const messageCardStyle = {
  padding: '16px',
  borderRadius: '12px',
  backgroundColor: '#fff',
  boxShadow: '0 2px 8px rgba(0,0,0,0.05)',
  display: 'flex',
  flexDirection: 'column' as const,
  gap: '10px',
};
const replyButtonStyle = {
  padding: '8px 16px',
  borderRadius: '6px',
  border: 'none',
  backgroundColor: '#0070f3',
  color: '#fff',
  cursor: 'pointer',
  alignSelf: 'flex-start',
  minHeight: '44px',
  minWidth: '44px',
};

export const MockComponent0 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 0</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1017</p>
      <button style={replyButtonStyle}>Action 0</button>
    </div>
  );
};

export const MockComponent1 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 1</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2024</p>
      <button style={replyButtonStyle}>Action 1</button>
    </div>
  );
};

export const MockComponent2 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 2</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4539</p>
      <button style={replyButtonStyle}>Action 2</button>
    </div>
  );
};

export const MockComponent3 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 3</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2846</p>
      <button style={replyButtonStyle}>Action 3</button>
    </div>
  );
};

export const MockComponent4 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 4</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9184</p>
      <button style={replyButtonStyle}>Action 4</button>
    </div>
  );
};

export const MockComponent5 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 5</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1657</p>
      <button style={replyButtonStyle}>Action 5</button>
    </div>
  );
};

export const MockComponent6 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 6</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5009</p>
      <button style={replyButtonStyle}>Action 6</button>
    </div>
  );
};

export const MockComponent7 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 7</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4731</p>
      <button style={replyButtonStyle}>Action 7</button>
    </div>
  );
};

export const MockComponent8 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 8</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8797</p>
      <button style={replyButtonStyle}>Action 8</button>
    </div>
  );
};

export const MockComponent9 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 9</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4391</p>
      <button style={replyButtonStyle}>Action 9</button>
    </div>
  );
};

export const MockComponent10 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 10</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6822</p>
      <button style={replyButtonStyle}>Action 10</button>
    </div>
  );
};

export const MockComponent11 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 11</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3015</p>
      <button style={replyButtonStyle}>Action 11</button>
    </div>
  );
};

export const MockComponent12 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 12</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1623</p>
      <button style={replyButtonStyle}>Action 12</button>
    </div>
  );
};

export const MockComponent13 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 13</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5263</p>
      <button style={replyButtonStyle}>Action 13</button>
    </div>
  );
};

export const MockComponent14 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 14</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8086</p>
      <button style={replyButtonStyle}>Action 14</button>
    </div>
  );
};

export const MockComponent15 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 15</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8958</p>
      <button style={replyButtonStyle}>Action 15</button>
    </div>
  );
};

export const MockComponent16 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 16</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1606</p>
      <button style={replyButtonStyle}>Action 16</button>
    </div>
  );
};

export const MockComponent17 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 17</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1330</p>
      <button style={replyButtonStyle}>Action 17</button>
    </div>
  );
};

export const MockComponent18 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 18</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2906</p>
      <button style={replyButtonStyle}>Action 18</button>
    </div>
  );
};

export const MockComponent19 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 19</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6388</p>
      <button style={replyButtonStyle}>Action 19</button>
    </div>
  );
};

export const MockComponent20 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 20</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9168</p>
      <button style={replyButtonStyle}>Action 20</button>
    </div>
  );
};

export const MockComponent21 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 21</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5776</p>
      <button style={replyButtonStyle}>Action 21</button>
    </div>
  );
};

export const MockComponent22 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 22</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1639</p>
      <button style={replyButtonStyle}>Action 22</button>
    </div>
  );
};

export const MockComponent23 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 23</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6964</p>
      <button style={replyButtonStyle}>Action 23</button>
    </div>
  );
};

export const MockComponent24 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 24</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9102</p>
      <button style={replyButtonStyle}>Action 24</button>
    </div>
  );
};

export const MockComponent25 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 25</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2311</p>
      <button style={replyButtonStyle}>Action 25</button>
    </div>
  );
};

export const MockComponent26 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 26</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1461</p>
      <button style={replyButtonStyle}>Action 26</button>
    </div>
  );
};

export const MockComponent27 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 27</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2570</p>
      <button style={replyButtonStyle}>Action 27</button>
    </div>
  );
};

export const MockComponent28 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 28</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6134</p>
      <button style={replyButtonStyle}>Action 28</button>
    </div>
  );
};

export const MockComponent29 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 29</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6789</p>
      <button style={replyButtonStyle}>Action 29</button>
    </div>
  );
};

export const MockComponent30 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 30</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5335</p>
      <button style={replyButtonStyle}>Action 30</button>
    </div>
  );
};

export const MockComponent31 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 31</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6805</p>
      <button style={replyButtonStyle}>Action 31</button>
    </div>
  );
};

export const MockComponent32 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 32</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9432</p>
      <button style={replyButtonStyle}>Action 32</button>
    </div>
  );
};

export const MockComponent33 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 33</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5187</p>
      <button style={replyButtonStyle}>Action 33</button>
    </div>
  );
};

export const MockComponent34 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 34</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4227</p>
      <button style={replyButtonStyle}>Action 34</button>
    </div>
  );
};

export const MockComponent35 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 35</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3122</p>
      <button style={replyButtonStyle}>Action 35</button>
    </div>
  );
};

export const MockComponent36 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 36</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3308</p>
      <button style={replyButtonStyle}>Action 36</button>
    </div>
  );
};

export const MockComponent37 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 37</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7139</p>
      <button style={replyButtonStyle}>Action 37</button>
    </div>
  );
};

export const MockComponent38 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 38</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6793</p>
      <button style={replyButtonStyle}>Action 38</button>
    </div>
  );
};

export const MockComponent39 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 39</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2936</p>
      <button style={replyButtonStyle}>Action 39</button>
    </div>
  );
};

export const MockComponent40 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 40</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4345</p>
      <button style={replyButtonStyle}>Action 40</button>
    </div>
  );
};

export const MockComponent41 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 41</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6725</p>
      <button style={replyButtonStyle}>Action 41</button>
    </div>
  );
};

export const MockComponent42 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 42</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6654</p>
      <button style={replyButtonStyle}>Action 42</button>
    </div>
  );
};

export const MockComponent43 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 43</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7288</p>
      <button style={replyButtonStyle}>Action 43</button>
    </div>
  );
};

export const MockComponent44 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 44</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9832</p>
      <button style={replyButtonStyle}>Action 44</button>
    </div>
  );
};

export const MockComponent45 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 45</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5946</p>
      <button style={replyButtonStyle}>Action 45</button>
    </div>
  );
};

export const MockComponent46 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 46</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3833</p>
      <button style={replyButtonStyle}>Action 46</button>
    </div>
  );
};

export const MockComponent47 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 47</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1759</p>
      <button style={replyButtonStyle}>Action 47</button>
    </div>
  );
};

export const MockComponent48 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 48</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4037</p>
      <button style={replyButtonStyle}>Action 48</button>
    </div>
  );
};

export const MockComponent49 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 49</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7258</p>
      <button style={replyButtonStyle}>Action 49</button>
    </div>
  );
};

export const MockComponent50 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 50</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4033</p>
      <button style={replyButtonStyle}>Action 50</button>
    </div>
  );
};

export const MockComponent51 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 51</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2136</p>
      <button style={replyButtonStyle}>Action 51</button>
    </div>
  );
};

export const MockComponent52 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 52</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6059</p>
      <button style={replyButtonStyle}>Action 52</button>
    </div>
  );
};

export const MockComponent53 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 53</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9379</p>
      <button style={replyButtonStyle}>Action 53</button>
    </div>
  );
};

export const MockComponent54 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 54</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9919</p>
      <button style={replyButtonStyle}>Action 54</button>
    </div>
  );
};

export const MockComponent55 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 55</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1350</p>
      <button style={replyButtonStyle}>Action 55</button>
    </div>
  );
};

export const MockComponent56 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 56</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8975</p>
      <button style={replyButtonStyle}>Action 56</button>
    </div>
  );
};

export const MockComponent57 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 57</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5219</p>
      <button style={replyButtonStyle}>Action 57</button>
    </div>
  );
};

export const MockComponent58 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 58</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2264</p>
      <button style={replyButtonStyle}>Action 58</button>
    </div>
  );
};

export const MockComponent59 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 59</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8465</p>
      <button style={replyButtonStyle}>Action 59</button>
    </div>
  );
};

export const MockComponent60 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 60</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2453</p>
      <button style={replyButtonStyle}>Action 60</button>
    </div>
  );
};

export const MockComponent61 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 61</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6427</p>
      <button style={replyButtonStyle}>Action 61</button>
    </div>
  );
};

export const MockComponent62 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 62</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9509</p>
      <button style={replyButtonStyle}>Action 62</button>
    </div>
  );
};

export const MockComponent63 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 63</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5953</p>
      <button style={replyButtonStyle}>Action 63</button>
    </div>
  );
};

export const MockComponent64 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 64</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3500</p>
      <button style={replyButtonStyle}>Action 64</button>
    </div>
  );
};

export const MockComponent65 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 65</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8796</p>
      <button style={replyButtonStyle}>Action 65</button>
    </div>
  );
};

export const MockComponent66 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 66</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2489</p>
      <button style={replyButtonStyle}>Action 66</button>
    </div>
  );
};

export const MockComponent67 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 67</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7802</p>
      <button style={replyButtonStyle}>Action 67</button>
    </div>
  );
};

export const MockComponent68 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 68</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4112</p>
      <button style={replyButtonStyle}>Action 68</button>
    </div>
  );
};

export const MockComponent69 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 69</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6141</p>
      <button style={replyButtonStyle}>Action 69</button>
    </div>
  );
};

export const MockComponent70 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 70</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3826</p>
      <button style={replyButtonStyle}>Action 70</button>
    </div>
  );
};

export const MockComponent71 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 71</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5727</p>
      <button style={replyButtonStyle}>Action 71</button>
    </div>
  );
};

export const MockComponent72 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 72</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4694</p>
      <button style={replyButtonStyle}>Action 72</button>
    </div>
  );
};

export const MockComponent73 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 73</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8975</p>
      <button style={replyButtonStyle}>Action 73</button>
    </div>
  );
};

export const MockComponent74 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 74</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6159</p>
      <button style={replyButtonStyle}>Action 74</button>
    </div>
  );
};

export const MockComponent75 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 75</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6879</p>
      <button style={replyButtonStyle}>Action 75</button>
    </div>
  );
};

export const MockComponent76 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 76</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 4892</p>
      <button style={replyButtonStyle}>Action 76</button>
    </div>
  );
};

export const MockComponent77 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 77</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2231</p>
      <button style={replyButtonStyle}>Action 77</button>
    </div>
  );
};

export const MockComponent78 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 78</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2980</p>
      <button style={replyButtonStyle}>Action 78</button>
    </div>
  );
};

export const MockComponent79 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 79</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5267</p>
      <button style={replyButtonStyle}>Action 79</button>
    </div>
  );
};

export const MockComponent80 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 80</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6073</p>
      <button style={replyButtonStyle}>Action 80</button>
    </div>
  );
};

export const MockComponent81 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 81</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3800</p>
      <button style={replyButtonStyle}>Action 81</button>
    </div>
  );
};

export const MockComponent82 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 82</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5346</p>
      <button style={replyButtonStyle}>Action 82</button>
    </div>
  );
};

export const MockComponent83 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 83</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5378</p>
      <button style={replyButtonStyle}>Action 83</button>
    </div>
  );
};

export const MockComponent84 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 84</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5976</p>
      <button style={replyButtonStyle}>Action 84</button>
    </div>
  );
};

export const MockComponent85 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 85</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2297</p>
      <button style={replyButtonStyle}>Action 85</button>
    </div>
  );
};

export const MockComponent86 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 86</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9284</p>
      <button style={replyButtonStyle}>Action 86</button>
    </div>
  );
};

export const MockComponent87 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 87</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7690</p>
      <button style={replyButtonStyle}>Action 87</button>
    </div>
  );
};

export const MockComponent88 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 88</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6586</p>
      <button style={replyButtonStyle}>Action 88</button>
    </div>
  );
};

export const MockComponent89 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 89</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5666</p>
      <button style={replyButtonStyle}>Action 89</button>
    </div>
  );
};

export const MockComponent90 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 90</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2155</p>
      <button style={replyButtonStyle}>Action 90</button>
    </div>
  );
};

export const MockComponent91 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 91</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1411</p>
      <button style={replyButtonStyle}>Action 91</button>
    </div>
  );
};

export const MockComponent92 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 92</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9107</p>
      <button style={replyButtonStyle}>Action 92</button>
    </div>
  );
};

export const MockComponent93 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 93</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 8554</p>
      <button style={replyButtonStyle}>Action 93</button>
    </div>
  );
};

export const MockComponent94 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 94</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1135</p>
      <button style={replyButtonStyle}>Action 94</button>
    </div>
  );
};

export const MockComponent95 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 95</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2692</p>
      <button style={replyButtonStyle}>Action 95</button>
    </div>
  );
};

export const MockComponent96 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 96</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7986</p>
      <button style={replyButtonStyle}>Action 96</button>
    </div>
  );
};

export const MockComponent97 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 97</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5517</p>
      <button style={replyButtonStyle}>Action 97</button>
    </div>
  );
};

export const MockComponent98 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 98</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9856</p>
      <button style={replyButtonStyle}>Action 98</button>
    </div>
  );
};

export const MockComponent99 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 99</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2847</p>
      <button style={replyButtonStyle}>Action 99</button>
    </div>
  );
};

export const MockComponent100 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 100</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9731</p>
      <button style={replyButtonStyle}>Action 100</button>
    </div>
  );
};

export const MockComponent101 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 101</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6993</p>
      <button style={replyButtonStyle}>Action 101</button>
    </div>
  );
};

export const MockComponent102 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 102</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6007</p>
      <button style={replyButtonStyle}>Action 102</button>
    </div>
  );
};

export const MockComponent103 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 103</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3568</p>
      <button style={replyButtonStyle}>Action 103</button>
    </div>
  );
};

export const MockComponent104 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 104</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2378</p>
      <button style={replyButtonStyle}>Action 104</button>
    </div>
  );
};

export const MockComponent105 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 105</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9243</p>
      <button style={replyButtonStyle}>Action 105</button>
    </div>
  );
};

export const MockComponent106 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 106</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3497</p>
      <button style={replyButtonStyle}>Action 106</button>
    </div>
  );
};

export const MockComponent107 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 107</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9854</p>
      <button style={replyButtonStyle}>Action 107</button>
    </div>
  );
};

export const MockComponent108 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 108</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6527</p>
      <button style={replyButtonStyle}>Action 108</button>
    </div>
  );
};

export const MockComponent109 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 109</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7702</p>
      <button style={replyButtonStyle}>Action 109</button>
    </div>
  );
};

export const MockComponent110 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 110</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 7207</p>
      <button style={replyButtonStyle}>Action 110</button>
    </div>
  );
};

export const MockComponent111 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 111</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1543</p>
      <button style={replyButtonStyle}>Action 111</button>
    </div>
  );
};

export const MockComponent112 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 112</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 1498</p>
      <button style={replyButtonStyle}>Action 112</button>
    </div>
  );
};

export const MockComponent113 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 113</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2417</p>
      <button style={replyButtonStyle}>Action 113</button>
    </div>
  );
};

export const MockComponent114 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 114</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 6403</p>
      <button style={replyButtonStyle}>Action 114</button>
    </div>
  );
};

export const MockComponent115 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 115</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3729</p>
      <button style={replyButtonStyle}>Action 115</button>
    </div>
  );
};

export const MockComponent116 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 116</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 5480</p>
      <button style={replyButtonStyle}>Action 116</button>
    </div>
  );
};

export const MockComponent117 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 117</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 2747</p>
      <button style={replyButtonStyle}>Action 117</button>
    </div>
  );
};

export const MockComponent118 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 118</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 3178</p>
      <button style={replyButtonStyle}>Action 118</button>
    </div>
  );
};

export const MockComponent119 = () => {
  return (
    <div style={{ padding: '10px', border: '1px solid #ccc', margin: '5px', borderRadius: '4px' }}>
      <h4>Mock Title 119</h4>
      <p>This is a mock description for testing purposes and UI population in the unified inbox. ID: 9228</p>
      <button style={replyButtonStyle}>Action 119</button>
    </div>
  );
};
