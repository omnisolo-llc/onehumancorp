package mcp_secret_vault

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

// MockSecretStorage is a mock implementation of SecretStorage.
type MockSecretStorage struct {
	mock.Mock
}

func (m *MockSecretStorage) GetSecret(ctx context.Context, key string, tenantID string) (string, error) {
	args := m.Called(ctx, key, tenantID)
	return args.String(0), args.Error(1)
}

func (m *MockSecretStorage) SetSecret(ctx context.Context, key string, value string, tenantID string) error {
	args := m.Called(ctx, key, value, tenantID)
	return args.Error(0)
}

func TestNewMCPSecretVault(t *testing.T) {
	mockStorage := new(MockSecretStorage)

	vault, err := NewMCPSecretVault("cloud", mockStorage)
	assert.NoError(t, err)
	assert.NotNil(t, vault)
	assert.Equal(t, "cloud", vault.mode)

	vault, err = NewMCPSecretVault("local", mockStorage)
	assert.NoError(t, err)
	assert.NotNil(t, vault)
	assert.Equal(t, "local", vault.mode)

	vault, err = NewMCPSecretVault("invalid", mockStorage)
	assert.Error(t, err)
	assert.Nil(t, vault)
	assert.Equal(t, "invalid mode, must be 'cloud' or 'local'", err.Error())

	vault, err = NewMCPSecretVault("cloud", nil)
	assert.Error(t, err)
	assert.Nil(t, vault)
	assert.Equal(t, "adapter cannot be nil", err.Error())
}

func TestMCPSecretVault_GetSecret(t *testing.T) {
	ctx := context.Background()

	t.Run("cloud mode valid", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		mockStorage.On("GetSecret", ctx, "mykey", "tenant1").Return("mysecret", nil)

		vault, _ := NewMCPSecretVault("cloud", mockStorage)
		val, err := vault.GetSecret(ctx, "mykey", "tenant1")

		assert.NoError(t, err)
		assert.Equal(t, "mysecret", val)
		mockStorage.AssertExpectations(t)
	})

	t.Run("cloud mode missing tenant", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		vault, _ := NewMCPSecretVault("cloud", mockStorage)

		_, err := vault.GetSecret(ctx, "mykey", "")
		assert.Error(t, err)
		assert.Equal(t, "tenantID is required in cloud mode", err.Error())
	})

	t.Run("local mode valid", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		mockStorage.On("GetSecret", ctx, "mykey", "").Return("mysecret", nil)

		vault, _ := NewMCPSecretVault("local", mockStorage)
		val, err := vault.GetSecret(ctx, "mykey", "")

		assert.NoError(t, err)
		assert.Equal(t, "mysecret", val)
		mockStorage.AssertExpectations(t)
	})

	t.Run("empty key", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		vault, _ := NewMCPSecretVault("local", mockStorage)

		_, err := vault.GetSecret(ctx, "", "")
		assert.Error(t, err)
		assert.Equal(t, "key cannot be empty", err.Error())
	})

    t.Run("adapter error", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		mockStorage.On("GetSecret", ctx, "mykey", "tenant1").Return("", errors.New("db error"))

		vault, _ := NewMCPSecretVault("cloud", mockStorage)
		_, err := vault.GetSecret(ctx, "mykey", "tenant1")

		assert.Error(t, err)
		assert.Contains(t, err.Error(), "failed to get secret: db error")
	})
}

func TestMCPSecretVault_SetSecret(t *testing.T) {
	ctx := context.Background()

	t.Run("cloud mode valid", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		mockStorage.On("SetSecret", ctx, "mykey", "myval", "tenant1").Return(nil)

		vault, _ := NewMCPSecretVault("cloud", mockStorage)
		err := vault.SetSecret(ctx, "mykey", "myval", "tenant1")

		assert.NoError(t, err)
		mockStorage.AssertExpectations(t)
	})

	t.Run("cloud mode missing tenant", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		vault, _ := NewMCPSecretVault("cloud", mockStorage)

		err := vault.SetSecret(ctx, "mykey", "myval", "")
		assert.Error(t, err)
		assert.Equal(t, "tenantID is required in cloud mode", err.Error())
	})

	t.Run("local mode valid", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		mockStorage.On("SetSecret", ctx, "mykey", "myval", "").Return(nil)

		vault, _ := NewMCPSecretVault("local", mockStorage)
		err := vault.SetSecret(ctx, "mykey", "myval", "")

		assert.NoError(t, err)
		mockStorage.AssertExpectations(t)
	})

	t.Run("empty key", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		vault, _ := NewMCPSecretVault("local", mockStorage)

		err := vault.SetSecret(ctx, "", "myval", "")
		assert.Error(t, err)
		assert.Equal(t, "key cannot be empty", err.Error())
	})

	t.Run("empty value", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		vault, _ := NewMCPSecretVault("local", mockStorage)

		err := vault.SetSecret(ctx, "mykey", "", "")
		assert.Error(t, err)
		assert.Equal(t, "value cannot be empty", err.Error())
	})

    t.Run("adapter error", func(t *testing.T) {
		mockStorage := new(MockSecretStorage)
		mockStorage.On("SetSecret", ctx, "mykey", "myval", "tenant1").Return(errors.New("db error"))

		vault, _ := NewMCPSecretVault("cloud", mockStorage)
		err := vault.SetSecret(ctx, "mykey", "myval", "tenant1")

		assert.Error(t, err)
		assert.Contains(t, err.Error(), "failed to set secret: db error")
	})
}
