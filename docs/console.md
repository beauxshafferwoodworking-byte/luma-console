# Luma Console

This is a draft Rust/SDL2 fork of Luma 2.16.2, upstream commit
`e5608cd053be5376a54a806129a332952dfabcef`. Luma's MIT license and all
third-party asset notices are retained. This project is independent of LG and Xbox.

## Appearance

Open Settings → Interface. Press OK on each setting to cycle its value:

- Console layout: console or inherited classic Home.
- Accent color: Xbox green, electric blue, violet, coral, gold, or white.
- Tile corners: square or rounded.
- Background dimming: 0% through 60%, in 10% steps.
- Per-app backgrounds: enabled or disabled.

Console Home has a single horizontal favorites strip. Focused artwork is larger,
with an accent outline and a title inside the tile. Its control slot stays fixed
when pointer focus changes. HDMI cards remain below it. Existing app opening,
favorite ordering, remote input, and recent-app switching are inherited from Luma.

The weather card uses the existing weather location, units, cache, offline status,
and worker. Set its location in Settings → Widgets → Weather. A new installation
does not infer a location or show fabricated weather.

New Console settings and notices are currently English; inherited settings remain localized.

## Per-app backgrounds

1. Enable Console layout.
2. Select a wallpaper in Settings → Background. Upload JPEG, PNG, or WebP through
   the existing temporary upload dialog if desired.
3. Return Home, focus an app, and open its options with Menu. Alternatively open
   the app's options in Apps.
4. Choose Assign current background.
5. Repeat for Plex, Disney+, Crunchyroll, and Apple TV, then select the wallpaper
   that should appear when no app-specific assignment applies.

Clear app background removes an assignment. Only known bundled or uploaded
wallpaper IDs are accepted. Missing or deleted images use the global background.
Moving focus never saves preferences or changes the chosen launch splash.
Reduced motion disables focus-background fading.

Installations are discovered from the TV inventory. Favorites can be added and
reordered normally; the fork does not invent app IDs for the four services.
This draft does not implement Continue Watching. Plex integration can be added
using its supported authenticated API. Disney+, Crunchyroll, and Apple TV resume
data still need service-specific support.

## Validation and package

Local Rust parsing and formatting can run with rustfmt 1.93.0. Full Rust type
checking, Clippy, native tests, native rendering, and ARM compilation are pending
until a Linux build runner is available. A formatter pass is not a compiled build.

The Console workflow runs formatting, lint, unit tests, mocked rendering suites,
and the ARM build. It stores `luma-console-test-package` as a workflow artifact.
It has read-only repository permission and no publishing or TV deployment step.
Upstream release workflows are preserved under `.fork/upstream-workflows` and
are outside the Actions workflow directory.

Use the toolchain documented in development.md on Linux, or create the hosted
fork, push these source changes, and run the Console workflow. Verify all checks
before treating an IPK as installable.

The package is `com.beaux.lumaconsole_2.16.2_arm.ipk`. Its version is inherited
from the source base; a Changeset records the requested next release increment.
Its title, app ID, preferences, update feed, update state, and optional Home state
are separate from stock Luma. No release feed exists until a tested release is
explicitly published in the new repository.

Test it as an ordinary app alongside stock Luma. Enabling Console takeover is a
separate, explicit action after TV testing. The native helper rejects takeover
while a stock Luma takeover marker or boot hook exists. Never enable two Home
integrations together. Existing stock Luma remains protected from app deletion
in both the launcher and elevated helper.

## First TV test

After a successful build and approval to install, verify ordinary app launch,
all four streaming apps, weather configuration and unavailable-weather behavior,
per-app backgrounds, uploaded-image removal fallback, the Magic Remote pointer,
remote navigation, reorder, and returning to stock Luma. Test Home takeover only
after those checks and separate approval.
