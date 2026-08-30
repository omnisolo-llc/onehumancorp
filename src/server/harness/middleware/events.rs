use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::lease::{FenceError, FenceToken, Lease};
use super::types::{EventDurability, EventEnvelope, ReplayRequirement};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum EventStoreError {
    MissingLeaseFence,
    UnknownLease,
    Fence(FenceError),
    FenceTokenMismatch,
    RequiredEventMustBeDurable,
    IdempotencyConflict,
    MissingParent(Uuid),
    CrossSessionParent,
    ParentNotDurable(Uuid),
    ParentNotEarlier(Uuid),
    Cycle,
    DuplicateEventId(Uuid),
    MissingEvent(Uuid),
    BranchHeadConflict {
        expected: Option<Uuid>,
        actual: Option<Uuid>,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AppendResult {
    pub event: EventEnvelope,
    pub duplicate: bool,
}

#[derive(Clone, Debug, Default)]
pub struct EventStore {
    events: BTreeMap<Uuid, EventEnvelope>,
    durable_by_sequence: BTreeMap<i64, Uuid>,
    idempotency: HashMap<(Uuid, String), (Uuid, String)>,
    branch_heads: HashMap<(Uuid, Option<Uuid>), Option<Uuid>>,
    leases: HashMap<Uuid, Lease>,
    next_durable_sequence: i64,
    next_delivery_sequence: i64,
}

impl EventStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_lease(&mut self, lease: Lease) {
        self.leases.insert(lease.lease_id, lease);
    }

    pub fn skip_delivery_slot(&mut self) {
        self.next_delivery_sequence += 1;
    }

    pub fn append(
        &mut self,
        mut event: EventEnvelope,
        fence: Option<FenceToken>,
        expected_branch_head: Option<Uuid>,
    ) -> Result<AppendResult, EventStoreError> {
        self.validate_fence(&event, fence.as_ref())?;

        if event.replay_requirement == ReplayRequirement::Required
            && event.durability != EventDurability::Durable
        {
            return Err(EventStoreError::RequiredEventMustBeDurable);
        }

        let fingerprint = semantic_fingerprint(&event);
        if let Some(idempotency_key) = &event.idempotency_key {
            let key = (event.session_id, idempotency_key.clone());
            if let Some((existing_id, existing_fingerprint)) = self.idempotency.get(&key) {
                if existing_fingerprint == &fingerprint {
                    return Ok(AppendResult {
                        event: self
                            .events
                            .get(existing_id)
                            .expect("idempotency index must reference an event")
                            .clone(),
                        duplicate: true,
                    });
                }
                return Err(EventStoreError::IdempotencyConflict);
            }
        }

        if self.events.contains_key(&event.event_id) {
            return Err(EventStoreError::DuplicateEventId(event.event_id));
        }

        self.validate_parents(&event)?;
        let branch_key = (event.session_id, event.branch_id);
        let actual_branch_head = self.branch_heads.get(&branch_key).copied().flatten();
        if actual_branch_head != expected_branch_head {
            return Err(EventStoreError::BranchHeadConflict {
                expected: expected_branch_head,
                actual: actual_branch_head,
            });
        }

        self.next_delivery_sequence += 1;
        event.delivery_sequence = Some(self.next_delivery_sequence);
        if event.durability == EventDurability::Durable {
            self.next_durable_sequence += 1;
            event.durable_sequence = Some(self.next_durable_sequence);
            self.durable_by_sequence
                .insert(self.next_durable_sequence, event.event_id);
        }

        if let Some(idempotency_key) = &event.idempotency_key {
            self.idempotency.insert(
                (event.session_id, idempotency_key.clone()),
                (event.event_id, fingerprint),
            );
        }
        self.events.insert(event.event_id, event.clone());
        self.branch_heads.insert(branch_key, Some(event.event_id));

        Ok(AppendResult {
            event,
            duplicate: false,
        })
    }

    pub fn branch_head(&self, session_id: Uuid, branch_id: Uuid) -> Option<Uuid> {
        self.branch_heads
            .get(&(session_id, Some(branch_id)))
            .copied()
            .flatten()
    }

    pub fn replay(
        &self,
        session_id: Uuid,
        from_sequence: i64,
        to_sequence: i64,
    ) -> Vec<EventEnvelope> {
        self.durable_by_sequence
            .range(from_sequence..=to_sequence)
            .filter_map(|(_, event_id)| self.events.get(event_id))
            .filter(|event| event.session_id == session_id)
            .cloned()
            .collect()
    }

    pub fn ancestor_closure(
        &self,
        session_id: Uuid,
        event_id: Uuid,
    ) -> Result<Vec<EventEnvelope>, EventStoreError> {
        let mut visiting = HashSet::new();
        let mut completed = HashSet::new();
        let mut closure = Vec::new();
        self.collect_ancestors(
            session_id,
            event_id,
            &mut visiting,
            &mut completed,
            &mut closure,
        )?;
        closure.sort_by_key(|event| event.durable_sequence);
        Ok(closure)
    }

    fn collect_ancestors(
        &self,
        session_id: Uuid,
        event_id: Uuid,
        visiting: &mut HashSet<Uuid>,
        completed: &mut HashSet<Uuid>,
        closure: &mut Vec<EventEnvelope>,
    ) -> Result<(), EventStoreError> {
        if completed.contains(&event_id) {
            return Ok(());
        }
        if !visiting.insert(event_id) {
            return Err(EventStoreError::Cycle);
        }
        let event = self
            .events
            .get(&event_id)
            .ok_or(EventStoreError::MissingEvent(event_id))?;
        if event.session_id != session_id {
            return Err(EventStoreError::CrossSessionParent);
        }
        for parent_id in &event.parent_event_ids {
            self.collect_ancestors(session_id, *parent_id, visiting, completed, closure)?;
        }
        visiting.remove(&event_id);
        completed.insert(event_id);
        closure.push(event.clone());
        Ok(())
    }

    fn validate_fence(
        &self,
        event: &EventEnvelope,
        fence: Option<&FenceToken>,
    ) -> Result<(), EventStoreError> {
        let has_fence_fields = event.ingest_attempt_id.is_some()
            || event.lease_id.is_some()
            || event.lease_generation.is_some()
            || event.fencing_token.is_some();
        if !has_fence_fields {
            return Ok(());
        }

        let lease_id = event.lease_id.ok_or(EventStoreError::MissingLeaseFence)?;
        let lease = self
            .leases
            .get(&lease_id)
            .ok_or(EventStoreError::UnknownLease)?;
        let fence = fence.ok_or(EventStoreError::MissingLeaseFence)?;
        lease
            .validate(fence.clone())
            .map_err(EventStoreError::Fence)?;
        if event
            .ingest_attempt_id
            .is_some_and(|attempt_id| lease.attempt_id != attempt_id.to_string())
        {
            return Err(EventStoreError::Fence(FenceError::WrongAttempt));
        }
        if event.lease_generation != Some(fence.generation) {
            return Err(EventStoreError::Fence(FenceError::StaleGeneration));
        }
        if event.fencing_token.as_deref() != Some(fence.value().as_str()) {
            return Err(EventStoreError::FenceTokenMismatch);
        }
        Ok(())
    }

    fn validate_parents(&self, event: &EventEnvelope) -> Result<(), EventStoreError> {
        for parent_id in &event.parent_event_ids {
            if *parent_id == event.event_id {
                return Err(EventStoreError::Cycle);
            }
            let parent = self
                .events
                .get(parent_id)
                .ok_or(EventStoreError::MissingParent(*parent_id))?;
            if parent.session_id != event.session_id {
                return Err(EventStoreError::CrossSessionParent);
            }
            let parent_sequence = parent
                .durable_sequence
                .ok_or(EventStoreError::ParentNotDurable(*parent_id))?;
            if parent_sequence >= self.next_durable_sequence + 1 {
                return Err(EventStoreError::ParentNotEarlier(*parent_id));
            }
        }
        Ok(())
    }
}

fn semantic_fingerprint(event: &EventEnvelope) -> String {
    serde_json::to_string(&(
        event.session_id,
        event.task_id,
        event.turn_id,
        event.source_attempt_id,
        event.event_type.as_str(),
        event.payload_schema.as_str(),
        event.payload_version,
        &event.parent_event_ids,
        &event.payload,
    ))
    .expect("canonical event fields must serialize")
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use uuid::Uuid;

    use super::super::lease::{FenceError, Lease};
    use super::super::types::{EventDurability, EventEnvelope, ReplayRequirement};
    use super::*;

    fn event(
        session_id: Uuid,
        event_id: Uuid,
        durability: EventDurability,
        replay_requirement: ReplayRequirement,
    ) -> EventEnvelope {
        EventEnvelope {
            event_id,
            tenant_id: "tenant-1".to_owned(),
            session_id,
            task_id: None,
            turn_id: None,
            source_attempt_id: None,
            ingest_attempt_id: None,
            actor_id: None,
            worker_id: None,
            harness_id: Some("omnisolo".to_owned()),
            binding_id: None,
            binding_generation: None,
            lease_id: None,
            lease_generation: None,
            fencing_token: None,
            durable_sequence: None,
            delivery_stream_id: Some("stream-1".to_owned()),
            delivery_sequence: None,
            aggregate_id: None,
            aggregate_sequence: None,
            branch_id: Some(Uuid::new_v4()),
            parent_event_ids: Vec::new(),
            event_type: "message.settled".to_owned(),
            payload_schema: "omnisolo.message.v1".to_owned(),
            payload_version: 1,
            occurred_at: Utc.timestamp_opt(1_700_000_000, 0).single().unwrap(),
            ingested_at: Utc.timestamp_opt(1_700_000_001, 0).single().unwrap(),
            correlation_id: None,
            causation_id: None,
            idempotency_key: None,
            durability,
            replay_requirement,
            visibility: Some("user".to_owned()),
            data_classification: Some("internal".to_owned()),
            native_provenance: Default::default(),
            payload: json!({"text": "hello"}),
            extensions: Default::default(),
        }
    }

    #[test]
    fn durable_and_delivery_sequences_are_separate() {
        let session_id = Uuid::new_v4();
        let mut store = EventStore::new();
        let first = store
            .append(
                event(
                    session_id,
                    Uuid::new_v4(),
                    EventDurability::Durable,
                    ReplayRequirement::Required,
                ),
                None,
                None,
            )
            .unwrap();
        let transient = store
            .append(
                event(
                    session_id,
                    Uuid::new_v4(),
                    EventDurability::Transient,
                    ReplayRequirement::Ignorable,
                ),
                None,
                None,
            )
            .unwrap();
        store.skip_delivery_slot();
        let second = store
            .append(
                event(
                    session_id,
                    Uuid::new_v4(),
                    EventDurability::Durable,
                    ReplayRequirement::Required,
                ),
                None,
                None,
            )
            .unwrap();

        assert_eq!(first.event.durable_sequence, Some(1));
        assert_eq!(first.event.delivery_sequence, Some(1));
        assert_eq!(transient.event.durable_sequence, None);
        assert_eq!(transient.event.delivery_sequence, Some(2));
        assert_eq!(second.event.durable_sequence, Some(2));
        assert_eq!(second.event.delivery_sequence, Some(4));
        assert_eq!(store.replay(session_id, 1, 2).len(), 2);
    }

    #[test]
    fn duplicate_idempotency_does_not_allocate_or_relabel_events() {
        let session_id = Uuid::new_v4();
        let mut store = EventStore::new();
        let mut original = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        original.idempotency_key = Some("message-1-settled".to_owned());
        let first = store.append(original.clone(), None, None).unwrap();

        let mut duplicate = original.clone();
        duplicate.event_id = Uuid::new_v4();
        let result = store.append(duplicate, None, None).unwrap();
        assert!(result.duplicate);
        assert_eq!(result.event.event_id, first.event.event_id);
        assert_eq!(result.event.durable_sequence, Some(1));

        let mut conflict = original;
        conflict.payload = json!({"text": "changed"});
        assert_eq!(
            store.append(conflict, None, None),
            Err(EventStoreError::IdempotencyConflict)
        );
    }

    #[test]
    fn duplicate_event_ids_are_rejected_without_overwriting_history() {
        let session_id = Uuid::new_v4();
        let event_id = Uuid::new_v4();
        let mut store = EventStore::new();
        let original = event(
            session_id,
            event_id,
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        store.append(original.clone(), None, None).unwrap();

        let mut duplicate = original;
        duplicate.payload = json!({"text": "different payload"});
        assert_eq!(
            store.append(duplicate, None, None),
            Err(EventStoreError::DuplicateEventId(event_id))
        );
        assert_eq!(
            store.replay(session_id, 1, 1)[0].payload,
            json!({"text": "hello"})
        );
    }

    #[test]
    fn stale_ingest_is_rejected_but_source_attempt_identity_is_preserved() {
        let session_id = Uuid::new_v4();
        let ingest_attempt = Uuid::new_v4();
        let source_attempt = Uuid::new_v4();
        let mut lease = Lease::active(ingest_attempt.to_string(), 3);
        let lease_id = lease.lease_id;
        let current_token = lease.token();
        let mut store = EventStore::new();
        store.register_lease(lease.clone());

        let mut accepted = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        accepted.source_attempt_id = Some(source_attempt);
        accepted.ingest_attempt_id = Some(ingest_attempt);
        accepted.lease_id = Some(lease_id);
        accepted.lease_generation = Some(3);
        accepted.fencing_token = Some(current_token.value());
        let stored = store
            .append(accepted, Some(current_token.clone()), None)
            .unwrap();
        assert_eq!(stored.event.source_attempt_id, Some(source_attempt));
        assert_eq!(stored.event.ingest_attempt_id, Some(ingest_attempt));

        lease.reassign().unwrap();
        store.register_lease(lease);
        let mut stale = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        stale.ingest_attempt_id = Some(ingest_attempt);
        stale.lease_id = Some(lease_id);
        stale.lease_generation = Some(3);
        stale.fencing_token = Some(current_token.value());
        assert_eq!(
            store.append(stale, Some(current_token), None),
            Err(EventStoreError::Fence(FenceError::StaleGeneration))
        );
    }

    #[test]
    fn branch_heads_use_cas_and_ancestor_closure_rejects_bad_parents() {
        let session_id = Uuid::new_v4();
        let other_session = Uuid::new_v4();
        let branch_id = Uuid::new_v4();
        let mut store = EventStore::new();
        let mut root = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        root.branch_id = Some(branch_id);
        let root_id = root.event_id;
        store.append(root, None, None).unwrap();

        let mut child = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        child.branch_id = Some(branch_id);
        child.parent_event_ids = vec![root_id];
        let child_id = child.event_id;
        store.append(child, None, Some(root_id)).unwrap();
        assert_eq!(store.branch_head(session_id, branch_id), Some(child_id));
        assert_eq!(
            store.ancestor_closure(session_id, child_id).unwrap().len(),
            2
        );

        let mut merge = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        merge.branch_id = Some(branch_id);
        merge.parent_event_ids = vec![root_id, child_id];
        let merge_id = merge.event_id;
        store.append(merge, None, Some(child_id)).unwrap();
        assert_eq!(
            store.ancestor_closure(session_id, merge_id).unwrap().len(),
            3
        );

        let mut stale_head = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        stale_head.branch_id = Some(branch_id);
        assert_eq!(
            store.append(stale_head, None, Some(root_id)),
            Err(EventStoreError::BranchHeadConflict {
                expected: Some(root_id),
                actual: Some(merge_id),
            })
        );

        let mut cross_session = event(
            other_session,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        cross_session.parent_event_ids = vec![root_id];
        assert_eq!(
            store.append(cross_session, None, None),
            Err(EventStoreError::CrossSessionParent)
        );

        let mut missing = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        missing.parent_event_ids = vec![Uuid::new_v4()];
        assert!(matches!(
            store.append(missing, None, None),
            Err(EventStoreError::MissingParent(_))
        ));

        let mut cycle = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        cycle.parent_event_ids = vec![cycle.event_id];
        assert_eq!(store.append(cycle, None, None), Err(EventStoreError::Cycle));
    }

    #[test]
    fn required_replay_events_cannot_be_transient() {
        let mut store = EventStore::new();
        let transient_required = event(
            Uuid::new_v4(),
            Uuid::new_v4(),
            EventDurability::Transient,
            ReplayRequirement::Required,
        );
        assert_eq!(
            store.append(transient_required, None, None),
            Err(EventStoreError::RequiredEventMustBeDurable)
        );
    }

    #[test]
    fn event_store_rejects_incomplete_and_mismatched_fences() {
        let session_id = Uuid::new_v4();
        let mut store = EventStore::new();
        let mut unknown = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        unknown.lease_id = Some(Uuid::new_v4());
        assert_eq!(
            store.append(unknown, None, None),
            Err(EventStoreError::UnknownLease)
        );

        let attempt_id = Uuid::new_v4();
        let lease = Lease::active(attempt_id.to_string(), 1);
        let lease_id = lease.lease_id;
        let token = lease.token();
        store.register_lease(lease);

        let mut missing_fence = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        missing_fence.ingest_attempt_id = Some(attempt_id);
        missing_fence.lease_id = Some(lease_id);
        missing_fence.lease_generation = Some(1);
        missing_fence.fencing_token = Some(token.value());
        assert_eq!(
            store.append(missing_fence, None, None),
            Err(EventStoreError::MissingLeaseFence)
        );

        let mut bad_token = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        bad_token.ingest_attempt_id = Some(attempt_id);
        bad_token.lease_id = Some(lease_id);
        bad_token.lease_generation = Some(1);
        bad_token.fencing_token = Some("tampered".to_owned());
        assert_eq!(
            store.append(bad_token, Some(token.clone()), None),
            Err(EventStoreError::FenceTokenMismatch)
        );

        let mut wrong_attempt = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        wrong_attempt.ingest_attempt_id = Some(Uuid::new_v4());
        wrong_attempt.lease_id = Some(lease_id);
        wrong_attempt.lease_generation = Some(1);
        wrong_attempt.fencing_token = Some(token.value());
        assert_eq!(
            store.append(wrong_attempt, Some(token), None),
            Err(EventStoreError::Fence(FenceError::WrongAttempt))
        );
    }

    #[test]
    fn parent_must_be_durable_and_ancestor_lookup_reports_missing_events() {
        let session_id = Uuid::new_v4();
        let mut store = EventStore::new();
        let transient = store
            .append(
                event(
                    session_id,
                    Uuid::new_v4(),
                    EventDurability::Transient,
                    ReplayRequirement::Ignorable,
                ),
                None,
                None,
            )
            .unwrap();
        let mut child = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        child.parent_event_ids = vec![transient.event.event_id];
        assert_eq!(
            store.append(child, None, None),
            Err(EventStoreError::ParentNotDurable(transient.event.event_id))
        );

        let missing = Uuid::new_v4();
        assert_eq!(
            store.ancestor_closure(session_id, missing),
            Err(EventStoreError::MissingEvent(missing))
        );
    }

    #[test]
    fn ancestor_walk_rejects_cross_session_cycles_and_non_earlier_parents() {
        let session_id = Uuid::new_v4();
        let other_session = Uuid::new_v4();
        let mut store = EventStore::new();
        let root = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        let root_id = root.event_id;
        store.append(root, None, None).unwrap();
        assert_eq!(
            store.ancestor_closure(other_session, root_id),
            Err(EventStoreError::CrossSessionParent)
        );

        let mut cyclic = store.events.get(&root_id).unwrap().clone();
        cyclic.parent_event_ids = vec![root_id];
        store.events.insert(root_id, cyclic);
        assert_eq!(
            store.ancestor_closure(session_id, root_id),
            Err(EventStoreError::Cycle)
        );

        let mut store = EventStore::new();
        let mut parent = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        let parent_id = parent.event_id;
        store.append(parent.clone(), None, None).unwrap();
        parent.durable_sequence = Some(store.next_durable_sequence + 1);
        store.events.insert(parent_id, parent);
        let mut child = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        child.parent_event_ids = vec![parent_id];
        assert_eq!(
            store.append(child, None, None),
            Err(EventStoreError::ParentNotEarlier(parent_id))
        );

        let attempt_id = Uuid::new_v4();
        let lease = Lease::active(attempt_id.to_string(), 1);
        let lease_id = lease.lease_id;
        let token = lease.token();
        store.register_lease(lease);
        let mut mismatched_generation = event(
            session_id,
            Uuid::new_v4(),
            EventDurability::Durable,
            ReplayRequirement::Required,
        );
        mismatched_generation.ingest_attempt_id = Some(attempt_id);
        mismatched_generation.lease_id = Some(lease_id);
        mismatched_generation.lease_generation = Some(2);
        mismatched_generation.fencing_token = Some(token.value());
        assert_eq!(
            store.append(mismatched_generation, Some(token), None),
            Err(EventStoreError::Fence(FenceError::StaleGeneration))
        );
    }
}
