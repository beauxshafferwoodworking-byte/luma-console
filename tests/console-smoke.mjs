import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import sharp from 'sharp';

const root = path.resolve(import.meta.dirname, '..');
const out = path.join(root, '.build');
await fs.mkdir(out, { recursive: true });
const temporary = await fs.mkdtemp(path.join(out, 'console-smoke-'));
const preferences = path.join(temporary, 'preferences.json');
const binary = path.resolve(root, process.env.CARGO_TARGET_DIR ?? 'target', 'debug/luma');
const services = ['plex', 'com.disney.disneyplus-prod', 'crunchyroll', 'com.apple.appletv'];
const defaults = {
  background: 'dunes',
  show_weather: false,
  reduced_motion: true,
  console: { enabled: true, backgrounds: { plex: 'forest' } },
};
async function reset(patch = {}) {
  await fs.writeFile(preferences, JSON.stringify({ ...defaults, ...patch }));
}
async function scenario(name, script) {
  const screenshot = path.join(out, `console-${name}.png`);
  const diagnostics = path.join(out, `console-${name}-render.json`);
  const dump = path.join(temporary, `${name}.json`);
  const result = spawnSync(
    binary,
    [
      '--preferences',
      preferences,
      '--script',
      script,
      '--frames',
      '180',
      '--time',
      '2026-10-31T16:55:00-04:00',
      '--screenshot',
      screenshot,
      '--dump-state',
      dump,
      '--diagnostics',
      diagnostics,
      '--weather-test-server',
      'http://127.0.0.1:1',
    ],
    { cwd: root, stdio: 'inherit', timeout: 120000 },
  );
  if (result.error) throw result.error;
  assert.equal(result.status, 0, name);
  const state = JSON.parse(await fs.readFile(dump, 'utf8'));
  const render = JSON.parse(await fs.readFile(diagnostics, 'utf8'));
  assert.equal(state.error, null);
  assert.equal(render.capture.missing_images, 0, name);
  assert.equal(render.capture.missing_glyphs, 0, name);
  assert.equal(render.capture.glyph_failures, 0, name);
  assert.equal((await sharp(screenshot).stats()).channels[3].min, 255);
  return state;
}
const appControls = (state) => state.controls.filter((control) => control.target.App);
try {
  await reset();
  const home = await scenario('home', 'home,down');
  assert.deepEqual(home.apps, services);
  assert.deepEqual(home.focus, { App: 'plex' });
  assert.equal(home.displayed_background, 'forest');
  assert.equal(home.preferences.background, 'dunes');
  assert.ok(home.ui_text.includes('LUMA CONSOLE'));
  assert.ok(home.ui_text.includes('Plex'));
  const other = await scenario('disney', 'home,down,right');
  assert.deepEqual(other.focus, { App: services[1] });
  assert.equal(other.displayed_background, 'dunes');
  assert.deepEqual(appControls(home), appControls(other), 'Focus must not shift hit regions');
  const assigned = await scenario('assigned', 'home,down,menu,down,down,down,enter');
  assert.equal(assigned.preferences.console.backgrounds.plex, 'dunes');
  assert.equal(assigned.modal, false);
  const cleared = await scenario('cleared', 'home,down,menu,down,down,down,down,enter');
  assert.equal(cleared.preferences.console.backgrounds.plex, undefined);
  assert.equal(cleared.displayed_background, 'dunes');
  await reset({ console: { enabled: true, backgrounds: { plex: 'uploaded-0123456789abcdef' } } });
  const missing = await scenario('missing-background', 'home,down');
  assert.equal(missing.displayed_background, 'dunes');
  await reset();
  const pointer = await scenario('pointer', 'home,click:430:700');
  assert.deepEqual(pointer.focus, { App: services[1] });
  assert.deepEqual(appControls(home), appControls(pointer));
  const settings = await scenario('settings', 'settings,down,enter,down,down,down,down');
  assert.deepEqual(settings.focus, { Setting: 10 });
  const classic = await scenario('classic', 'settings,down,enter,down,down,down,down,enter,home');
  assert.equal(classic.preferences.console.enabled, false);
  assert.ok(!classic.ui_text.includes('LUMA CONSOLE'));
  await reset({ show_weather: true });
  const unconfigured = await scenario('weather-unconfigured', 'home');
  assert.ok(unconfigured.controls.some((control) => control.target === 'WeatherConfigure'));
  await reset({
    show_weather: true,
    weather_location: {
      name: 'Vienna',
      region: 'Vienna',
      country: 'Austria',
      latitude: 48.2085,
      longitude: 16.3721,
    },
  });
  await fs.writeFile(
    path.join(temporary, 'preferences.weather.json'),
    JSON.stringify({
      key: '48.2085,16.3721',
      updated_at: Math.floor(Date.now() / 1000),
      reading: { temperature: 14.2, code: 2, is_day: true },
    }),
  );
  const weather = await scenario('weather', 'home,down');
  assert.ok(weather.ui_text.includes('Vienna'));
  assert.ok(weather.ui_text.some((value) => value.includes('14')));
  assert.equal(weather.weather.failed, false);
  console.log(
    'Console smoke checks passed: focus, backgrounds, options, pointer, settings, and cached weather.',
  );
} finally {
  await fs.rm(temporary, { recursive: true, force: true });
}
