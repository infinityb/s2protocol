use super::TypeInfo;
use phf::Map as PhfMap;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (64, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (56, "NNet.Game.SBankFileEvent"),
    8_u32 => (58, "NNet.Game.SBankSectionEvent"),
    9_u32 => (59, "NNet.Game.SBankKeyEvent"),
    10_u32 => (60, "NNet.Game.SBankValueEvent"),
    11_u32 => (62, "NNet.Game.SBankSignatureEvent"),
    12_u32 => (63, "NNet.Game.SUserOptionsEvent"),
    22_u32 => (65, "NNet.Game.SSaveGameEvent"),
    23_u32 => (64, "NNet.Game.SSaveGameDoneEvent"),
    25_u32 => (64, "NNet.Game.SPlayerLeaveEvent"),
    26_u32 => (69, "NNet.Game.SGameCheatEvent"),
    27_u32 => (80, "NNet.Game.SCmdEvent"),
    28_u32 => (87, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (88, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (90, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (92, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (93, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (96, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (97, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (98, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    37_u32 => (99, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (100, "NNet.Game.SAllianceEvent"),
    39_u32 => (101, "NNet.Game.SUnitClickEvent"),
    40_u32 => (102, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (103, "NNet.Game.STriggerReplySelectedEvent"),
    44_u32 => (64, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (108, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (111, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (112, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (112, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (115, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (64, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (104, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (64, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (105, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (64, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (107, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (110, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (117, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (120, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (121, "NNet.Game.STriggerMouseMovedEvent"),
    63_u32 => (64, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (122, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (123, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (124, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (136, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (64, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (64, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (125, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (126, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (127, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (64, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (64, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (128, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    76_u32 => (129, "NNet.Game.SLagMessageEvent"),
    77_u32 => (64, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (64, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (130, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (64, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (64, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (131, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (132, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (132, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (105, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (64, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (64, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (134, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (135, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (137, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (138, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    92_u32 => (139, "NNet.Game.STriggerCameraMoveEvent"),
    93_u32 => (104, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (140, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (141, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (64, "NNet.Game.STriggerGameCreditsFinishedEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (142, "NNet.Game.SChatMessage"),
    1_u32 => (143, "NNet.Game.SPingMessage"),
    2_u32 => (144, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (64, "NNet.Game.SServerPingMessage"),
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
    TypeInfo::Struct {
        items: &[
            ("m_control", 10, -10),
            ("m_userId", 47, -9),
            ("m_teamId", 1, -8),
            ("m_colorPref", 49, -7),
            ("m_racePref", 34, -6),
            ("m_difficulty", 2, -5),
            ("m_handicap", 0, -4),
            ("m_observe", 19, -3),
            ("m_rewards", 50, -2),
            ("m_toonHandle", 15, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 51 },
    TypeInfo::Struct {
        items: &[
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
        items: &[
            ("m_userInitialData", 37, -3),
            ("m_gameDescription", 46, -2),
            ("m_lobbyState", 53, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncLobbyState", 54, -1),
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
            ("m_name", 57, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 57, -3),
            ("m_type", 5, -2),
            ("m_data", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_type", 5, -3),
            ("m_name", 57, -2),
            ("m_data", 28, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 10 },
    TypeInfo::Struct {
        items: &[
            ("m_signature", 61, -1),
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
            ("x", 66, -2),
            ("y", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_point", 67, -4),
            ("m_time", 66, -3),
            ("m_verb", 23, -2),
            ("m_arguments", 23, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_data", 68, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 17 },
    TypeInfo::Int { min: 0, bits: 16 },
    TypeInfo::Struct {
        items: &[
            ("m_abilLink", 71, -3),
            ("m_abilCmdIndex", 7, -2),
            ("m_abilCmdData", 33, -1),
        ],
    },
    TypeInfo::Optional { typeid: 72 },
    TypeInfo::Null,
    TypeInfo::Int { min: 0, bits: 20 },
    TypeInfo::Struct {
        items: &[
            ("x", 75, -3),
            ("y", 75, -2),
            ("z", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_targetUnitFlags", 10, -6),
            ("m_timer", 10, -5),
            ("m_tag", 5, -4),
            ("m_snapshotUnitLink", 71, -3),
            ("m_snapshotPlayerId", 47, -2),
            ("m_snapshotPoint", 76, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 74),
            1_u32 => ("TargetPoint", 76),
            2_u32 => ("TargetUnit", 77),
            3_u32 => ("Data", 5),
        },
    },
    TypeInfo::Optional { typeid: 5 },
    TypeInfo::Struct {
        items: &[
            ("m_cmdFlags", 70, -4),
            ("m_abil", 73, -3),
            ("m_data", 78, -2),
            ("m_otherUnit", 79, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 8), typeid: 10 },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 74),
            1_u32 => ("Mask", 42),
            2_u32 => ("OneIndices", 81),
            3_u32 => ("ZeroIndices", 81),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitLink", 71, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 10, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 8), typeid: 83 },
    TypeInfo::Array { bounds: (0, 8), typeid: 5 },
    TypeInfo::Struct {
        items: &[
            ("m_subgroupIndex", 10, -4),
            ("m_removeMask", 82, -3),
            ("m_addSubgroups", 84, -2),
            ("m_addUnitTags", 85, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 86, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 19, -2),
            ("m_mask", 82, -1),
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
            ("m_selectionSyncData", 89, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 3), typeid: 66 },
    TypeInfo::Struct {
        items: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 91, -1),
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
            ("x", 66, -3),
            ("y", 66, -2),
            ("z", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_beacon", 94, -7),
            ("m_ally", 94, -6),
            ("m_autocast", 94, -5),
            ("m_targetUnitTag", 5, -4),
            ("m_targetUnitSnapshotUnitLink", 71, -3),
            ("m_targetUnitSnapshotPlayerId", 47, -2),
            ("m_targetPoint", 95, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_delta", 94, -1),
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
            ("m_conversationId", 66, -2),
            ("m_replyId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseItemId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_difficultyLevel", 66, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 3,
        types: phf_map! {
            0_u32 => ("None", 74),
            1_u32 => ("Checked", 26),
            2_u32 => ("ValueChanged", 5),
            3_u32 => ("SelectionChanged", 66),
            4_u32 => ("TextChanged", 24),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlId", 66, -3),
            ("m_eventType", 66, -2),
            ("m_eventData", 106, -1),
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
            ("m_soundHash", 85, -2),
            ("m_length", 85, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncInfo", 109, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_sound", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_transmissionId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("x", 71, -2),
            ("y", 71, -1),
        ],
    },
    TypeInfo::Optional { typeid: 71 },
    TypeInfo::Struct {
        items: &[
            ("m_target", 113, -4),
            ("m_distance", 114, -3),
            ("m_pitch", 114, -2),
            ("m_yaw", 114, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 1 },
    TypeInfo::Struct {
        items: &[
            ("m_skipType", 116, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 11 },
    TypeInfo::Struct {
        items: &[
            ("x", 118, -2),
            ("y", 118, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 5, -4),
            ("m_down", 26, -3),
            ("m_posUI", 119, -2),
            ("m_posWorld", 76, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_posUI", 119, -2),
            ("m_posWorld", 76, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_soundtrack", 5, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_planetId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_key", 94, -2),
            ("m_flags", 94, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_resources", 91, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_fulfillRequestId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cancelRequestId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_researchItemId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_laggingPlayerId", 1, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_mercenaryId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 66, -2),
            ("m_difficultyLevel", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 66, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 19 },
    TypeInfo::Struct {
        items: &[
            ("m_decrementMs", 133, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_portraitId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_functionName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_result", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_gameMenuItemIndex", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_reason", 94, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseCategoryId", 66, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 71, -1),
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
            ("m_point", 67, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_progress", 66, -1),
        ],
    },
];

