import { useState } from 'react';
import { hireAgent } from '../api';

interface HireAgentModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
}

const ROLES = [
  'SOFTWARE_ENGINEER',
  'PRODUCT_MANAGER',
  'DESIGNER',
  'QA_TESTER',
  'SECURITY_ENGINEER'
];

const HireAgentModal = ({ isOpen, onClose, onSuccess }: HireAgentModalProps) => {
  const [name, setName] = useState('');
  const [role, setRole] = useState(ROLES[0]);
  const [providerType, setProviderType] = useState('builtin');
  const [isHiring, setIsHiring] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) {
      setError('Name is required');
      return;
    }

    try {
      setIsHiring(true);
      setError(null);
      await hireAgent(name, role, providerType);
      onSuccess();
      onClose();
      setName('');
      setRole(ROLES[0]);
    } catch (err: any) {
      setError(err.message || 'Failed to hire agent');
    } finally {
      setIsHiring(false);
    }
  };

  return (
    <div className="modal-overlay">
      <div className="glass-panel modal-content">
        <div className="modal-header">
          <h2>Hire New Agent</h2>
          <p>Deploy a new AI worker to your organization.</p>
        </div>

        {error && (
          <div style={{ color: 'var(--danger)', marginBottom: '16px', fontSize: '14px' }}>
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label>Agent Name</label>
            <input
              type="text"
              className="form-control"
              placeholder="e.g. Senior Software Engineer"
              value={name}
              onChange={(e) => setName(e.target.value)}
              disabled={isHiring}
            />
          </div>

          <div className="form-group">
            <label>Role</label>
            <select
              className="form-control"
              value={role}
              onChange={(e) => setRole(e.target.value)}
              disabled={isHiring}
            >
              {ROLES.map(r => (
                <option key={r} value={r}>{r.replace('_', ' ')}</option>
              ))}
            </select>
          </div>

          <div className="form-group">
            <label>Provider Type</label>
            <select
              className="form-control"
              value={providerType}
              onChange={(e) => setProviderType(e.target.value)}
              disabled={isHiring}
            >
              <option value="builtin">Built-in (Default)</option>
              <option value="openclaw">OpenClaw (Local)</option>
              <option value="minimax">Minimax</option>
            </select>
          </div>

          <div className="modal-actions">
            <button
              type="button"
              className="btn"
              onClick={onClose}
              disabled={isHiring}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="btn btn-primary"
              disabled={isHiring}
            >
              {isHiring ? 'Deploying...' : 'Deploy Agent'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default HireAgentModal;