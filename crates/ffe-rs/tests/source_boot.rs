//! Existing backlog PAR-23/PAR-24: source entry guards and fresh-commander replay.
use ffe_rs::{
    boot,
    navigation::Navigator,
    saves::{Commander, SaveGame},
};

fn cache() -> ffe_content::VerifiedCache {
    ffe_content::read(&std::fs::read(std::env::var_os("FFE_WORLD_CACHE").unwrap()).unwrap())
        .unwrap()
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn a_normal_load_rejects_missing_world_and_prototype_flight() {
    let cache = cache();
    let nav = Navigator::new_game(&cache, 0).unwrap();
    let (state, _) = boot::new_commander(&cache, &nav).unwrap();
    let mut game = SaveGame::new(Commander::default(), state, None);
    assert!(boot::validate_save(&cache, &game)
        .unwrap_err()
        .contains("navigation"));
    game.navigation = Some(nav.save_state());
    boot::validate_save(&cache, &game).unwrap();
    game.state.system_flight = false;
    assert!(boot::validate_save(&cache, &game)
        .unwrap_err()
        .contains("prototype"));
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn missing_essential_records_fail_without_constructing_a_fallback() {
    let cache = cache();
    for id in [
        "base:model/records",
        "base:ship/stats",
        "base:gameplay/tables",
        "base:gameplay/pads",
        "base:terrain/tables",
        "base:terrain/maps",
        "base:world/system-tables",
        "base:world/galaxy-tables",
    ] {
        let assets = cache
            .manifest()
            .files
            .iter()
            .filter(|record| record.id != id)
            .map(|record| ffe_content::Asset {
                id: record.id.clone(),
                path: record.path.clone(),
                kind: record.asset.clone(),
                bytes: cache.asset(&record.id).unwrap().to_vec(),
            })
            .collect();
        let bytes = ffe_content::build(cache.manifest().clone(), assets).unwrap();
        let broken = ffe_content::read(&bytes).unwrap();
        assert!(boot::validate_assets(&broken).unwrap_err().contains(id));
    }
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn invalid_essential_records_fail_at_cache_validation_or_source_boot() {
    let cache = cache();
    // The cache reader validates table layouts before a VerifiedCache exists.
    // A truncated essential block must therefore fail import, not reach startup.
    for id in [
        "base:model/records",
        "base:ship/stats",
        "base:gameplay/tables",
        "base:gameplay/pads",
        "base:terrain/tables",
        "base:terrain/maps",
        "base:world/system-tables",
        "base:world/galaxy-tables",
    ] {
        let assets = cache
            .manifest()
            .files
            .iter()
            .map(|record| {
                let mut bytes = cache.asset(&record.id).unwrap().to_vec();
                if record.id == id {
                    bytes.truncate(1);
                }
                ffe_content::Asset {
                    id: record.id.clone(),
                    path: record.path.clone(),
                    kind: record.asset.clone(),
                    bytes,
                }
            })
            .collect();
        let error = ffe_content::build(cache.manifest().clone(), assets).unwrap_err();
        assert!(!error.is_empty(), "truncated {id} was accepted");
    }

    // A correctly sized but empty ship slot passes the raw table format check;
    // source boot must reject the selected ship's unusable statistics.
    let nav = Navigator::new_game(&cache, 0).unwrap();
    let ship = nav.new_game.unwrap().ship as usize;
    let stats_id = "base:ship/stats";
    let assets = cache
        .manifest()
        .files
        .iter()
        .map(|record| {
            let mut bytes = cache.asset(&record.id).unwrap().to_vec();
            if record.id == stats_id {
                bytes[ship * 48..(ship + 1) * 48].fill(0);
            }
            ffe_content::Asset {
                id: record.id.clone(),
                path: record.path.clone(),
                kind: record.asset.clone(),
                bytes,
            }
        })
        .collect();
    let invalid =
        ffe_content::read(&ffe_content::build(cache.manifest().clone(), assets).unwrap()).unwrap();
    let error = boot::new_commander(&invalid, &nav)
        .err()
        .expect("invalid ship data must fail");
    assert!(
        error.contains("flight statistics"),
        "unexpected boot error: {error}"
    );
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn fresh_boot_and_post_death_boot_have_identical_source_state() {
    let cache = cache();
    for start in [0, 1, 2, 3, 5] {
        let nav = Navigator::new_game(&cache, start).unwrap();
        let (fresh, fresh_model) = boot::new_commander(&cache, &nav).unwrap();
        let (mut destroyed, _) = boot::new_commander(&cache, &nav).unwrap();
        destroyed.take_damage(u32::MAX, 28);
        assert!(destroyed.destroyed());
        let restarted_nav = Navigator::new_game(&cache, start).unwrap();
        let (restarted, restarted_model) = boot::new_commander(&cache, &restarted_nav).unwrap();
        assert_eq!(fresh, restarted);
        assert_eq!(fresh_model.player(), restarted_model.player());
        assert_eq!(nav.save_state(), restarted_nav.save_state());
        assert_eq!(fresh_model.mode(), nav.new_game.unwrap().player_state);
    }
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE and FFE_START_RECORDS"]
fn all_delivered_start_records_match_the_executed_original() {
    let cache = cache();
    let rows: Vec<serde_json::Value> =
        std::fs::read_to_string(std::env::var_os("FFE_START_RECORDS").unwrap())
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
    assert_eq!(rows.len(), 5);
    for row in rows {
        let start = row["start_type"].as_u64().unwrap() as u8;
        let nav = Navigator::new_game(&cache, start).unwrap();
        let (mut state, model) = boot::new_commander(&cache, &nav).unwrap();
        let stats = boot::ship_stats(&cache, state.ship).unwrap();
        state.ensure_damage(stats.mass * 4);
        assert_eq!(u64::from(state.ship), row["ship"].as_u64().unwrap());
        assert_eq!(u64::from(state.credits), row["cash"].as_u64().unwrap());
        assert_eq!(u64::from(model.mode()), row["mode"].as_u64().unwrap());
        assert_eq!(
            u64::from(nav.new_game.unwrap().player_index),
            row["player"].as_u64().unwrap()
        );
        assert_eq!(
            nav.hold(&cache, state.ship) as u64,
            row["hold"].as_u64().unwrap()
        );
        assert_eq!(
            u64::from(state.loadout.general),
            row["general"].as_u64().unwrap()
        );
        assert_eq!(
            u64::from(state.combat.drive),
            row["drive"].as_u64().unwrap()
        );
        assert_eq!(
            u64::from(state.combat.energy),
            row["energy"].as_u64().unwrap()
        );
        assert_eq!(u64::from(state.hull[0]), row["hull"].as_u64().unwrap());
        assert_eq!(
            u64::from(state.shields[1]),
            row["shields"].as_u64().unwrap()
        );
        assert_eq!(
            serde_json::to_value(state.combat.lasers).unwrap(),
            row["lasers"]
        );
        assert_eq!(
            serde_json::to_value(&state.combat.missiles[..4]).unwrap(),
            row["missiles"]
        );
        assert_eq!(
            u64::from(state.hold[ffe_core::hyperspace::HYDROGEN]),
            row["fuel"].as_u64().unwrap()
        );
        assert_eq!(
            state.hold.iter().map(|v| u64::from(*v)).sum::<u64>(),
            row["used"].as_u64().unwrap()
        );
        assert_eq!(
            ffe_rs::trade::capacity(&nav, &cache, &state),
            nav.hold(&cache, state.ship)
        );
        if start == 3 {
            assert_eq!(nav.legal.fines[1..3], [700000, 700000]);
        }
        state.validate().unwrap();
    }
}

#[test]
#[ignore = "Local-only: needs FFE_WORLD_CACHE"]
fn a_saved_pad_resumes_its_supporting_planet_and_parked_handler() {
    let cache = cache();
    for start in [0, 1, 2] {
        let nav = Navigator::new_game(&cache, start).unwrap();
        let (state, _) = boot::new_commander(&cache, &nav).unwrap();
        let game = SaveGame::new(Commander::default(), state.clone(), Some(nav.save_state()));
        let loaded =
            ffe_rs::saves::decode_game(&ffe_rs::saves::encode_game(&game).unwrap()).unwrap();
        let nav = Navigator::from_save(&cache, loaded.navigation.as_ref().unwrap()).unwrap();
        let mut model = boot::resume_model(&cache, &loaded.state, &nav, 0).unwrap();
        assert!(model.on_pad());
        assert_eq!(model.mode(), 0x30);
        let before = model.absolute_position();
        for _ in 0..600 {
            model.step_landed(0.02, &Default::default(), 0x30);
        }
        assert_eq!(model.absolute_position(), before);
        assert!(model.on_pad());
    }
}
