# Console test build

Tested commit: `c3a6b78d6f495039a1e972c14de5690cb85947bf`.

[Successful workflow run](https://github.com/beauxshafferwoodworking-byte/luma-console/actions/runs/41235876871)
completed on October 31, 2026. Documentation-only commits may follow that tested source.

Passed checks:

- Frozen pnpm dependency installation and version consistency.
- Rust and tooling formatting, Oxlint, and workspace Clippy with warnings denied.
- JavaScript tooling and installer tests, plus 296 launcher and 44 native helper Rust tests.
- Native smoke suites for rendering, navigation, persistence, pointer input, weather,
  translations, and 4K capture.
- Console-specific rendering, stable focus hit regions, app wallpaper assignment and
  clearing, removed-background fallback, pointer input, settings, and cached weather.
- ARM soft-float package compilation and packaging.

[Download the test package ZIP](https://github.com/beauxshafferwoodworking-byte/luma-console/actions/runs/41235876871/artifacts/13055149746)
while signed into GitHub. It contains `com.beaux.lumaconsole_2.16.2_arm.ipk`,
`sha256sums`, and repository metadata. This is a test artifact rather than a published release.
GitHub reports the artifact ZIP digest as
`3c532b0650abaf32551dc03faa7d380a7c9b9e793e7223db24e8a5ed9a5274b1`.
This is the ZIP digest, not the inner IPK digest.

[Rendering diagnostics and screenshots](https://github.com/beauxshafferwoodworking-byte/luma-console/actions/runs/41235876871/artifacts/13056111787)
are available separately. Automated rendering assertions passed. Manual screenshot
review could not be completed because artifact transfer to the local workspace failed.

The earlier `.fork/validation.json` records the initial local source-only checks;
the workflow linked above is the authoritative result for this compiled test build.

No TV installation, Home replacement, autostart change, or hardware validation has been
performed for this fork. Obtain approval for a trial installation as a regular app
alongside stock Luma, then test all four streaming apps, weather, wallpaper changes,
remote input, pointer input, and returning to stock Luma. Home takeover requires a
separate decision after those tests.
