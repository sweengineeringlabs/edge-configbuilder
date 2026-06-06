//! API contract surface — one subdir per theme (ADR-007), plus cross-theme
//! `error/` consumed by every theme.

pub mod configbuilder;
pub mod error;
pub mod loader;
pub mod preflight;
pub mod substitution;
pub mod validator;
