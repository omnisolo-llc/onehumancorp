use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const LOCAL_SERVICE_BUNDLE_SCHEMA: &str = "omnisolo.local_service_bundle.v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalServiceKind {
    Memory,
    Mcp,
    Workspace,
    Artifact,
    Browser,
    Cache,
    Integration,
    ProviderFacade,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalServiceScope {
    Tenant,
    Project,
    Workspace,
    Session,
    Task,
    Attempt,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocalServiceDescriptor {
    pub service_id: String,
    pub kind: LocalServiceKind,
    pub implementation_id: String,
    pub implementation_version: String,
    pub state_locality: String,
    pub scope_model: LocalServiceScope,
    pub capabilities: BTreeSet<String>,
    pub configuration_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocalServiceBinding {
    pub binding_id: Uuid,
    pub service_id: String,
    pub kind: LocalServiceKind,
    pub scope: LocalServiceScope,
    pub tenant_id: String,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub granted_capabilities: BTreeSet<String>,
    pub generation: i64,
    pub configuration_digest: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocalServiceBundle {
    pub schema: String,
    pub bindings: Vec<LocalServiceBinding>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocalServiceScopeContext {
    pub tenant_id: String,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
}

impl LocalServiceScopeContext {
    pub fn for_attempt(
        tenant_id: impl Into<String>,
        project_id: Option<&str>,
        workspace_id: Option<&str>,
        session_id: Uuid,
        task_id: Option<Uuid>,
        attempt_id: Option<Uuid>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            project_id: project_id.map(str::to_owned),
            workspace_id: workspace_id.map(str::to_owned),
            session_id,
            task_id,
            attempt_id,
        }
    }

    pub fn new(
        tenant_id: impl Into<String>,
        project_id: Option<String>,
        workspace_id: Option<String>,
        session_id: Uuid,
        task_id: Option<Uuid>,
        attempt_id: Option<Uuid>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            project_id,
            workspace_id,
            session_id,
            task_id,
            attempt_id,
        }
    }

    fn validate_identity(&self) -> Result<(), LocalServiceError> {
        if self.tenant_id.trim().is_empty() {
            return Err(LocalServiceError::InvalidIdentity(
                "tenant_id must not be blank".to_owned(),
            ));
        }
        if self.session_id.is_nil() {
            return Err(LocalServiceError::InvalidIdentity(
                "session_id must not be nil".to_owned(),
            ));
        }
        Ok(())
    }

    fn identity_for_scope(&self, scope: LocalServiceScope) -> Result<(), LocalServiceError> {
        match scope {
            LocalServiceScope::Tenant | LocalServiceScope::Session => Ok(()),
            LocalServiceScope::Project => self
                .project_id
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .map(|_| ())
                .ok_or(LocalServiceError::MissingScopeIdentity("project_id")),
            LocalServiceScope::Workspace => self
                .workspace_id
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .map(|_| ())
                .ok_or(LocalServiceError::MissingScopeIdentity("workspace_id")),
            LocalServiceScope::Task => self
                .task_id
                .filter(|value| !value.is_nil())
                .map(|_| ())
                .ok_or(LocalServiceError::MissingScopeIdentity("task_id")),
            LocalServiceScope::Attempt => self
                .attempt_id
                .filter(|value| !value.is_nil())
                .map(|_| ())
                .ok_or(LocalServiceError::MissingScopeIdentity("attempt_id")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LocalServiceError {
    InvalidSchema(String),
    InvalidIdentity(String),
    MissingScopeIdentity(&'static str),
    DuplicateKind(LocalServiceKind),
    InvalidGeneration(i64),
    UnknownService(LocalServiceKind),
    ServiceMismatch(LocalServiceKind),
    TenantMismatch,
    SessionMismatch,
    ScopeMismatch(LocalServiceScope),
    CapabilityDenied(String),
}

impl std::fmt::Display for LocalServiceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSchema(schema) => write!(formatter, "invalid local service schema: {schema}"),
            Self::InvalidIdentity(reason) => write!(formatter, "invalid local service identity: {reason}"),
            Self::MissingScopeIdentity(identity) => {
                write!(formatter, "missing local service scope identity: {identity}")
            }
            Self::DuplicateKind(kind) => write!(formatter, "duplicate local service kind: {kind:?}"),
            Self::InvalidGeneration(generation) => {
                write!(formatter, "invalid local service generation: {generation}")
            }
            Self::UnknownService(kind) => write!(formatter, "unknown local service: {kind:?}"),
            Self::ServiceMismatch(kind) => write!(formatter, "local service descriptor mismatch: {kind:?}"),
            Self::TenantMismatch => formatter.write_str("local service tenant mismatch"),
            Self::SessionMismatch => formatter.write_str("local service session mismatch"),
            Self::ScopeMismatch(scope) => write!(formatter, "local service scope mismatch: {scope:?}"),
            Self::CapabilityDenied(capability) => {
                write!(formatter, "local service capability denied: {capability}")
            }
        }
    }
}

impl std::error::Error for LocalServiceError {}

impl LocalServiceBundle {
    pub fn binding(&self, kind: LocalServiceKind) -> Option<&LocalServiceBinding> {
        self.bindings.iter().find(|binding| binding.kind == kind)
    }

    pub fn validate(&self) -> Result<(), LocalServiceError> {
        if self.schema != LOCAL_SERVICE_BUNDLE_SCHEMA {
            return Err(LocalServiceError::InvalidSchema(self.schema.clone()));
        }

        let mut kinds = BTreeSet::new();
        for binding in &self.bindings {
            if !kinds.insert(binding.kind) {
                return Err(LocalServiceError::DuplicateKind(binding.kind));
            }
            if binding.service_id.trim().is_empty() || binding.tenant_id.trim().is_empty() {
                return Err(LocalServiceError::InvalidIdentity(
                    "service_id and tenant_id must not be blank".to_owned(),
                ));
            }
            if binding.binding_id.is_nil() || binding.session_id.is_nil() {
                return Err(LocalServiceError::InvalidIdentity(
                    "binding_id and session_id must not be nil".to_owned(),
                ));
            }
            if binding.generation <= 0 {
                return Err(LocalServiceError::InvalidGeneration(binding.generation));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct LocalServiceRegistry {
    descriptors: BTreeMap<LocalServiceKind, LocalServiceDescriptor>,
}

impl LocalServiceRegistry {
    pub fn with_defaults() -> Self {
        let descriptors = [
            descriptor(
                "omnisolo.memory",
                LocalServiceKind::Memory,
                "memory-service",
                "workspace",
                LocalServiceScope::Workspace,
                ["memory.read", "memory.search", "memory.write"],
            ),
            descriptor(
                "omnisolo.mcp",
                LocalServiceKind::Mcp,
                "mcp-service",
                "project",
                LocalServiceScope::Project,
                ["mcp.catalog", "mcp.invoke"],
            ),
            descriptor(
                "omnisolo.workspace",
                LocalServiceKind::Workspace,
                "workspace-service",
                "workspace",
                LocalServiceScope::Workspace,
                ["workspace.read", "workspace.write"],
            ),
            descriptor(
                "omnisolo.artifact",
                LocalServiceKind::Artifact,
                "artifact-service",
                "workspace",
                LocalServiceScope::Workspace,
                ["artifact.read", "artifact.write"],
            ),
            descriptor(
                "omnisolo.browser",
                LocalServiceKind::Browser,
                "browser-service",
                "task",
                LocalServiceScope::Task,
                ["browser.navigate", "browser.snapshot"],
            ),
            descriptor(
                "omnisolo.cache",
                LocalServiceKind::Cache,
                "cache-service",
                "project",
                LocalServiceScope::Project,
                ["cache.read", "cache.write"],
            ),
            descriptor(
                "omnisolo.integration",
                LocalServiceKind::Integration,
                "integration-service",
                "project",
                LocalServiceScope::Project,
                ["integration.invoke", "integration.read"],
            ),
            descriptor(
                "omnisolo.provider_facade",
                LocalServiceKind::ProviderFacade,
                "provider-facade",
                "attempt",
                LocalServiceScope::Attempt,
                [
                    "provider.chat_completions",
                    "provider.models",
                    "provider.responses",
                ],
            ),
        ]
        .into_iter()
        .map(|descriptor| (descriptor.kind, descriptor))
        .collect();
        Self { descriptors }
    }

    pub fn descriptors(&self) -> impl Iterator<Item = &LocalServiceDescriptor> {
        self.descriptors.values()
    }

    pub fn resolve(
        &self,
        context: LocalServiceScopeContext,
    ) -> Result<LocalServiceBundle, LocalServiceError> {
        context.validate_identity()?;

        let mut bindings = Vec::with_capacity(self.descriptors.len());
        for descriptor in self.descriptors.values() {
            context.identity_for_scope(descriptor.scope_model)?;
            bindings.push(LocalServiceBinding {
                binding_id: Uuid::new_v4(),
                service_id: descriptor.service_id.clone(),
                kind: descriptor.kind,
                scope: descriptor.scope_model,
                tenant_id: context.tenant_id.clone(),
                project_id: context.project_id.clone(),
                workspace_id: context.workspace_id.clone(),
                session_id: context.session_id,
                task_id: context.task_id,
                attempt_id: context.attempt_id,
                granted_capabilities: descriptor.capabilities.clone(),
                generation: 1,
                configuration_digest: descriptor.configuration_digest.clone(),
            });
        }

        let bundle = LocalServiceBundle {
            schema: LOCAL_SERVICE_BUNDLE_SCHEMA.to_owned(),
            bindings,
        };
        self.validate(&bundle, &context)?;
        Ok(bundle)
    }

    pub fn validate(
        &self,
        bundle: &LocalServiceBundle,
        context: &LocalServiceScopeContext,
    ) -> Result<(), LocalServiceError> {
        context.validate_identity()?;
        bundle.validate()?;
        for binding in &bundle.bindings {
            self.validate_binding(binding, context)?;
        }
        Ok(())
    }

    pub fn authorize(
        &self,
        binding: &LocalServiceBinding,
        context: &LocalServiceScopeContext,
        capability: &str,
    ) -> Result<(), LocalServiceError> {
        self.validate_binding(binding, context)?;
        if !binding.granted_capabilities.contains(capability) {
            return Err(LocalServiceError::CapabilityDenied(capability.to_owned()));
        }
        Ok(())
    }

    fn validate_binding(
        &self,
        binding: &LocalServiceBinding,
        context: &LocalServiceScopeContext,
    ) -> Result<(), LocalServiceError> {
        let descriptor = self
            .descriptors
            .get(&binding.kind)
            .ok_or(LocalServiceError::UnknownService(binding.kind))?;
        if binding.service_id != descriptor.service_id
            || binding.scope != descriptor.scope_model
            || binding.configuration_digest != descriptor.configuration_digest
            || !binding
                .granted_capabilities
                .is_subset(&descriptor.capabilities)
        {
            return Err(LocalServiceError::ServiceMismatch(binding.kind));
        }
        if binding.generation <= 0 {
            return Err(LocalServiceError::InvalidGeneration(binding.generation));
        }
        if binding.tenant_id != context.tenant_id {
            return Err(LocalServiceError::TenantMismatch);
        }
        if binding.session_id != context.session_id {
            return Err(LocalServiceError::SessionMismatch);
        }

        let scope_matches = match binding.scope {
            LocalServiceScope::Tenant => true,
            LocalServiceScope::Project => {
                binding.project_id.as_deref() == context.project_id.as_deref()
                    && context
                        .project_id
                        .as_deref()
                        .is_some_and(|value| !value.trim().is_empty())
            }
            LocalServiceScope::Workspace => {
                binding.workspace_id.as_deref() == context.workspace_id.as_deref()
                    && context
                        .workspace_id
                        .as_deref()
                        .is_some_and(|value| !value.trim().is_empty())
            }
            LocalServiceScope::Session => true,
            LocalServiceScope::Task => {
                binding.task_id == context.task_id
                    && context.task_id.is_some_and(|value| !value.is_nil())
            }
            LocalServiceScope::Attempt => {
                binding.attempt_id == context.attempt_id
                    && context.attempt_id.is_some_and(|value| !value.is_nil())
            }
        };
        if !scope_matches {
            return Err(LocalServiceError::ScopeMismatch(binding.scope));
        }
        Ok(())
    }
}

fn descriptor<const N: usize>(
    service_id: &str,
    kind: LocalServiceKind,
    implementation_id: &str,
    state_locality: &str,
    scope_model: LocalServiceScope,
    capabilities: [&str; N],
) -> LocalServiceDescriptor {
    LocalServiceDescriptor {
        service_id: service_id.to_owned(),
        kind,
        implementation_id: implementation_id.to_owned(),
        implementation_version: "v1".to_owned(),
        state_locality: state_locality.to_owned(),
        scope_model,
        capabilities: capabilities
            .into_iter()
            .map(str::to_owned)
            .collect(),
        configuration_digest: format!("sha256:local-service-{service_id}"),
    }
}
