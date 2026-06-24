//! Preflight theme — dry-run feature-load reporting.
//!
//! Owns the [`PreflightIssue`], [`PreflightIssueKind`], and [`PreflightReport`]
//! types produced by the `preflight!` macro.
//!
//! [`PreflightIssue`]: types::preflight_issue::PreflightIssue
//! [`PreflightIssueKind`]: types::preflight_issue_kind::PreflightIssueKind
//! [`PreflightReport`]: types::preflight_report::PreflightReport

pub mod traits;
pub mod types;
