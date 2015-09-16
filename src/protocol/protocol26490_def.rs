use super::{TypeInfo, IntBounds};
use phf::Map as PhfMap;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (71, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (70, "NNet.Game.SUserOptionsEvent"),
    9_u32 => (63, "NNet.Game.SBankFileEvent"),
    10_u32 => (65, "NNet.Game.SBankSectionEvent"),
    11_u32 => (66, "NNet.Game.SBankKeyEvent"),
    12_u32 => (67, "NNet.Game.SBankValueEvent"),
    13_u32 => (69, "NNet.Game.SBankSignatureEvent"),
    14_u32 => (74, "NNet.Game.SCameraSaveEvent"),
    21_u32 => (75, "NNet.Game.SSaveGameEvent"),
    22_u32 => (71, "NNet.Game.SSaveGameDoneEvent"),
    23_u32 => (71, "NNet.Game.SLoadGameDoneEvent"),
    26_u32 => (79, "NNet.Game.SGameCheatEvent"),
    27_u32 => (87, "NNet.Game.SCmdEvent"),
    28_u32 => (95, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (96, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (98, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (100, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (101, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (104, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (105, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (106, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    36_u32 => (107, "NNet.Game.STriggerPingEvent"),
    37_u32 => (108, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (109, "NNet.Game.SAllianceEvent"),
    39_u32 => (110, "NNet.Game.SUnitClickEvent"),
    40_u32 => (111, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (112, "NNet.Game.STriggerReplySelectedEvent"),
    43_u32 => (117, "NNet.Game.SHijackReplayGameEvent"),
    44_u32 => (71, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (122, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (126, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (127, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (128, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (131, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (71, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (118, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (71, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (119, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (71, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (121, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (125, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (132, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (135, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (136, "NNet.Game.STriggerMouseMovedEvent"),
    60_u32 => (137, "NNet.Game.SAchievementAwardedEvent"),
    62_u32 => (138, "NNet.Game.STriggerTargetModeUpdateEvent"),
    63_u32 => (71, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (139, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (140, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (141, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (152, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (71, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (71, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (142, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (143, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (144, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (71, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (71, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (145, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    77_u32 => (71, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (71, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (146, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (71, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (71, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (147, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (148, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (148, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (119, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (71, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (71, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (150, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (151, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (153, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (154, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    92_u32 => (155, "NNet.Game.STriggerCameraMoveEvent"),
    93_u32 => (118, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (156, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (157, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (71, "NNet.Game.STriggerGameCreditsFinishedEvent"),
    97_u32 => (158, "NNet.Game.STriggerCutsceneBookmarkFiredEvent"),
    98_u32 => (159, "NNet.Game.STriggerCutsceneEndSceneFiredEvent"),
    99_u32 => (160, "NNet.Game.STriggerCutsceneConversationLineEvent"),
    100_u32 => (161, "NNet.Game.STriggerCutsceneConversationLineMissingEvent"),
    101_u32 => (71, "NNet.Game.SGameUserLeaveEvent"),
    102_u32 => (162, "NNet.Game.SGameUserJoinEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (163, "NNet.Game.SChatMessage"),
    1_u32 => (164, "NNet.Game.SPingMessage"),
    2_u32 => (165, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (71, "NNet.Game.SServerPingMessage"),
};

pub static TYPEINFOS: &'static [TypeInfo] = &[
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 7 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 4 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 5 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 14 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 22 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 32 } },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("m_uint6", 3),
            1_u32 => ("m_uint14", 4),
            2_u32 => ("m_uint22", 5),
            3_u32 => ("m_uint32", 6),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_userId", 2, -1),
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
            ("m_build", 6, 4),
            ("m_baseBuild", 6, 5),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 3 } },
    TypeInfo::Bool,
    TypeInfo::Struct {
        fields: &[
            ("m_signature", 9, 0),
            ("m_version", 11, 1),
            ("m_type", 12, 2),
            ("m_elapsedGameLoops", 6, 3),
            ("m_useScaledTime", 13, 4),
        ],
    },
    TypeInfo::FourCC,
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 7 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 64 } },
    TypeInfo::Struct {
        fields: &[
            ("m_region", 10, 0),
            ("m_programId", 15, 1),
            ("m_realm", 6, 2),
            ("m_name", 16, 3),
            ("m_id", 17, 4),
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
    TypeInfo::Optional { typeid: 10 },
    TypeInfo::Struct {
        fields: &[
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
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 22 },
    TypeInfo::Optional { typeid: 23 },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 10 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("m_file", 26, 0),
        ],
    },
    TypeInfo::Optional { typeid: 13 },
    TypeInfo::Int { bounds: IntBounds { min: -9223372036854775808, bitlen: 64 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 12 } },
    TypeInfo::Blob { len: IntBounds { min: 40, bitlen: 0 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 31 },
    TypeInfo::Optional { typeid: 32 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 26 },
    TypeInfo::Optional { typeid: 34 },
    TypeInfo::Struct {
        fields: &[
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
    TypeInfo::Optional { typeid: 6 },
    TypeInfo::Struct {
        fields: &[
            ("m_race", 21, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_team", 21, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 9, -12),
            ("m_clanTag", 37, -11),
            ("m_highestLeague", 21, -10),
            ("m_combinedRaceLevels", 38, -9),
            ("m_randomSeed", 6, -8),
            ("m_racePreference", 39, -7),
            ("m_teamPreference", 40, -6),
            ("m_testMap", 13, -5),
            ("m_testAuto", 13, -4),
            ("m_examine", 13, -3),
            ("m_customInterface", 13, -2),
            ("m_observe", 20, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 41 },
    TypeInfo::Struct {
        fields: &[
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
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 4 } },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 2 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 7 } },
    TypeInfo::Struct {
        fields: &[
            ("m_allowedColors", 46, -6),
            ("m_allowedRaces", 47, -5),
            ("m_allowedDifficulty", 46, -4),
            ("m_allowedControls", 47, -3),
            ("m_allowedObserveTypes", 48, -2),
            ("m_allowedAIBuilds", 49, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 50 },
    TypeInfo::Struct {
        fields: &[
            ("m_randomValue", 6, -25),
            ("m_gameCacheName", 25, -24),
            ("m_gameOptions", 43, -23),
            ("m_gameSpeed", 12, -22),
            ("m_gameType", 12, -21),
            ("m_maxUsers", 2, -20),
            ("m_maxObservers", 2, -19),
            ("m_maxPlayers", 2, -18),
            ("m_maxTeams", 44, -17),
            ("m_maxColors", 3, -16),
            ("m_maxRaces", 45, -15),
            ("m_maxControls", 10, -14),
            ("m_mapSizeX", 10, -13),
            ("m_mapSizeY", 10, -12),
            ("m_mapFileSyncChecksum", 6, -11),
            ("m_mapFileName", 26, -10),
            ("m_mapAuthorName", 9, -9),
            ("m_modFileSyncChecksum", 6, -8),
            ("m_slotDescriptions", 51, -7),
            ("m_defaultDifficulty", 3, -6),
            ("m_defaultAIBuild", 0, -5),
            ("m_cacheHandles", 32, -4),
            ("m_isBlizzardMap", 13, -3),
            ("m_isPremadeFFA", 13, -2),
            ("m_isCoopMode", 13, -1),
        ],
    },
    TypeInfo::Optional { typeid: 1 },
    TypeInfo::Optional { typeid: 2 },
    TypeInfo::Struct {
        fields: &[
            ("m_color", 54, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 6 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 6 },
    TypeInfo::Struct {
        fields: &[
            ("m_control", 10, -13),
            ("m_userId", 53, -12),
            ("m_teamId", 1, -11),
            ("m_colorPref", 55, -10),
            ("m_racePref", 39, -9),
            ("m_difficulty", 3, -8),
            ("m_aiBuild", 0, -7),
            ("m_handicap", 0, -6),
            ("m_observe", 20, -5),
            ("m_workingSetSlotId", 21, -4),
            ("m_rewards", 56, -3),
            ("m_toonHandle", 16, -2),
            ("m_licenses", 57, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 58 },
    TypeInfo::Struct {
        fields: &[
            ("m_phase", 12, -10),
            ("m_maxUsers", 2, -9),
            ("m_maxObservers", 2, -8),
            ("m_slots", 59, -7),
            ("m_randomSeed", 6, -6),
            ("m_hostUserId", 53, -5),
            ("m_isSinglePlayer", 13, -4),
            ("m_gameDuration", 6, -3),
            ("m_defaultDifficulty", 3, -2),
            ("m_defaultAIBuild", 0, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_userInitialData", 42, -3),
            ("m_gameDescription", 52, -2),
            ("m_lobbyState", 60, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncLobbyState", 61, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 16, -1),
        ],
    },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 64, -3),
            ("m_type", 6, -2),
            ("m_data", 16, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_type", 6, -3),
            ("m_name", 64, -2),
            ("m_data", 30, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 10 },
    TypeInfo::Struct {
        fields: &[
            ("m_signature", 68, -2),
            ("m_toonHandle", 16, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
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
        fields: &[
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 16 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 72, -2),
            ("y", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_which", 12, -2),
            ("m_target", 73, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fileName", 26, -5),
            ("m_automatic", 13, -4),
            ("m_overwrite", 13, -3),
            ("m_name", 9, -2),
            ("m_description", 25, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -2147483648, bitlen: 32 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 76, -2),
            ("y", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 77, -4),
            ("m_time", 76, -3),
            ("m_verb", 25, -2),
            ("m_arguments", 25, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_data", 78, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 20 } },
    TypeInfo::Struct {
        fields: &[
            ("m_abilLink", 72, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_abilCmdData", 21, -1),
        ],
    },
    TypeInfo::Optional { typeid: 81 },
    TypeInfo::Null,
    TypeInfo::Struct {
        fields: &[
            ("x", 80, -3),
            ("y", 80, -2),
            ("z", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_targetUnitFlags", 10, -7),
            ("m_timer", 10, -6),
            ("m_tag", 6, -5),
            ("m_snapshotUnitLink", 72, -4),
            ("m_snapshotControlPlayerId", 53, -3),
            ("m_snapshotUpkeepPlayerId", 53, -2),
            ("m_snapshotPoint", 84, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 83),
            1_u32 => ("TargetPoint", 84),
            2_u32 => ("TargetUnit", 85),
            3_u32 => ("Data", 6),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cmdFlags", 80, -4),
            ("m_abil", 82, -3),
            ("m_data", 86, -2),
            ("m_otherUnit", 38, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 88 },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 83),
            1_u32 => ("Mask", 89),
            2_u32 => ("OneIndices", 90),
            3_u32 => ("ZeroIndices", 90),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitLink", 72, -4),
            ("m_subgroupPriority", 10, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 88, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 92 },
    TypeInfo::Struct {
        fields: &[
            ("m_subgroupIndex", 88, -4),
            ("m_removeMask", 91, -3),
            ("m_addSubgroups", 93, -2),
            ("m_addUnitTags", 57, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 94, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 20, -2),
            ("m_mask", 91, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_count", 88, -6),
            ("m_subgroupCount", 88, -5),
            ("m_activeSubgroupIndex", 88, -4),
            ("m_unitTagsChecksum", 6, -3),
            ("m_subgroupIndicesChecksum", 6, -2),
            ("m_subgroupsChecksum", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 97, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 3 }, typeid: 76 },
    TypeInfo::Struct {
        fields: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 99, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_chatMessage", 25, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -128, bitlen: 8 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 76, -3),
            ("y", 76, -2),
            ("z", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_beacon", 102, -9),
            ("m_ally", 102, -8),
            ("m_flags", 102, -7),
            ("m_build", 102, -6),
            ("m_targetUnitTag", 6, -5),
            ("m_targetUnitSnapshotUnitLink", 72, -4),
            ("m_targetUnitSnapshotUpkeepPlayerId", 102, -3),
            ("m_targetUnitSnapshotControlPlayerId", 102, -2),
            ("m_targetPoint", 103, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_delta", 102, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 77, -3),
            ("m_unit", 6, -2),
            ("m_pingedMinimap", 13, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_verb", 25, -2),
            ("m_arguments", 25, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_alliance", 6, -2),
            ("m_control", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTag", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTag", 6, -2),
            ("m_flags", 10, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_conversationId", 76, -2),
            ("m_replyId", 76, -1),
        ],
    },
    TypeInfo::Optional { typeid: 16 },
    TypeInfo::Struct {
        fields: &[
            ("m_gameUserId", 1, -5),
            ("m_observe", 20, -4),
            ("m_name", 9, -3),
            ("m_toonHandle", 113, -2),
            ("m_clanTag", 37, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 114 },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 1 } },
    TypeInfo::Struct {
        fields: &[
            ("m_userInfos", 115, -2),
            ("m_method", 116, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseItemId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_difficultyLevel", 76, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 3 },
        types: phf_map! {
            0_u32 => ("None", 83),
            1_u32 => ("Checked", 13),
            2_u32 => ("ValueChanged", 6),
            3_u32 => ("SelectionChanged", 76),
            4_u32 => ("TextChanged", 26),
            5_u32 => ("MouseButton", 6),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlId", 76, -3),
            ("m_eventType", 76, -2),
            ("m_eventData", 120, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 6, -2),
            ("m_length", 6, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 7 }, typeid: 6 },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 123, -2),
            ("m_length", 123, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncInfo", 124, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_sound", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 76, -2),
            ("m_thread", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 76, -1),
        ],
    },
    TypeInfo::Optional { typeid: 73 },
    TypeInfo::Optional { typeid: 72 },
    TypeInfo::Struct {
        fields: &[
            ("m_target", 129, -4),
            ("m_distance", 130, -3),
            ("m_pitch", 130, -2),
            ("m_yaw", 130, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_skipType", 116, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 133, -2),
            ("y", 133, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 6, -5),
            ("m_down", 13, -4),
            ("m_posUI", 134, -3),
            ("m_posWorld", 84, -2),
            ("m_flags", 102, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_posUI", 134, -3),
            ("m_posWorld", 84, -2),
            ("m_flags", 102, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_achievementLink", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_abilLink", 72, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_state", 102, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundtrack", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_planetId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_key", 102, -2),
            ("m_flags", 102, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_resources", 99, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fulfillRequestId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cancelRequestId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_researchItemId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_mercenaryId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 76, -2),
            ("m_difficultyLevel", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 76, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 19 } },
    TypeInfo::Struct {
        fields: &[
            ("m_decrementMs", 149, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_portraitId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_functionName", 16, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_result", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameMenuItemIndex", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_reason", 102, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseCategoryId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 76, -2),
            ("m_bookmarkName", 16, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 76, -3),
            ("m_conversationLine", 16, -2),
            ("m_altConversationLine", 16, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 76, -2),
            ("m_conversationLine", 16, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_observe", 20, -4),
            ("m_name", 9, -3),
            ("m_toonHandle", 113, -2),
            ("m_clanTag", 37, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_string", 26, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_point", 77, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_progress", 76, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_scoreValueMineralsCurrent", 76, 0),
            ("m_scoreValueVespeneCurrent", 76, 1),
            ("m_scoreValueMineralsCollectionRate", 76, 2),
            ("m_scoreValueVespeneCollectionRate", 76, 3),
            ("m_scoreValueWorkersActiveCount", 76, 4),
            ("m_scoreValueMineralsUsedInProgressArmy", 76, 5),
            ("m_scoreValueMineralsUsedInProgressEconomy", 76, 6),
            ("m_scoreValueMineralsUsedInProgressTechnology", 76, 7),
            ("m_scoreValueVespeneUsedInProgressArmy", 76, 8),
            ("m_scoreValueVespeneUsedInProgressEconomy", 76, 9),
            ("m_scoreValueVespeneUsedInProgressTechnology", 76, 10),
            ("m_scoreValueMineralsUsedCurrentArmy", 76, 11),
            ("m_scoreValueMineralsUsedCurrentEconomy", 76, 12),
            ("m_scoreValueMineralsUsedCurrentTechnology", 76, 13),
            ("m_scoreValueVespeneUsedCurrentArmy", 76, 14),
            ("m_scoreValueVespeneUsedCurrentEconomy", 76, 15),
            ("m_scoreValueVespeneUsedCurrentTechnology", 76, 16),
            ("m_scoreValueMineralsLostArmy", 76, 17),
            ("m_scoreValueMineralsLostEconomy", 76, 18),
            ("m_scoreValueMineralsLostTechnology", 76, 19),
            ("m_scoreValueVespeneLostArmy", 76, 20),
            ("m_scoreValueVespeneLostEconomy", 76, 21),
            ("m_scoreValueVespeneLostTechnology", 76, 22),
            ("m_scoreValueMineralsKilledArmy", 76, 23),
            ("m_scoreValueMineralsKilledEconomy", 76, 24),
            ("m_scoreValueMineralsKilledTechnology", 76, 25),
            ("m_scoreValueVespeneKilledArmy", 76, 26),
            ("m_scoreValueVespeneKilledEconomy", 76, 27),
            ("m_scoreValueVespeneKilledTechnology", 76, 28),
            ("m_scoreValueFoodUsed", 76, 29),
            ("m_scoreValueFoodMade", 76, 30),
            ("m_scoreValueMineralsUsedActiveForces", 76, 31),
            ("m_scoreValueVespeneUsedActiveForces", 76, 32),
            ("m_scoreValueMineralsFriendlyFireArmy", 76, 33),
            ("m_scoreValueMineralsFriendlyFireEconomy", 76, 34),
            ("m_scoreValueMineralsFriendlyFireTechnology", 76, 35),
            ("m_scoreValueVespeneFriendlyFireArmy", 76, 36),
            ("m_scoreValueVespeneFriendlyFireEconomy", 76, 37),
            ("m_scoreValueVespeneFriendlyFireTechnology", 76, 38),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_playerId", 1, 0),
            ("m_stats", 166, 1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
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
        fields: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_killerPlayerId", 53, 2),
            ("m_x", 10, 3),
            ("m_y", 10, 4),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_controlPlayerId", 1, 2),
            ("m_upkeepPlayerId", 1, 3),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_unitTypeName", 25, 2),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_playerId", 1, 0),
            ("m_upgradeTypeName", 25, 1),
            ("m_count", 76, 2),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 10 }, typeid: 76 },
    TypeInfo::Struct {
        fields: &[
            ("m_firstUnitIndex", 6, 0),
            ("m_items", 174, 1),
        ],
    },
];

