-- Your SQL goes here
CREATE TABLE gen1_typedex (
  id SMALLINT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  category TEXT NOT NULL
);

CREATE TABLE gen1_type_matchup (
  attacker_id SMALLINT REFERENCES gen1_typedex(id),
  defender_id SMALLINT REFERENCES gen1_typedex(id),
  multiplier SMALLINT NOT NULL,
  PRIMARY KEY (attacker_id, defender_id)
);

CREATE TABLE gen1_effectdex (
  guid UUID PRIMARY KEY,
  id SMALLINT NOT NULL UNIQUE,
  name TEXT NOT NULL
);

CREATE TABLE gen1_movedex (
  guid UUID PRIMARY KEY,
  id SMALLINT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  effect SMALLINT REFERENCES gen1_effectdex(id),
  power SMALLINT NOT NULL,
  type SMALLINT REFERENCES gen1_typedex(id),
  accuracy SMALLINT NOT NULL,
  pp SMALLINT NOT NULL,
  priority SMALLINT NOT NULL
);

CREATE TABLE gen1_graphics (
  id UUID PRIMARY KEY,
  width SMALLINT NOT NULL,
  height SMALLINT NOT NULL,
  data TEXT NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE gen1_pokedex (
  guid UUID PRIMARY KEY,
  id SMALLINT NOT NULL,
  pokedex SMALLINT NOT NULL,
  name TEXT NOT NULL,
  hp SMALLINT NOT NULL,
  attack SMALLINT NOT NULL,
  defense SMALLINT NOT NULL,
  speed SMALLINT NOT NULL,
  special SMALLINT NOT NULL,
  type1_id SMALLINT NOT NULL REFERENCES gen1_typedex(id),
  type2_id SMALLINT NOT NULL REFERENCES gen1_typedex(id),
  catch_rate SMALLINT NOT NULL,
  exp_yield SMALLINT NOT NULL,
  front_sprite UUID NOT NULL REFERENCES gen1_graphics(id),
  back_sprite UUID NOT NULL REFERENCES gen1_graphics(id),
  species TEXT NOT NULL,
  height SMALLINT NOT NULL,
  weight SMALLINT NOT NULL,
  pokedex_entry TEXT NOT NULL
);

CREATE TABLE gen1_poke_move (
  pokemon_id UUID REFERENCES gen1_pokedex(guid),
  move_id SMALLINT REFERENCES gen1_movedex(id),
  level SMAllINT NOT NULL,
  hmtm_teachable BOOLEAN NOT NULL,
  PRIMARY KEY(pokemon_id, move_id)
);