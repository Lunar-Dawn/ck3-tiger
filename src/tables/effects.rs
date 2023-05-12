use crate::everything::Everything;
use crate::item::Item;
use crate::scopes::*;
use crate::token::Token;

use ControlEffect::*;
use Effect::*;
use SpecialEffect::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ControlEffect {
    CustomDescription,
    CustomTooltip,
    Else,
    If,
    InterfaceMessage,
    HiddenEffect,
    Random,
    RandomList,
    ShowAsTooltip,
    Switch,
    While,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SpecialEffect {
    ActivateCatalyst,
    ArtifactHistory,
    ArtifactTitleHistory,
    AddCharacterFlag,
    AddModifier,
    AddFromContribution,
    AddHook,
    AddOpinion,
    AddRandomInnovation,
    RelationFlag,
    AddSchemeCooldown,
    AddToList,
    AddTruce,
    AssignCouncilTask,
    AssignCouncillor,
    BattleEvent,
    CreateHolding,
    CulturalAcceptance,
    ChangeName,
    ChangeVariable,
    ClampVariable,
    ChangeLiege,
    ChangeTitleHolder,
    ChangeTraitRank,
    CloseView,
    CopyLocalizedText,
    CreateAlliance,
    CreateArtifact,
    CreateCharacter,
    CreateMemory,
    CreateTitle,
    CreateHolyOrder,
    CreateInspiration,
    CreateStory,
    CreateTitleChange,
    Death,
    DivideWarChest,
    Duel,
    EndWar,
    FactionStartWar,
    AddToScheme,
    ForceVote,
    Imprison,
    JoinFactionForced,
    MakePregnant,
    MoveBudget,
    OpenInteraction,
    OpenView,
    PayIncome,
    ReforgeArtifact,
    RemoveGuest,
    RemoveOpinion,
    ReplaceCourtPosition,
    RoundVariable,
    RunInteraction,
    SaveOpinion,
    SaveValue,
    SetCoa,
    SetCultureName,
    SetVariable,
    SetGhwTarget,
    SetRelation,
    KnightStatus,
    ArtifactOwner,
    SetTraitRank,
    SetupCb,
    SpawnActivity,
    SpawnArmy,
    StartGhw,
    StartStruggle,
    StartWar,
    Stress,
    TriggerEvent,
    CreateImportantAction,
    CreateSuggestion,
    VassalContractSet,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Effect {
    Yes,  // no special value, just effect = yes
    Bool, // yes and no are both meaningful
    Integer,
    Value,       // probably can be a named script value, possibly can open a script math block
    ScriptValue, // definitely can be a named script value, probably can open a script math block
    NonNegativeValue, // warn if negative
    Scope(Scopes),
    Item(Item),
    Target(&'static str, Scopes),
    TargetValue(&'static str, Scopes, &'static str),
    ItemTarget(&'static str, Item, &'static str, Scopes),
    ItemValue(&'static str, Item),
    Desc,
    Gender, // male/female/random
    Special(SpecialEffect),
    Control(ControlEffect),
    Unchecked, // so special that we just accept whatever argument
}

pub fn scope_effect(name: &Token, data: &Everything) -> Option<(Scopes, Effect)> {
    let lwname = name.as_str().to_lowercase();

    for (from, s, effect) in SCOPE_EFFECT {
        if lwname == *s {
            return Some((Scopes::from_bits_truncate(*from), *effect));
        }
    }
    if let Some(x) = lwname.strip_suffix("_perk_points") {
        if let Some(lifestyle) = x.strip_prefix("add_") {
            data.verify_exists_implied(Item::Lifestyle, lifestyle, name);
            return Some((Scopes::Character, Effect::Integer));
        }
    }
    if let Some(x) = lwname.strip_suffix("_xp") {
        if let Some(lifestyle) = x.strip_prefix("add_") {
            data.verify_exists_implied(Item::Lifestyle, lifestyle, name);
            return Some((Scopes::Character, Effect::Value));
        }
    }
    if let Some(relation) = lwname.strip_prefix("set_relation_") {
        data.verify_exists_implied(Item::Relation, relation, name);
        return Some((
            Scopes::Character,
            Effect::Special(SpecialEffect::SetRelation),
        ));
    }
    if let Some(relation) = lwname.strip_prefix("remove_relation_") {
        data.verify_exists_implied(Item::Relation, relation, name);
        return Some((Scopes::Character, Effect::Scope(Scopes::Character)));
    }
    std::option::Option::None
}

/// LAST UPDATED VERSION 1.8.1
/// See `effects.log` from the game data dumps
const SCOPE_EFFECT: &[(u64, &str, Effect)] = &[
    (
        Activity,
        "accept_invitation_for_character",
        Scope(Scopes::Character),
    ),
    (Faith, "activate_holy_site", Item(Item::HolySite)),
    (
        Struggle,
        "activate_struggle_catalyst",
        Special(ActivateCatalyst),
    ),
    (
        Character,
        "add_amenity_level",
        ItemValue("type", Item::Amenity),
    ),
    (Artifact, "add_artifact_history", Special(ArtifactHistory)),
    (Artifact, "add_artifact_modifier", Item(Item::Modifier)),
    (
        Artifact,
        "add_artifact_title_history",
        Special(ArtifactTitleHistory),
    ),
    (War, "add_attacker", Scope(Scopes::Character)),
    (Province, "add_building", Item(Item::Building)),
    (Province, "add_building_slot", Integer),
    (Character, "add_character_flag", Special(AddCharacterFlag)),
    (Character, "add_character_modifier", Special(AddModifier)),
    (LandedTitle, "add_county_modifier", Special(AddModifier)),
    (Character, "add_courtier", Scope(Scopes::Character)),
    (Culture, "add_culture_tradition", Item(Item::Tradition)),
    (War, "add_defender", Scope(Scopes::Character)),
    (Character, "add_diplomacy_skill", Effect::Value),
    (Faith, "add_doctrine", Item(Item::Doctrine)),
    (Character, "add_dread", Effect::Value),
    (Artifact, "add_durability", Effect::Value),
    (Dynasty, "add_dynasty_modifier", Special(AddModifier)),
    (Dynasty, "add_dynasty_perk", Item(Item::DynastyPerk)),
    (Dynasty, "add_dynasty_prestige", Effect::Value),
    (Dynasty, "add_dynasty_prestige_level", Effect::Value),
    (Faction, "add_faction_discontent", Effect::Value),
    (Character, "add_focus_progress", Effect::Value),
    (
        CasusBelli,
        "add_from_contribution_attackers",
        Special(AddFromContribution),
    ),
    (
        CasusBelli,
        "add_from_contribution_defenders",
        Special(AddFromContribution),
    ),
    (Character, "add_gold", NonNegativeValue),
    (Character, "add_hook", Special(AddHook)),
    (Character, "add_hook_no_toast", Special(AddHook)),
    (
        DynastyHouse,
        "add_house_artifact_claim",
        Scope(Scopes::Artifact),
    ),
    (DynastyHouse, "add_house_modifier", Special(AddModifier)),
    (Culture, "add_innovation", Item(Item::Innovation)),
    (None, "add_internal_flag", Unchecked),
    (Character, "add_intrigue_skill", Effect::Value),
    (Character, "add_joined_faction_discontent", Effect::Value),
    (Character, "add_knows_of_killer", Scope(Scopes::Character)),
    (Character, "add_learning_skill", Effect::Value),
    (Character, "add_long_term_gold", Effect::Value),
    (Army, "add_loot", Effect::Value),
    (Character, "add_martial_skill", Effect::Value),
    (Culture, "add_name_list", Item(Item::NameList)),
    (Character, "add_opinion", Special(AddOpinion)),
    (Character, "add_perk", Item(Item::Perk)),
    (
        Character,
        "add_personal_artifact_claim",
        Scope(Scopes::Artifact),
    ),
    (Character, "add_piety", Effect::ScriptValue),
    (Character, "add_piety_experience", Effect::Value),
    (Character, "add_piety_level", Effect::Value),
    (Character, "add_piety_no_experience", Effect::Value),
    (Character, "add_pressed_claim", Scope(Scopes::LandedTitle)),
    (Character, "add_prestige", Effect::ScriptValue),
    (Character, "add_prestige_experience", Effect::Value),
    (Character, "add_prestige_level", Effect::Value),
    (Character, "add_prestige_no_experience", Effect::Value),
    (Province, "add_province_modifier", Special(AddModifier)),
    (Character, "add_prowess_skill", Effect::Value),
    (
        Culture,
        "add_random_innovation",
        Special(AddRandomInnovation),
    ),
    (
        Culture,
        "add_random_valid_tradition",
        Scope(Scopes::Character),
    ),
    (
        Culture,
        "add_random_valid_tradition_replace_if_necessary",
        Scope(Scopes::Character),
    ),
    (Character, "add_realm_law", Item(Item::Law)),
    (Character, "add_realm_law_skip_effects", Item(Item::Law)),
    (Character, "add_relation_flag", Special(RelationFlag)),
    (Character, "add_reserved_gold", Effect::Value),
    (Character, "add_scheme_cooldown", Special(AddSchemeCooldown)),
    (Scheme, "add_scheme_modifier", Special(AddModifier)),
    (Scheme, "add_scheme_progress", Effect::Value),
    (
        Character,
        "add_secret",
        ItemTarget("type", Item::Secret, "target", Scopes::Character),
    ),
    (Secret, "add_secret_participant", Scope(Scopes::Character)),
    (Character, "add_short_term_gold", Effect::Value),
    (
        Province,
        "add_special_building",
        Item(Item::SpecialBuilding),
    ),
    (
        Province,
        "add_special_building_slot",
        Item(Item::SpecialBuilding),
    ),
    (Character, "add_stewardship_skill", Effect::Value),
    (Character, "add_stress", Effect::Value),
    (
        Character,
        "add_targeting_factions_discontent",
        Effect::Value,
    ),
    (LandedTitle, "add_title_law", Item(Item::Law)),
    (LandedTitle, "add_title_law_effects", Item(Item::Law)),
    (None, "add_to_global_variable_list", Special(AddToList)),
    (ALL_BUT_NONE, "add_to_list", Unchecked),
    (None, "add_to_local_variable_list", Special(AddToList)),
    (Character, "add_to_scheme", Scope(Scopes::Scheme)),
    (ALL_BUT_NONE, "add_to_temporary_list", Unchecked),
    (None, "add_to_variable_list", Special(AddToList)),
    (Character, "add_trait", Item(Item::Trait)),
    (Character, "add_trait_force_tooltip", Item(Item::Trait)),
    (Character, "add_truce_both_ways", Special(AddTruce)),
    (Character, "add_truce_one_way", Special(AddTruce)),
    (Character, "add_tyranny", Effect::Value),
    (Character, "add_unpressed_claim", Scope(Scopes::LandedTitle)),
    (Character, "add_visiting_courtier", Scope(Scopes::Character)),
    (Character, "add_war_chest_gold", Effect::Value),
    (Character, "allow_alliance", Scope(Scopes::Character)),
    (Character, "allow_in_scheme", Scope(Scopes::Scheme)),
    (
        Character,
        "apply_ai_vassal_obligation_liege_most_desired",
        Yes,
    ),
    (
        Character,
        "apply_ai_vassal_obligation_vassal_most_desired",
        Yes,
    ),
    (
        Character,
        "appoint_court_position",
        ItemTarget(
            "court_position",
            Item::CourtPosition,
            "recipient",
            Scopes::Character,
        ),
    ),
    (None, "assert_if", Unchecked),
    (Army, "assign_commander", Scope(Scopes::Character)),
    (Character, "assign_council_task", Special(AssignCouncilTask)),
    (
        Character,
        "assign_councillor_type",
        Special(AssignCouncillor),
    ),
    (Character, "banish", Yes),
    (CombatSide, "battle_event", Special(BattleEvent)),
    (
        Character,
        "becomes_independent",
        Target("change", Scopes::TitleAndVassalChange),
    ),
    (Province, "begin_create_holding", Special(CreateHolding)),
    (Character, "break_alliance", Scope(Scopes::Character)),
    (Character, "break_betrothal", Scope(Scopes::Character)),
    (
        Character,
        "cancel_truce_both_ways",
        Scope(Scopes::Character),
    ),
    (Character, "cancel_truce_one_way", Scope(Scopes::Character)),
    (Character, "change_age", ScriptValue),
    (LandedTitle, "change_county_control", Effect::Value),
    (
        Culture,
        "change_cultural_acceptance",
        Special(CulturalAcceptance),
    ),
    (Character, "change_current_court_grandeur", Effect::Value),
    (Character, "change_current_weight", Effect::Value),
    (
        LandedTitle,
        "change_de_jure_drift_progress",
        TargetValue("target", Scopes::LandedTitle, "value"),
    ),
    (LandedTitle, "change_development_level", Effect::Value),
    (LandedTitle, "change_development_progress", Effect::Value),
    (
        LandedTitle,
        "change_development_progress_with_overflow",
        Effect::Value,
    ),
    (Faith, "change_fervor", ScriptValue),
    (Character, "change_first_name", Special(ChangeName)),
    (None, "change_global_variable", Special(ChangeVariable)),
    (Character, "change_government", Item(Item::Government)),
    (Inspiration, "change_inspiration_progress", Integer),
    (Character, "change_liege", Special(ChangeLiege)),
    (None, "change_local_variable", Special(ChangeVariable)),
    (Character, "change_prison_type", Item(Item::PrisonType)),
    (Struggle, "change_struggle_phase", Item(Item::StrugglePhase)),
    (Character, "change_target_weight", Effect::Value),
    (
        LandedTitle,
        "change_title_holder",
        Special(ChangeTitleHolder),
    ),
    (
        LandedTitle,
        "change_title_holder_include_vassals",
        Special(ChangeTitleHolder),
    ),
    (Character, "change_trait_rank", Special(ChangeTraitRank)),
    (None, "change_variable", Special(ChangeVariable)),
    (GreatHolyWar, "change_war_chest_gold", ScriptValue),
    (GreatHolyWar, "change_war_chest_piety", ScriptValue),
    (GreatHolyWar, "change_war_chest_prestige", ScriptValue),
    (None, "clamp_global_variable", Special(ClampVariable)),
    (None, "clamp_local_variable", Special(ClampVariable)),
    (None, "clamp_variable", Special(ClampVariable)),
    (Artifact, "clear_artifact_modifiers", Yes),
    (War, "clear_claimant", Yes),
    (Culture, "clear_culture_traditions", Yes),
    (Character, "clear_forced_vote", Yes),
    (None, "clear_global_variable_list", Unchecked),
    (None, "clear_local_variable_list", Unchecked),
    (None, "clear_saved_scope", Unchecked),
    (LandedTitle, "clear_title_laws", Yes),
    (LandedTitle, "clear_title_laws_effects", Yes),
    (Character, "clear_traits", Yes),
    (None, "clear_variable_list", Unchecked),
    (None, "close_all_views", Yes),
    (None, "close_view", Special(CloseView)),
    (Activity, "complete_activity", Effect::Bool),
    (
        Character,
        "consume_banish_reasons",
        Scope(Scopes::Character),
    ),
    (
        Character,
        "consume_divorce_reasons",
        Scope(Scopes::Character),
    ),
    (
        Character,
        "consume_execute_reasons",
        Scope(Scopes::Character),
    ),
    (
        Character,
        "consume_imprisonment_reasons",
        Scope(Scopes::Character),
    ),
    (
        Character,
        "consume_revoke_title_reason",
        Scope(Scopes::Character),
    ),
    (Culture, "copy_all_traditions_from", Scope(Scopes::Culture)),
    (Artifact, "copy_artifact_modifiers", Scope(Scopes::Artifact)),
    (
        Character,
        "copy_inheritable_appearance_from",
        Scope(Scopes::Character),
    ),
    (Character, "copy_localized_text", Special(CopyLocalizedText)),
    (
        LandedTitle,
        "copy_title_history",
        Scope(Scopes::LandedTitle),
    ),
    (Character, "copy_traits", Scope(Scopes::Character)),
    (Character, "create_alliance", Special(CreateAlliance)),
    (Character, "create_artifact", Special(CreateArtifact)),
    (Character, "create_betrothal", Scope(Scopes::Character)),
    (
        Character,
        "create_betrothal_matrilineal",
        Scope(Scopes::Character),
    ),
    (Character, "create_cadet_branch", Yes),
    (None, "create_character", Special(CreateCharacter)),
    (Character, "create_character_memory", Special(CreateMemory)),
    (Character, "create_divergent_culture", Yes),
    (Character, "create_divergent_culture_with_side_effects", Yes),
    (
        Character,
        "create_divergent_culture_with_side_effects_excluding_cost",
        Yes,
    ),
    (None, "create_dynamic_title", Special(CreateTitle)),
    (
        Character,
        "create_faction",
        ItemTarget("type", Item::Faction, "target", Scopes::Character),
    ),
    (None, "create_holy_order", Special(CreateHolyOrder)),
    (Character, "create_hybrid_culture", Scope(Scopes::Culture)),
    (
        Character,
        "create_hybrid_culture_with_side_effects",
        Scope(Scopes::Culture),
    ),
    (Character, "create_inspiration", Special(CreateInspiration)),
    (Character, "create_story", Special(CreateStory)),
    (
        None,
        "create_title_and_vassal_change",
        Special(CreateTitleChange),
    ),
    (None, "custom_description", Control(CustomDescription)),
    (
        None,
        "custom_description_no_bullet",
        Control(CustomDescription),
    ),
    (None, "custom_label", Control(CustomTooltip)),
    (None, "custom_tooltip", Control(CustomTooltip)),
    (Faith, "deactivate_holy_site", Item(Item::HolySite)),
    (Character, "death", Special(Death)),
    (None, "debug_log", Unchecked),
    (None, "debug_log_date", Yes),
    (None, "debug_log_scopes", Effect::Bool),
    (None, "debug_trigger_event", Item(Item::Event)),
    (
        Activity,
        "decline_invitation_for_character",
        Scope(Scopes::Character),
    ),
    (Character, "depose", Yes),
    (None, "destroy_artifact", Scope(Scopes::Artifact)),
    (
        None,
        "destroy_character_memory",
        Scope(Scopes::CharacterMemory),
    ),
    (Faction, "destroy_faction", Yes),
    (None, "destroy_inspiration", Scope(Scopes::Inspiration)),
    (Character, "destroy_title", Scope(Scopes::LandedTitle)),
    (Secret, "disable_exposure_by", Scope(Scopes::Character)),
    (GreatHolyWar, "divide_war_chest", Special(DivideWarChest)),
    (Character, "divorce", Scope(Scopes::Character)),
    (
        Character,
        "do_ghw_title_handout",
        Scope(Scopes::TitleAndVassalChange),
    ),
    (None, "duel", Special(Duel)),
    (None, "else", Control(Else)),
    (None, "else_if", Control(If)),
    (
        Character,
        "end_inspiration_sponsorship",
        Scope(Scopes::Inspiration),
    ),
    (Character, "end_pregnancy", Yes),
    (Scheme, "end_scheme", Yes),
    (StoryCycle, "end_story", Yes),
    (Struggle, "end_struggle", Unchecked), // not clear how this parameter is used
    (War, "end_war", Special(EndWar)),
    (Artifact, "equip_artifact_to_owner", Yes),
    (Artifact, "equip_artifact_to_owner_replace", Yes),
    (Character, "execute_decision", Item(Item::Decision)),
    (Scheme, "expose_scheme", Yes),
    (Scheme, "expose_scheme_agent", Scope(Scopes::Character)),
    (Secret, "expose_secret", Scope(Scopes::Character)),
    (Faction, "faction_remove_war", Yes),
    (Faction, "faction_start_war", Special(FactionStartWar)),
    (Character, "finish_council_task", Yes),
    (Character, "fire_councillor", Scope(Scopes::Character)),
    (Character, "forbid_from_scheme", Scope(Scopes::Scheme)),
    (Character, "force_add_to_scheme", Special(AddToScheme)),
    (Character, "force_character_skill_recalculation", Yes),
    (Character, "force_vote_as", Special(ForceVote)),
    (Province, "generate_building", Yes),
    // not sure what the argument to generate_coa means
    (
        LandedTitle | Dynasty | DynastyHouse,
        "generate_coa",
        Unchecked,
    ),
    (Culture, "get_all_innovations_from", Scope(Scopes::Culture)),
    (
        Culture,
        "get_random_innovation_from",
        Scope(Scopes::Culture),
    ),
    (Character, "get_title", Scope(Scopes::LandedTitle)),
    (Character, "give_nickname", Item(Item::Nickname)),
    (None, "hidden_effect", Control(HiddenEffect)),
    (None, "hidden_effect_new_artifact", Control(HiddenEffect)),
    (None, "if", Control(If)),
    (Character, "imprison", Special(Imprison)),
    (Inspiration, "invest_gold", NonNegativeValue),
    (
        Activity,
        "invite_character_to_activity",
        Scope(Scopes::Character),
    ),
    (Culture, "join_era", Item(Item::CultureEra)),
    (Character, "join_faction", Scope(Scopes::Faction)),
    (Character, "join_faction_forced", Special(JoinFactionForced)),
    (Character, "join_faction_skip_check", Scope(Scopes::Faction)),
    (
        Character,
        "learn_court_language_of",
        Scope(Scopes::Character),
    ),
    (Character, "learn_language", Item(Item::Language)),
    (
        Character,
        "learn_language_of_culture",
        Scope(Scopes::Culture),
    ),
    (LandedTitle, "lease_out_to", Scope(Scopes::HolyOrder)),
    (Culture, "leave_era", Item(Item::CultureEra)),
    (Character, "leave_faction", Scope(Scopes::Faction)),
    (CombatSide, "lose_combat", Effect::Bool),
    (Character, "make_claim_strong", Scope(Scopes::LandedTitle)),
    (Character, "make_claim_weak", Scope(Scopes::LandedTitle)),
    (Character, "make_concubine", Scope(Scopes::Character)),
    (Character, "make_pregnant", Special(MakePregnant)),
    (Character, "make_pregnant_no_checks", Special(MakePregnant)),
    (StoryCycle, "make_story_owner", Scope(Scopes::Character)),
    (Character, "make_trait_active", Item(Item::Trait)),
    (
        Character,
        "make_trait_active_force_tooltip",
        Item(Item::Trait),
    ),
    (Character, "make_trait_inactive", Item(Item::Trait)),
    (
        Character,
        "make_trait_inactive_force_tooltip",
        Item(Item::Trait),
    ),
    (Character, "make_unprunable", Yes),
    (Character, "marry", Scope(Scopes::Character)),
    (Character, "marry_matrilineal", Scope(Scopes::Character)),
    (Activity, "move_activity", Scope(Scopes::Province)),
    (Character, "move_budget_gold", Special(MoveBudget)),
    (Character, "move_to_pool", Yes),
    (Character, "move_to_pool_at", Scope(Scopes::Province)),
    // multiply_focus_progress -- no idea of syntax or effect
    (
        Character,
        "open_appoint_court_position_window",
        Item(Item::CourtPosition),
    ),
    (None, "open_interaction_window", Special(OpenInteraction)),
    (None, "open_view", Special(OpenView)),
    (None, "open_view_data", Special(OpenView)),
    (None, "pan_camera_to_province", Scope(Scopes::Province)),
    (None, "pan_camera_to_title", Scope(Scopes::LandedTitle)),
    (
        Character,
        "pay_long_term_gold",
        TargetValue("target", Scopes::Character, "gold"),
    ),
    (Character, "pay_long_term_income", Special(PayIncome)),
    (
        Character,
        "pay_reserved_gold",
        TargetValue("target", Scopes::Character, "gold"),
    ),
    (Character, "pay_reserved_income", Special(PayIncome)),
    (
        Character,
        "pay_short_term_gold",
        TargetValue("target", Scopes::Character, "gold"),
    ),
    (Character, "pay_short_term_income", Special(PayIncome)),
    (
        Character,
        "pay_war_chest_gold",
        TargetValue("target", Scopes::Character, "gold"),
    ),
    (Character, "pay_war_chest_income", Special(PayIncome)),
    (Character, "play_music_cue", Item(Item::Music)),
    (Character, "play_sound_effect", Unchecked),
    (GreatHolyWar, "pledge_attacker", Scope(Scopes::Character)),
    (GreatHolyWar, "pledge_defender", Scope(Scopes::Character)),
    (None, "random", Control(Random)),
    (None, "random_list", Control(RandomList)),
    (None, "random_log_scopes", Effect::Bool),
    (Character, "recruit_courtier", Scope(Scopes::Character)),
    (Province, "refill_garrison", Yes),
    (Province, "refill_levy", Yes),
    (Artifact, "reforge_artifact", Special(ReforgeArtifact)),
    (Character, "refund_all_perks", Yes),
    (Character, "refund_perks", Item(Item::Lifestyle)),
    (Character, "release_from_prison", Yes),
    (
        Character,
        "remove_all_character_modifier_instances",
        Item(Item::Modifier),
    ),
    (
        LandedTitle,
        "remove_all_county_modifier_instances",
        Item(Item::Modifier),
    ),
    (
        Dynasty,
        "remove_all_dynasty_modifier_instances",
        Item(Item::Modifier),
    ),
    (
        DynastyHouse,
        "remove_all_house_modifier_instances",
        Item(Item::Modifier),
    ),
    (
        Province,
        "remove_all_province_modifier_instances",
        Item(Item::Modifier),
    ),
    (
        Artifact,
        "remove_artifact_feature_group",
        Item(Item::ArtifactFeatureGroup),
    ),
    (Artifact, "remove_artifact_modifier", Item(Item::Modifier)),
    (Province, "remove_building", Item(Item::Building)),
    (Character, "remove_character_flag", Unchecked),
    (Character, "remove_character_modifier", Item(Item::Modifier)),
    (Character, "remove_claim", Scope(Scopes::LandedTitle)),
    (Army, "remove_commanded", Yes),
    (Character, "remove_concubine", Scope(Scopes::Character)),
    (LandedTitle, "remove_county_modifier", Item(Item::Modifier)),
    (Character, "remove_courtier_or_guest", Special(RemoveGuest)),
    (Culture, "remove_culture_tradition", Item(Item::Tradition)),
    (Character, "remove_decision_cooldown", Item(Item::Decision)),
    (Faith, "remove_doctrine", Item(Item::Doctrine)),
    (Dynasty, "remove_dynasty_modifier", Item(Item::Modifier)),
    (Dynasty, "remove_dynasty_perk", Item(Item::DynastyPerk)),
    (ALL_BUT_NONE, "remove_from_list", Unchecked),
    (None, "remove_global_variable", Unchecked),
    (Province, "remove_holding", Yes),
    (
        Character,
        "remove_hook",
        ItemTarget("type", Item::Hook, "target", Scopes::Character),
    ),
    (
        DynastyHouse,
        "remove_house_artifact_claim",
        Scope(Scopes::Artifact),
    ),
    (DynastyHouse, "remove_house_modifier", Item(Item::Modifier)),
    (Culture, "remove_innovation", Item(Item::Innovation)),
    (
        Character,
        "remove_interaction_cooldown",
        Item(Item::Interaction),
    ),
    (
        Character,
        "remove_interaction_cooldown_against",
        ItemTarget(
            "interaction",
            Item::Interaction,
            "target",
            Scopes::Character,
        ),
    ),
    (None, "remove_list_global_variable", Special(AddToList)),
    (None, "remove_list_local_variable", Special(AddToList)),
    (None, "remove_list_variable", Special(AddToList)),
    (None, "remove_local_variable", Unchecked),
    (Character, "remove_localized_text", Unchecked),
    (Character, "remove_long_term_gold", NonNegativeValue),
    (Character, "remove_nickname", Item(Item::Nickname)),
    (Character, "remove_opinion", Special(RemoveOpinion)),
    (War, "remove_participant", Scope(Scopes::Character)),
    (Character, "remove_perk", Item(Item::Perk)),
    (
        Character,
        "remove_personal_artifact_claim",
        Scope(Scopes::Artifact),
    ),
    (Province, "remove_province_modifier", Item(Item::Modifier)),
    (Culture, "remove_random_culture_tradition", Yes),
    (Character, "remove_realm_law", Item(Item::Law)),
    (Character, "remove_relation_flag", Special(RelationFlag)),
    (Faith, "remove_religious_head_title", Yes),
    (Character, "remove_reserved_gold", NonNegativeValue),
    (
        Character,
        "remove_scheme_cooldown_against",
        ItemTarget("scheme", Item::Scheme, "target", Scopes::Character),
    ),
    (Scheme, "remove_province_modifier", Item(Item::Modifier)),
    (Secret, "remove_secret", Yes),
    (Character, "remove_short_term_gold", NonNegativeValue),
    (Faction, "remove_special_character", Yes),
    (Faction, "remove_special_title", Yes),
    // remove_title_law -- no idea
    // remove_title_law_effects -- no idea
    (Character, "remove_trait", Item(Item::Trait)),
    (Character, "remove_trait_force_tooltip", Item(Item::Trait)),
    (None, "remove_variable", Unchecked),
    (Character, "remove_war_chest_gold", NonNegativeValue),
    (
        Character,
        "replace_court_position",
        Special(ReplaceCourtPosition),
    ),
    (Character, "reset_beneficiary", Yes),
    (LandedTitle | Dynasty | DynastyHouse, "reset_coa", Yes),
    (Culture, "reset_culture_creation_date", Yes),
    (GreatHolyWar, "reset_designated_winner", Yes),
    (LandedTitle, "reset_title_name", Yes),
    (LandedTitle, "reset_title_prefix", Yes),
    (
        None,
        "resolve_title_and_vassal_change",
        Scope(Scopes::TitleAndVassalChange),
    ),
    (Character, "return_to_court", Yes),
    (Secret, "reveal_to", Scope(Scopes::Character)),
    (Character, "reverse_add_opinion", Special(AddOpinion)),
    // Docs are incorrect on this one. And "recipient" is the one getting fired.
    (
        Character,
        "revoke_court_position",
        ItemTarget(
            "court_position",
            Item::CourtPosition,
            "recipient",
            Scopes::Character,
        ),
    ),
    (LandedTitle, "revoke_lease", Yes),
    (None, "round_global_variable", Special(RoundVariable)),
    (None, "round_local_variable", Special(RoundVariable)),
    (None, "round_variable", Special(RoundVariable)),
    (None, "run_interaction", Special(RunInteraction)),
    (Character, "save_opinion_value_as", Special(SaveOpinion)),
    (ALL_BUT_NONE, "save_scope_as", Unchecked),
    (None, "save_scope_value_as", Special(SaveValue)),
    (
        Character,
        "save_temporary_opinion_value_as",
        Special(SaveOpinion),
    ),
    (ALL_BUT_NONE, "save_temporary_scope_as", Unchecked),
    (None, "save_temporary_scope_value_as", Special(SaveValue)),
    (Scheme, "scheme_freeze_days", Effect::Value),
    (
        Character,
        "send_interface_message",
        Control(InterfaceMessage),
    ),
    (Character, "send_interface_toast", Control(InterfaceMessage)),
    (Character, "set_absolute_country_control", Effect::Bool),
    (Character, "set_age", ScriptValue),
    (LandedTitle, "set_always_follows_primary_heir", Yes),
    (
        Character,
        "set_amenity_level",
        ItemValue("type", Item::Amenity),
    ),
    (Army, "set_army_location", Scope(Scopes::Province)),
    (Artifact, "set_artifact_description", Desc),
    (
        Artifact,
        "set_artifact_feature",
        Item(Item::ArtifactFeature),
    ),
    (
        Artifact,
        "set_artifact_feature_group",
        Item(Item::ArtifactFeatureGroup),
    ),
    (Artifact, "set_artifact_name", Desc),
    (Artifact, "set_artifact_rarity", Item(Item::ArtifactRarity)),
    (Character, "set_beneficiary", Scope(Scopes::Character)),
    (War, "set_called_to", Scope(Scopes::Character)),
    (LandedTitle, "set_can_be_named_after_dynasty", Effect::Bool),
    (LandedTitle, "set_capital_barony", Yes),
    (
        LandedTitle,
        "set_capital_county",
        Scope(Scopes::LandedTitle),
    ),
    (War, "set_casus_belli", Item(Item::CasusBelli)),
    (Character, "set_character_faith", Scope(Scopes::Faith)),
    (
        Character,
        "set_character_faith_history",
        Scope(Scopes::Faith),
    ),
    (
        Character,
        "set_character_faith_with_conversion",
        Scope(Scopes::Faith),
    ),
    (
        Character,
        "set_child_of_concubine_on_pregnancy",
        Effect::Bool,
    ),
    (
        LandedTitle | Dynasty | DynastyHouse,
        "set_coa",
        Special(SetCoa),
    ),
    (
        LandedTitle,
        "set_color_from_title",
        Scope(Scopes::LandedTitle),
    ),
    (
        Character,
        "set_council_task",
        ItemTarget("task_type", Item::CouncilTask, "target", Scopes::Character),
    ),
    (LandedTitle, "set_county_culture", Scope(Scopes::Culture)),
    (LandedTitle, "set_county_faith", Scope(Scopes::Faith)),
    (Character, "set_court_language", Item(Item::Language)),
    (Character, "set_court_type", Item(Item::CourtType)),
    (
        Culture,
        "set_cultural_acceptance",
        TargetValue("target", Scopes::Culture, "value"),
    ),
    (Character, "set_culture", Scope(Scopes::Culture)),
    (Struggle, "set_culture_as_involved", Scope(Scopes::Culture)),
    (
        Struggle,
        "set_culture_as_uninvolved",
        Scope(Scopes::Culture),
    ),
    (Culture, "set_culture_name", Special(SetCultureName)),
    (Culture, "set_culture_pillar", Item(Item::CulturePillar)),
    (Character, "set_culture_same_as", Scope(Scopes::Character)),
    (Character, "set_current_court_grandeur", Effect::Value),
    (
        LandedTitle,
        "set_de_jure_liege_title",
        Scope(Scopes::LandedTitle),
    ),
    (Character, "set_death_reason", Special(Death)),
    (Character, "set_default_education", Yes),
    (LandedTitle, "set_definitive_form", Effect::Bool),
    (LandedTitle, "set_delete_on_destroy", Effect::Bool),
    (Character, "set_designated_heir", Scope(Scopes::Character)),
    (
        GreatHolyWar,
        "set_designated_winner",
        Scope(Scopes::Character),
    ),
    (LandedTitle, "set_destroy_if_invalid_heir", Effect::Bool),
    (LandedTitle, "set_destroy_on_gain_same_tier", Effect::Bool),
    (LandedTitle, "set_destroy_on_succession", Effect::Bool),
    (Dynasty, "set_dynasty_name", Item(Item::Localization)),
    (Character, "set_employer", Scope(Scopes::Character)),
    (Culture, "set_ethos_from", Scope(Scopes::Culture)),
    (Struggle, "set_faith_as_involved", Scope(Scopes::Faith)),
    (Struggle, "set_faith_as_uninvolved", Scope(Scopes::Faith)),
    (Character, "set_father", Scope(Scopes::Character)),
    (Character, "set_focus", Item(Item::EducationFocus)),
    // set_focus_progress ??
    (None, "set_generated_asexuality_chance", Effect::Value),
    (None, "set_generated_bisexuality_chance", Effect::Value),
    (None, "set_generated_homosexuality_chance", Effect::Value),
    (None, "set_global_variable", Special(SetVariable)),
    (
        GreatHolyWar,
        "set_great_holy_war_target",
        Special(SetGhwTarget),
    ),
    (Culture, "set_heritage_from", Scope(Scopes::Culture)),
    (Province, "set_holding_type", Item(Item::Holding)),
    (Character, "set_house", Scope(Scopes::DynastyHouse)),
    (DynastyHouse, "set_house_name", Item(Item::Localization)),
    (
        DynastyHouse,
        "set_house_name_from_dynasty",
        Scope(Scopes::Dynasty),
    ),
    (
        DynastyHouse,
        "set_house_name_from_house",
        Scope(Scopes::DynastyHouse),
    ),
    (Character, "set_immortal_age", Effect::Value),
    (Character, "set_killer_public", Effect::Bool),
    (Character, "set_knight_status", Special(KnightStatus)),
    (Character, "set_known_bastard_on_pregnancy", Effect::Bool),
    (LandedTitle, "set_landless_title", Effect::Bool),
    (Culture, "set_language_from", Scope(Scopes::Culture)),
    (None, "set_local_variable", Special(SetVariable)),
    (Culture, "set_martial_custom_from", Scope(Scopes::Culture)),
    (Artifact, "set_max_durability", Effect::Value),
    (Character, "set_mother", Scope(Scopes::Character)),
    (Culture, "set_name_list", Item(Item::NameList)),
    (LandedTitle, "set_no_automatic_claims", Yes),
    (Character, "set_num_pregnancy_children", Integer),
    (Character, "set_override_designated_winner", Effect::Bool),
    (Artifact, "set_owner", Special(ArtifactOwner)),
    (Character, "set_player_character", Scope(Scopes::Character)),
    (
        Character,
        "set_pregnancy_assumed_father",
        Scope(Scopes::Character),
    ),
    (Character, "set_pregnancy_gender", Gender),
    (Character, "set_primary_spouse", Scope(Scopes::Character)),
    (
        Character,
        "set_primary_title_to",
        Scope(Scopes::LandedTitle),
    ),
    (Character, "set_real_father", Scope(Scopes::Character)),
    (Character, "set_realm_capital", Scope(Scopes::LandedTitle)),
    (
        Faith,
        "set_religious_head_title",
        Scope(Scopes::LandedTitle),
    ),
    (Character, "set_reserved_gold_maximum", Effect::Value),
    (Secret, "set_secret_owner", Scope(Scopes::Character)),
    (Character, "set_sexuality", Item(Item::Sexuality)),
    (Artifact, "set_should_decay", Effect::Bool),
    (Faction, "set_special_character", Scope(Scopes::Character)),
    (Faction, "set_special_title", Scope(Scopes::LandedTitle)),
    (LandedTitle, "set_title_name", Item(Item::Localization)),
    (LandedTitle, "set_title_prefix", Item(Item::Localization)),
    (Character, "set_to_lowborn", Yes),
    (Character, "set_trait_rank", Special(SetTraitRank)),
    (None, "set_variable", Special(SetVariable)),
    (
        Character,
        "set_vassal_contract_modification_blocked",
        Effect::Bool,
    ),
    (GreatHolyWar, "set_war_declarer", Scope(Scopes::Character)),
    (None, "setup_claim_cb", Special(SetupCb)),
    (None, "setup_de_jure_cb", Special(SetupCb)),
    (None, "setup_invasion_cb", Special(SetupCb)),
    (None, "show_as_tooltip", Control(ShowAsTooltip)),
    (Province, "spawn_activity", Special(SpawnActivity)),
    (Character, "spawn_army", Special(SpawnArmy)),
    (Secret, "spend_by", Scope(Scopes::Character)),
    (Character, "sponsor_inspiration", Scope(Scopes::Inspiration)),
    (Character, "start_default_task", Yes),
    (GreatHolyWar, "start_ghw_war", Item(Item::CasusBelli)),
    (Faith, "start_great_holy_war", Special(StartGhw)),
    (
        Character,
        "start_scheme",
        ItemTarget("type", Item::Scheme, "target", Scopes::Character),
    ),
    (None, "start_struggle", Special(StartStruggle)),
    (None, "start_tutorial_lesson", Unchecked),
    (Character, "start_war", Special(StartWar)),
    (Character, "store_localized_text_in_death", Unchecked),
    (Character, "stress_impact", Special(Stress)),
    (None, "switch", Control(Switch)),
    (
        LandedTitle,
        "title_create_faction",
        ItemTarget("type", Item::Faction, "target", Scopes::Character),
    ),
    (LandedTitle, "title_join_faction", Scope(Scopes::Faction)),
    (LandedTitle, "title_leave_faction", Scope(Scopes::Faction)),
    (None, "trigger_event", Special(TriggerEvent)),
    (
        None,
        "try_create_important_action",
        Special(CreateImportantAction),
    ),
    (None, "try_create_suggestion", Special(CreateSuggestion)),
    (Artifact, "unequip_artifact_from_owner", Yes),
    (
        Character,
        "unlearn_court_language_of",
        Scope(Scopes::Character),
    ),
    (Character, "unlearn_language", Item(Item::Language)),
    (
        Character,
        "unlearn_language_of_culture",
        Scope(Scopes::Culture),
    ),
    (GreatHolyWar, "unpledge_attacker", Scope(Scopes::Character)),
    (GreatHolyWar, "unpledge_defender", Scope(Scopes::Character)),
    (LandedTitle, "update_dynamic_coa", Yes),
    (Character, "use_hook", Scope(Scopes::Character)),
    (
        Character,
        "vassal_contract_decrease_obligation_level",
        Item(Item::VassalObligation),
    ),
    (
        Character,
        "vassal_contract_increase_obligation_level",
        Item(Item::VassalObligation),
    ),
    (
        Character,
        "vassal_contract_set_obligation_level",
        Special(VassalContractSet),
    ),
    (Character, "visit_court_of", Scope(Scopes::Character)),
    (None, "while", Control(While)),
    (CombatSide, "win_combat", Effect::Bool),
    // TODO special: add_<lifestyle>_perk_points
    // TODO special: add_<lifestyle>_xp
    // TODO special: remove_relation_<relation>
    // TODO special: set_relation_<relation>
];
