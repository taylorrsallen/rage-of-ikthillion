use crate::*;

//================================-================================-================================
pub struct AudioPlayerPlugin;
impl Plugin for AudioPlayerPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_event::<BGMEvent>()
            .add_event::<UISoundEvent>()
            .add_event::<SpatialSoundEvent>()
            .add_systems((
                update_delayed_ui_sounds,
                update_spatial_sounds,
                receive_bgm_events,
                receive_ui_sound_events,
                receive_spatial_sound_events,
            ).in_base_set(CoreSet::PostUpdate));
    }
}

//================================-================================-================================
pub struct UISoundEvent {
    sound: SoundAsset,
    volume: f32,
    delay: f32,
}

impl UISoundEvent {
    pub fn new(
        sound: SoundAsset,
        volume: f32,
    ) -> Self {
        Self {
            sound,
            volume,
            delay: 0.0,
        }
    }

    pub fn new_with_delay(
        sound: SoundAsset,
        volume: f32,
        delay: f32,
    ) -> Self {
        Self {
            sound,
            volume,
            delay,
        }
    }
}

pub struct SpatialSoundEvent {
    sound: SoundAsset,
    volume: f32,
    emitter: Vec3,
}

impl SpatialSoundEvent {
    pub fn new(
        sound: SoundAsset,
        volume: f32,
        emitter: Vec3,
    ) -> Self {
        Self {
            sound,
            volume,
            emitter,
        }
    }
}

#[derive(Component)]
pub struct SoundEmitter(Handle<SpatialAudioSink>);

//================================-================================-================================
pub struct BGMEvent {
    sound: SoundAsset,
    volume: f32,
    pub bgm_type: BGMType,
}

impl BGMEvent {
    pub fn new(
        sound: SoundAsset,
        volume: f32,
        bgm_type: BGMType,
    ) -> Self {
        Self {
            sound,
            volume,
            bgm_type,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum BGMType {
    Ambience,
    Music,
    HolyHandgunOfAntinuk,
}

#[derive(Component)]
pub struct ActiveBGM {
    pub sound: Handle<AudioSink>,
    pub bgm_type: BGMType,
}

impl ActiveBGM {
    pub fn new(
        sound: Handle<AudioSink>,
        bgm_type: BGMType,
    ) -> Self {
        Self {
            sound,
            bgm_type,
        }
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct DelayedUISound {
    sound: SoundAsset,
    volume: f32,
    timer: Timer,
}

//================================-================================-================================
fn update_delayed_ui_sounds(
    mut commands: Commands,
    mut ui_sound_events: EventWriter<UISoundEvent>,
    mut delayed_ui_sound_query: Query<(Entity, &mut DelayedUISound)>,
    time: Res<Time>,
) {
    for (entity, mut delayed_ui_sound) in delayed_ui_sound_query.iter_mut() {
        delayed_ui_sound.timer.tick(time.delta());
        if delayed_ui_sound.timer.finished() {
            ui_sound_events.send(UISoundEvent::new(delayed_ui_sound.sound, delayed_ui_sound.volume));
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn receive_bgm_events(
    mut commands: Commands,
    mut bgm_events: EventReader<BGMEvent>,
    active_bgm_query: Query<(Entity, &ActiveBGM)>,
    asset_map: Res<AssetMap>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    for bgm_event in bgm_events.iter() {
        for (active_bgm_entity, active_bgm) in active_bgm_query.iter() {
            if active_bgm.bgm_type == bgm_event.bgm_type {
                let audio_sink = audio_sinks.get(&active_bgm.sound).unwrap();
                audio_sink.stop();
                commands.entity(active_bgm_entity).despawn_recursive();
            }
        }

        if bgm_event.sound != SoundAsset::Null {
            let handle = audio_sinks.get_handle(audio.play_with_settings(
                asset_map.get_sound(bgm_event.sound),
                PlaybackSettings::LOOP.with_volume(bgm_event.volume)
            ));
            
            commands.spawn(ActiveBGM::new(handle, bgm_event.bgm_type));
        }
    }
}

fn receive_ui_sound_events(
    mut commands: Commands,
    mut ui_sound_events: EventReader<UISoundEvent>,
    settings: Res<Settings>,
    asset_map: Res<AssetMap>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let mut mult = 1.0;
    if settings.ikthillion_unraged { mult = 0.01; }
    for ui_sound_event in ui_sound_events.iter() {
        if ui_sound_event.delay != 0.0 {
            commands.spawn(DelayedUISound {
                sound: ui_sound_event.sound,
                volume: ui_sound_event.volume * mult,
                timer: Timer::from_seconds(ui_sound_event.delay, TimerMode::Once),
            });
        } else {
            audio.play_with_settings(
                asset_map.get_sound(ui_sound_event.sound),
                PlaybackSettings::ONCE.with_volume(ui_sound_event.volume * mult)
            );
        }
    }
}

fn receive_spatial_sound_events(
    mut commands: Commands,
    mut spatial_sound_events: EventReader<SpatialSoundEvent>,
    settings: Res<Settings>,
    camera_query: Query<&Transform, With<Camera>>,
    asset_map: Res<AssetMap>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<SpatialAudioSink>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let mut mult = 1.0;
        if settings.ikthillion_unraged { mult = 0.15; }
        for spatial_sound_event in spatial_sound_events.iter() {
            let handle = audio_sinks.get_handle(audio.play_spatial_with_settings(
                asset_map.get_sound(spatial_sound_event.sound),
                PlaybackSettings::ONCE.with_volume(spatial_sound_event.volume * mult),
                *camera_transform,
                1.0,
                spatial_sound_event.emitter
            ));
            
            commands.spawn(SoundEmitter(handle))
                .insert(Lifetime::new(5.0));
        }
    }
}

fn update_spatial_sounds(
    mut commands: Commands,
    mut sound_emitter_query: Query<(Entity, &mut SoundEmitter)>,
    camera_query: Query<&Transform, With<Camera3d>>,
    audio_sinks: Res<Assets<SpatialAudioSink>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        for (entity, mut sound_emitter) in sound_emitter_query.iter_mut() {
            if let Some(spatial_audio_sink) = audio_sinks.get(&sound_emitter.0) {
                spatial_audio_sink.set_listener_position(*camera_transform, 1.0);
            }
        }
    }
}