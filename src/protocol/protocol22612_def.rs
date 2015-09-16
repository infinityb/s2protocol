use super::{TypeInfo, IntBounds};
use phf::Map as PhfMap;

pub static REPLAY_HEADER_TYPEID: u32 = 13;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (67, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (59, "NNet.Game.SBankFileEvent"),
    8_u32 => (61, "NNet.Game.SBankSectionEvent"),
    9_u32 => (62, "NNet.Game.SBankKeyEvent"),
    10_u32 => (63, "NNet.Game.SBankValueEvent"),
    11_u32 => (65, "NNet.Game.SBankSignatureEvent"),
    12_u32 => (66, "NNet.Game.SUserOptionsEvent"),
    22_u32 => (68, "NNet.Game.SSaveGameEvent"),
    23_u32 => (67, "NNet.Game.SSaveGameDoneEvent"),
    25_u32 => (67, "NNet.Game.SPlayerLeaveEvent"),
    26_u32 => (72, "NNet.Game.SGameCheatEvent"),
    27_u32 => (82, "NNet.Game.SCmdEvent"),
    28_u32 => (90, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (91, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (93, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (95, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (96, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (99, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (100, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (101, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    36_u32 => (102, "NNet.Game.STriggerPingEvent"),
    37_u32 => (103, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (104, "NNet.Game.SAllianceEvent"),
    39_u32 => (105, "NNet.Game.SUnitClickEvent"),
    40_u32 => (106, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (107, "NNet.Game.STriggerReplySelectedEvent"),
    44_u32 => (67, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (112, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (116, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (117, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (118, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (121, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (67, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (108, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (67, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (109, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (67, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (111, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (115, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (123, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (126, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (127, "NNet.Game.STriggerMouseMovedEvent"),
    60_u32 => (128, "NNet.Game.SAchievementAwardedEvent"),
    63_u32 => (67, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (129, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (130, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (131, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (143, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (67, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (67, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (132, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (133, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (134, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (67, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (67, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (135, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    76_u32 => (136, "NNet.Game.SLagMessageEvent"),
    77_u32 => (67, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (67, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (137, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (67, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (67, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (138, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (139, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (139, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (109, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (67, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (67, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (141, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (142, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (144, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (145, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    92_u32 => (146, "NNet.Game.STriggerCameraMoveEvent"),
    93_u32 => (108, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (147, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (148, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (67, "NNet.Game.STriggerGameCreditsFinishedEvent"),
    97_u32 => (149, "NNet.Game.STriggerCutsceneBookmarkFiredEvent"),
    98_u32 => (150, "NNet.Game.STriggerCutsceneEndSceneFiredEvent"),
    99_u32 => (151, "NNet.Game.STriggerCutsceneConversationLineEvent"),
    100_u32 => (152, "NNet.Game.STriggerCutsceneConversationLineMissingEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (153, "NNet.Game.SChatMessage"),
    1_u32 => (154, "NNet.Game.SPingMessage"),
    2_u32 => (155, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (67, "NNet.Game.SServerPingMessage"),
};

pub static TYPEINFOS: &'static [TypeInfo] = &[
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 7 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 4 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 14 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 22 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 32 } },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("m_uint6", 2),
            1_u32 => ("m_uint14", 3),
            2_u32 => ("m_uint22", 4),
            3_u32 => ("m_uint32", 5),
        },
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 5 } },
    TypeInfo::Struct {
        fields: &[
            ("m_playerId", 7, -1),
        ],
    },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 8 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 8 } },
    TypeInfo::Struct {
        fields: &[
            ("m_flags", 10, 0),
            ("m_major", 10, 1),
            ("m_minor", 10, 2),
            ("m_revision", 10, 3),
            ("m_build", 5, 4),
            ("m_baseBuild", 5, 5),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 3 } },
    TypeInfo::Struct {
        fields: &[
            ("m_signature", 9, 0),
            ("m_version", 11, 1),
            ("m_type", 12, 2),
            ("m_elapsedGameLoops", 5, 3),
        ],
    },
    TypeInfo::FourCC,
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 7 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 64 } },
    TypeInfo::Struct {
        fields: &[
            ("m_region", 10, 0),
            ("m_programId", 14, 1),
            ("m_realm", 5, 2),
            ("m_name", 15, 3),
            ("m_id", 16, 4),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_a", 10, 0),
            ("m_r", 10, 1),
            ("m_g", 10, 2),
            ("m_b", 10, 3),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 2 } },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 9, 0),
            ("m_toon", 17, 1),
            ("m_race", 9, 2),
            ("m_color", 18, 3),
            ("m_control", 10, 4),
            ("m_teamId", 1, 5),
            ("m_handicap", 0, 6),
            ("m_observe", 19, 7),
            ("m_result", 19, 8),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 20 },
    TypeInfo::Optional { typeid: 21 },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 10 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("m_file", 24, 0),
        ],
    },
    TypeInfo::Bool,
    TypeInfo::Int { bounds: IntBounds { min: -9223372036854775808, bitlen: 64 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 12 } },
    TypeInfo::Blob { len: IntBounds { min: 40, bitlen: 0 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 29 },
    TypeInfo::Optional { typeid: 30 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 24 },
    TypeInfo::Optional { typeid: 32 },
    TypeInfo::Struct {
        fields: &[
            ("m_playerList", 22, 0),
            ("m_title", 23, 1),
            ("m_difficulty", 9, 2),
            ("m_thumbnail", 25, 3),
            ("m_isBlizzardMap", 26, 4),
            ("m_timeUTC", 27, 5),
            ("m_timeLocalOffset", 27, 6),
            ("m_description", 28, 7),
            ("m_imageFilePath", 24, 8),
            ("m_mapFileName", 24, 9),
            ("m_cacheHandles", 31, 10),
            ("m_miniSave", 26, 11),
            ("m_gameSpeed", 12, 12),
            ("m_defaultDifficulty", 2, 13),
            ("m_modPaths", 33, 14),
        ],
    },
    TypeInfo::Optional { typeid: 10 },
    TypeInfo::Struct {
        fields: &[
            ("m_race", 35, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_team", 35, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 9, -8),
            ("m_randomSeed", 5, -7),
            ("m_racePreference", 36, -6),
            ("m_teamPreference", 37, -5),
            ("m_testMap", 26, -4),
            ("m_testAuto", 26, -3),
            ("m_examine", 26, -2),
            ("m_observe", 19, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 38 },
    TypeInfo::Struct {
        fields: &[
            ("m_lockTeams", 26, -12),
            ("m_teamsTogether", 26, -11),
            ("m_advancedSharedControl", 26, -10),
            ("m_randomRaces", 26, -9),
            ("m_battleNet", 26, -8),
            ("m_amm", 26, -7),
            ("m_ranked", 26, -6),
            ("m_noVictoryOrDefeat", 26, -5),
            ("m_fog", 19, -4),
            ("m_observers", 19, -3),
            ("m_userDifficulty", 19, -2),
            ("m_clientDebugFlags", 16, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 4 } },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 2 } },
    TypeInfo::Struct {
        fields: &[
            ("m_allowedColors", 43, -5),
            ("m_allowedRaces", 44, -4),
            ("m_allowedDifficulty", 43, -3),
            ("m_allowedControls", 44, -2),
            ("m_allowedObserveTypes", 45, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 46 },
    TypeInfo::Struct {
        fields: &[
            ("m_randomValue", 5, -23),
            ("m_gameCacheName", 23, -22),
            ("m_gameOptions", 40, -21),
            ("m_gameSpeed", 12, -20),
            ("m_gameType", 12, -19),
            ("m_maxUsers", 7, -18),
            ("m_maxObservers", 7, -17),
            ("m_maxPlayers", 7, -16),
            ("m_maxTeams", 41, -15),
            ("m_maxColors", 2, -14),
            ("m_maxRaces", 42, -13),
            ("m_maxControls", 42, -12),
            ("m_mapSizeX", 10, -11),
            ("m_mapSizeY", 10, -10),
            ("m_mapFileSyncChecksum", 5, -9),
            ("m_mapFileName", 24, -8),
            ("m_mapAuthorName", 9, -7),
            ("m_modFileSyncChecksum", 5, -6),
            ("m_slotDescriptions", 47, -5),
            ("m_defaultDifficulty", 2, -4),
            ("m_cacheHandles", 30, -3),
            ("m_isBlizzardMap", 26, -2),
            ("m_isPremadeFFA", 26, -1),
        ],
    },
    TypeInfo::Optional { typeid: 1 },
    TypeInfo::Optional { typeid: 7 },
    TypeInfo::Struct {
        fields: &[
            ("m_color", 50, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 5 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_control", 10, -11),
            ("m_userId", 49, -10),
            ("m_teamId", 1, -9),
            ("m_colorPref", 51, -8),
            ("m_racePref", 36, -7),
            ("m_difficulty", 2, -6),
            ("m_handicap", 0, -5),
            ("m_observe", 19, -4),
            ("m_rewards", 52, -3),
            ("m_toonHandle", 15, -2),
            ("m_licenses", 53, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 54 },
    TypeInfo::Struct {
        fields: &[
            ("m_phase", 12, -9),
            ("m_maxUsers", 7, -8),
            ("m_maxObservers", 7, -7),
            ("m_slots", 55, -6),
            ("m_randomSeed", 5, -5),
            ("m_hostUserId", 49, -4),
            ("m_isSinglePlayer", 26, -3),
            ("m_gameDuration", 5, -2),
            ("m_defaultDifficulty", 2, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_userInitialData", 39, -3),
            ("m_gameDescription", 48, -2),
            ("m_lobbyState", 56, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncLobbyState", 57, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 15, -1),
        ],
    },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 60, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 60, -3),
            ("m_type", 5, -2),
            ("m_data", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_type", 5, -3),
            ("m_name", 60, -2),
            ("m_data", 28, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 10 },
    TypeInfo::Struct {
        fields: &[
            ("m_signature", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameFullyDownloaded", 26, -6),
            ("m_developmentCheatsEnabled", 26, -5),
            ("m_multiplayerCheatsEnabled", 26, -4),
            ("m_syncChecksummingEnabled", 26, -3),
            ("m_isMapToMapTransition", 26, -2),
            ("m_useAIBeacons", 26, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fileName", 24, -5),
            ("m_automatic", 26, -4),
            ("m_overwrite", 26, -3),
            ("m_name", 9, -2),
            ("m_description", 23, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -2147483648, bitlen: 32 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 69, -2),
            ("y", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 70, -4),
            ("m_time", 69, -3),
            ("m_verb", 23, -2),
            ("m_arguments", 23, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_data", 71, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 20 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 16 } },
    TypeInfo::Struct {
        fields: &[
            ("m_abilLink", 74, -3),
            ("m_abilCmdIndex", 7, -2),
            ("m_abilCmdData", 35, -1),
        ],
    },
    TypeInfo::Optional { typeid: 75 },
    TypeInfo::Null,
    TypeInfo::Struct {
        fields: &[
            ("x", 73, -3),
            ("y", 73, -2),
            ("z", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_targetUnitFlags", 10, -7),
            ("m_timer", 10, -6),
            ("m_tag", 5, -5),
            ("m_snapshotUnitLink", 74, -4),
            ("m_snapshotControlPlayerId", 49, -3),
            ("m_snapshotUpkeepPlayerId", 49, -2),
            ("m_snapshotPoint", 78, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 77),
            1_u32 => ("TargetPoint", 78),
            2_u32 => ("TargetUnit", 79),
            3_u32 => ("Data", 5),
        },
    },
    TypeInfo::Optional { typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_cmdFlags", 73, -4),
            ("m_abil", 76, -3),
            ("m_data", 80, -2),
            ("m_otherUnit", 81, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 83 },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 77),
            1_u32 => ("Mask", 84),
            2_u32 => ("OneIndices", 85),
            3_u32 => ("ZeroIndices", 85),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitLink", 74, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 83, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 87 },
    TypeInfo::Struct {
        fields: &[
            ("m_subgroupIndex", 83, -4),
            ("m_removeMask", 86, -3),
            ("m_addSubgroups", 88, -2),
            ("m_addUnitTags", 53, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 89, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 19, -2),
            ("m_mask", 86, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_count", 83, -6),
            ("m_subgroupCount", 83, -5),
            ("m_activeSubgroupIndex", 83, -4),
            ("m_unitTagsChecksum", 5, -3),
            ("m_subgroupIndicesChecksum", 5, -2),
            ("m_subgroupsChecksum", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 92, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 3 }, typeid: 69 },
    TypeInfo::Struct {
        fields: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 94, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_chatMessage", 23, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -128, bitlen: 8 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 69, -3),
            ("y", 69, -2),
            ("z", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_beacon", 97, -9),
            ("m_ally", 97, -8),
            ("m_flags", 97, -7),
            ("m_build", 97, -6),
            ("m_targetUnitTag", 5, -5),
            ("m_targetUnitSnapshotUnitLink", 74, -4),
            ("m_targetUnitSnapshotUpkeepPlayerId", 97, -3),
            ("m_targetUnitSnapshotControlPlayerId", 97, -2),
            ("m_targetPoint", 98, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_delta", 97, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 70, -3),
            ("m_unit", 5, -2),
            ("m_pingedMinimap", 26, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_verb", 23, -2),
            ("m_arguments", 23, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_alliance", 5, -2),
            ("m_control", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTag", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTag", 5, -2),
            ("m_flags", 10, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_conversationId", 69, -2),
            ("m_replyId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseItemId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_difficultyLevel", 69, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 3 },
        types: phf_map! {
            0_u32 => ("None", 77),
            1_u32 => ("Checked", 26),
            2_u32 => ("ValueChanged", 5),
            3_u32 => ("SelectionChanged", 69),
            4_u32 => ("TextChanged", 24),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlId", 69, -3),
            ("m_eventType", 69, -2),
            ("m_eventData", 110, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 5, -2),
            ("m_length", 5, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 8 }, typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 113, -2),
            ("m_length", 113, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncInfo", 114, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_sound", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 69, -2),
            ("m_thread", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("x", 74, -2),
            ("y", 74, -1),
        ],
    },
    TypeInfo::Optional { typeid: 74 },
    TypeInfo::Struct {
        fields: &[
            ("m_target", 119, -4),
            ("m_distance", 120, -3),
            ("m_pitch", 120, -2),
            ("m_yaw", 120, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 1 } },
    TypeInfo::Struct {
        fields: &[
            ("m_skipType", 122, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 124, -2),
            ("y", 124, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 5, -4),
            ("m_down", 26, -3),
            ("m_posUI", 125, -2),
            ("m_posWorld", 78, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_posUI", 125, -2),
            ("m_posWorld", 78, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_achievementLink", 74, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundtrack", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_planetId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_key", 97, -2),
            ("m_flags", 97, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_resources", 94, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fulfillRequestId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cancelRequestId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_researchItemId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_laggingPlayerId", 1, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_mercenaryId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 69, -2),
            ("m_difficultyLevel", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 69, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 19 } },
    TypeInfo::Struct {
        fields: &[
            ("m_decrementMs", 140, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_portraitId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_functionName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_result", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameMenuItemIndex", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_reason", 97, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseCategoryId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 74, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 69, -2),
            ("m_bookmarkName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 69, -3),
            ("m_conversationLine", 15, -2),
            ("m_altConversationLine", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 69, -2),
            ("m_conversationLine", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_string", 24, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_point", 70, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_progress", 69, -1),
        ],
    },
];

