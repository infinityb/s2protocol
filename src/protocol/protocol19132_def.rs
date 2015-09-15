use super::TypeInfo;
use phf::Map as PhfMap;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (65, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (57, "NNet.Game.SBankFileEvent"),
    8_u32 => (59, "NNet.Game.SBankSectionEvent"),
    9_u32 => (60, "NNet.Game.SBankKeyEvent"),
    10_u32 => (61, "NNet.Game.SBankValueEvent"),
    11_u32 => (63, "NNet.Game.SBankSignatureEvent"),
    12_u32 => (64, "NNet.Game.SUserOptionsEvent"),
    22_u32 => (66, "NNet.Game.SSaveGameEvent"),
    23_u32 => (65, "NNet.Game.SSaveGameDoneEvent"),
    25_u32 => (65, "NNet.Game.SPlayerLeaveEvent"),
    26_u32 => (70, "NNet.Game.SGameCheatEvent"),
    27_u32 => (81, "NNet.Game.SCmdEvent"),
    28_u32 => (88, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (89, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (91, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (93, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (94, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (97, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (98, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (99, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    37_u32 => (100, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (101, "NNet.Game.SAllianceEvent"),
    39_u32 => (102, "NNet.Game.SUnitClickEvent"),
    40_u32 => (103, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (104, "NNet.Game.STriggerReplySelectedEvent"),
    44_u32 => (65, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (109, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (112, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (113, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (113, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (116, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (65, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (105, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (65, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (106, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (65, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (108, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (111, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (118, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (121, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (122, "NNet.Game.STriggerMouseMovedEvent"),
    63_u32 => (65, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (123, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (124, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (125, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (137, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (65, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (65, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (126, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (127, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (128, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (65, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (65, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (129, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    76_u32 => (130, "NNet.Game.SLagMessageEvent"),
    77_u32 => (65, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (65, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (131, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (65, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (65, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (132, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (133, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (133, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (106, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (65, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (65, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (135, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (136, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (138, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (139, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    92_u32 => (140, "NNet.Game.STriggerCameraMoveEvent"),
    93_u32 => (105, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (141, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (142, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (65, "NNet.Game.STriggerGameCreditsFinishedEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (143, "NNet.Game.SChatMessage"),
    1_u32 => (144, "NNet.Game.SPingMessage"),
    2_u32 => (145, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (65, "NNet.Game.SServerPingMessage"),
};

pub static TYPEINFOS: &'static [TypeInfo] = &[
    TypeInfo::Int { min: 0, bits: 7 },
    TypeInfo::Int { min: 0, bits: 4 },
    TypeInfo::Int { min: 0, bits: 6 },
    TypeInfo::Int { min: 0, bits: 14 },
    TypeInfo::Int { min: 0, bits: 22 },
    TypeInfo::Int { min: 0, bits: 32 },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("m_uint6", 2),
            1_u32 => ("m_uint14", 3),
            2_u32 => ("m_uint22", 4),
            3_u32 => ("m_uint32", 5),
        },
    },
    TypeInfo::Int { min: 0, bits: 5 },
    TypeInfo::Struct {
        items: &[
            ("m_playerId", 7, -1),
        ],
    },
    TypeInfo::Blob { len_min: 0, len_bits: 8 },
    TypeInfo::Int { min: 0, bits: 8 },
    TypeInfo::Struct {
        items: &[
            ("m_flags", 10, 0),
            ("m_major", 10, 1),
            ("m_minor", 10, 2),
            ("m_revision", 10, 3),
            ("m_build", 5, 4),
            ("m_baseBuild", 5, 5),
        ],
    },
    TypeInfo::Int { min: 0, bits: 3 },
    TypeInfo::Struct {
        items: &[
            ("m_signature", 9, 0),
            ("m_version", 11, 1),
            ("m_type", 12, 2),
            ("m_elapsedGameLoops", 5, 3),
        ],
    },
    TypeInfo::FourCC,
    TypeInfo::Blob { len_min: 0, len_bits: 7 },
    TypeInfo::Int { min: 0, bits: 64 },
    TypeInfo::Struct {
        items: &[
            ("m_region", 10, 0),
            ("m_programId", 14, 1),
            ("m_realm", 5, 2),
            ("m_name", 15, 3),
            ("m_id", 16, 4),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_a", 10, 0),
            ("m_r", 10, 1),
            ("m_g", 10, 2),
            ("m_b", 10, 3),
        ],
    },
    TypeInfo::Int { min: 0, bits: 2 },
    TypeInfo::Struct {
        items: &[
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
    TypeInfo::Array { bounds: (0, 5), typeid: 20 },
    TypeInfo::Optional { typeid: 21 },
    TypeInfo::Blob { len_min: 0, len_bits: 10 },
    TypeInfo::Blob { len_min: 0, len_bits: 11 },
    TypeInfo::Struct {
        items: &[
            ("m_file", 24, 0),
        ],
    },
    TypeInfo::Bool,
    TypeInfo::Int { min: -9223372036854775808, bits: 64 },
    TypeInfo::Blob { len_min: 0, len_bits: 12 },
    TypeInfo::Blob { len_min: 40, len_bits: 0 },
    TypeInfo::Array { bounds: (0, 4), typeid: 29 },
    TypeInfo::Optional { typeid: 30 },
    TypeInfo::Struct {
        items: &[
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
        items: &[
            ("m_race", 33, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_team", 33, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 9, -7),
            ("m_randomSeed", 5, -6),
            ("m_racePreference", 34, -5),
            ("m_teamPreference", 35, -4),
            ("m_testMap", 26, -3),
            ("m_testAuto", 26, -2),
            ("m_observe", 19, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 36 },
    TypeInfo::Struct {
        items: &[
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
    TypeInfo::Int { min: 1, bits: 4 },
    TypeInfo::Int { min: 1, bits: 8 },
    TypeInfo::BitArray { len_min: 0, len_bits: 6 },
    TypeInfo::BitArray { len_min: 0, len_bits: 8 },
    TypeInfo::BitArray { len_min: 0, len_bits: 2 },
    TypeInfo::Struct {
        items: &[
            ("m_allowedColors", 41, -5),
            ("m_allowedRaces", 42, -4),
            ("m_allowedDifficulty", 41, -3),
            ("m_allowedControls", 42, -2),
            ("m_allowedObserveTypes", 43, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 44 },
    TypeInfo::Struct {
        items: &[
            ("m_randomValue", 5, -23),
            ("m_gameCacheName", 23, -22),
            ("m_gameOptions", 38, -21),
            ("m_gameSpeed", 12, -20),
            ("m_gameType", 12, -19),
            ("m_maxUsers", 7, -18),
            ("m_maxObservers", 7, -17),
            ("m_maxPlayers", 7, -16),
            ("m_maxTeams", 39, -15),
            ("m_maxColors", 2, -14),
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
        items: &[
            ("m_color", 48, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 5 },
    TypeInfo::Array { bounds: (0, 9), typeid: 5 },
    TypeInfo::Struct {
        items: &[
            ("m_control", 10, -11),
            ("m_userId", 47, -10),
            ("m_teamId", 1, -9),
            ("m_colorPref", 49, -8),
            ("m_racePref", 34, -7),
            ("m_difficulty", 2, -6),
            ("m_handicap", 0, -5),
            ("m_observe", 19, -4),
            ("m_rewards", 50, -3),
            ("m_toonHandle", 15, -2),
            ("m_licenses", 51, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 52 },
    TypeInfo::Struct {
        items: &[
            ("m_phase", 12, -9),
            ("m_maxUsers", 7, -8),
            ("m_maxObservers", 7, -7),
            ("m_slots", 53, -6),
            ("m_randomSeed", 5, -5),
            ("m_hostUserId", 47, -4),
            ("m_isSinglePlayer", 26, -3),
            ("m_gameDuration", 5, -2),
            ("m_defaultDifficulty", 2, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_userInitialData", 37, -3),
            ("m_gameDescription", 46, -2),
            ("m_lobbyState", 54, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncLobbyState", 55, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 15, -1),
        ],
    },
    TypeInfo::Blob { len_min: 0, len_bits: 6 },
    TypeInfo::Struct {
        items: &[
            ("m_name", 58, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 58, -3),
            ("m_type", 5, -2),
            ("m_data", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_type", 5, -3),
            ("m_name", 58, -2),
            ("m_data", 28, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 10 },
    TypeInfo::Struct {
        items: &[
            ("m_signature", 62, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_developmentCheatsEnabled", 26, -4),
            ("m_multiplayerCheatsEnabled", 26, -3),
            ("m_syncChecksummingEnabled", 26, -2),
            ("m_isMapToMapTransition", 26, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_fileName", 24, -5),
            ("m_automatic", 26, -4),
            ("m_overwrite", 26, -3),
            ("m_name", 9, -2),
            ("m_description", 23, -1),
        ],
    },
    TypeInfo::Int { min: -2147483648, bits: 32 },
    TypeInfo::Struct {
        items: &[
            ("x", 67, -2),
            ("y", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_point", 68, -4),
            ("m_time", 67, -3),
            ("m_verb", 23, -2),
            ("m_arguments", 23, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_data", 69, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 18 },
    TypeInfo::Int { min: 0, bits: 16 },
    TypeInfo::Struct {
        items: &[
            ("m_abilLink", 72, -3),
            ("m_abilCmdIndex", 7, -2),
            ("m_abilCmdData", 33, -1),
        ],
    },
    TypeInfo::Optional { typeid: 73 },
    TypeInfo::Null,
    TypeInfo::Int { min: 0, bits: 20 },
    TypeInfo::Struct {
        items: &[
            ("x", 76, -3),
            ("y", 76, -2),
            ("z", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_targetUnitFlags", 10, -6),
            ("m_timer", 10, -5),
            ("m_tag", 5, -4),
            ("m_snapshotUnitLink", 72, -3),
            ("m_snapshotPlayerId", 47, -2),
            ("m_snapshotPoint", 77, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 75),
            1_u32 => ("TargetPoint", 77),
            2_u32 => ("TargetUnit", 78),
            3_u32 => ("Data", 5),
        },
    },
    TypeInfo::Optional { typeid: 5 },
    TypeInfo::Struct {
        items: &[
            ("m_cmdFlags", 71, -4),
            ("m_abil", 74, -3),
            ("m_data", 79, -2),
            ("m_otherUnit", 80, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 8), typeid: 10 },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 75),
            1_u32 => ("Mask", 42),
            2_u32 => ("OneIndices", 82),
            3_u32 => ("ZeroIndices", 82),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitLink", 72, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 10, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 8), typeid: 84 },
    TypeInfo::Array { bounds: (0, 8), typeid: 5 },
    TypeInfo::Struct {
        items: &[
            ("m_subgroupIndex", 10, -4),
            ("m_removeMask", 83, -3),
            ("m_addSubgroups", 85, -2),
            ("m_addUnitTags", 86, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 87, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 19, -2),
            ("m_mask", 83, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_count", 10, -6),
            ("m_subgroupCount", 10, -5),
            ("m_activeSubgroupIndex", 10, -4),
            ("m_unitTagsChecksum", 5, -3),
            ("m_subgroupIndicesChecksum", 5, -2),
            ("m_subgroupsChecksum", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 90, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 3), typeid: 67 },
    TypeInfo::Struct {
        items: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 92, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_chatMessage", 23, -1),
        ],
    },
    TypeInfo::Int { min: -128, bits: 8 },
    TypeInfo::Struct {
        items: &[
            ("x", 67, -3),
            ("y", 67, -2),
            ("z", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_beacon", 95, -7),
            ("m_ally", 95, -6),
            ("m_autocast", 95, -5),
            ("m_targetUnitTag", 5, -4),
            ("m_targetUnitSnapshotUnitLink", 72, -3),
            ("m_targetUnitSnapshotPlayerId", 47, -2),
            ("m_targetPoint", 96, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_delta", 95, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_verb", 23, -2),
            ("m_arguments", 23, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_alliance", 5, -2),
            ("m_control", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTag", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTag", 5, -2),
            ("m_flags", 10, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_conversationId", 67, -2),
            ("m_replyId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseItemId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_difficultyLevel", 67, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 3,
        types: phf_map! {
            0_u32 => ("None", 75),
            1_u32 => ("Checked", 26),
            2_u32 => ("ValueChanged", 5),
            3_u32 => ("SelectionChanged", 67),
            4_u32 => ("TextChanged", 24),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlId", 67, -3),
            ("m_eventType", 67, -2),
            ("m_eventData", 107, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_soundHash", 5, -2),
            ("m_length", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_soundHash", 86, -2),
            ("m_length", 86, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncInfo", 110, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_sound", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_transmissionId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("x", 72, -2),
            ("y", 72, -1),
        ],
    },
    TypeInfo::Optional { typeid: 72 },
    TypeInfo::Struct {
        items: &[
            ("m_target", 114, -4),
            ("m_distance", 115, -3),
            ("m_pitch", 115, -2),
            ("m_yaw", 115, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 1 },
    TypeInfo::Struct {
        items: &[
            ("m_skipType", 117, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 11 },
    TypeInfo::Struct {
        items: &[
            ("x", 119, -2),
            ("y", 119, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 5, -4),
            ("m_down", 26, -3),
            ("m_posUI", 120, -2),
            ("m_posWorld", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_posUI", 120, -2),
            ("m_posWorld", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_soundtrack", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_planetId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_key", 95, -2),
            ("m_flags", 95, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_resources", 92, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_fulfillRequestId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cancelRequestId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_researchItemId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_laggingPlayerId", 1, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_mercenaryId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 67, -2),
            ("m_difficultyLevel", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 67, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 19 },
    TypeInfo::Struct {
        items: &[
            ("m_decrementMs", 134, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_portraitId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_functionName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_result", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_gameMenuItemIndex", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_reason", 95, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseCategoryId", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 72, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_recipient", 19, -2),
            ("m_string", 24, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_recipient", 19, -2),
            ("m_point", 68, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_progress", 67, -1),
        ],
    },
];

