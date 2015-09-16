use super::{TypeInfo, IntBounds};
use phf::Map as PhfMap;

pub static REPLAY_HEADER_TYPEID: u32 = 13;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (70, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (69, "NNet.Game.SUserOptionsEvent"),
    9_u32 => (62, "NNet.Game.SBankFileEvent"),
    10_u32 => (64, "NNet.Game.SBankSectionEvent"),
    11_u32 => (65, "NNet.Game.SBankKeyEvent"),
    12_u32 => (66, "NNet.Game.SBankValueEvent"),
    13_u32 => (68, "NNet.Game.SBankSignatureEvent"),
    21_u32 => (71, "NNet.Game.SSaveGameEvent"),
    22_u32 => (70, "NNet.Game.SSaveGameDoneEvent"),
    23_u32 => (70, "NNet.Game.SLoadGameDoneEvent"),
    26_u32 => (75, "NNet.Game.SGameCheatEvent"),
    27_u32 => (84, "NNet.Game.SCmdEvent"),
    28_u32 => (92, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (93, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (95, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (97, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (98, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (101, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (102, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (103, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    36_u32 => (104, "NNet.Game.STriggerPingEvent"),
    37_u32 => (105, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (106, "NNet.Game.SAllianceEvent"),
    39_u32 => (107, "NNet.Game.SUnitClickEvent"),
    40_u32 => (108, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (109, "NNet.Game.STriggerReplySelectedEvent"),
    43_u32 => (113, "NNet.Game.SHijackReplayGameEvent"),
    44_u32 => (70, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (118, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (122, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (123, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (124, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (128, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (70, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (114, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (70, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (115, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (70, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (117, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (121, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (130, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (133, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (134, "NNet.Game.STriggerMouseMovedEvent"),
    60_u32 => (135, "NNet.Game.SAchievementAwardedEvent"),
    62_u32 => (136, "NNet.Game.STriggerTargetModeUpdateEvent"),
    63_u32 => (70, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (137, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (138, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (139, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (150, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (70, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (70, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (140, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (141, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (142, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (70, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (70, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (143, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    77_u32 => (70, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (70, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (144, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (70, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (70, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (145, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (146, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (146, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (115, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (70, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (70, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (148, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (149, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (151, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (152, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    92_u32 => (153, "NNet.Game.STriggerCameraMoveEvent"),
    93_u32 => (114, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (154, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (155, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (70, "NNet.Game.STriggerGameCreditsFinishedEvent"),
    97_u32 => (156, "NNet.Game.STriggerCutsceneBookmarkFiredEvent"),
    98_u32 => (157, "NNet.Game.STriggerCutsceneEndSceneFiredEvent"),
    99_u32 => (158, "NNet.Game.STriggerCutsceneConversationLineEvent"),
    100_u32 => (159, "NNet.Game.STriggerCutsceneConversationLineMissingEvent"),
    101_u32 => (70, "NNet.Game.SGameUserLeaveEvent"),
    102_u32 => (160, "NNet.Game.SGameUserJoinEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (161, "NNet.Game.SChatMessage"),
    1_u32 => (162, "NNet.Game.SPingMessage"),
    2_u32 => (163, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (70, "NNet.Game.SServerPingMessage"),
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
            ("m_userId", 7, -1),
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
    TypeInfo::Optional { typeid: 10 },
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
            ("m_workingSetSlotId", 20, 9),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 21 },
    TypeInfo::Optional { typeid: 22 },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 10 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("m_file", 25, 0),
        ],
    },
    TypeInfo::Bool,
    TypeInfo::Int { bounds: IntBounds { min: -9223372036854775808, bitlen: 64 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 12 } },
    TypeInfo::Blob { len: IntBounds { min: 40, bitlen: 0 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 30 },
    TypeInfo::Optional { typeid: 31 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 25 },
    TypeInfo::Optional { typeid: 33 },
    TypeInfo::Struct {
        fields: &[
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
            ("m_defaultDifficulty", 2, 13),
            ("m_modPaths", 34, 14),
        ],
    },
    TypeInfo::Optional { typeid: 9 },
    TypeInfo::Optional { typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_race", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_team", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 9, -12),
            ("m_clanTag", 36, -11),
            ("m_highestLeague", 20, -10),
            ("m_combinedRaceLevels", 37, -9),
            ("m_randomSeed", 5, -8),
            ("m_racePreference", 38, -7),
            ("m_teamPreference", 39, -6),
            ("m_testMap", 27, -5),
            ("m_testAuto", 27, -4),
            ("m_examine", 27, -3),
            ("m_customInterface", 27, -2),
            ("m_observe", 19, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 40 },
    TypeInfo::Struct {
        fields: &[
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
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 4 } },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 8 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 2 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 7 } },
    TypeInfo::Struct {
        fields: &[
            ("m_allowedColors", 45, -6),
            ("m_allowedRaces", 46, -5),
            ("m_allowedDifficulty", 45, -4),
            ("m_allowedControls", 46, -3),
            ("m_allowedObserveTypes", 47, -2),
            ("m_allowedAIBuilds", 48, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 49 },
    TypeInfo::Struct {
        fields: &[
            ("m_randomValue", 5, -25),
            ("m_gameCacheName", 24, -24),
            ("m_gameOptions", 42, -23),
            ("m_gameSpeed", 12, -22),
            ("m_gameType", 12, -21),
            ("m_maxUsers", 7, -20),
            ("m_maxObservers", 7, -19),
            ("m_maxPlayers", 7, -18),
            ("m_maxTeams", 43, -17),
            ("m_maxColors", 2, -16),
            ("m_maxRaces", 44, -15),
            ("m_maxControls", 44, -14),
            ("m_mapSizeX", 10, -13),
            ("m_mapSizeY", 10, -12),
            ("m_mapFileSyncChecksum", 5, -11),
            ("m_mapFileName", 25, -10),
            ("m_mapAuthorName", 9, -9),
            ("m_modFileSyncChecksum", 5, -8),
            ("m_slotDescriptions", 50, -7),
            ("m_defaultDifficulty", 2, -6),
            ("m_defaultAIBuild", 0, -5),
            ("m_cacheHandles", 31, -4),
            ("m_isBlizzardMap", 27, -3),
            ("m_isPremadeFFA", 27, -2),
            ("m_isCoopMode", 27, -1),
        ],
    },
    TypeInfo::Optional { typeid: 1 },
    TypeInfo::Optional { typeid: 7 },
    TypeInfo::Struct {
        fields: &[
            ("m_color", 53, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 5 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_control", 10, -13),
            ("m_userId", 52, -12),
            ("m_teamId", 1, -11),
            ("m_colorPref", 54, -10),
            ("m_racePref", 38, -9),
            ("m_difficulty", 2, -8),
            ("m_aiBuild", 0, -7),
            ("m_handicap", 0, -6),
            ("m_observe", 19, -5),
            ("m_workingSetSlotId", 20, -4),
            ("m_rewards", 55, -3),
            ("m_toonHandle", 15, -2),
            ("m_licenses", 56, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 57 },
    TypeInfo::Struct {
        fields: &[
            ("m_phase", 12, -10),
            ("m_maxUsers", 7, -9),
            ("m_maxObservers", 7, -8),
            ("m_slots", 58, -7),
            ("m_randomSeed", 5, -6),
            ("m_hostUserId", 52, -5),
            ("m_isSinglePlayer", 27, -4),
            ("m_gameDuration", 5, -3),
            ("m_defaultDifficulty", 2, -2),
            ("m_defaultAIBuild", 0, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_userInitialData", 41, -3),
            ("m_gameDescription", 51, -2),
            ("m_lobbyState", 59, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncLobbyState", 60, -1),
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
            ("m_name", 63, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 63, -3),
            ("m_type", 5, -2),
            ("m_data", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_type", 5, -3),
            ("m_name", 63, -2),
            ("m_data", 29, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 10 },
    TypeInfo::Struct {
        fields: &[
            ("m_signature", 67, -2),
            ("m_toonHandle", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameFullyDownloaded", 27, -7),
            ("m_developmentCheatsEnabled", 27, -6),
            ("m_multiplayerCheatsEnabled", 27, -5),
            ("m_syncChecksummingEnabled", 27, -4),
            ("m_isMapToMapTransition", 27, -3),
            ("m_startingRally", 27, -2),
            ("m_baseBuildNum", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fileName", 25, -5),
            ("m_automatic", 27, -4),
            ("m_overwrite", 27, -3),
            ("m_name", 9, -2),
            ("m_description", 24, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -2147483648, bitlen: 32 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 72, -2),
            ("y", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 73, -4),
            ("m_time", 72, -3),
            ("m_verb", 24, -2),
            ("m_arguments", 24, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_data", 74, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 20 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 16 } },
    TypeInfo::Struct {
        fields: &[
            ("m_abilLink", 77, -3),
            ("m_abilCmdIndex", 7, -2),
            ("m_abilCmdData", 20, -1),
        ],
    },
    TypeInfo::Optional { typeid: 78 },
    TypeInfo::Null,
    TypeInfo::Struct {
        fields: &[
            ("x", 76, -3),
            ("y", 76, -2),
            ("z", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_targetUnitFlags", 10, -7),
            ("m_timer", 10, -6),
            ("m_tag", 5, -5),
            ("m_snapshotUnitLink", 77, -4),
            ("m_snapshotControlPlayerId", 52, -3),
            ("m_snapshotUpkeepPlayerId", 52, -2),
            ("m_snapshotPoint", 81, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 80),
            1_u32 => ("TargetPoint", 81),
            2_u32 => ("TargetUnit", 82),
            3_u32 => ("Data", 5),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cmdFlags", 76, -4),
            ("m_abil", 79, -3),
            ("m_data", 83, -2),
            ("m_otherUnit", 37, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 85 },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 80),
            1_u32 => ("Mask", 86),
            2_u32 => ("OneIndices", 87),
            3_u32 => ("ZeroIndices", 87),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitLink", 77, -4),
            ("m_subgroupPriority", 10, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 85, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 89 },
    TypeInfo::Struct {
        fields: &[
            ("m_subgroupIndex", 85, -4),
            ("m_removeMask", 88, -3),
            ("m_addSubgroups", 90, -2),
            ("m_addUnitTags", 56, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 91, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 19, -2),
            ("m_mask", 88, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_count", 85, -6),
            ("m_subgroupCount", 85, -5),
            ("m_activeSubgroupIndex", 85, -4),
            ("m_unitTagsChecksum", 5, -3),
            ("m_subgroupIndicesChecksum", 5, -2),
            ("m_subgroupsChecksum", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 94, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 3 }, typeid: 72 },
    TypeInfo::Struct {
        fields: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 96, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_chatMessage", 24, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -128, bitlen: 8 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 72, -3),
            ("y", 72, -2),
            ("z", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_beacon", 99, -9),
            ("m_ally", 99, -8),
            ("m_flags", 99, -7),
            ("m_build", 99, -6),
            ("m_targetUnitTag", 5, -5),
            ("m_targetUnitSnapshotUnitLink", 77, -4),
            ("m_targetUnitSnapshotUpkeepPlayerId", 99, -3),
            ("m_targetUnitSnapshotControlPlayerId", 99, -2),
            ("m_targetPoint", 100, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_delta", 99, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 73, -3),
            ("m_unit", 5, -2),
            ("m_pingedMinimap", 27, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_verb", 24, -2),
            ("m_arguments", 24, -1),
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
            ("m_conversationId", 72, -2),
            ("m_replyId", 72, -1),
        ],
    },
    TypeInfo::Optional { typeid: 15 },
    TypeInfo::Struct {
        fields: &[
            ("m_gameUserId", 1, -4),
            ("m_name", 9, -3),
            ("m_toonHandle", 110, -2),
            ("m_clanTag", 36, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 111 },
    TypeInfo::Struct {
        fields: &[
            ("m_userInfos", 112, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseItemId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_difficultyLevel", 72, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 3 },
        types: phf_map! {
            0_u32 => ("None", 80),
            1_u32 => ("Checked", 27),
            2_u32 => ("ValueChanged", 5),
            3_u32 => ("SelectionChanged", 72),
            4_u32 => ("TextChanged", 25),
            5_u32 => ("MouseButton", 5),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlId", 72, -3),
            ("m_eventType", 72, -2),
            ("m_eventData", 116, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 5, -2),
            ("m_length", 5, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 7 }, typeid: 5 },
    TypeInfo::Struct {
        fields: &[
            ("m_soundHash", 119, -2),
            ("m_length", 119, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncInfo", 120, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_sound", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 72, -2),
            ("m_thread", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("x", 77, -2),
            ("y", 77, -1),
        ],
    },
    TypeInfo::Optional { typeid: 125 },
    TypeInfo::Optional { typeid: 77 },
    TypeInfo::Struct {
        fields: &[
            ("m_target", 126, -4),
            ("m_distance", 127, -3),
            ("m_pitch", 127, -2),
            ("m_yaw", 127, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 1 } },
    TypeInfo::Struct {
        fields: &[
            ("m_skipType", 129, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 131, -2),
            ("y", 131, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 5, -4),
            ("m_down", 27, -3),
            ("m_posUI", 132, -2),
            ("m_posWorld", 81, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_posUI", 132, -2),
            ("m_posWorld", 81, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_achievementLink", 77, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_abilLink", 77, -3),
            ("m_abilCmdIndex", 7, -2),
            ("m_state", 99, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundtrack", 5, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_planetId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_key", 99, -2),
            ("m_flags", 99, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_resources", 96, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fulfillRequestId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cancelRequestId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_researchItemId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_mercenaryId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 72, -2),
            ("m_difficultyLevel", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 72, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 19 } },
    TypeInfo::Struct {
        fields: &[
            ("m_decrementMs", 147, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_portraitId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_functionName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_result", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameMenuItemIndex", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_reason", 99, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseCategoryId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 77, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 72, -2),
            ("m_bookmarkName", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 72, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 72, -3),
            ("m_conversationLine", 15, -2),
            ("m_altConversationLine", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 72, -2),
            ("m_conversationLine", 15, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_observe", 19, -3),
            ("m_name", 9, -2),
            ("m_toonHandle", 110, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_string", 25, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_point", 73, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_progress", 72, -1),
        ],
    },
];

