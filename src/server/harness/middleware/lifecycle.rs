use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::types::{AttemptState, BindingState, SessionState, TaskState, TurnState};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleFamily {
    Session,
    Task,
    Turn,
    Attempt,
    ToolCall,
    Interaction,
    Process,
    Binding,
    Lease,
    Handoff,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum LifecycleError {
    StaleVersion {
        expected: i64,
        actual: i64,
    },
    IllegalTransition {
        family: LifecycleFamily,
        from: String,
        to: String,
    },
    TerminalState,
}

pub trait LifecycleState: Clone + Debug + PartialEq + Eq {
    fn family() -> LifecycleFamily
    where
        Self: Sized;
    fn is_terminal(&self) -> bool;
    fn can_transition_to(&self, next: &Self) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionedState<S> {
    pub state: S,
    pub state_version: i64,
}

impl<S> VersionedState<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            state_version: 0,
        }
    }
}

impl<S: LifecycleState> VersionedState<S> {
    pub fn transition(&self, expected_version: i64, next: S) -> Result<Self, LifecycleError> {
        if expected_version != self.state_version {
            return Err(LifecycleError::StaleVersion {
                expected: expected_version,
                actual: self.state_version,
            });
        }

        if self.state == next {
            return Ok(self.clone());
        }

        if self.state.is_terminal() {
            return Err(LifecycleError::TerminalState);
        }

        if !self.state.can_transition_to(&next) {
            return Err(LifecycleError::IllegalTransition {
                family: S::family(),
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
            });
        }

        Ok(Self {
            state: next,
            state_version: self.state_version + 1,
        })
    }
}

impl LifecycleState for SessionState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Session
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Deleted | Self::Error)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (
                Self::Open,
                Self::HandingOff | Self::Archived | Self::Deleting | Self::Error
            ) | (Self::HandingOff, Self::Open | Self::Archived | Self::Error)
                | (Self::Archived, Self::Open | Self::Deleting)
                | (Self::Deleting, Self::Deleted | Self::Error)
        )
    }
}

impl LifecycleState for TaskState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Task
    }

    fn is_terminal(&self) -> bool {
        Self::is_terminal(self)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (Self::Queued, Self::Running | Self::Cancelled | Self::Failed)
                | (
                    Self::Running,
                    Self::HandingOff
                        | Self::WaitingInput
                        | Self::Paused
                        | Self::Completed
                        | Self::Failed
                        | Self::Cancelled
                        | Self::Lost
                )
                | (
                    Self::HandingOff,
                    Self::Running | Self::Failed | Self::Cancelled | Self::Lost
                )
                | (
                    Self::WaitingInput,
                    Self::Running | Self::Paused | Self::Cancelled
                )
                | (Self::Paused, Self::Running | Self::Cancelled)
        )
    }
}

impl LifecycleState for TurnState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Turn
    }

    fn is_terminal(&self) -> bool {
        Self::is_terminal(self)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (
                Self::Admitted,
                Self::Queued | Self::Running | Self::Cancelled | Self::Failed
            ) | (
                Self::Queued,
                Self::Running | Self::WaitingInput | Self::Cancelled | Self::Failed
            ) | (
                Self::Running,
                Self::WaitingInput
                    | Self::Completed
                    | Self::Interrupted
                    | Self::Failed
                    | Self::Cancelled
            ) | (
                Self::WaitingInput,
                Self::Running | Self::Completed | Self::Cancelled | Self::Failed
            )
        )
    }
}

impl LifecycleState for AttemptState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Attempt
    }

    fn is_terminal(&self) -> bool {
        Self::is_terminal(self)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Leased | Self::Cancelled)
                | (
                    Self::Leased,
                    Self::Starting | Self::Quiescing | Self::Cancelled | Self::Lost | Self::Fenced
                )
                | (
                    Self::Starting,
                    Self::Running
                        | Self::Quiescing
                        | Self::Failed
                        | Self::Cancelled
                        | Self::Lost
                        | Self::Fenced
                )
                | (
                    Self::Running,
                    Self::Quiescing
                        | Self::Checkpointing
                        | Self::Succeeded
                        | Self::Failed
                        | Self::Cancelled
                        | Self::Lost
                        | Self::Fenced
                )
                | (
                    Self::Quiescing,
                    Self::Checkpointing
                        | Self::Succeeded
                        | Self::Failed
                        | Self::Cancelled
                        | Self::Lost
                        | Self::Fenced
                )
                | (
                    Self::Checkpointing,
                    Self::Running
                        | Self::Succeeded
                        | Self::Failed
                        | Self::Cancelled
                        | Self::Lost
                        | Self::Fenced
                )
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Unknown,
}

impl LifecycleState for ToolCallState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::ToolCall
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (
                Self::Pending,
                Self::Running | Self::Cancelled | Self::Failed
            ) | (
                Self::Running,
                Self::Completed | Self::Failed | Self::Cancelled
            )
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionState {
    Pending,
    Accepted,
    Declined,
    Cancelled,
    Expired,
    Superseded,
    Error,
    Unknown,
}

impl LifecycleState for InteractionState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Interaction
    }

    fn is_terminal(&self) -> bool {
        !matches!(self, Self::Pending)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (
                Self::Pending,
                Self::Accepted
                    | Self::Declined
                    | Self::Cancelled
                    | Self::Expired
                    | Self::Superseded
                    | Self::Error
            )
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessState {
    Pending,
    Running,
    Exited,
    Failed,
    Killed,
    TimedOut,
    Lost,
    Unknown,
}

impl LifecycleState for ProcessState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Process
    }

    fn is_terminal(&self) -> bool {
        !matches!(self, Self::Pending | Self::Running)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (
                Self::Pending,
                Self::Running | Self::Failed | Self::Killed | Self::TimedOut
            ) | (
                Self::Running,
                Self::Exited | Self::Failed | Self::Killed | Self::TimedOut | Self::Lost
            )
        )
    }
}

impl LifecycleState for BindingState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Binding
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Closed | Self::Error)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (Self::Creating, Self::Inactive | Self::Active | Self::Error)
                | (Self::Inactive, Self::Active | Self::Closed | Self::Error)
                | (Self::Active, Self::Quiescing | Self::Fenced | Self::Error)
                | (Self::Quiescing, Self::Fenced | Self::Closed | Self::Error)
                | (Self::Fenced, Self::Closed)
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffState {
    #[default]
    Requested,
    Fencing,
    Quiescing,
    Snapshotting,
    Compiling,
    AwaitingLossAck,
    TargetCreating,
    Activating,
    Completed,
    Failed,
    Cancelled,
}

impl LifecycleState for HandoffState {
    fn family() -> LifecycleFamily {
        LifecycleFamily::Handoff
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (
                Self::Requested,
                Self::Fencing | Self::Cancelled | Self::Failed
            ) | (
                Self::Fencing,
                Self::Quiescing | Self::Cancelled | Self::Failed
            ) | (
                Self::Quiescing,
                Self::Snapshotting | Self::Cancelled | Self::Failed
            ) | (
                Self::Snapshotting,
                Self::Compiling | Self::Cancelled | Self::Failed
            ) | (
                Self::Compiling,
                Self::AwaitingLossAck | Self::TargetCreating | Self::Cancelled | Self::Failed
            ) | (
                Self::AwaitingLossAck,
                Self::TargetCreating | Self::Cancelled | Self::Failed
            ) | (
                Self::TargetCreating,
                Self::Activating | Self::Cancelled | Self::Failed
            ) | (
                Self::Activating,
                Self::Completed | Self::Cancelled | Self::Failed
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::{AttemptState, BindingState, SessionState, TaskState, TurnState};
    use super::*;

    #[test]
    fn task_transition_uses_compare_and_swap_and_increments_version() {
        let state = VersionedState::new(TaskState::Queued);
        let running = state.transition(0, TaskState::Running).unwrap();

        assert_eq!(running.state, TaskState::Running);
        assert_eq!(running.state_version, 1);
        assert_eq!(
            running.transition(0, TaskState::WaitingInput),
            Err(LifecycleError::StaleVersion {
                expected: 0,
                actual: 1
            })
        );
    }

    #[test]
    fn illegal_edges_are_rejected_and_terminal_duplicates_are_idempotent() {
        let running = VersionedState::new(AttemptState::Running);
        assert!(matches!(
            running.transition(0, AttemptState::Pending),
            Err(LifecycleError::IllegalTransition { .. })
        ));

        let succeeded = running.transition(0, AttemptState::Succeeded).unwrap();
        assert_eq!(succeeded.state_version, 1);
        assert_eq!(
            succeeded.transition(1, AttemptState::Succeeded).unwrap(),
            succeeded
        );
        assert_eq!(
            succeeded.transition(1, AttemptState::Failed),
            Err(LifecycleError::TerminalState)
        );
    }

    #[test]
    fn canonical_families_cover_handoff_and_recovery_edges() {
        assert!(
            VersionedState::new(SessionState::Open)
                .transition(0, SessionState::HandingOff)
                .is_ok()
        );
        assert!(
            VersionedState::new(SessionState::HandingOff)
                .transition(0, SessionState::Open)
                .is_ok()
        );
        assert!(
            VersionedState::new(TurnState::Running)
                .transition(0, TurnState::Interrupted)
                .is_ok()
        );
        assert!(
            VersionedState::new(TurnState::Interrupted)
                .transition(0, TurnState::Running)
                .is_err()
        );
        assert!(
            VersionedState::new(BindingState::Active)
                .transition(0, BindingState::Quiescing)
                .is_ok()
        );
        assert!(
            VersionedState::new(BindingState::Fenced)
                .transition(0, BindingState::Active)
                .is_err()
        );
    }

    #[test]
    fn all_terminal_state_families_are_immutable() {
        for state in [SessionState::Deleted, SessionState::Error] {
            assert!(
                VersionedState::new(state)
                    .transition(0, SessionState::Open)
                    .is_err()
            );
        }
        for state in [
            TaskState::Completed,
            TaskState::Failed,
            TaskState::Cancelled,
            TaskState::Lost,
        ] {
            assert!(
                VersionedState::new(state)
                    .transition(0, TaskState::Running)
                    .is_err()
            );
        }
        for state in [
            AttemptState::Succeeded,
            AttemptState::Failed,
            AttemptState::Cancelled,
            AttemptState::Lost,
            AttemptState::Fenced,
        ] {
            assert!(
                VersionedState::new(state)
                    .transition(0, AttemptState::Running)
                    .is_err()
            );
        }
    }

    #[test]
    fn tool_interaction_process_and_handoff_transitions_are_explicit() {
        assert!(
            VersionedState::new(ToolCallState::Pending)
                .transition(0, ToolCallState::Running)
                .is_ok()
        );
        assert!(
            VersionedState::new(InteractionState::Pending)
                .transition(0, InteractionState::Accepted)
                .is_ok()
        );
        assert!(
            VersionedState::new(ProcessState::Running)
                .transition(0, ProcessState::Lost)
                .is_ok()
        );

        let handoff = VersionedState::new(HandoffState::Requested)
            .transition(0, HandoffState::Fencing)
            .unwrap()
            .transition(1, HandoffState::Quiescing)
            .unwrap()
            .transition(2, HandoffState::Snapshotting)
            .unwrap()
            .transition(3, HandoffState::Compiling)
            .unwrap()
            .transition(4, HandoffState::AwaitingLossAck)
            .unwrap()
            .transition(5, HandoffState::TargetCreating)
            .unwrap()
            .transition(6, HandoffState::Activating)
            .unwrap()
            .transition(7, HandoffState::Completed)
            .unwrap();
        assert_eq!(handoff.state_version, 8);
        assert_eq!(
            handoff.transition(8, HandoffState::Completed).unwrap(),
            handoff
        );
    }
}
