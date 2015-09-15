use super::TypeInfo;
use phf::Map as PhfMap;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (70, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (69, "NNet.Game.SUserOptionsEvent"),
    9_u32 => (62, "NNet.Game.SBankFileEvent"),
    10_u32 => (64, "NNet.Game.SBankSectionEvent"),
    11_u32 => (65, "NNet.Game.SBankKeyEvent"),
    12_u32 => (66, "NNet.Game.SBankValueEvent"),
    13_u32 => (68, "NNet.Game.SBankSignatureEvent"),
    14_u32 => (73, "NNet.Game.SCameraSaveEvent"),
    21_u32 => (74, "NNet.Game.SSaveGameEvent"),
    22_u32 => (70, "NNet.Game.SSaveGameDoneEvent"),
    23_u32 => (70, "NNet.Game.SLoadGameDoneEvent"),
    26_u32 => (78, "NNet.Game.SGameCheatEvent"),
    27_u32 => (86, "NNet.Game.SCmdEvent"),
    28_u32 => (94, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (95, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (97, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (99, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (100, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (103, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (104, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (105, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    36_u32 => (106, "NNet.Game.STriggerPingEvent"),
    37_u32 => (107, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (108, "NNet.Game.SAllianceEvent"),
    39_u32 => (109, "NNet.Game.SUnitClickEvent"),
    40_u32 => (110, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (111, "NNet.Game.STriggerReplySelectedEvent"),
    43_u32 => (116, "NNet.Game.SHijackReplayGameEvent"),
    44_u32 => (70, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (121, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (125, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (126, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (127, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (130, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (70, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (117, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (70, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (118, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (70, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (120, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (124, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (131, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (134, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (135, "NNet.Game.STriggerMouseMovedEvent"),
    60_u32 => (136, "NNet.Game.SAchievementAwardedEvent"),
    62_u32 => (137, "NNet.Game.STriggerTargetModeUpdateEvent"),
    63_u32 => (70, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (138, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (139, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (140, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (151, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (70, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (70, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (141, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (142, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (143, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (70, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (70, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (144, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    77_u32 => (70, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (70, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (145, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (70, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (70, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (146, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (147, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (147, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (118, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (70, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (70, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (149, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (150, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (152, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (153, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    92_u32 => (154, "NNet.Game.STriggerCameraMoveEvent"),
    93_u32 => (117, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (155, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (156, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (70, "NNet.Game.STriggerGameCreditsFinishedEvent"),
    97_u32 => (157, "NNet.Game.STriggerCutsceneBookmarkFiredEvent"),
    98_u32 => (158, "NNet.Game.STriggerCutsceneEndSceneFiredEvent"),
    99_u32 => (159, "NNet.Game.STriggerCutsceneConversationLineEvent"),
    100_u32 => (160, "NNet.Game.STriggerCutsceneConversationLineMissingEvent"),
    101_u32 => (70, "NNet.Game.SGameUserLeaveEvent"),
    102_u32 => (161, "NNet.Game.SGameUserJoinEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (162, "NNet.Game.SChatMessage"),
    1_u32 => (163, "NNet.Game.SPingMessage"),
    2_u32 => (164, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (70, "NNet.Game.SServerPingMessage"),
};

pub static TYPEINFOS: &'static [TypeInfo] = &[
    TypeInfo::Int { min: 0, bits: 7 },
    TypeInfo::Int { min: 0, bits: 4 },
    TypeInfo::Int { min: 0, bits: 5 },
    TypeInfo::Int { min: 0, bits: 6 },
    TypeInfo::Int { min: 0, bits: 14 },
    TypeInfo::Int { min: 0, bits: 22 },
    TypeInfo::Int { min: 0, bits: 32 },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("m_uint6", 3),
            1_u32 => ("m_uint14", 4),
            2_u32 => ("m_uint22", 5),
            3_u32 => ("m_uint32", 6),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_userId", 2, -1),
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
            ("m_build", 6, 4),
            ("m_baseBuild", 6, 5),
        ],
    },
    TypeInfo::Int { min: 0, bits: 3 },
    TypeInfo::Struct {
        items: &[
            ("m_signature", 9, 0),
            ("m_version", 11, 1),
            ("m_type", 12, 2),
            ("m_elapsedGameLoops", 6, 3),
        ],
    },
    TypeInfo::FourCC,
    TypeInfo::Blob { len_min: 0, len_bits: 7 },
    TypeInfo::Int { min: 0, bits: 64 },
    TypeInfo::Struct {
        items: &[
            ("m_region", 10, 0),
            ("m_programId", 14, 1),
            ("m_realm", 6, 2),
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
    TypeInfo::Optional { typeid: 10 },
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
            ("m_workingSetSlotId", 20, 9),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 21 },
    TypeInfo::Optional { typeid: 22 },
    TypeInfo::Blob { len_min: 0, len_bits: 10 },
    TypeInfo::Blob { len_min: 0, len_bits: 11 },
    TypeInfo::Struct {
        items: &[
            ("m_file", 25, 0),
        ],
    },
    TypeInfo::Bool,
    TypeInfo::Int { min: -9223372036854775808, bits: 64 },
    TypeInfo::Blob { len_min: 0, len_bits: 12 },
    TypeInfo::Blob { len_min: 40, len_bits: 0 },
    TypeInfo::Array { bounds: (0, 6), typeid: 30 },
    TypeInfo::Optional { typeid: 31 },
    TypeInfo::Array { bounds: (0, 6), typeid: 25 },
    TypeInfo::Optional { typeid: 33 },
    TypeInfo::Struct {
        items: &[
            ("m_playerList", 23, 0),
            ("m_title", 24, 1),
            ("m_difficulty", 9, 2),
            ("m_thumbnail", 26, 3),
            ("m_isBlizzardMap", 27, 4),
            ("m_timeUTC", 28, 5),
            ("m_timeLocalOffset", 28, 6),
            ("m_description", 29, 7),
            ("m_imageFilePath", 25, 8),
            ("m_campaignIndex", 10, 15),
            ("m_mapFileName", 25, 9),
            ("m_cacheHandles", 32, 10),
            ("m_miniSave", 27, 11),
            ("m_gameSpeed", 12, 12),
            ("m_defaultDifficulty", 3, 13),
            ("m_modPaths", 34, 14),
        ],
    },
    TypeInfo::Optional { typeid: 9 },
    TypeInfo::Optional { typeid: 6 },
    TypeInfo::Struct {
        items: &[
            ("m_race", 20, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_team", 20, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 9, -12),
            ("m_clanTag", 36, -11),
            ("m_highestLeague", 20, -10),
            ("m_combinedRaceLevels", 37, -9),
            ("m_randomSeed", 6, -8),
            ("m_racePreference", 38, -7),
            ("m_teamPreference", 39, -6),
            ("m_testMap", 27, -5),
            ("m_testAuto", 27, -4),
            ("m_examine", 27, -3),
            ("m_customInterface", 27, -2),
            ("m_observe", 19, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 40 },
    TypeInfo::Struct {
        items: &[
            ("m_lockTeams", 27, -12),
            ("m_teamsTogether", 27, -11),
            ("m_advancedSharedControl", 27, -10),
            ("m_randomRaces", 27, -9),
            ("m_battleNet", 27, -8),
            ("m_amm", 27, -7),
            ("m_competitive", 27, -6),
            ("m_noVictoryOrDefeat", 27, -5),
            ("m_fog", 19, -4),
            ("m_observers", 19, -3),
            ("m_userDifficulty", 19, -2),
            ("m_clientDebugFlags", 16, -1),
        ],
    },
    TypeInfo::Int { min: 1, bits: 4 },
    TypeInfo::Int { min: 1, bits: 8 },
    TypeInfo::BitArray { len_min: 0, len_bits: 6 },
    TypeInfo::BitArray { len_min: 0, len_bits: 8 },
    TypeInfo::BitArray { len_min: 0, len_bits: 2 },
    TypeInfo::BitArray { len_min: 0, len_bits: 7 },
    TypeInfo::Struct {
        items: &[
            ("m_allowedColors", 45, -6),
            ("m_allowedRaces", 46, -5),
            ("m_allowedDifficulty", 45, -4),
            ("m_allowedControls", 46, -3),
            ("m_allowedObserveTypes", 47, -2),
            ("m_allowedAIBuilds", 48, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 49 },
    TypeInfo::Struct {
        items: &[
            ("m_randomValue", 6, -25),
            ("m_gameCacheName", 24, -24),
            ("m_gameOptions", 42, -23),
            ("m_gameSpeed", 12, -22),
            ("m_gameType", 12, -21),
            ("m_maxUsers", 2, -20),
            ("m_maxObservers", 2, -19),
            ("m_maxPlayers", 2, -18),
            ("m_maxTeams", 43, -17),
            ("m_maxColors", 3, -16),
            ("m_maxRaces", 44, -15),
            ("m_maxControls", 44, -14),
            ("m_mapSizeX", 10, -13),
            ("m_mapSizeY", 10, -12),
            ("m_mapFileSyncChecksum", 6, -11),
            ("m_mapFileName", 25, -10),
            ("m_mapAuthorName", 9, -9),
            ("m_modFileSyncChecksum", 6, -8),
            ("m_slotDescriptions", 50, -7),
            ("m_defaultDifficulty", 3, -6),
            ("m_defaultAIBuild", 0, -5),
            ("m_cacheHandles", 31, -4),
            ("m_isBlizzardMap", 27, -3),
            ("m_isPremadeFFA", 27, -2),
            ("m_isCoopMode", 27, -1),
        ],
    },
    TypeInfo::Optional { typeid: 1 },
    TypeInfo::Optional { typeid: 2 },
    TypeInfo::Struct {
        items: &[
            ("m_color", 53, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 6), typeid: 6 },
    TypeInfo::Array { bounds: (0, 9), typeid: 6 },
    TypeInfo::Struct {
        items: &[
            ("m_control", 10, -13),
            ("m_userId", 52, -12),
            ("m_teamId", 1, -11),
            ("m_colorPref", 54, -10),
            ("m_racePref", 38, -9),
            ("m_difficulty", 3, -8),
            ("m_aiBuild", 0, -7),
            ("m_handicap", 0, -6),
            ("m_observe", 19, -5),
            ("m_workingSetSlotId", 20, -4),
            ("m_rewards", 55, -3),
            ("m_toonHandle", 15, -2),
            ("m_licenses", 56, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 57 },
    TypeInfo::Struct {
        items: &[
            ("m_phase", 12, -10),
            ("m_maxUsers", 2, -9),
            ("m_maxObservers", 2, -8),
            ("m_slots", 58, -7),
            ("m_randomSeed", 6, -6),
            ("m_hostUserId", 52, -5),
            ("m_isSinglePlayer", 27, -4),
            ("m_gameDuration", 6, -3),
            ("m_defaultDifficulty", 3, -2),
            ("m_defaultAIBuild", 0, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_userInitialData", 41, -3),
            ("m_gameDescription", 51, -2),
            ("m_lobbyState", 59, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncLobbyState", 60, -1),
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
            ("m_name", 63, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 63, -3),
            ("m_type", 6, -2),
            ("m_data", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_type", 6, -3),
            ("m_name", 63, -2),
            ("m_data", 29, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 10 },
    TypeInfo::Struct {
        items: &[
            ("m_signature", 67, -2),
            ("m_toonHandle", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_gameFullyDownloaded", 27, -7),
            ("m_developmentCheatsEnabled", 27, -6),
            ("m_multiplayerCheatsEnabled", 27, -5),
            ("m_syncChecksummingEnabled", 27, -4),
            ("m_isMapToMapTransition", 27, -3),
            ("m_startingRally", 27, -2),
            ("m_baseBuildNum", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
        ],
    },
    TypeInfo::Int { min: 0, bits: 16 },
    TypeInfo::Struct {
        items: &[
            ("x", 71, -2),
            ("y", 71, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_which", 12, -2),
            ("m_target", 72, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_fileName", 25, -5),
            ("m_automatic", 27, -4),
            ("m_overwrite", 27, -3),
            ("m_name", 9, -2),
            ("m_description", 24, -1),
        ],
    },
    TypeInfo::Int { min: -2147483648, bits: 32 },
    TypeInfo::Struct {
        items: &[
            ("x", 75, -2),
            ("y", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_point", 76, -4),
            ("m_time", 75, -3),
            ("m_verb", 24, -2),
            ("m_arguments", 24, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_data", 77, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 20 },
    TypeInfo::Struct {
        items: &[
            ("m_abilLink", 71, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_abilCmdData", 20, -1),
        ],
    },
    TypeInfo::Optional { typeid: 80 },
    TypeInfo::Null,
    TypeInfo::Struct {
        items: &[
            ("x", 79, -3),
            ("y", 79, -2),
            ("z", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_targetUnitFlags", 10, -7),
            ("m_timer", 10, -6),
            ("m_tag", 6, -5),
            ("m_snapshotUnitLink", 71, -4),
            ("m_snapshotControlPlayerId", 52, -3),
            ("m_snapshotUpkeepPlayerId", 52, -2),
            ("m_snapshotPoint", 83, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 82),
            1_u32 => ("TargetPoint", 83),
            2_u32 => ("TargetUnit", 84),
            3_u32 => ("Data", 6),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_cmdFlags", 79, -4),
            ("m_abil", 81, -3),
            ("m_data", 85, -2),
            ("m_otherUnit", 37, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 9 },
    TypeInfo::BitArray { len_min: 0, len_bits: 9 },
    TypeInfo::Array { bounds: (0, 9), typeid: 87 },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 82),
            1_u32 => ("Mask", 88),
            2_u32 => ("OneIndices", 89),
            3_u32 => ("ZeroIndices", 89),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitLink", 71, -4),
            ("m_subgroupPriority", 10, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 87, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 9), typeid: 91 },
    TypeInfo::Struct {
        items: &[
            ("m_subgroupIndex", 87, -4),
            ("m_removeMask", 90, -3),
            ("m_addSubgroups", 92, -2),
            ("m_addUnitTags", 56, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 93, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 19, -2),
            ("m_mask", 90, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_count", 87, -6),
            ("m_subgroupCount", 87, -5),
            ("m_activeSubgroupIndex", 87, -4),
            ("m_unitTagsChecksum", 6, -3),
            ("m_subgroupIndicesChecksum", 6, -2),
            ("m_subgroupsChecksum", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 96, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 3), typeid: 75 },
    TypeInfo::Struct {
        items: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 98, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_chatMessage", 24, -1),
        ],
    },
    TypeInfo::Int { min: -128, bits: 8 },
    TypeInfo::Struct {
        items: &[
            ("x", 75, -3),
            ("y", 75, -2),
            ("z", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_beacon", 101, -9),
            ("m_ally", 101, -8),
            ("m_flags", 101, -7),
            ("m_build", 101, -6),
            ("m_targetUnitTag", 6, -5),
            ("m_targetUnitSnapshotUnitLink", 71, -4),
            ("m_targetUnitSnapshotUpkeepPlayerId", 101, -3),
            ("m_targetUnitSnapshotControlPlayerId", 101, -2),
            ("m_targetPoint", 102, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_delta", 101, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_point", 76, -3),
            ("m_unit", 6, -2),
            ("m_pingedMinimap", 27, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_verb", 24, -2),
            ("m_arguments", 24, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_alliance", 6, -2),
            ("m_control", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTag", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTag", 6, -2),
            ("m_flags", 10, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_conversationId", 75, -2),
            ("m_replyId", 75, -1),
        ],
    },
    TypeInfo::Optional { typeid: 15 },
    TypeInfo::Struct {
        items: &[
            ("m_gameUserId", 1, -5),
            ("m_observe", 19, -4),
            ("m_name", 9, -3),
            ("m_toonHandle", 112, -2),
            ("m_clanTag", 36, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 113 },
    TypeInfo::Int { min: 0, bits: 1 },
    TypeInfo::Struct {
        items: &[
            ("m_userInfos", 114, -2),
            ("m_method", 115, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseItemId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_difficultyLevel", 75, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 3,
        types: phf_map! {
            0_u32 => ("None", 82),
            1_u32 => ("Checked", 27),
            2_u32 => ("ValueChanged", 6),
            3_u32 => ("SelectionChanged", 75),
            4_u32 => ("TextChanged", 25),
            5_u32 => ("MouseButton", 6),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlId", 75, -3),
            ("m_eventType", 75, -2),
            ("m_eventData", 119, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_soundHash", 6, -2),
            ("m_length", 6, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 7), typeid: 6 },
    TypeInfo::Struct {
        items: &[
            ("m_soundHash", 122, -2),
            ("m_length", 122, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncInfo", 123, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_sound", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_transmissionId", 75, -2),
            ("m_thread", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_transmissionId", 75, -1),
        ],
    },
    TypeInfo::Optional { typeid: 72 },
    TypeInfo::Optional { typeid: 71 },
    TypeInfo::Struct {
        items: &[
            ("m_target", 128, -4),
            ("m_distance", 129, -3),
            ("m_pitch", 129, -2),
            ("m_yaw", 129, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_skipType", 115, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 11 },
    TypeInfo::Struct {
        items: &[
            ("x", 132, -2),
            ("y", 132, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 6, -4),
            ("m_down", 27, -3),
            ("m_posUI", 133, -2),
            ("m_posWorld", 83, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_posUI", 133, -2),
            ("m_posWorld", 83, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_achievementLink", 71, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_abilLink", 71, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_state", 101, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_soundtrack", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_planetId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_key", 101, -2),
            ("m_flags", 101, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_resources", 98, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_fulfillRequestId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cancelRequestId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_researchItemId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_mercenaryId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 75, -2),
            ("m_difficultyLevel", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 75, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 19 },
    TypeInfo::Struct {
        items: &[
            ("m_decrementMs", 148, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_portraitId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_functionName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_result", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_gameMenuItemIndex", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_reason", 101, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseCategoryId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 71, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 75, -2),
            ("m_bookmarkName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 75, -3),
            ("m_conversationLine", 15, -2),
            ("m_altConversationLine", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 75, -2),
            ("m_conversationLine", 15, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_observe", 19, -4),
            ("m_name", 9, -3),
            ("m_toonHandle", 112, -2),
            ("m_clanTag", 36, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_recipient", 12, -2),
            ("m_string", 25, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_recipient", 12, -2),
            ("m_point", 76, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_progress", 75, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_scoreValueMineralsCurrent", 75, 0),
            ("m_scoreValueVespeneCurrent", 75, 1),
            ("m_scoreValueMineralsCollectionRate", 75, 2),
            ("m_scoreValueVespeneCollectionRate", 75, 3),
            ("m_scoreValueWorkersActiveCount", 75, 4),
            ("m_scoreValueMineralsUsedInProgressArmy", 75, 5),
            ("m_scoreValueMineralsUsedInProgressEconomy", 75, 6),
            ("m_scoreValueMineralsUsedInProgressTechnology", 75, 7),
            ("m_scoreValueVespeneUsedInProgressArmy", 75, 8),
            ("m_scoreValueVespeneUsedInProgressEconomy", 75, 9),
            ("m_scoreValueVespeneUsedInProgressTechnology", 75, 10),
            ("m_scoreValueMineralsUsedCurrentArmy", 75, 11),
            ("m_scoreValueMineralsUsedCurrentEconomy", 75, 12),
            ("m_scoreValueMineralsUsedCurrentTechnology", 75, 13),
            ("m_scoreValueVespeneUsedCurrentArmy", 75, 14),
            ("m_scoreValueVespeneUsedCurrentEconomy", 75, 15),
            ("m_scoreValueVespeneUsedCurrentTechnology", 75, 16),
            ("m_scoreValueMineralsLostArmy", 75, 17),
            ("m_scoreValueMineralsLostEconomy", 75, 18),
            ("m_scoreValueMineralsLostTechnology", 75, 19),
            ("m_scoreValueVespeneLostArmy", 75, 20),
            ("m_scoreValueVespeneLostEconomy", 75, 21),
            ("m_scoreValueVespeneLostTechnology", 75, 22),
            ("m_scoreValueMineralsKilledArmy", 75, 23),
            ("m_scoreValueMineralsKilledEconomy", 75, 24),
            ("m_scoreValueMineralsKilledTechnology", 75, 25),
            ("m_scoreValueVespeneKilledArmy", 75, 26),
            ("m_scoreValueVespeneKilledEconomy", 75, 27),
            ("m_scoreValueVespeneKilledTechnology", 75, 28),
            ("m_scoreValueFoodUsed", 75, 29),
            ("m_scoreValueFoodMade", 75, 30),
            ("m_scoreValueMineralsUsedActiveForces", 75, 31),
            ("m_scoreValueVespeneUsedActiveForces", 75, 32),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_playerId", 1, 0),
            ("m_stats", 165, 1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_unitTypeName", 24, 2),
            ("m_controlPlayerId", 1, 3),
            ("m_upkeepPlayerId", 1, 4),
            ("m_x", 10, 5),
            ("m_y", 10, 6),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_killerPlayerId", 52, 2),
            ("m_x", 10, 3),
            ("m_y", 10, 4),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_controlPlayerId", 1, 2),
            ("m_upkeepPlayerId", 1, 3),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_unitTypeName", 24, 2),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_playerId", 1, 0),
            ("m_upgradeTypeName", 24, 1),
            ("m_count", 75, 2),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
        ],
    },
    TypeInfo::Array { bounds: (0, 10), typeid: 75 },
    TypeInfo::Struct {
        items: &[
            ("m_firstUnitIndex", 6, 0),
            ("m_items", 173, 1),
        ],
    },
];

