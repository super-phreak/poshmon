import json

models = {}

pokemon_model = [
    {
        "name": "index",
        "data_type": "int",
        "required": True
    },
    {
        "name": "pokedex",
        "data_type": "int",
        "required": True
    },
    {
        "name": "name",
        "data_type": "string",
        "required": True
    },
    {
        "name": "base_stats",
        "data_type": "stats",
        "required": True
    },
    {
        "name": "types_id",
        "data_type": "seq[int]",
        "required": True
    },
    {
        "name": "catch_rate",
        "data_type": "int",
        "required": True
    },
    {
        "name": "base_exp_yield",
        "data_type": "int",
        "required": True
    },
    {
        "name": "front_sprite",
        "data_type": "model(sprite)",
        "required": True
    },
    {
        "name": "back_sprite",
        "data_type": "model(sprite)",
        "required": True
    },
    {
        "name": "attacks_lvl_1",
        "data_type": "seq[int]",
        "required": True
    },
    {
        "name": "growth_rate",
        "data_type": "int",
        "required": True
    },
    {
        "name": "learnable_moves",
        "data_type": "seq[int]",
        "required": True
    },
    {
        "name": "pokedex_entry",
        "data_type": "model(pokedex_entry)",
        "required": True
    },
    {
        "name": "evo_moves",
        "data_type": "table[string, int]",
        "required": True
    },
    {
        "name": "evo_info",
        "data_type": "seq[model(evo_info)]",
        "required": True
    }
]

stats_model = [
    {
        "name": "hp",
        "data_type": "int",
        "required": True
    },
    {
        "name": "attack",
        "data_type": "int",
        "required": True
    },
    {
        "name": "defense",
        "data_type": "int",
        "required": True
    },
    {
        "name": "speed",
        "data_type": "int",
        "required": True
    },
    {
        "name": "special",
        "data_type": "int",
        "required": True
    }
]

sprite_model = [
    {
        "name": "width",
        "data_type": "int",
        "required": True
    },
    {
        "name": "height",
        "data_type": "int",
        "required": True
    },
    {
        "name": "data",
        "data_type": "string",
        "required": True
    }
]

pokedex_entry_model = [
    {
        "name": "species",
        "data_type": "string",
        "required": True
    },
    {
        "name": "height",
        "data_type": "model(height)",
        "required": True
    },
    {
        "name": "weight",
        "data_type": "int",
        "required": True
    },
    {
        "name": "text",
        "data_type": "string",
        "required": True
    }
]

height_model = [
    {
        "name": "feet",
        "data_type": "int",
        "required": True
    },
    {
        "name": "inches",
        "data_type": "int",
        "required": True
    }
]

evo_info_model = [
    {
        "name": "evo_method",
        "data_type": "int",
        "required": True
    },
    {
        "name": "evo_item_id",
        "data_type": "int",
        "required": False
    },
    {
        "name": "evo_level",
        "data_type": "int",
        "required": True
    },
    {
        "name": "evo_mon_index",
        "data_type": "int",
        "required": True
    }
]

models['pokemon'] = pokemon_model
models['stats'] = stats_model
models['sprite'] = sprite_model
models['pokedex_entry'] = pokedex_entry_model
models['height'] = height_model
models['evo_info'] = evo_info_model

model_list = ['pokemon','stats','sprite','pokedex_entry','height','evo_info']
output = {'model_list': model_list, 'models': models}

with open('data/models.json', 'w') as models_file:
    json.dump(output, models_file, indent=2)
