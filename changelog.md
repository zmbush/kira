# Unreleased

## `wasm32` support

Kira now runs on any platform supporting `wasm32` and having a
`cpal` backend. This means one can now run an instance of Kira
in a web browser.

## Handle-based API
TODO

## Multiple metronomes
TODO

## Other changes
- Add `GroupSet` struct
- Derive `Clone` for `SequenceInstanceHandle`

# v0.3.0 - December 26th, 2020

## Per-sequence custom event types
Previously, custom events emitted by sequences were retrieved
by calling `AudioManager::pop_event`, which meant that the
audio manager had a generic type parameter for custom events,
and every sequence had to emit custom events of the same type.

Now, each sequence has its own custom event type, and you receive
those events from an `EventReceiver` that the audio manager
returns when you call `AudioManager::add_sequence`. This gives
you more flexibility in how you organize your custom events
as well as moving some functionality out of the `AudioManager`
struct, which already has a lot of methods.

## Audio streams
Audio streams provide a way of sending arbitrary audio data
to the mixer. Sometimes, you need to play audio that you
generate in real time, like voice chats. This feature
lets you do that.

## Groups
Sounds, arrangements, and sequences can now be grouped so that
they can all be paused, resumed, or stopped at once. This is
useful for controlling broad categories of sounds, like gameplay
sounds or music.

## Other changes
- Added `Sequence::play_random`
- Added `Value::Random`
- Renamed `Sound::new` to `Sound::from_frames`
- Audio file format decoding is now gated behind feature flags
- Changed functions for pausing, resuming, and stopping instances
to take settings structs (`PauseInstanceSettings`,
`ResumeInstanceSettings`, and `StopInstanceSettings`)
- When resuming an instance, you can now choose to have it seek
backward to the time when it was paused. This is useful if you
need to keep audio synced up with something in-game, but you
still want a smooth fade out when pausing the game.
- Renamed `Sequence::emit_custom_event` to `Sequence::emit`
- Added `AudioManager::seek_instance` and `AudioManager::seek_instance_to`
for changing the playback positions of running instances
- Refined the behavior of looping backwards instances
- Added `Tween::linear`
- Make `Arrangement::settings` private and add settings parameters
to `Arrangement::new_loop` and `Arrangement::new_loop_with_intro`

# v0.2.0 - December 6th, 2020

## Arrangements
This release adds `Arrangement`s, which allow you to stitch together
multiple sounds into a larger sound that you can play instances of.

The main use case for this is setting up seamless loops, including
looping songs with intros. The previous release could create seamless
loops, but only if the sound didn't have an intro.

Functions that dealt with instances of sounds now deal with instances
of sounds *or* arrangements. `InstanceSettings` has become
`PlayableSettings` to correspond to the new `Playable` enum.

Arrangements also help reduce some complexity from the code for
instances and make some more unusual behaviors, like playing
instances backwards or rewinding instances (not implemented yet,
but planned), easier to reason about.

I'm sure there's other good uses for arrangements, too! But I think
the most common use case will be songs with intros.

## Other changes
- Remove `SoundMetadata` and move `semantic_duration` into `PlayableSettings`
- Rename `StereoSample` to `Frame`
- Added support for panning instances:
	- Added `Frame::panned`
	- Added a `panning` field and method to `InstanceSettings`
	- Added `AudioManager::set_instance_panning`
	- Added `Sequence::set_instance_panning`
- Added parameter mappings, which allow `Value`s to map to parameters with
custom scaling and offsets. `Value::Parameter` now contains a `Mapping`
as its second piece of data. `ParameterId`s can be converted into
`Value::Parameter`s with the default 1:1 mapping.
- Changed `Tween` to a C-style struct with named fields:
	- `duration` - the duration of the tween
	- `easing` - the easing function to use (linear, power, etc.)
	- `ease_direction` - the easing direction (in, out, in-out)
- Added chainable methods to more settings structs
- Replaced `AudioManager::events` with `AudioManager::pop_event`
- Add `Sequence:pause/stop/resume_sequence`
- Replace `AudioError::SupportedStreamConfigsError` with `AudioError::DefaultStreamConfigError`

# v0.1.2 - December 4th, 2020
Changes:
- Update cpal to 0.13.1
- Fix a crash when using a mono output config
- Use the system's default output config instead of the config with the highest sample rate in the first valid range

# v0.1.1 - November 18th, 2020
Changes:
- Fix readme path in Cargo.toml

# v0.1.0 - November 18th, 2020
First public release
