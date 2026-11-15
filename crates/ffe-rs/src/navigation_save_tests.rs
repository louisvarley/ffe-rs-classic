//! Deterministic resume regressions for the ten PAR-18 follow-up tasks.
use super::*;
use crate::saves::{self, Commander, SaveGame};

fn cache() -> VerifiedCache {
    ffe_content::read(&std::fs::read(std::env::var_os("FFE_WORLD_CACHE").unwrap()).unwrap())
        .unwrap()
}

fn resume(cache: &VerifiedCache, nav: &Navigator) -> Navigator {
    let game = SaveGame::new(
        Commander::default(),
        crate::flight::Flight::default(),
        Some(nav.save_state()),
    );
    let loaded = saves::decode_game(&saves::encode_game(&game).unwrap()).unwrap();
    Navigator::from_save(cache, loaded.navigation.as_ref().unwrap()).unwrap()
}

fn launch(nav: &mut Navigator) {
    nav.missiles.launch(
        ffe_core::missile::Spec::from_words(6, 2000, -1000, 100),
        0x82,
        ffe_core::missile::PLAYER_TARGET,
        None,
        [0; 3],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        1,
        100,
    );
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn career_points_and_medals_resume() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    nav.career.award_federal(250);
    nav.career.award_imperial(400);
    nav.career.award_kill(200);
    nav.career.award_medal(7);
    let mut loaded = resume(&cache, &nav);
    assert_eq!(loaded.career, nav.career);
    assert_eq!(
        loaded.career.award_federal(100),
        nav.career.award_federal(100)
    );
    assert!(!loaded.career.award_medal(7));
    assert_eq!(loaded.career, nav.career);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn live_missiles_resume_guidance_and_allocate_new_ids() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    launch(&mut nav);
    let decoy = nav.deploy_chaff([100_000_000, 50_000_000, 100_000_000], 100);
    nav.missiles.missiles[0].target = Some(decoy);
    nav.missile_view = Some(0);
    nav.tick_missiles(0.23, [0; 3]);
    let mut loaded = resume(&cache, &nav);
    assert_eq!(loaded.viewed_missile(), nav.viewed_missile());
    assert!(loaded.viewed_missile().is_some());
    for _ in 0..30 {
        assert_eq!(
            loaded.tick_missiles(0.02, [0; 3]),
            nav.tick_missiles(0.02, [0; 3])
        );
        assert_eq!(loaded.missiles, nav.missiles);
    }
    launch(&mut nav);
    launch(&mut loaded);
    assert_eq!(loaded.missiles, nav.missiles);
    assert_ne!(
        loaded.missiles.missiles[0].id,
        loaded.missiles.missiles[1].id
    );
    assert_eq!(loaded.ecm(false), nav.ecm(false));
    assert_eq!(loaded.effects, nav.effects);
    assert_eq!(loaded.random, nav.random);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn chaff_and_next_object_id_resume() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    for i in 0..20 {
        nav.deploy_chaff([i, 100, 200], 50);
    }
    let mut loaded = resume(&cache, &nav);
    assert_eq!(loaded.decoys, nav.decoys);
    assert_eq!(
        loaded.deploy_chaff([1, 2, 3], 70),
        nav.deploy_chaff([1, 2, 3], 70)
    );
    assert_eq!(loaded.decoys, nav.decoys);
    assert_eq!(loaded.decoys.len(), 16);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn effects_resume_without_duplicate_script_canisters_or_rng_draws() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    nav.effects.clear();
    nav.effects
        .push(effects::departure_cloud([1, 2, 3], 100000));
    nav.effects.push(effects::exit_cloud([4, 5, 6], 100000));
    let axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    nav.effects.extend(effects::explosion(
        [100, 0, 0],
        [0.0; 3],
        3,
        axes,
        &mut nav.random,
    ));
    let mut canister =
        effects::canister([1000, 0, 0], [1.0; 3], axes, [0.0; 3], 15, &mut nav.random);
    canister.tag = Some(123);
    nav.effects.push(canister);
    nav.tick_effects(0.17, 8000);
    let mut loaded = resume(&cache, &nav);
    assert_eq!(loaded.effects, nav.effects);
    assert_eq!(loaded.random, nav.random);
    for _ in 0..20 {
        nav.tick_effects(0.02, 994);
        loaded.tick_effects(0.02, 994);
        assert_eq!(loaded.effects, nav.effects);
        assert_eq!(loaded.random, nav.random);
    }
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn ship_autopilot_target_resumes() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    nav.docked = None;
    nav.landed = None;
    nav.takeoff = None;
    nav.sequence = None;
    let id = nav.ships.iter().find(|s| s.visible()).unwrap().id;
    assert!(nav.engage_ship(id).is_some());
    let loaded = resume(&cache, &nav);
    assert_eq!(loaded.ship_target, Some(id));
    // Saving between target removal and the next autopilot update stays valid.
    nav.ships.retain(|ship| ship.id != id);
    assert_eq!(resume(&cache, &nav).ship_target, None);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn launch_clearance_resumes() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    assert!(nav.launch_needs_permission());
    nav.request_launch().unwrap();
    let loaded = resume(&cache, &nav);
    assert!(loaded.launch_cleared);
    assert!(!loaded.launch_needs_permission());
    assert_eq!(loaded.docked, nav.docked);
    assert_eq!(loaded.save_state().traffic, nav.save_state().traffic);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn landing_warning_is_not_repeated_after_resume() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    let station = nav.docked.unwrap();
    nav.sequence = None;
    nav.target = None;
    nav.traffic.clear();
    nav.warned_at = Some(station);
    let mut loaded = resume(&cache, &nav);
    // from_save makes a traffic record for the docked station, so retain the
    // same empty, uncleared record in both paths before comparing the touch.
    nav.traffic = loaded.traffic.clone();
    assert_eq!(loaded.warned_at, nav.warned_at);
    assert_eq!(
        loaded.region_touch(station, 0),
        nav.region_touch(station, 0)
    );
    assert_eq!(loaded.warned_at, Some(station));
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn fractional_clearance_time_resumes_on_the_same_boundary() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    let station = nav.docked.unwrap();
    nav.sequence = None;
    let port = nav.starport(station);
    assert!(port.request(PLAYER));
    port.timeout = 1;
    nav.tick_traffic(0, 0.75);
    let mut loaded = resume(&cache, &nav);
    assert_eq!(loaded.carry, 0.75);
    assert_eq!(loaded.tick_traffic(0, 0.25), nav.tick_traffic(0, 0.25));
    assert_eq!(loaded.traffic, nav.traffic);
    assert_eq!(loaded.traffic[&station].timeout, 0);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn mission_targets_do_not_respawn_after_reload() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    nav.spawned = Some(nav.current);
    assert!(!nav.place_contract_targets(&cache, &[123]).is_empty());
    nav.ships.retain(|ship| ship.contract != Some(123));
    let mut loaded = resume(&cache, &nav);
    let rng = loaded.random;
    let count = loaded.ships.len();
    assert!(loaded.place_contract_targets(&cache, &[123]).is_empty());
    assert_eq!(loaded.ships.len(), count);
    assert_eq!(loaded.random, rng);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn empty_npc_population_stays_empty_after_reload() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    nav.spawned = Some(nav.current);
    nav.ships.clear();
    let mut loaded = resume(&cache, &nav);
    let rng = loaded.random;
    assert!(!loaded.spawn_traffic(&cache, [0; 3], [0.0, 0.0, 1.0]));
    assert!(loaded.ships.is_empty());
    assert_eq!(loaded.random, rng);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn old_version_two_saves_retain_the_canister_migration() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    let mut effect = effects::canister(
        [100, 200, 300],
        [0.0; 3],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        [0.0; 3],
        15,
        &mut nav.random,
    );
    effect.tag = Some(77);
    nav.effects.push(effect);
    let game = SaveGame::new(
        Commander::default(),
        crate::flight::Flight::default(),
        Some(nav.save_state()),
    );
    let mut json: serde_json::Value =
        serde_json::from_slice(&saves::encode_game(&game).unwrap()).unwrap();
    json["navigation"]
        .as_object_mut()
        .unwrap()
        .remove("runtime");
    let loaded = saves::decode_game(&serde_json::to_vec(&json).unwrap()).unwrap();
    let loaded = Navigator::from_save(&cache, loaded.navigation.as_ref().unwrap()).unwrap();
    assert_eq!(loaded.career, npc::Career::default());
    assert!(loaded.missiles.missiles.is_empty());
    assert_eq!(
        loaded.effects.iter().filter(|e| e.tag == Some(77)).count(),
        1
    );
    assert_eq!(loaded.random, nav.random);
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn malformed_runtime_records_are_rejected_before_resume() {
    let cache = cache();
    let mut nav = Navigator::new_game(&cache, 1).unwrap();
    launch(&mut nav);
    nav.deploy_chaff([0; 3], 10);
    nav.effects.push(effects::departure_cloud([0; 3], 1000));
    let save = nav.save_state();
    let invalid: [fn(&mut NavigatorRuntimeSave); 14] = [
        |r| r.traffic_carry = -0.1,
        |r| r.traffic_carry = 1.0,
        |r| r.traffic_carry = f64::NAN,
        |r| r.missiles.missiles[0].velocity[0] = f64::INFINITY,
        |r| r.missiles.missiles[0].spec.model = 466,
        |r| r.missiles.missiles[0].age = -1.0,
        |r| r.missiles.missiles.push(r.missiles.missiles[0]),
        |r| r.decoys[0].radius = -1,
        |r| r.decoys[0].id = 1,
        |r| r.next_decoy = 0,
        |r| r.decoys.push(r.decoys[0]),
        |r| r.effects.last_mut().unwrap().velocity[0] = f64::NAN,
        |r| r.effects.last_mut().unwrap().cargo = Some(u16::MAX),
        |r| r.effects.last_mut().unwrap().model = u32::MAX,
    ];
    for mutate in invalid {
        let mut bad = save.clone();
        mutate(bad.runtime.as_mut().unwrap());
        assert!(bad.validate().is_err());
        assert!(Navigator::from_save(&cache, &bad).is_err());
        assert!(saves::encode_game(&SaveGame::new(
            Commander::default(),
            crate::flight::Flight::default(),
            Some(bad)
        ))
        .is_err());
    }
    let mut bad = save.clone();
    bad.runtime.as_mut().unwrap().warned_at = Some(nav.bodies.len());
    assert!(Navigator::from_save(&cache, &bad).is_err());
    let mut bad = save;
    bad.runtime.as_mut().unwrap().ship_target = Some(123456);
    assert!(Navigator::from_save(&cache, &bad).is_err());
}
