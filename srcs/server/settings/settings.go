package settings

import (
	"encoding/json"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/utils"
	"os"
	"path/filepath"
	"sync"
)

// AiProvider represents an AI service configuration.
// ProviderType matches the ohc.model.ProviderType proto enum values.
type AiProvider struct {
	ID           string   `json:"id,omitempty"`
	Name         string   `json:"name"`
	ProviderType int32    `json:"provider_type,omitempty"`
	APIKey       string   `json:"api_key,omitempty"`
	BaseURL      string   `json:"base_url,omitempty"`
	Model        string   `json:"model"`
	Models       []string `json:"models,omitempty"`
	Enabled      bool     `json:"enabled"`
	IsOfficial   bool     `json:"is_official,omitempty"`
}

// AppSettings represents the global configuration for the OHC platform.
type AppSettings struct {
	ListenAddr    string            `json:"listen_addr"`
	DBPath        string            `json:"db_path,omitempty"`
	PostgresURL   string            `json:"postgres_url,omitempty"`
	RedisURL      string            `json:"redis_url,omitempty"`
	CentrifugeURL string            `json:"centrifuge_url,omitempty"`
	MinimaxAPIKey string            `json:"minimax_api_key,omitempty"`
	AiProviders   []AiProvider      `json:"ai_providers"`
	Extras        map[string]string `json:"extras,omitempty"`
}

// DefaultSettings returns the default configuration.
// If MINIMAX_API_KEY is set, a MiniMax provider is pre-configured.
func DefaultSettings() AppSettings {
	providers := []AiProvider{}
	if key := os.Getenv("MINIMAX_API_KEY"); key != "" {
		providers = append(providers, AiProvider{
			ID:           "minimax-default",
			Name:         "MiniMax",
			ProviderType: 10, // PROVIDER_TYPE_MINIMAX from model.proto
			APIKey:       key,
			BaseURL:      "https://api.minimax.io/v1",
			Model:        "MiniMax-Text-01",
			Models:       []string{"MiniMax-Text-01", "abab6.5s-chat"},
			Enabled:      true,
			IsOfficial:   true,
		})
	}
	return AppSettings{
		ListenAddr:    "0.0.0.0:18789",
		DBPath:        "ohc.db",
		CentrifugeURL: "ws://localhost:8000/connection/websocket",
		AiProviders:   providers,
		Extras:        make(map[string]string),
	}
}

// Store handles persistence of AppSettings.
type Store struct {
	mu   sync.RWMutex
	data AppSettings
	path string
}

// NewStore creates an in-memory store.
func NewStore() *Store {
	return &Store{
		data: DefaultSettings(),
	}
}

// FromFile loads settings from a JSON file.
func FromFile(path string) (*Store, error) {
	s := &Store{path: path}
	if _, err := os.Stat(path); os.IsNotExist(err) {
		s.data = DefaultSettings()
		return s, nil
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to read settings file: %w", err)
	}

	if err := json.Unmarshal(data, &s.data); err != nil {
		return nil, fmt.Errorf("failed to unmarshal settings: %w", err)
	}

	return s, nil
}

// Save persists the current settings to disk.
func (s *Store) Save() error {
	s.mu.RLock()
	defer s.mu.RUnlock()

	if s.path == "" {
		return nil // In-memory only
	}

	if err := os.MkdirAll(filepath.Dir(s.path), 0700); err != nil {
		return fmt.Errorf("failed to create config directory: %w", err)
	}

	data, err := json.MarshalIndent(s.data, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal settings: %w", err)
	}

	if err := utils.WriteFileAtomic(s.path, data, 0600); err != nil {
		return fmt.Errorf("failed to write settings file: %w", err)
	}

	return nil
}

// Get returns a copy of the current settings.
func (s *Store) Get() AppSettings {
	s.mu.RLock()
	defer s.mu.RUnlock()

	// Create a deep copy
	copy := s.data
	if s.data.AiProviders != nil {
		copy.AiProviders = make([]AiProvider, len(s.data.AiProviders))
		for i, p := range s.data.AiProviders {
			copy.AiProviders[i] = p
		}
	}
	if s.data.Extras != nil {
		copy.Extras = make(map[string]string)
		for k, v := range s.data.Extras {
			copy.Extras[k] = v
		}
	}
	return copy
}

// Update replaces the entire settings object.
func (s *Store) Update(settings AppSettings) error {
	s.mu.Lock()
	s.data = settings
	s.mu.Unlock()
	return s.Save()
}

// SetExtra updates a single extra key/value pair.
func (s *Store) SetExtra(key, value string) error {
	s.mu.Lock()
	if s.data.Extras == nil {
		s.data.Extras = make(map[string]string)
	}
	s.data.Extras[key] = value
	s.mu.Unlock()
	return s.Save()
}
