use crate::services::check_architecture;

#[test]
fn run_check_architecture() {
    check_architecture("src/tests/check_architecture/test_architecture", true);
}

/// Regression test for the bug where a workspace root package
/// (a Cargo.toml containing both [package] and [workspace] sections)
/// was never checked when [workspace] members were non-empty.
///
/// Before the fix, `main()` iterated over workspace members and skipped
/// excluded ones, but never invoked `check_architecture` for the root
/// package itself. As a result, architecture violations in the root
/// package went undetected.
///
/// This test calls `check_architecture` directly on the workspace root
/// package fixture, which mirrors the behavior that `main()` must now
/// perform after iterating workspace members.
#[test]
fn run_check_architecture_workspace_root_package() {
    check_architecture(
        "src/tests/check_architecture/workspace_root_package",
        true,
    );
}
