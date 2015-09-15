use super::TypeInfo;
use phf::Map as PhfMap;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (72, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (71, "NNet.Game.SUserOptionsEvent"),
    9_u32 => (64, "NNet.Game.SBankFileEvent"),
    10_u32 => (66, "NNet.Game.SBankSectionEvent"),
    11_u32 => (67, "NNet.Game.SBankKeyEvent"),
    12_u32 => (68, "NNet.Game.SBankValueEvent"),
    13_u32 => (70, "NNet.Game.SBankSignatureEvent"),
    14_u32 => (75, "NNet.Game.SCameraSaveEvent"),
    21_u32 => (76, "NNet.Game.SSaveGameEvent"),
    22_u32 => (72, "NNet.Game.SSaveGameDoneEvent"),
    23_u32 => (72, "NNet.Game.SLoadGameDoneEvent"),
    26_u32 => (80, "NNet.Game.SGameCheatEvent"),
    27_u32 => (88, "NNet.Game.SCmdEvent"),
    28_u32 => (96, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (97, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (99, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (101, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (102, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (105, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (106, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (107, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    36_u32 => (108, "NNet.Game.STriggerPingEvent"),
    37_u32 => (109, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (110, "NNet.Game.SAllianceEvent"),
    39_u32 => (111, "NNet.Game.SUnitClickEvent"),
    40_u32 => (112, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (113, "NNet.Game.STriggerReplySelectedEvent"),
    43_u32 => (118, "NNet.Game.SHijackReplayGameEvent"),
    44_u32 => (72, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (123, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (127, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (128, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (129, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (133, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (72, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (119, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (72, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (120, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (72, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (122, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (126, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (134, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (137, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (138, "NNet.Game.STriggerMouseMovedEvent"),
    60_u32 => (139, "NNet.Game.SAchievementAwardedEvent"),
    62_u32 => (140, "NNet.Game.STriggerTargetModeUpdateEvent"),
    63_u32 => (72, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (141, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (142, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (143, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (154, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (72, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (72, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (144, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (145, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (146, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (72, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (72, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (147, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    77_u32 => (72, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (72, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (148, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (72, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (72, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (149, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (150, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (150, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (120, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (72, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (72, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (152, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (153, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (155, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (156, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    93_u32 => (119, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (157, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (158, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (72, "NNet.Game.STriggerGameCreditsFinishedEvent"),
    97_u32 => (159, "NNet.Game.STriggerCutsceneBookmarkFiredEvent"),
    98_u32 => (160, "NNet.Game.STriggerCutsceneEndSceneFiredEvent"),
    99_u32 => (161, "NNet.Game.STriggerCutsceneConversationLineEvent"),
    100_u32 => (162, "NNet.Game.STriggerCutsceneConversationLineMissingEvent"),
    101_u32 => (72, "NNet.Game.SGameUserLeaveEvent"),
    102_u32 => (163, "NNet.Game.SGameUserJoinEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (164, "NNet.Game.SChatMessage"),
    1_u32 => (165, "NNet.Game.SPingMessage"),
    2_u32 => (166, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (72, "NNet.Game.SServerPingMessage"),
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
    TypeInfo::Bool,
    TypeInfo::Struct {
        items: &[
            ("m_signature", 9, 0),
            ("m_version", 11, 1),
            ("m_type", 12, 2),
            ("m_elapsedGameLoops", 6, 3),
            ("m_useScaledTime", 13, 4),
        ],
    },
    TypeInfo::FourCC,
    TypeInfo::Blob { len_min: 0, len_bits: 7 },
    TypeInfo::Int { min: 0, bits: 64 },
    TypeInfo::Struct {
        items: &[
            ("m_region", 10, 0),
            ("m_programId", 15, 1),
            ("m_realm", 6, 2),
            ("m_name", 16, 3),
            ("m_id", 17, 4),
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
            ("m_toon", 18, 1),
            ("m_race", 9, 2),
            ("m_color", 19, 3),
            ("m_control", 10, 4),
            ("m_teamId", 1, 5),
            ("m_handicap", 0, 6),
            ("m_observe", 20, 7),
            ("m_result", 20, 8),
            ("m_workingSetSlotId", 21, 9),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 22 },
    TypeInfo::Optional { typeid: 23 },
    TypeInfo::Blob { len_min: 0, len_bits: 10 },
    TypeInfo::Blob { len_min: 0, len_bits: 11 },
    TypeInfo::Struct {
        items: &[
            ("m_file", 26, 0),
        ],
    },
    TypeInfo::Optional { typeid: 13 },
    TypeInfo::Int { min: -9223372036854775808, bits: 64 },
    TypeInfo::Blob { len_min: 0, len_bits: 12 },
    TypeInfo::Blob { len_min: 40, len_bits: 0 },
    TypeInfo::Array { bounds: (0, 6), typeid: 31 },
    TypeInfo::Optional { typeid: 32 },
    TypeInfo::Array { bounds: (0, 6), typeid: 26 },
    TypeInfo::Optional { typeid: 34 },
    TypeInfo::Struct {
        items: &[
            ("m_playerList", 24, 0),
            ("m_title", 25, 1),
            ("m_difficulty", 9, 2),
            ("m_thumbnail", 27, 3),
            ("m_isBlizzardMap", 13, 4),
            ("m_restartAsTransitionMap", 28, 16),
            ("m_timeUTC", 29, 5),
            ("m_timeLocalOffset", 29, 6),
            ("m_description", 30, 7),
            ("m_imageFilePath", 26, 8),
            ("m_campaignIndex", 10, 15),
            ("m_mapFileName", 26, 9),
            ("m_cacheHandles", 33, 10),
            ("m_miniSave", 13, 11),
            ("m_gameSpeed", 12, 12),
            ("m_defaultDifficulty", 3, 13),
            ("m_modPaths", 35, 14),
        ],
    },
    TypeInfo::Optional { typeid: 9 },
    TypeInfo::Optional { typeid: 31 },
    TypeInfo::Optional { typeid: 6 },
    TypeInfo::Struct {
        items: &[
            ("m_race", 21, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_team", 21, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 9, -13),
            ("m_clanTag", 37, -12),
            ("m_clanLogo", 38, -11),
            ("m_highestLeague", 21, -10),
            ("m_combinedRaceLevels", 39, -9),
            ("m_randomSeed", 6, -8),
            ("m_racePreference", 40, -7),
            ("m_teamPreference", 41, -6),
            ("m_testMap", 13, -5),
            ("m_testAuto", 13, -4),
            ("m_examine", 13, -3),
            ("m_customInterface", 13, -2),
            ("m_observe", 20, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 42 },
    TypeInfo::Struct {
        items: &[
            ("m_lockTeams", 13, -12),
            ("m_teamsTogether", 13, -11),
            ("m_advancedSharedControl", 13, -10),
            ("m_randomRaces", 13, -9),
            ("m_battleNet", 13, -8),
            ("m_amm", 13, -7),
            ("m_competitive", 13, -6),
            ("m_noVictoryOrDefeat", 13, -5),
            ("m_fog", 20, -4),
            ("m_observers", 20, -3),
            ("m_userDifficulty", 20, -2),
            ("m_clientDebugFlags", 17, -1),
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
            ("m_allowedColors", 47, -6),
            ("m_allowedRaces", 48, -5),
            ("m_allowedDifficulty", 47, -4),
            ("m_allowedControls", 48, -3),
            ("m_allowedObserveTypes", 49, -2),
            ("m_allowedAIBuilds", 50, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 51 },
    TypeInfo::Struct {
        items: &[
            ("m_randomValue", 6, -26),
            ("m_gameCacheName", 25, -25),
            ("m_gameOptions", 44, -24),
            ("m_gameSpeed", 12, -23),
            ("m_gameType", 12, -22),
            ("m_maxUsers", 2, -21),
            ("m_maxObservers", 2, -20),
            ("m_maxPlayers", 2, -19),
            ("m_maxTeams", 45, -18),
            ("m_maxColors", 3, -17),
            ("m_maxRaces", 46, -16),
            ("m_maxControls", 10, -15),
            ("m_mapSizeX", 10, -14),
            ("m_mapSizeY", 10, -13),
            ("m_mapFileSyncChecksum", 6, -12),
            ("m_mapFileName", 26, -11),
            ("m_mapAuthorName", 9, -10),
            ("m_modFileSyncChecksum", 6, -9),
            ("m_slotDescriptions", 52, -8),
            ("m_defaultDifficulty", 3, -7),
            ("m_defaultAIBuild", 0, -6),
            ("m_cacheHandles", 32, -5),
            ("m_hasExtensionMod", 13, -4),
            ("m_isBlizzardMap", 13, -3),
            ("m_isPremadeFFA", 13, -2),
            ("m_isCoopMode", 13, -1),
        ],
    },
    TypeInfo::Optional { typeid: 1 },
    TypeInfo::Optional { typeid: 2 },
    TypeInfo::Struct {
        items: &[
            ("m_color", 55, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 6), typeid: 6 },
    TypeInfo::Array { bounds: (0, 9), typeid: 6 },
    TypeInfo::Struct {
        items: &[
            ("m_control", 10, -14),
            ("m_userId", 54, -13),
            ("m_teamId", 1, -12),
            ("m_colorPref", 56, -11),
            ("m_racePref", 40, -10),
            ("m_difficulty", 3, -9),
            ("m_aiBuild", 0, -8),
            ("m_handicap", 0, -7),
            ("m_observe", 20, -6),
            ("m_logoIndex", 6, -5),
            ("m_workingSetSlotId", 21, -4),
            ("m_rewards", 57, -3),
            ("m_toonHandle", 16, -2),
            ("m_licenses", 58, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 59 },
    TypeInfo::Struct {
        items: &[
            ("m_phase", 12, -10),
            ("m_maxUsers", 2, -9),
            ("m_maxObservers", 2, -8),
            ("m_slots", 60, -7),
            ("m_randomSeed", 6, -6),
            ("m_hostUserId", 54, -5),
            ("m_isSinglePlayer", 13, -4),
            ("m_gameDuration", 6, -3),
            ("m_defaultDifficulty", 3, -2),
            ("m_defaultAIBuild", 0, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_userInitialData", 43, -3),
            ("m_gameDescription", 53, -2),
            ("m_lobbyState", 61, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncLobbyState", 62, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 16, -1),
        ],
    },
    TypeInfo::Blob { len_min: 0, len_bits: 6 },
    TypeInfo::Struct {
        items: &[
            ("m_name", 65, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_name", 65, -3),
            ("m_type", 6, -2),
            ("m_data", 16, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_type", 6, -3),
            ("m_name", 65, -2),
            ("m_data", 30, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 10 },
    TypeInfo::Struct {
        items: &[
            ("m_signature", 69, -2),
            ("m_toonHandle", 16, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_gameFullyDownloaded", 13, -8),
            ("m_developmentCheatsEnabled", 13, -7),
            ("m_multiplayerCheatsEnabled", 13, -6),
            ("m_syncChecksummingEnabled", 13, -5),
            ("m_isMapToMapTransition", 13, -4),
            ("m_startingRally", 13, -3),
            ("m_debugPauseEnabled", 13, -2),
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
            ("x", 73, -2),
            ("y", 73, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_which", 12, -2),
            ("m_target", 74, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_fileName", 26, -5),
            ("m_automatic", 13, -4),
            ("m_overwrite", 13, -3),
            ("m_name", 9, -2),
            ("m_description", 25, -1),
        ],
    },
    TypeInfo::Int { min: -2147483648, bits: 32 },
    TypeInfo::Struct {
        items: &[
            ("x", 77, -2),
            ("y", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_point", 78, -4),
            ("m_time", 77, -3),
            ("m_verb", 25, -2),
            ("m_arguments", 25, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_data", 79, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 20 },
    TypeInfo::Struct {
        items: &[
            ("m_abilLink", 73, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_abilCmdData", 21, -1),
        ],
    },
    TypeInfo::Optional { typeid: 82 },
    TypeInfo::Null,
    TypeInfo::Struct {
        items: &[
            ("x", 81, -3),
            ("y", 81, -2),
            ("z", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_targetUnitFlags", 10, -7),
            ("m_timer", 10, -6),
            ("m_tag", 6, -5),
            ("m_snapshotUnitLink", 73, -4),
            ("m_snapshotControlPlayerId", 54, -3),
            ("m_snapshotUpkeepPlayerId", 54, -2),
            ("m_snapshotPoint", 85, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 84),
            1_u32 => ("TargetPoint", 85),
            2_u32 => ("TargetUnit", 86),
            3_u32 => ("Data", 6),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_cmdFlags", 81, -4),
            ("m_abil", 83, -3),
            ("m_data", 87, -2),
            ("m_otherUnit", 39, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 9 },
    TypeInfo::BitArray { len_min: 0, len_bits: 9 },
    TypeInfo::Array { bounds: (0, 9), typeid: 89 },
    TypeInfo::Choice {
        min: 0,
        bits: 2,
        types: phf_map! {
            0_u32 => ("None", 84),
            1_u32 => ("Mask", 90),
            2_u32 => ("OneIndices", 91),
            3_u32 => ("ZeroIndices", 91),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitLink", 73, -4),
            ("m_subgroupPriority", 10, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 89, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 9), typeid: 93 },
    TypeInfo::Struct {
        items: &[
            ("m_subgroupIndex", 89, -4),
            ("m_removeMask", 92, -3),
            ("m_addSubgroups", 94, -2),
            ("m_addUnitTags", 58, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 95, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 20, -2),
            ("m_mask", 92, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_count", 89, -6),
            ("m_subgroupCount", 89, -5),
            ("m_activeSubgroupIndex", 89, -4),
            ("m_unitTagsChecksum", 6, -3),
            ("m_subgroupIndicesChecksum", 6, -2),
            ("m_subgroupsChecksum", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 98, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 3), typeid: 77 },
    TypeInfo::Struct {
        items: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 100, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_chatMessage", 25, -1),
        ],
    },
    TypeInfo::Int { min: -128, bits: 8 },
    TypeInfo::Struct {
        items: &[
            ("x", 77, -3),
            ("y", 77, -2),
            ("z", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_beacon", 103, -9),
            ("m_ally", 103, -8),
            ("m_flags", 103, -7),
            ("m_build", 103, -6),
            ("m_targetUnitTag", 6, -5),
            ("m_targetUnitSnapshotUnitLink", 73, -4),
            ("m_targetUnitSnapshotUpkeepPlayerId", 103, -3),
            ("m_targetUnitSnapshotControlPlayerId", 103, -2),
            ("m_targetPoint", 104, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_delta", 103, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_point", 78, -3),
            ("m_unit", 6, -2),
            ("m_pingedMinimap", 13, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_verb", 25, -2),
            ("m_arguments", 25, -1),
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
            ("m_conversationId", 77, -2),
            ("m_replyId", 77, -1),
        ],
    },
    TypeInfo::Optional { typeid: 16 },
    TypeInfo::Struct {
        items: &[
            ("m_gameUserId", 1, -6),
            ("m_observe", 20, -5),
            ("m_name", 9, -4),
            ("m_toonHandle", 114, -3),
            ("m_clanTag", 37, -2),
            ("m_clanLogo", 38, -1),
        ],
    },
    TypeInfo::Array { bounds: (0, 5), typeid: 115 },
    TypeInfo::Int { min: 0, bits: 1 },
    TypeInfo::Struct {
        items: &[
            ("m_userInfos", 116, -2),
            ("m_method", 117, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseItemId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_difficultyLevel", 77, -1),
        ],
    },
    TypeInfo::Choice {
        min: 0,
        bits: 3,
        types: phf_map! {
            0_u32 => ("None", 84),
            1_u32 => ("Checked", 13),
            2_u32 => ("ValueChanged", 6),
            3_u32 => ("SelectionChanged", 77),
            4_u32 => ("TextChanged", 26),
            5_u32 => ("MouseButton", 6),
        },
    },
    TypeInfo::Struct {
        items: &[
            ("m_controlId", 77, -3),
            ("m_eventType", 77, -2),
            ("m_eventData", 121, -1),
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
            ("m_soundHash", 124, -2),
            ("m_length", 124, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_syncInfo", 125, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_sound", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_transmissionId", 77, -2),
            ("m_thread", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_transmissionId", 77, -1),
        ],
    },
    TypeInfo::Optional { typeid: 74 },
    TypeInfo::Optional { typeid: 73 },
    TypeInfo::Optional { typeid: 103 },
    TypeInfo::Struct {
        items: &[
            ("m_target", 130, -5),
            ("m_distance", 131, -4),
            ("m_pitch", 131, -3),
            ("m_yaw", 131, -2),
            ("m_reason", 132, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_skipType", 117, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 11 },
    TypeInfo::Struct {
        items: &[
            ("x", 135, -2),
            ("y", 135, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 6, -5),
            ("m_down", 13, -4),
            ("m_posUI", 136, -3),
            ("m_posWorld", 85, -2),
            ("m_flags", 103, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_posUI", 136, -3),
            ("m_posWorld", 85, -2),
            ("m_flags", 103, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_achievementLink", 73, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_abilLink", 73, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_state", 103, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_soundtrack", 6, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_planetId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_key", 103, -2),
            ("m_flags", 103, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_resources", 100, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_fulfillRequestId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cancelRequestId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_researchItemId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_mercenaryId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 77, -2),
            ("m_difficultyLevel", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_battleReportId", 77, -1),
        ],
    },
    TypeInfo::Int { min: 0, bits: 19 },
    TypeInfo::Struct {
        items: &[
            ("m_decrementMs", 151, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_portraitId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_functionName", 16, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_result", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_gameMenuItemIndex", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_purchaseCategoryId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_button", 73, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 77, -2),
            ("m_bookmarkName", 16, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 77, -3),
            ("m_conversationLine", 16, -2),
            ("m_altConversationLine", 16, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_cutsceneId", 77, -2),
            ("m_conversationLine", 16, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_observe", 20, -5),
            ("m_name", 9, -4),
            ("m_toonHandle", 114, -3),
            ("m_clanTag", 37, -2),
            ("m_clanLogo", 38, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_recipient", 12, -2),
            ("m_string", 26, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_recipient", 12, -2),
            ("m_point", 78, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_progress", 77, -1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_scoreValueMineralsCurrent", 77, 0),
            ("m_scoreValueVespeneCurrent", 77, 1),
            ("m_scoreValueMineralsCollectionRate", 77, 2),
            ("m_scoreValueVespeneCollectionRate", 77, 3),
            ("m_scoreValueWorkersActiveCount", 77, 4),
            ("m_scoreValueMineralsUsedInProgressArmy", 77, 5),
            ("m_scoreValueMineralsUsedInProgressEconomy", 77, 6),
            ("m_scoreValueMineralsUsedInProgressTechnology", 77, 7),
            ("m_scoreValueVespeneUsedInProgressArmy", 77, 8),
            ("m_scoreValueVespeneUsedInProgressEconomy", 77, 9),
            ("m_scoreValueVespeneUsedInProgressTechnology", 77, 10),
            ("m_scoreValueMineralsUsedCurrentArmy", 77, 11),
            ("m_scoreValueMineralsUsedCurrentEconomy", 77, 12),
            ("m_scoreValueMineralsUsedCurrentTechnology", 77, 13),
            ("m_scoreValueVespeneUsedCurrentArmy", 77, 14),
            ("m_scoreValueVespeneUsedCurrentEconomy", 77, 15),
            ("m_scoreValueVespeneUsedCurrentTechnology", 77, 16),
            ("m_scoreValueMineralsLostArmy", 77, 17),
            ("m_scoreValueMineralsLostEconomy", 77, 18),
            ("m_scoreValueMineralsLostTechnology", 77, 19),
            ("m_scoreValueVespeneLostArmy", 77, 20),
            ("m_scoreValueVespeneLostEconomy", 77, 21),
            ("m_scoreValueVespeneLostTechnology", 77, 22),
            ("m_scoreValueMineralsKilledArmy", 77, 23),
            ("m_scoreValueMineralsKilledEconomy", 77, 24),
            ("m_scoreValueMineralsKilledTechnology", 77, 25),
            ("m_scoreValueVespeneKilledArmy", 77, 26),
            ("m_scoreValueVespeneKilledEconomy", 77, 27),
            ("m_scoreValueVespeneKilledTechnology", 77, 28),
            ("m_scoreValueFoodUsed", 77, 29),
            ("m_scoreValueFoodMade", 77, 30),
            ("m_scoreValueMineralsUsedActiveForces", 77, 31),
            ("m_scoreValueVespeneUsedActiveForces", 77, 32),
            ("m_scoreValueMineralsFriendlyFireArmy", 77, 33),
            ("m_scoreValueMineralsFriendlyFireEconomy", 77, 34),
            ("m_scoreValueMineralsFriendlyFireTechnology", 77, 35),
            ("m_scoreValueVespeneFriendlyFireArmy", 77, 36),
            ("m_scoreValueVespeneFriendlyFireEconomy", 77, 37),
            ("m_scoreValueVespeneFriendlyFireTechnology", 77, 38),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_playerId", 1, 0),
            ("m_stats", 167, 1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_unitTypeName", 25, 2),
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
            ("m_killerPlayerId", 54, 2),
            ("m_x", 10, 3),
            ("m_y", 10, 4),
            ("m_killerUnitTagIndex", 39, 5),
            ("m_killerUnitTagRecycle", 39, 6),
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
            ("m_unitTypeName", 25, 2),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_playerId", 1, 0),
            ("m_upgradeTypeName", 25, 1),
            ("m_count", 77, 2),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
        ],
    },
    TypeInfo::Array { bounds: (0, 10), typeid: 77 },
    TypeInfo::Struct {
        items: &[
            ("m_firstUnitIndex", 6, 0),
            ("m_items", 175, 1),
        ],
    },
    TypeInfo::Struct {
        items: &[
            ("m_playerId", 1, 0),
            ("m_type", 6, 1),
            ("m_userId", 39, 2),
            ("m_slotId", 39, 3),
        ],
    },
];

