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

    #[test]
    fn every_declared_non_terminal_edge_is_executable() {
        macro_rules! assert_edges {
            ($($from:expr => $to:expr),+ $(,)?) => {
                $(assert!(VersionedState::new($from).transition(0, $to).is_ok());)+
            };
        }

        assert_edges!(
            SessionState::Open => SessionState::HandingOff,
            SessionState::Open => SessionState::Archived,
            SessionState::Open => SessionState::Deleting,
            SessionState::Open => SessionState::Error,
            SessionState::HandingOff => SessionState::Open,
            SessionState::HandingOff => SessionState::Archived,
            SessionState::HandingOff => SessionState::Error,
            SessionState::Archived => SessionState::Open,
            SessionState::Archived => SessionState::Deleting,
            SessionState::Deleting => SessionState::Deleted,
            SessionState::Deleting => SessionState::Error,
        );
        assert_edges!(
            TaskState::Queued => TaskState::Running,
            TaskState::Queued => TaskState::Cancelled,
            TaskState::Queued => TaskState::Failed,
            TaskState::Running => TaskState::HandingOff,
            TaskState::Running => TaskState::WaitingInput,
            TaskState::Running => TaskState::Paused,
            TaskState::Running => TaskState::Completed,
            TaskState::Running => TaskState::Failed,
            TaskState::Running => TaskState::Cancelled,
            TaskState::Running => TaskState::Lost,
            TaskState::HandingOff => TaskState::Running,
            TaskState::HandingOff => TaskState::Failed,
            TaskState::HandingOff => TaskState::Cancelled,
            TaskState::HandingOff => TaskState::Lost,
            TaskState::WaitingInput => TaskState::Running,
            TaskState::WaitingInput => TaskState::Paused,
            TaskState::WaitingInput => TaskState::Cancelled,
            TaskState::Paused => TaskState::Running,
            TaskState::Paused => TaskState::Cancelled,
        );
        assert_edges!(
            TurnState::Admitted => TurnState::Queued,
            TurnState::Admitted => TurnState::Running,
            TurnState::Admitted => TurnState::Cancelled,
            TurnState::Admitted => TurnState::Failed,
            TurnState::Queued => TurnState::Running,
            TurnState::Queued => TurnState::WaitingInput,
            TurnState::Queued => TurnState::Cancelled,
            TurnState::Queued => TurnState::Failed,
            TurnState::Running => TurnState::WaitingInput,
            TurnState::Running => TurnState::Completed,
            TurnState::Running => TurnState::Interrupted,
            TurnState::Running => TurnState::Failed,
            TurnState::Running => TurnState::Cancelled,
            TurnState::WaitingInput => TurnState::Running,
            TurnState::WaitingInput => TurnState::Completed,
            TurnState::WaitingInput => TurnState::Cancelled,
            TurnState::WaitingInput => TurnState::Failed,
        );
        assert_edges!(
            AttemptState::Pending => AttemptState::Leased,
            AttemptState::Pending => AttemptState::Cancelled,
            AttemptState::Leased => AttemptState::Starting,
            AttemptState::Leased => AttemptState::Quiescing,
            AttemptState::Leased => AttemptState::Cancelled,
            AttemptState::Leased => AttemptState::Lost,
            AttemptState::Leased => AttemptState::Fenced,
            AttemptState::Starting => AttemptState::Running,
            AttemptState::Starting => AttemptState::Quiescing,
            AttemptState::Starting => AttemptState::Failed,
            AttemptState::Starting => AttemptState::Cancelled,
            AttemptState::Starting => AttemptState::Lost,
            AttemptState::Starting => AttemptState::Fenced,
            AttemptState::Running => AttemptState::Quiescing,
            AttemptState::Running => AttemptState::Checkpointing,
            AttemptState::Running => AttemptState::Succeeded,
            AttemptState::Running => AttemptState::Failed,
            AttemptState::Running => AttemptState::Cancelled,
            AttemptState::Running => AttemptState::Lost,
            AttemptState::Running => AttemptState::Fenced,
            AttemptState::Quiescing => AttemptState::Checkpointing,
            AttemptState::Quiescing => AttemptState::Succeeded,
            AttemptState::Quiescing => AttemptState::Failed,
            AttemptState::Quiescing => AttemptState::Cancelled,
            AttemptState::Quiescing => AttemptState::Lost,
            AttemptState::Quiescing => AttemptState::Fenced,
            AttemptState::Checkpointing => AttemptState::Running,
            AttemptState::Checkpointing => AttemptState::Succeeded,
            AttemptState::Checkpointing => AttemptState::Failed,
            AttemptState::Checkpointing => AttemptState::Cancelled,
            AttemptState::Checkpointing => AttemptState::Lost,
            AttemptState::Checkpointing => AttemptState::Fenced,
        );
        assert_edges!(
            ToolCallState::Pending => ToolCallState::Running,
            ToolCallState::Pending => ToolCallState::Cancelled,
            ToolCallState::Pending => ToolCallState::Failed,
            ToolCallState::Running => ToolCallState::Completed,
            ToolCallState::Running => ToolCallState::Failed,
            ToolCallState::Running => ToolCallState::Cancelled,
            InteractionState::Pending => InteractionState::Accepted,
            InteractionState::Pending => InteractionState::Declined,
            InteractionState::Pending => InteractionState::Cancelled,
            InteractionState::Pending => InteractionState::Expired,
            InteractionState::Pending => InteractionState::Superseded,
            InteractionState::Pending => InteractionState::Error,
            ProcessState::Pending => ProcessState::Running,
            ProcessState::Pending => ProcessState::Failed,
            ProcessState::Pending => ProcessState::Killed,
            ProcessState::Pending => ProcessState::TimedOut,
            ProcessState::Running => ProcessState::Exited,
            ProcessState::Running => ProcessState::Failed,
            ProcessState::Running => ProcessState::Killed,
            ProcessState::Running => ProcessState::TimedOut,
            ProcessState::Running => ProcessState::Lost,
            BindingState::Creating => BindingState::Inactive,
            BindingState::Creating => BindingState::Active,
            BindingState::Creating => BindingState::Error,
            BindingState::Inactive => BindingState::Active,
            BindingState::Inactive => BindingState::Closed,
            BindingState::Inactive => BindingState::Error,
            BindingState::Active => BindingState::Quiescing,
            BindingState::Active => BindingState::Fenced,
            BindingState::Active => BindingState::Error,
            BindingState::Quiescing => BindingState::Fenced,
            BindingState::Quiescing => BindingState::Closed,
            BindingState::Quiescing => BindingState::Error,
            BindingState::Fenced => BindingState::Closed,
            HandoffState::Requested => HandoffState::Fencing,
            HandoffState::Requested => HandoffState::Cancelled,
            HandoffState::Requested => HandoffState::Failed,
            HandoffState::Fencing => HandoffState::Quiescing,
            HandoffState::Fencing => HandoffState::Cancelled,
            HandoffState::Fencing => HandoffState::Failed,
            HandoffState::Quiescing => HandoffState::Snapshotting,
            HandoffState::Quiescing => HandoffState::Cancelled,
            HandoffState::Quiescing => HandoffState::Failed,
            HandoffState::Snapshotting => HandoffState::Compiling,
            HandoffState::Snapshotting => HandoffState::Cancelled,
            HandoffState::Snapshotting => HandoffState::Failed,
            HandoffState::Compiling => HandoffState::AwaitingLossAck,
            HandoffState::Compiling => HandoffState::TargetCreating,
            HandoffState::Compiling => HandoffState::Cancelled,
            HandoffState::Compiling => HandoffState::Failed,
            HandoffState::AwaitingLossAck => HandoffState::TargetCreating,
            HandoffState::AwaitingLossAck => HandoffState::Cancelled,
            HandoffState::AwaitingLossAck => HandoffState::Failed,
            HandoffState::TargetCreating => HandoffState::Activating,
            HandoffState::TargetCreating => HandoffState::Cancelled,
            HandoffState::TargetCreating => HandoffState::Failed,
            HandoffState::Activating => HandoffState::Completed,
            HandoffState::Activating => HandoffState::Cancelled,
            HandoffState::Activating => HandoffState::Failed,
        );
    }

    #[test]
    fn every_lifecycle_family_and_unknown_state_has_explicit_semantics() {
        assert_eq!(SessionState::family(), LifecycleFamily::Session);
        assert_eq!(TaskState::family(), LifecycleFamily::Task);
        assert_eq!(TurnState::family(), LifecycleFamily::Turn);
        assert_eq!(AttemptState::family(), LifecycleFamily::Attempt);
        assert_eq!(ToolCallState::family(), LifecycleFamily::ToolCall);
        assert_eq!(InteractionState::family(), LifecycleFamily::Interaction);
        assert_eq!(ProcessState::family(), LifecycleFamily::Process);
        assert_eq!(BindingState::family(), LifecycleFamily::Binding);
        assert_eq!(HandoffState::family(), LifecycleFamily::Handoff);

        assert!(!SessionState::Unknown.is_terminal());
        assert!(!TaskState::Unknown.is_terminal());
        assert!(!TurnState::Unknown.is_terminal());
        assert!(!AttemptState::Unknown.is_terminal());
        assert!(!ToolCallState::Unknown.is_terminal());
        assert!(InteractionState::Unknown.is_terminal());
        assert!(ProcessState::Unknown.is_terminal());
        assert!(BindingState::Error.is_terminal());

        assert!(!SessionState::Open.can_transition_to(&SessionState::Open));
        assert!(!TaskState::Unknown.can_transition_to(&TaskState::Queued));
        assert!(!TurnState::Unknown.can_transition_to(&TurnState::Queued));
        assert!(!AttemptState::Unknown.can_transition_to(&AttemptState::Pending));
        assert!(!ToolCallState::Unknown.can_transition_to(&ToolCallState::Pending));
        assert!(!InteractionState::Unknown.can_transition_to(&InteractionState::Pending));
        assert!(!ProcessState::Unknown.can_transition_to(&ProcessState::Pending));
        assert!(!BindingState::Error.can_transition_to(&BindingState::Creating));
        assert!(!HandoffState::Completed.can_transition_to(&HandoffState::Requested));
    }
}
