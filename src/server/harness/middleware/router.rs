use std::collections::BTreeMap;

use uuid::Uuid;

use super::harness::{HarnessDescriptor, HarnessRegistry};
use super::types::{BindingAccessMode, BindingScope, BindingState, SessionBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RouterError {
    UnknownHarness(String),
    InvalidBinding,
    DuplicateBinding(Uuid),
    DuplicateWritableBinding,
    BindingNotFound(Uuid),
    SessionBindingNotFound(Uuid),
    TaskBindingNotFound { session_id: Uuid, task_id: Uuid },
}

#[derive(Clone, Debug)]
pub struct HarnessRouter {
    registry: HarnessRegistry,
    bindings: BTreeMap<Uuid, SessionBinding>,
}

impl HarnessRouter {
    pub fn new(registry: HarnessRegistry) -> Self {
        Self {
            registry,
            bindings: BTreeMap::new(),
        }
    }

    pub fn registry(&self) -> &HarnessRegistry {
        &self.registry
    }

    pub fn descriptor(&self, harness_id: &str) -> Result<&HarnessDescriptor, RouterError> {
        self.registry
            .descriptor(harness_id)
            .ok_or_else(|| RouterError::UnknownHarness(harness_id.to_owned()))
    }

    pub fn register_binding(&mut self, binding: SessionBinding) -> Result<(), RouterError> {
        self.descriptor(&binding.harness_id)?;
        if binding.binding_id.is_nil()
            || binding.session_id.is_nil()
            || binding.owner_id.is_nil()
            || binding.workspace_mutation_scope_id.trim().is_empty()
            || binding.generation <= 0
            || matches!(
                (&binding.scope, binding.task_id),
                (BindingScope::Session, Some(_)) | (BindingScope::Task, None)
            )
        {
            return Err(RouterError::InvalidBinding);
        }
        if self.bindings.contains_key(&binding.binding_id) {
            return Err(RouterError::DuplicateBinding(binding.binding_id));
        }
        if binding.state == BindingState::Active
            && binding.access_mode == BindingAccessMode::ReadWrite
            && self.bindings.values().any(|existing| {
                existing.state == BindingState::Active
                    && existing.access_mode == BindingAccessMode::ReadWrite
                    && existing.session_id == binding.session_id
                    && existing.workspace_mutation_scope_id == binding.workspace_mutation_scope_id
            })
        {
            return Err(RouterError::DuplicateWritableBinding);
        }
        self.bindings.insert(binding.binding_id, binding);
        Ok(())
    }

    pub fn binding(&self, binding_id: Uuid) -> Result<&SessionBinding, RouterError> {
        self.bindings
            .get(&binding_id)
            .ok_or(RouterError::BindingNotFound(binding_id))
    }

    pub fn select_for_session(&self, session_id: Uuid) -> Result<&SessionBinding, RouterError> {
        self.bindings
            .values()
            .filter(|binding| {
                binding.session_id == session_id
                    && binding.scope == BindingScope::Session
                    && binding.state == BindingState::Active
            })
            .min_by_key(|binding| {
                if binding.access_mode == BindingAccessMode::ReadWrite {
                    0
                } else {
                    1
                }
            })
            .ok_or(RouterError::SessionBindingNotFound(session_id))
    }

    pub fn select_for_task(
        &self,
        session_id: Uuid,
        task_id: Uuid,
    ) -> Result<&SessionBinding, RouterError> {
        self.bindings
            .values()
            .filter(|binding| {
                binding.session_id == session_id
                    && binding.task_id == Some(task_id)
                    && binding.scope == BindingScope::Task
                    && binding.state == BindingState::Active
            })
            .min_by_key(|binding| {
                if binding.access_mode == BindingAccessMode::ReadWrite {
                    0
                } else {
                    1
                }
            })
            .or_else(|| {
                self.bindings.values().find(|binding| {
                    binding.session_id == session_id
                        && binding.scope == BindingScope::Session
                        && binding.state == BindingState::Active
                })
            })
            .ok_or(RouterError::TaskBindingNotFound {
                session_id,
                task_id,
            })
    }

    pub fn active_harnesses(&self, session_id: Uuid) -> Vec<String> {
        self.bindings
            .values()
            .filter(|binding| {
                binding.session_id == session_id && binding.state == BindingState::Active
            })
            .map(|binding| binding.harness_id.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::harness::HarnessRegistry;
    use super::super::types::{BindingAccessMode, BindingScope, BindingState, SessionBinding};
    use super::*;

    fn binding(
        binding_id: Uuid,
        session_id: Uuid,
        task_id: Option<Uuid>,
        scope: BindingScope,
        access_mode: BindingAccessMode,
    ) -> SessionBinding {
        SessionBinding {
            binding_id,
            session_id,
            task_id,
            harness_id: "codex".to_owned(),
            scope,
            owner_id: task_id.unwrap_or(session_id),
            workspace_mutation_scope_id: "workspace".to_owned(),
            access_mode,
            native_session_id: Some("native".to_owned()),
            state: BindingState::Active,
            generation: 1,
            created_at: chrono::Utc::now(),
            last_used_at: Some(chrono::Utc::now()),
            capability_snapshot_id: None,
            adapter_config_digest: None,
            last_imported_native_cursor: None,
            last_exported_durable_sequence: None,
            native_checkpoint_ref: None,
            worker_pool: Some("codex".to_owned()),
            state_locality: None,
            exact_resume_eligible: true,
            invalidation_reason: None,
            native_record_digest: None,
            extensions: Default::default(),
        }
    }

    #[test]
    fn router_exposes_registry_and_prefers_read_write_task_bindings() {
        let registry = HarnessRegistry::with_defaults();
        let mut router = HarnessRouter::new(registry);
        let session_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let session_binding = binding(
            Uuid::new_v4(),
            session_id,
            None,
            BindingScope::Session,
            BindingAccessMode::ReadWrite,
        );
        router.register_binding(session_binding.clone()).unwrap();
        assert_eq!(
            router.registry().descriptor("codex").unwrap().harness_id,
            "codex"
        );
        let read_only_task = binding(
            Uuid::new_v4(),
            session_id,
            Some(task_id),
            BindingScope::Task,
            BindingAccessMode::ReadOnly,
        );
        router.register_binding(read_only_task.clone()).unwrap();
        assert_eq!(
            router
                .select_for_task(session_id, task_id)
                .unwrap()
                .binding_id,
            read_only_task.binding_id
        );
        let mut writable_task = binding(
            Uuid::new_v4(),
            session_id,
            Some(task_id),
            BindingScope::Task,
            BindingAccessMode::ReadWrite,
        );
        writable_task.workspace_mutation_scope_id = "task-workspace".to_owned();
        router.register_binding(writable_task.clone()).unwrap();
        assert_eq!(
            router
                .select_for_task(session_id, task_id)
                .unwrap()
                .binding_id,
            writable_task.binding_id
        );
        assert_eq!(
            router
                .select_for_task(session_id, Uuid::new_v4())
                .unwrap()
                .binding_id,
            session_binding.binding_id
        );
        assert!(
            router
                .active_harnesses(session_id)
                .contains(&"codex".to_owned())
        );
        assert!(matches!(
            router.binding(Uuid::new_v4()),
            Err(RouterError::BindingNotFound(_))
        ));
        assert!(matches!(
            router.select_for_session(Uuid::new_v4()),
            Err(RouterError::SessionBindingNotFound(_))
        ));

        let mut read_only_session = binding(
            Uuid::new_v4(),
            session_id,
            None,
            BindingScope::Session,
            BindingAccessMode::ReadOnly,
        );
        read_only_session.workspace_mutation_scope_id = "other-workspace".to_owned();
        router.register_binding(read_only_session.clone()).unwrap();
        assert_eq!(
            router.select_for_session(session_id).unwrap().binding_id,
            session_binding.binding_id
        );

        let mut invalid = session_binding.clone();
        invalid.binding_id = Uuid::nil();
        assert_eq!(
            router.register_binding(invalid),
            Err(RouterError::InvalidBinding)
        );
        assert_eq!(
            router.register_binding(session_binding.clone()),
            Err(RouterError::DuplicateBinding(session_binding.binding_id))
        );
        let duplicate_writable = binding(
            Uuid::new_v4(),
            session_id,
            None,
            BindingScope::Session,
            BindingAccessMode::ReadWrite,
        );
        assert_eq!(
            router.register_binding(duplicate_writable),
            Err(RouterError::DuplicateWritableBinding)
        );
    }
}
