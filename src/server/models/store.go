package models

import (
	"context"
	"errors"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/billing"
	"github.com/onehumancorp/mono/src/server/domain"
)

var (
	ErrProviderNotFound      = errors.New("provider not found")
	ErrModelInstanceNotFound = errors.New("model instance not found")
	ErrBindingNotFound       = errors.New("model binding not found")
	ErrProviderInUse         = errors.New("provider is in use by model instances")
	ErrModelInUse            = errors.New("model instance is in use by bindings")
)

type ProviderStore interface {
	Create(ctx context.Context, orgID string, provider *domain.ModelProvider) error
	Get(ctx context.Context, orgID, providerID string) (*domain.ModelProvider, error)
	Update(ctx context.Context, orgID string, provider *domain.ModelProvider) error
	Delete(ctx context.Context, orgID, providerID string) error
	List(ctx context.Context, orgID string) ([]domain.ModelProvider, error)
}

type ModelStore interface {
	Create(ctx context.Context, orgID string, model *domain.ModelInstance) error
	Get(ctx context.Context, orgID, modelID string) (*domain.ModelInstance, error)
	Update(ctx context.Context, orgID string, model *domain.ModelInstance) error
	Delete(ctx context.Context, orgID, modelID string) error
	List(ctx context.Context, orgID string, providerID string, status domain.ModelStatus) ([]domain.ModelInstance, error)
}

type BindingStore interface {
	Create(ctx context.Context, orgID string, binding *domain.ModelBinding) error
	Get(ctx context.Context, orgID, bindingID string) (*domain.ModelBinding, error)
	Update(ctx context.Context, orgID string, binding *domain.ModelBinding) error
	Delete(ctx context.Context, orgID, bindingID string) error
	ListByAgent(ctx context.Context, orgID, agentID string) ([]domain.ModelBinding, error)
	GetDefault(ctx context.Context, orgID, agentID string) (*domain.ModelBinding, error)
}

type ModelRegistry struct {
	mu          sync.RWMutex
	providers   map[string]map[string]*domain.ModelProvider
	models      map[string]map[string]*domain.ModelInstance
	bindings    map[string]map[string]*domain.ModelBinding
	billingRepo billing.UsageRepository
}

func NewModelRegistry() *ModelRegistry {
	return &ModelRegistry{
		providers: make(map[string]map[string]*domain.ModelProvider),
		models:    make(map[string]map[string]*domain.ModelInstance),
		bindings:  make(map[string]map[string]*domain.ModelBinding),
	}
}

func NewModelRegistryWithBilling(repo billing.UsageRepository) *ModelRegistry {
	return &ModelRegistry{
		providers:   make(map[string]map[string]*domain.ModelProvider),
		models:      make(map[string]map[string]*domain.ModelInstance),
		bindings:    make(map[string]map[string]*domain.ModelBinding),
		billingRepo: repo,
	}
}

func (r *ModelRegistry) orgProviders(orgID string) map[string]*domain.ModelProvider {
	if r.providers[orgID] == nil {
		r.providers[orgID] = make(map[string]*domain.ModelProvider)
	}
	return r.providers[orgID]
}

func (r *ModelRegistry) orgModels(orgID string) map[string]*domain.ModelInstance {
	if r.models[orgID] == nil {
		r.models[orgID] = make(map[string]*domain.ModelInstance)
	}
	return r.models[orgID]
}

func (r *ModelRegistry) orgBindings(orgID string) map[string]*domain.ModelBinding {
	if r.bindings[orgID] == nil {
		r.bindings[orgID] = make(map[string]*domain.ModelBinding)
	}
	return r.bindings[orgID]
}

func (r *ModelRegistry) CreateProvider(ctx context.Context, orgID string, provider *domain.ModelProvider) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	provider.ID = uuid.New().String()
	provider.OrganizationID = orgID

	r.orgProviders(orgID)[provider.ID] = provider
	return nil
}

func (r *ModelRegistry) GetProvider(ctx context.Context, orgID, providerID string) (*domain.ModelProvider, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	provider, ok := r.orgProviders(orgID)[providerID]
	if !ok {
		return nil, ErrProviderNotFound
	}
	return provider, nil
}

func (r *ModelRegistry) UpdateProvider(ctx context.Context, orgID string, provider *domain.ModelProvider) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	existing, ok := r.orgProviders(orgID)[provider.ID]
	if !ok {
		return ErrProviderNotFound
	}

	if existing.OrganizationID != orgID {
		return ErrProviderNotFound
	}

	r.orgProviders(orgID)[provider.ID] = provider
	return nil
}

func (r *ModelRegistry) DeleteProvider(ctx context.Context, orgID, providerID string) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	for _, model := range r.orgModels(orgID) {
		if model.ProviderID == providerID {
			return ErrProviderInUse
		}
	}

	delete(r.orgProviders(orgID), providerID)
	return nil
}

func (r *ModelRegistry) ListProviders(ctx context.Context, orgID string) ([]domain.ModelProvider, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	providers := make([]domain.ModelProvider, 0, len(r.orgProviders(orgID)))
	for _, p := range r.orgProviders(orgID) {
		providers = append(providers, *p)
	}
	return providers, nil
}

func (r *ModelRegistry) CreateModel(ctx context.Context, orgID string, model *domain.ModelInstance) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	model.ID = uuid.New().String()
	model.OrganizationID = orgID
	model.CreatedAtUnix = time.Now().Unix()
	model.UpdatedAtUnix = model.CreatedAtUnix

	r.orgModels(orgID)[model.ID] = model
	return nil
}

func (r *ModelRegistry) GetModel(ctx context.Context, orgID, modelID string) (*domain.ModelInstance, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	model, ok := r.orgModels(orgID)[modelID]
	if !ok {
		return nil, ErrModelInstanceNotFound
	}
	return model, nil
}

func (r *ModelRegistry) UpdateModel(ctx context.Context, orgID string, model *domain.ModelInstance) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	existing, ok := r.orgModels(orgID)[model.ID]
	if !ok {
		return ErrModelInstanceNotFound
	}

	if existing.OrganizationID != orgID {
		return ErrModelInstanceNotFound
	}

	model.UpdatedAtUnix = time.Now().Unix()
	r.orgModels(orgID)[model.ID] = model
	return nil
}

func (r *ModelRegistry) DeleteModel(ctx context.Context, orgID, modelID string) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	for _, binding := range r.orgBindings(orgID) {
		if binding.ModelInstanceID == modelID {
			return ErrModelInUse
		}
	}

	delete(r.orgModels(orgID), modelID)
	return nil
}

func (r *ModelRegistry) ListModels(ctx context.Context, orgID, providerID string, status domain.ModelStatus) ([]domain.ModelInstance, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	models := make([]domain.ModelInstance, 0)
	for _, m := range r.orgModels(orgID) {
		if providerID != "" && m.ProviderID != providerID {
			continue
		}
		if status != "" && m.Status != status {
			continue
		}
		models = append(models, *m)
	}
	return models, nil
}

func (r *ModelRegistry) CreateBinding(ctx context.Context, orgID string, binding *domain.ModelBinding) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	binding.ID = uuid.New().String()
	binding.OrganizationID = orgID
	binding.CreatedAtUnix = time.Now().Unix()

	r.orgBindings(orgID)[binding.ID] = binding
	return nil
}

func (r *ModelRegistry) GetBinding(ctx context.Context, orgID, bindingID string) (*domain.ModelBinding, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	binding, ok := r.orgBindings(orgID)[bindingID]
	if !ok {
		return nil, ErrBindingNotFound
	}
	return binding, nil
}

func (r *ModelRegistry) UpdateBinding(ctx context.Context, orgID string, binding *domain.ModelBinding) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	existing, ok := r.orgBindings(orgID)[binding.ID]
	if !ok {
		return ErrBindingNotFound
	}

	if existing.OrganizationID != orgID {
		return ErrBindingNotFound
	}

	r.orgBindings(orgID)[binding.ID] = binding
	return nil
}

func (r *ModelRegistry) DeleteBinding(ctx context.Context, orgID, bindingID string) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	delete(r.orgBindings(orgID), bindingID)
	return nil
}

func (r *ModelRegistry) ListBindingsByAgent(ctx context.Context, orgID, agentID string) ([]domain.ModelBinding, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	bindings := make([]domain.ModelBinding, 0)
	for _, b := range r.orgBindings(orgID) {
		if b.AgentID == agentID {
			bindings = append(bindings, *b)
		}
	}
	return bindings, nil
}

func (r *ModelRegistry) GetDefaultBinding(ctx context.Context, orgID, agentID string) (*domain.ModelBinding, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var defaultBinding *domain.ModelBinding
	var highestPriority int32 = -1

	for _, b := range r.orgBindings(orgID) {
		if b.AgentID == agentID && b.IsDefault {
			return b, nil
		}
		if b.AgentID == agentID && b.Priority > highestPriority {
			highestPriority = b.Priority
			defaultBinding = b
		}
	}

	if defaultBinding == nil {
		return nil, ErrBindingNotFound
	}
	return defaultBinding, nil
}

func (r *ModelRegistry) ResolveModel(ctx context.Context, orgID, agentID, modelInstanceID string) (*domain.ResolvedModel, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var model *domain.ModelInstance

	if modelInstanceID != "" {
		m, ok := r.orgModels(orgID)[modelInstanceID]
		if !ok {
			return nil, ErrModelInstanceNotFound
		}
		model = m
	} else if agentID != "" {
		binding, err := r.GetDefaultBinding(ctx, orgID, agentID)
		if err != nil {
			return nil, ErrBindingNotFound
		}
		m, ok := r.orgModels(orgID)[binding.ModelInstanceID]
		if !ok {
			return nil, ErrModelInstanceNotFound
		}
		model = m
	} else {
		return nil, ErrModelInstanceNotFound
	}

	provider, ok := r.orgProviders(orgID)[model.ProviderID]
	if !ok {
		return nil, ErrProviderNotFound
	}

	return &domain.ResolvedModel{
		Model:    model,
		Provider: provider,
		Endpoint: provider.BaseURL,
		Headers:  provider.Headers,
	}, nil
}

type ModelService struct {
	registry *ModelRegistry
}

func NewModelService(registry *ModelRegistry) *ModelService {
	return &ModelService{registry: registry}
}

func (s *ModelService) CreateProvider(ctx context.Context, orgID string, provider *domain.ModelProvider) error {
	return s.registry.CreateProvider(ctx, orgID, provider)
}

func (s *ModelService) GetProvider(ctx context.Context, orgID, providerID string) (*domain.ModelProvider, error) {
	return s.registry.GetProvider(ctx, orgID, providerID)
}

func (s *ModelService) UpdateProvider(ctx context.Context, orgID string, provider *domain.ModelProvider) error {
	return s.registry.UpdateProvider(ctx, orgID, provider)
}

func (s *ModelService) DeleteProvider(ctx context.Context, orgID, providerID string) error {
	return s.registry.DeleteProvider(ctx, orgID, providerID)
}

func (s *ModelService) ListProviders(ctx context.Context, orgID string) ([]domain.ModelProvider, error) {
	return s.registry.ListProviders(ctx, orgID)
}

func (s *ModelService) CreateModel(ctx context.Context, orgID string, model *domain.ModelInstance) error {
	return s.registry.CreateModel(ctx, orgID, model)
}

func (s *ModelService) GetModel(ctx context.Context, orgID, modelID string) (*domain.ModelInstance, error) {
	return s.registry.GetModel(ctx, orgID, modelID)
}

func (s *ModelService) UpdateModel(ctx context.Context, orgID string, model *domain.ModelInstance) error {
	return s.registry.UpdateModel(ctx, orgID, model)
}

func (s *ModelService) DeleteModel(ctx context.Context, orgID, modelID string) error {
	return s.registry.DeleteModel(ctx, orgID, modelID)
}

func (s *ModelService) ListModels(ctx context.Context, orgID, providerID string, status domain.ModelStatus) ([]domain.ModelInstance, error) {
	return s.registry.ListModels(ctx, orgID, providerID, status)
}

func (s *ModelService) CreateBinding(ctx context.Context, orgID string, binding *domain.ModelBinding) error {
	return s.registry.CreateBinding(ctx, orgID, binding)
}

func (s *ModelService) GetBinding(ctx context.Context, orgID, bindingID string) (*domain.ModelBinding, error) {
	return s.registry.GetBinding(ctx, orgID, bindingID)
}

func (s *ModelService) UpdateBinding(ctx context.Context, orgID string, binding *domain.ModelBinding) error {
	return s.registry.UpdateBinding(ctx, orgID, binding)
}

func (s *ModelService) DeleteBinding(ctx context.Context, orgID, bindingID string) error {
	return s.registry.DeleteBinding(ctx, orgID, bindingID)
}

func (s *ModelService) ListBindingsByAgent(ctx context.Context, orgID, agentID string) ([]domain.ModelBinding, error) {
	return s.registry.ListBindingsByAgent(ctx, orgID, agentID)
}

func (s *ModelService) GetDefaultBinding(ctx context.Context, orgID, agentID string) (*domain.ModelBinding, error) {
	return s.registry.GetDefaultBinding(ctx, orgID, agentID)
}

func (s *ModelService) ResolveModel(ctx context.Context, orgID, agentID, modelInstanceID string) (*domain.ResolvedModel, error) {
	return s.registry.ResolveModel(ctx, orgID, agentID, modelInstanceID)
}

func (s *ModelService) GetAgentModel(ctx context.Context, orgID, agentID string) (*domain.ResolvedModel, error) {
	binding, err := s.registry.GetDefaultBinding(ctx, orgID, agentID)
	if err != nil {
		return nil, err
	}
	return s.registry.ResolveModel(ctx, orgID, agentID, binding.ModelInstanceID)
}
