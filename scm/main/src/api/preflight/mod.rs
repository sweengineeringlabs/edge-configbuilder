//! Preflight theme — dry-run feature-load reporting.
//!
//! Owns the [`PreflightIssue`], [`PreflightIssueKind`], and [`PreflightReport`]
//! types produced by the `preflight!` macro.
//!
//! [`PreflightIssue`]: vo::preflight_issue::PreflightIssue
//! [`PreflightIssueKind`]: vo::preflight_issue_kind::PreflightIssueKind
//! [`PreflightReport`]: vo::preflight_report::PreflightReport

pub mod traits;
pub mod vo;
