use super::{TypeInfo, IntBounds};
use phf::Map as PhfMap;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (62, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (56, "NNet.Game.SBankFileEvent"),
    8_u32 => (58, "NNet.Game.SBankSectionEvent"),
    9_u32 => (59, "NNet.Game.SBankKeyEvent"),
    10_u32 => (60, "NNet.Game.SBankValueEvent"),
    11_u32 => (61, "NNet.Game.SUserOptionsEvent"),
    22_u32 => (63, "NNet.Game.SSaveGameEvent"),
    23_u32 => (62, "NNet.Game.SSaveGameDoneEvent"),
    25_u32 => (62, "NNet.Game.SPlayerLeaveEvent"),
    26_u32 => (67, "NNet.Game.SGameCheatEvent"),
    27_u32 => (70, "NNet.Game.SCmdEvent"),
    28_u32 => (76, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (78, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (80, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (82, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (83, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (85, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (86, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (87, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    37_u32 => (88, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (89, "NNet.Game.SAllianceEvent"),
    39_u32 => (90, "NNet.Game.SUnitClickEvent"),
    40_u32 => (91, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (92, "NNet.Game.STriggerReplySelectedEvent"),
    44_u32 => (62, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (98, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (101, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (102, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (102, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (105, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (62, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (93, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (62, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (94, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (62, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (97, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (100, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (107, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (108, "NNet.Game.STriggerMouseClickedEvent"),
    63_u32 => (62, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (109, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (110, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (111, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (122, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (62, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (62, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (112, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (113, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (114, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (62, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (62, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (115, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    76_u32 => (116, "NNet.Game.SLagMessageEvent"),
    77_u32 => (62, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (62, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (117, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (62, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (62, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (118, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (119, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (119, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (94, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (62, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (62, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (120, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (121, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (123, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (124, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    92_u32 => (125, "NNet.Game.STriggerCameraMoveEvent"),
    93_u32 => (93, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (126, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (127, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (62, "NNet.Game.STriggerGameCreditsFinishedEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (128, "NNet.Game.SChatMessage"),
    1_u32 => (129, "NNet.Game.SPingMessage"),
    2_u32 => (130, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (62, "NNet.Game.SServerPingMessage"),
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
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 4 }, typeid: 29 },
    TypeInfo::Optional { typeid: 30 },
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
        ],
    },
    TypeInfo::Optional { typeid: 10 },
    TypeInfo::Struct {
        fields: &[
            ("m_race", 33, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 9, -6),
            ("m_randomSeed", 5, -5),
            ("m_racePreference", 34, -4),
            ("m_testMap", 26, -3),
            ("m_testAuto", 26, -2),
            ("m_observe", 19, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 35 },
    TypeInfo::Struct {
        fields: &[
            ("m_lockTeams", 26, -11),
            ("m_teamsTogether", 26, -10),
            ("m_advancedSharedControl", 26, -9),
            ("m_randomRaces", 26, -8),
            ("m_battleNet", 26, -7),
            ("m_amm", 26, -6),
            ("m_ranked", 26, -5),
            ("m_noVictoryOrDefeat", 26, -4),
            ("m_fog", 19, -3),
            ("m_observers", 19, -2),
            ("m_userDifficulty", 19, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 4 } },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 5 } },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 2 } },
    TypeInfo::Struct {
        fields: &[
            ("m_allowedColors", 41, -5),
            ("m_allowedRaces", 42, -4),
            ("m_allowedDifficulty", 41, -3),
            ("m_allowedControls", 42, -2),
            ("m_allowedObserveTypes", 43, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 44 },
    TypeInfo::Struct {
        fields: &[
            ("m_randomValue", 5, -23),
            ("m_gameCacheName", 23, -22),
            ("m_gameOptions", 37, -21),
            ("m_gameSpeed", 12, -20),
            ("m_gameType", 12, -19),
            ("m_maxUsers", 7, -18),
            ("m_maxObservers", 7, -17),
            ("m_maxPlayers", 7, -16),
            ("m_maxTeams", 38, -15),
            ("m_maxColors", 39, -14),
            ("m_maxRaces", 40, -13),
            ("m_maxControls", 40, -12),
            ("m_mapSizeX", 10, -11),
            ("m_mapSizeY", 10, -10),
            ("m_mapFileSyncChecksum", 5, -9),
            ("m_mapFileName", 24, -8),
            ("m_mapAuthorName", 9, -7),
            ("m_modFileSyncChecksum", 5, -6),
            ("m_slotDescriptions", 45, -5),
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
            ("m_color", 48, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_control", 10, -9),
            ("m_userId", 47, -8),
            ("m_teamId", 1, -7),
            ("m_colorPref", 49, -6),
            ("m_racePref", 34, -5),
            ("m_difficulty", 2, -4),
            ("m_handicap", 0, -3),
            ("m_observe", 19, -2),
            ("m_rewards", 50, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 51 },
    TypeInfo::Struct {
        fields: &[
            ("m_phase", 12, -9),
            ("m_maxUsers", 7, -8),
            ("m_maxObservers", 7, -7),
            ("m_slots", 52, -6),
            ("m_randomSeed", 5, -5),
            ("m_hostUserId", 47, -4),
            ("m_isSinglePlayer", 26, -3),
            ("m_gameDuration", 5, -2),
            ("m_defaultDifficulty", 2, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_userInitialData", 36, -3),
            ("m_gameDescription", 46, -2),
            ("m_lobbyState", 53, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncLobbyState", 54, -1),
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
            ("m_name", 57, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 57, -3),
            ("m_type", 5, -2),
            ("m_data", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_type", 5, -3),
            ("m_name", 57, -2),
            ("m_data", 28, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_developmentCheatsEnabled", 26, -4),
            ("m_multiplayerCheatsEnabled", 26, -3),
            ("m_syncChecksummingEnabled", 26, -2),
            ("m_isMapToMapTransition", 26, -1),
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
            ("x", 64, -2),
            ("y", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 65, -4),
            ("m_time", 64, -3),
            ("m_verb", 23, -2),
            ("m_arguments", 23, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_data", 66, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 16 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 64, -3),
            ("y", 64, -2),
            ("z", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cmdFlags", 5, -11),
            ("m_abilLink", 68, -10),
            ("m_abilCmdIndex", 10, -9),
            ("m_abilCmdData", 10, -8),
            ("m_targetUnitFlags", 10, -7),
            ("m_targetUnitTimer", 10, -6),
            ("m_otherUnit", 5, -5),
            ("m_targetUnitTag", 5, -4),
            ("m_targetUnitSnapshotUnitLink", 68, -3),
            ("m_targetUnitSnapshotPlayerId", 47, -2),
            ("m_targetPoint", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("__parent", 42, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitLink", 68, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 10, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 8 }, typeid: 72 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 8 }, typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_subgroupIndex", 10, -4),
            ("m_removeMask", 71, -3),
            ("m_addSubgroups", 73, -2),
            ("m_addUnitTags", 74, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 75, -1),
        ],
    },
    TypeInfo::Optional { typeid: 71 },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 19, -2),
            ("m_mask", 77, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_count", 10, -6),
            ("m_subgroupCount", 10, -5),
            ("m_activeSubgroupIndex", 10, -4),
            ("m_unitTagsChecksum", 5, -3),
            ("m_subgroupIndicesChecksum", 5, -2),
            ("m_subgroupsChecksum", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 79, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 3 }, typeid: 64 },
    TypeInfo::Struct {
        fields: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 81, -1),
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
            ("m_beacon", 84, -7),
            ("m_ally", 84, -6),
            ("m_autocast", 84, -5),
            ("m_targetUnitTag", 5, -4),
            ("m_targetUnitSnapshotUnitLink", 68, -3),
            ("m_targetUnitSnapshotPlayerId", 47, -2),
            ("m_targetPoint", 69, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_delta", 84, -1),
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
            ("m_conversationId", 64, -2),
            ("m_replyId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseItemId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_difficultyLevel", 64, -1),
        ],
    },
    TypeInfo::Null,
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 3 },
        types: phf_map! {
            0_u32 => ("None", 95),
            1_u32 => ("Checked", 26),
            2_u32 => ("ValueChanged", 5),
            3_u32 => ("SelectionChanged", 64),
            4_u32 => ("TextChanged", 24),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlId", 64, -3),
            ("m_eventType", 64, -2),
            ("m_eventData", 96, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 5, -2),
            ("m_length", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 74, -2),
            ("m_length", 74, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncInfo", 99, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_sound", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("x", 68, -2),
            ("y", 68, -1),
        ],
    },
    TypeInfo::Optional { typeid: 68 },
    TypeInfo::Struct {
        fields: &[
            ("m_target", 103, -4),
            ("m_distance", 104, -3),
            ("m_pitch", 104, -2),
            ("m_yaw", 104, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 1 } },
    TypeInfo::Struct {
        fields: &[
            ("m_skipType", 106, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 5, -7),
            ("m_down", 26, -6),
            ("m_posXUI", 5, -5),
            ("m_posYUI", 5, -4),
            ("m_posXWorld", 64, -3),
            ("m_posYWorld", 64, -2),
            ("m_posZWorld", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundtrack", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_planetId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_key", 84, -2),
            ("m_flags", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_resources", 81, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fulfillRequestId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cancelRequestId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_researchItemId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_laggingPlayerId", 1, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_mercenaryId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 64, -2),
            ("m_difficultyLevel", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_decrementMs", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_portraitId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_functionName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_result", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameMenuItemIndex", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_reason", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseCategoryId", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 68, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 19, -2),
            ("m_string", 24, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 19, -2),
            ("m_point", 65, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_progress", 64, -1),
        ],
    },
];

