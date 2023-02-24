// @generated automatically by Diesel CLI.

diesel::table! {
    gen1_effectdex (guid) {
        guid -> Uuid,
        id -> Int2,
    }
}

diesel::table! {
    gen1_graphics (id) {
        id -> Uuid,
        width -> Int2,
        height -> Int2,
        data -> Text,
    }
}

diesel::table! {
    gen1_movedex (guid) {
        guid -> Uuid,
        id -> Int2,
        name -> Text,
        effect -> Nullable<Int2>,
    }
}

diesel::table! {
    gen1_poke_move (pokemon_id, move_id) {
        pokemon_id -> Uuid,
        move_id -> Int2,
        level -> Int2,
        tm_teachable -> Bool,
    }
}

diesel::table! {
    gen1_pokedex (guid) {
        guid -> Uuid,
        id -> Int2,
        pokedex -> Int2,
        name -> Text,
        hp -> Int2,
        attack -> Int2,
        defense -> Int2,
        speed -> Int2,
        special -> Int2,
        type1_id -> Int2,
        type2_id -> Int2,
        catch_rate -> Int2,
        exp_yield -> Int2,
        front_sprite -> Uuid,
        back_sprite -> Uuid,
        species -> Text,
        height -> Int2,
        weight -> Int2,
        pokedex_entry -> Text,
    }
}

diesel::table! {
    gen1_type_matchup (attacker_id, defender_id) {
        attacker_id -> Int2,
        defender_id -> Int2,
        multiplier -> Int2,
    }
}

diesel::table! {
    gen1_typedex (id) {
        id -> Int2,
        name -> Text,
        category -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        hash -> Text,
    }
}

diesel::joinable!(gen1_poke_move -> gen1_pokedex (pokemon_id));

diesel::allow_tables_to_appear_in_same_query!(
    gen1_effectdex,
    gen1_graphics,
    gen1_movedex,
    gen1_poke_move,
    gen1_pokedex,
    gen1_type_matchup,
    gen1_typedex,
    users,
);
