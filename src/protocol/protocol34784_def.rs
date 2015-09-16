use super::{TypeInfo, IntBounds};
use phf::Map as PhfMap;

pub static REPLAY_HEADER_TYPEID: u32 = 18;

pub static GAME_EVENTID_TYPEID: u32 = 0;

pub static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    5_u32 => (78, "NNet.Game.SUserFinishedLoadingSyncEvent"),
    7_u32 => (77, "NNet.Game.SUserOptionsEvent"),
    9_u32 => (70, "NNet.Game.SBankFileEvent"),
    10_u32 => (72, "NNet.Game.SBankSectionEvent"),
    11_u32 => (73, "NNet.Game.SBankKeyEvent"),
    12_u32 => (74, "NNet.Game.SBankValueEvent"),
    13_u32 => (76, "NNet.Game.SBankSignatureEvent"),
    14_u32 => (81, "NNet.Game.SCameraSaveEvent"),
    21_u32 => (82, "NNet.Game.SSaveGameEvent"),
    22_u32 => (78, "NNet.Game.SSaveGameDoneEvent"),
    23_u32 => (78, "NNet.Game.SLoadGameDoneEvent"),
    25_u32 => (83, "NNet.Game.SCommandManagerResetEvent"),
    26_u32 => (87, "NNet.Game.SGameCheatEvent"),
    27_u32 => (97, "NNet.Game.SCmdEvent"),
    28_u32 => (105, "NNet.Game.SSelectionDeltaEvent"),
    29_u32 => (106, "NNet.Game.SControlGroupUpdateEvent"),
    30_u32 => (108, "NNet.Game.SSelectionSyncCheckEvent"),
    31_u32 => (110, "NNet.Game.SResourceTradeEvent"),
    32_u32 => (111, "NNet.Game.STriggerChatMessageEvent"),
    33_u32 => (114, "NNet.Game.SAICommunicateEvent"),
    34_u32 => (115, "NNet.Game.SSetAbsoluteGameSpeedEvent"),
    35_u32 => (116, "NNet.Game.SAddAbsoluteGameSpeedEvent"),
    36_u32 => (117, "NNet.Game.STriggerPingEvent"),
    37_u32 => (118, "NNet.Game.SBroadcastCheatEvent"),
    38_u32 => (119, "NNet.Game.SAllianceEvent"),
    39_u32 => (120, "NNet.Game.SUnitClickEvent"),
    40_u32 => (121, "NNet.Game.SUnitHighlightEvent"),
    41_u32 => (122, "NNet.Game.STriggerReplySelectedEvent"),
    43_u32 => (127, "NNet.Game.SHijackReplayGameEvent"),
    44_u32 => (78, "NNet.Game.STriggerSkippedEvent"),
    45_u32 => (132, "NNet.Game.STriggerSoundLengthQueryEvent"),
    46_u32 => (139, "NNet.Game.STriggerSoundOffsetEvent"),
    47_u32 => (140, "NNet.Game.STriggerTransmissionOffsetEvent"),
    48_u32 => (141, "NNet.Game.STriggerTransmissionCompleteEvent"),
    49_u32 => (145, "NNet.Game.SCameraUpdateEvent"),
    50_u32 => (78, "NNet.Game.STriggerAbortMissionEvent"),
    51_u32 => (128, "NNet.Game.STriggerPurchaseMadeEvent"),
    52_u32 => (78, "NNet.Game.STriggerPurchaseExitEvent"),
    53_u32 => (129, "NNet.Game.STriggerPlanetMissionLaunchedEvent"),
    54_u32 => (78, "NNet.Game.STriggerPlanetPanelCanceledEvent"),
    55_u32 => (131, "NNet.Game.STriggerDialogControlEvent"),
    56_u32 => (135, "NNet.Game.STriggerSoundLengthSyncEvent"),
    57_u32 => (146, "NNet.Game.STriggerConversationSkippedEvent"),
    58_u32 => (149, "NNet.Game.STriggerMouseClickedEvent"),
    59_u32 => (150, "NNet.Game.STriggerMouseMovedEvent"),
    60_u32 => (151, "NNet.Game.SAchievementAwardedEvent"),
    61_u32 => (152, "NNet.Game.STriggerHotkeyPressedEvent"),
    62_u32 => (153, "NNet.Game.STriggerTargetModeUpdateEvent"),
    63_u32 => (78, "NNet.Game.STriggerPlanetPanelReplayEvent"),
    64_u32 => (154, "NNet.Game.STriggerSoundtrackDoneEvent"),
    65_u32 => (155, "NNet.Game.STriggerPlanetMissionSelectedEvent"),
    66_u32 => (156, "NNet.Game.STriggerKeyPressedEvent"),
    67_u32 => (167, "NNet.Game.STriggerMovieFunctionEvent"),
    68_u32 => (78, "NNet.Game.STriggerPlanetPanelBirthCompleteEvent"),
    69_u32 => (78, "NNet.Game.STriggerPlanetPanelDeathCompleteEvent"),
    70_u32 => (157, "NNet.Game.SResourceRequestEvent"),
    71_u32 => (158, "NNet.Game.SResourceRequestFulfillEvent"),
    72_u32 => (159, "NNet.Game.SResourceRequestCancelEvent"),
    73_u32 => (78, "NNet.Game.STriggerResearchPanelExitEvent"),
    74_u32 => (78, "NNet.Game.STriggerResearchPanelPurchaseEvent"),
    75_u32 => (160, "NNet.Game.STriggerResearchPanelSelectionChangedEvent"),
    77_u32 => (78, "NNet.Game.STriggerMercenaryPanelExitEvent"),
    78_u32 => (78, "NNet.Game.STriggerMercenaryPanelPurchaseEvent"),
    79_u32 => (161, "NNet.Game.STriggerMercenaryPanelSelectionChangedEvent"),
    80_u32 => (78, "NNet.Game.STriggerVictoryPanelExitEvent"),
    81_u32 => (78, "NNet.Game.STriggerBattleReportPanelExitEvent"),
    82_u32 => (162, "NNet.Game.STriggerBattleReportPanelPlayMissionEvent"),
    83_u32 => (163, "NNet.Game.STriggerBattleReportPanelPlaySceneEvent"),
    84_u32 => (163, "NNet.Game.STriggerBattleReportPanelSelectionChangedEvent"),
    85_u32 => (129, "NNet.Game.STriggerVictoryPanelPlayMissionAgainEvent"),
    86_u32 => (78, "NNet.Game.STriggerMovieStartedEvent"),
    87_u32 => (78, "NNet.Game.STriggerMovieFinishedEvent"),
    88_u32 => (165, "NNet.Game.SDecrementGameTimeRemainingEvent"),
    89_u32 => (166, "NNet.Game.STriggerPortraitLoadedEvent"),
    90_u32 => (168, "NNet.Game.STriggerCustomDialogDismissedEvent"),
    91_u32 => (169, "NNet.Game.STriggerGameMenuItemSelectedEvent"),
    93_u32 => (128, "NNet.Game.STriggerPurchasePanelSelectedPurchaseItemChangedEvent"),
    94_u32 => (170, "NNet.Game.STriggerPurchasePanelSelectedPurchaseCategoryChangedEvent"),
    95_u32 => (171, "NNet.Game.STriggerButtonPressedEvent"),
    96_u32 => (78, "NNet.Game.STriggerGameCreditsFinishedEvent"),
    97_u32 => (172, "NNet.Game.STriggerCutsceneBookmarkFiredEvent"),
    98_u32 => (173, "NNet.Game.STriggerCutsceneEndSceneFiredEvent"),
    99_u32 => (174, "NNet.Game.STriggerCutsceneConversationLineEvent"),
    100_u32 => (175, "NNet.Game.STriggerCutsceneConversationLineMissingEvent"),
    101_u32 => (176, "NNet.Game.SGameUserLeaveEvent"),
    102_u32 => (177, "NNet.Game.SGameUserJoinEvent"),
    103_u32 => (179, "NNet.Game.SCommandManagerStateEvent"),
    104_u32 => (180, "NNet.Game.SCmdUpdateTargetPointEvent"),
    105_u32 => (181, "NNet.Game.SCmdUpdateTargetUnitEvent"),
    106_u32 => (136, "NNet.Game.STriggerAnimLengthQueryByNameEvent"),
    107_u32 => (137, "NNet.Game.STriggerAnimLengthQueryByPropsEvent"),
    108_u32 => (138, "NNet.Game.STriggerAnimOffsetEvent"),
    109_u32 => (182, "NNet.Game.SCatalogModifyEvent"),
    110_u32 => (183, "NNet.Game.SHeroTalentTreeSelectedEvent"),
    111_u32 => (78, "NNet.Game.STriggerProfilerLoggingFinishedEvent"),
    112_u32 => (184, "NNet.Game.SHeroTalentTreeSelectionPanelToggledEvent"),
};

pub static MESSAGE_EVENTID_TYPEID: u32 = 1;

pub static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (185, "NNet.Game.SChatMessage"),
    1_u32 => (186, "NNet.Game.SPingMessage"),
    2_u32 => (187, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (78, "NNet.Game.SServerPingMessage"),
    4_u32 => (188, "NNet.Game.SReconnectNotifyMessage"),
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
    TypeInfo::Array { bounds: IntBounds { min: 16, bitlen: 0 }, typeid: 10 },
    TypeInfo::Optional { typeid: 14 },
    TypeInfo::Blob { len: IntBounds { min: 16, bitlen: 0 } },
    TypeInfo::Struct {
        fields: &[
            ("m_dataDeprecated", 15, 0),
            ("m_data", 16, 1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_signature", 9, 0),
            ("m_version", 11, 1),
            ("m_type", 12, 2),
            ("m_elapsedGameLoops", 6, 3),
            ("m_useScaledTime", 13, 4),
            ("m_ngdpRootKey", 17, 5),
            ("m_dataBuildNum", 6, 6),
        ],
    },
    TypeInfo::FourCC,
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 7 } },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 64 } },
    TypeInfo::Struct {
        fields: &[
            ("m_region", 10, 0),
            ("m_programId", 19, 1),
            ("m_realm", 6, 2),
            ("m_name", 20, 3),
            ("m_id", 21, 4),
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
            ("m_toon", 22, 1),
            ("m_race", 9, 2),
            ("m_color", 23, 3),
            ("m_control", 10, 4),
            ("m_teamId", 1, 5),
            ("m_handicap", 0, 6),
            ("m_observe", 24, 7),
            ("m_result", 24, 8),
            ("m_workingSetSlotId", 25, 9),
            ("m_hero", 9, 10),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 26 },
    TypeInfo::Optional { typeid: 27 },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 10 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("m_file", 30, 0),
        ],
    },
    TypeInfo::Optional { typeid: 13 },
    TypeInfo::Int { bounds: IntBounds { min: -9223372036854775808, bitlen: 64 } },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 12 } },
    TypeInfo::Blob { len: IntBounds { min: 40, bitlen: 0 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 35 },
    TypeInfo::Optional { typeid: 36 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 6 }, typeid: 30 },
    TypeInfo::Optional { typeid: 38 },
    TypeInfo::Struct {
        fields: &[
            ("m_playerList", 28, 0),
            ("m_title", 29, 1),
            ("m_difficulty", 9, 2),
            ("m_thumbnail", 31, 3),
            ("m_isBlizzardMap", 13, 4),
            ("m_restartAsTransitionMap", 32, 16),
            ("m_timeUTC", 33, 5),
            ("m_timeLocalOffset", 33, 6),
            ("m_description", 34, 7),
            ("m_imageFilePath", 30, 8),
            ("m_campaignIndex", 10, 15),
            ("m_mapFileName", 30, 9),
            ("m_cacheHandles", 37, 10),
            ("m_miniSave", 13, 11),
            ("m_gameSpeed", 12, 12),
            ("m_defaultDifficulty", 3, 13),
            ("m_modPaths", 39, 14),
        ],
    },
    TypeInfo::Optional { typeid: 9 },
    TypeInfo::Optional { typeid: 35 },
    TypeInfo::Optional { typeid: 6 },
    TypeInfo::Struct {
        fields: &[
            ("m_race", 25, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_team", 25, -1),
        ],
    },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 9, -18),
            ("m_clanTag", 41, -17),
            ("m_clanLogo", 42, -16),
            ("m_highestLeague", 25, -15),
            ("m_combinedRaceLevels", 43, -14),
            ("m_randomSeed", 6, -13),
            ("m_racePreference", 44, -12),
            ("m_teamPreference", 45, -11),
            ("m_testMap", 13, -10),
            ("m_testAuto", 13, -9),
            ("m_examine", 13, -8),
            ("m_customInterface", 13, -7),
            ("m_testType", 6, -6),
            ("m_observe", 24, -5),
            ("m_hero", 46, -4),
            ("m_skin", 46, -3),
            ("m_mount", 46, -2),
            ("m_toonHandle", 20, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 47 },
    TypeInfo::Struct {
        fields: &[
            ("m_lockTeams", 13, -16),
            ("m_teamsTogether", 13, -15),
            ("m_advancedSharedControl", 13, -14),
            ("m_randomRaces", 13, -13),
            ("m_battleNet", 13, -12),
            ("m_amm", 13, -11),
            ("m_ranked", 13, -10),
            ("m_competitive", 13, -9),
            ("m_practice", 13, -8),
            ("m_cooperative", 13, -7),
            ("m_noVictoryOrDefeat", 13, -6),
            ("m_heroDuplicatesAllowed", 13, -5),
            ("m_fog", 24, -4),
            ("m_observers", 24, -3),
            ("m_userDifficulty", 24, -2),
            ("m_clientDebugFlags", 21, -1),
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
            ("m_allowedColors", 52, -6),
            ("m_allowedRaces", 53, -5),
            ("m_allowedDifficulty", 52, -4),
            ("m_allowedControls", 53, -3),
            ("m_allowedObserveTypes", 54, -2),
            ("m_allowedAIBuilds", 55, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 56 },
    TypeInfo::Struct {
        fields: &[
            ("m_randomValue", 6, -26),
            ("m_gameCacheName", 29, -25),
            ("m_gameOptions", 49, -24),
            ("m_gameSpeed", 12, -23),
            ("m_gameType", 12, -22),
            ("m_maxUsers", 2, -21),
            ("m_maxObservers", 2, -20),
            ("m_maxPlayers", 2, -19),
            ("m_maxTeams", 50, -18),
            ("m_maxColors", 3, -17),
            ("m_maxRaces", 51, -16),
            ("m_maxControls", 10, -15),
            ("m_mapSizeX", 10, -14),
            ("m_mapSizeY", 10, -13),
            ("m_mapFileSyncChecksum", 6, -12),
            ("m_mapFileName", 30, -11),
            ("m_mapAuthorName", 9, -10),
            ("m_modFileSyncChecksum", 6, -9),
            ("m_slotDescriptions", 57, -8),
            ("m_defaultDifficulty", 3, -7),
            ("m_defaultAIBuild", 0, -6),
            ("m_cacheHandles", 36, -5),
            ("m_hasExtensionMod", 13, -4),
            ("m_isBlizzardMap", 13, -3),
            ("m_isPremadeFFA", 13, -2),
            ("m_isCoopMode", 13, -1),
        ],
    },
    TypeInfo::Optional { typeid: 1 },
    TypeInfo::Optional { typeid: 2 },
    TypeInfo::Struct {
        fields: &[
            ("m_color", 60, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 4 }, typeid: 46 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 17 }, typeid: 6 },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 6 },
    TypeInfo::Struct {
        fields: &[
            ("m_control", 10, -20),
            ("m_userId", 59, -19),
            ("m_teamId", 1, -18),
            ("m_colorPref", 61, -17),
            ("m_racePref", 44, -16),
            ("m_difficulty", 3, -15),
            ("m_aiBuild", 0, -14),
            ("m_handicap", 0, -13),
            ("m_observe", 24, -12),
            ("m_logoIndex", 6, -11),
            ("m_hero", 46, -10),
            ("m_skin", 46, -9),
            ("m_mount", 46, -8),
            ("m_artifacts", 62, -7),
            ("m_workingSetSlotId", 25, -6),
            ("m_rewards", 63, -5),
            ("m_toonHandle", 20, -4),
            ("m_licenses", 64, -3),
            ("m_tandemLeaderUserId", 59, -2),
            ("m_commander", 46, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 65 },
    TypeInfo::Struct {
        fields: &[
            ("m_phase", 12, -10),
            ("m_maxUsers", 2, -9),
            ("m_maxObservers", 2, -8),
            ("m_slots", 66, -7),
            ("m_randomSeed", 6, -6),
            ("m_hostUserId", 59, -5),
            ("m_isSinglePlayer", 13, -4),
            ("m_gameDuration", 6, -3),
            ("m_defaultDifficulty", 3, -2),
            ("m_defaultAIBuild", 0, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_userInitialData", 48, -3),
            ("m_gameDescription", 58, -2),
            ("m_lobbyState", 67, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncLobbyState", 68, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 20, -1),
        ],
    },
    TypeInfo::Blob { len: IntBounds { min: 0, bitlen: 6 } },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 71, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_name", 71, -3),
            ("m_type", 6, -2),
            ("m_data", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_type", 6, -3),
            ("m_name", 71, -2),
            ("m_data", 34, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 10 },
    TypeInfo::Struct {
        fields: &[
            ("m_signature", 75, -2),
            ("m_toonHandle", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameFullyDownloaded", 13, -15),
            ("m_developmentCheatsEnabled", 13, -14),
            ("m_testCheatsEnabled", 13, -13),
            ("m_multiplayerCheatsEnabled", 13, -12),
            ("m_syncChecksummingEnabled", 13, -11),
            ("m_isMapToMapTransition", 13, -10),
            ("m_startingRally", 13, -9),
            ("m_debugPauseEnabled", 13, -8),
            ("m_useGalaxyAsserts", 13, -7),
            ("m_platformMac", 13, -6),
            ("m_cameraFollow", 13, -5),
            ("m_baseBuildNum", 6, -4),
            ("m_buildNum", 6, -3),
            ("m_versionFlags", 6, -2),
            ("m_hotkeyProfile", 46, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 16 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 79, -2),
            ("y", 79, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_which", 12, -2),
            ("m_target", 80, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fileName", 30, -5),
            ("m_automatic", 13, -4),
            ("m_overwrite", 13, -3),
            ("m_name", 9, -2),
            ("m_description", 29, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_sequence", 6, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -2147483648, bitlen: 32 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 84, -2),
            ("y", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 85, -4),
            ("m_time", 84, -3),
            ("m_verb", 29, -2),
            ("m_arguments", 29, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_data", 86, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 23 } },
    TypeInfo::Struct {
        fields: &[
            ("m_abilLink", 79, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_abilCmdData", 25, -1),
        ],
    },
    TypeInfo::Optional { typeid: 89 },
    TypeInfo::Null,
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 20 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 92, -3),
            ("y", 92, -2),
            ("z", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_targetUnitFlags", 79, -7),
            ("m_timer", 10, -6),
            ("m_tag", 6, -5),
            ("m_snapshotUnitLink", 79, -4),
            ("m_snapshotControlPlayerId", 59, -3),
            ("m_snapshotUpkeepPlayerId", 59, -2),
            ("m_snapshotPoint", 93, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 91),
            1_u32 => ("TargetPoint", 93),
            2_u32 => ("TargetUnit", 94),
            3_u32 => ("Data", 6),
        },
    },
    TypeInfo::Int { bounds: IntBounds { min: 1, bitlen: 32 } },
    TypeInfo::Struct {
        fields: &[
            ("m_cmdFlags", 88, -6),
            ("m_abil", 90, -5),
            ("m_data", 95, -4),
            ("m_sequence", 96, -3),
            ("m_otherUnit", 43, -2),
            ("m_unitGroup", 43, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::BitArray { len: IntBounds { min: 0, bitlen: 9 } },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 98 },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 2 },
        types: phf_map! {
            0_u32 => ("None", 91),
            1_u32 => ("Mask", 99),
            2_u32 => ("OneIndices", 100),
            3_u32 => ("ZeroIndices", 100),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitLink", 79, -4),
            ("m_subgroupPriority", 10, -3),
            ("m_intraSubgroupPriority", 10, -2),
            ("m_count", 98, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 9 }, typeid: 102 },
    TypeInfo::Struct {
        fields: &[
            ("m_subgroupIndex", 98, -4),
            ("m_removeMask", 101, -3),
            ("m_addSubgroups", 103, -2),
            ("m_addUnitTags", 64, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_delta", 104, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupIndex", 1, -3),
            ("m_controlGroupUpdate", 24, -2),
            ("m_mask", 101, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_count", 98, -6),
            ("m_subgroupCount", 98, -5),
            ("m_activeSubgroupIndex", 98, -4),
            ("m_unitTagsChecksum", 6, -3),
            ("m_subgroupIndicesChecksum", 6, -2),
            ("m_subgroupsChecksum", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlGroupId", 1, -2),
            ("m_selectionSyncData", 107, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 3 }, typeid: 84 },
    TypeInfo::Struct {
        fields: &[
            ("m_recipientId", 1, -2),
            ("m_resources", 109, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_chatMessage", 29, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: -128, bitlen: 8 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 84, -3),
            ("y", 84, -2),
            ("z", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_beacon", 112, -9),
            ("m_ally", 112, -8),
            ("m_flags", 112, -7),
            ("m_build", 112, -6),
            ("m_targetUnitTag", 6, -5),
            ("m_targetUnitSnapshotUnitLink", 79, -4),
            ("m_targetUnitSnapshotUpkeepPlayerId", 112, -3),
            ("m_targetUnitSnapshotControlPlayerId", 112, -2),
            ("m_targetPoint", 113, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_speed", 12, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_delta", 112, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_point", 85, -4),
            ("m_unit", 6, -3),
            ("m_pingedMinimap", 13, -2),
            ("m_option", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_verb", 29, -2),
            ("m_arguments", 29, -1),
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
            ("m_conversationId", 84, -2),
            ("m_replyId", 84, -1),
        ],
    },
    TypeInfo::Optional { typeid: 20 },
    TypeInfo::Struct {
        fields: &[
            ("m_gameUserId", 1, -6),
            ("m_observe", 24, -5),
            ("m_name", 9, -4),
            ("m_toonHandle", 123, -3),
            ("m_clanTag", 41, -2),
            ("m_clanLogo", 42, -1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 5 }, typeid: 124 },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 1 } },
    TypeInfo::Struct {
        fields: &[
            ("m_userInfos", 125, -2),
            ("m_method", 126, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseItemId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_difficultyLevel", 84, -1),
        ],
    },
    TypeInfo::Choice {
        bounds: IntBounds { min: 0, bitlen: 3 },
        types: phf_map! {
            0_u32 => ("None", 91),
            1_u32 => ("Checked", 13),
            2_u32 => ("ValueChanged", 6),
            3_u32 => ("SelectionChanged", 84),
            4_u32 => ("TextChanged", 30),
            5_u32 => ("MouseButton", 6),
        },
    },
    TypeInfo::Struct {
        fields: &[
            ("m_controlId", 84, -3),
            ("m_eventType", 84, -2),
            ("m_eventData", 130, -1),
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
            ("m_soundHash", 133, -2),
            ("m_length", 133, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_syncInfo", 134, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_queryId", 79, -3),
            ("m_lengthMs", 6, -2),
            ("m_finishGameLoop", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_queryId", 79, -2),
            ("m_lengthMs", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_animWaitQueryId", 79, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_sound", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 84, -2),
            ("m_thread", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_transmissionId", 84, -1),
        ],
    },
    TypeInfo::Optional { typeid: 80 },
    TypeInfo::Optional { typeid: 79 },
    TypeInfo::Optional { typeid: 112 },
    TypeInfo::Struct {
        fields: &[
            ("m_target", 142, -6),
            ("m_distance", 143, -5),
            ("m_pitch", 143, -4),
            ("m_yaw", 143, -3),
            ("m_reason", 144, -2),
            ("m_follow", 13, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_skipType", 126, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 11 } },
    TypeInfo::Struct {
        fields: &[
            ("x", 147, -2),
            ("y", 147, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 6, -5),
            ("m_down", 13, -4),
            ("m_posUI", 148, -3),
            ("m_posWorld", 93, -2),
            ("m_flags", 112, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_posUI", 148, -3),
            ("m_posWorld", 93, -2),
            ("m_flags", 112, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_achievementLink", 79, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_hotkey", 6, -2),
            ("m_down", 13, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_abilLink", 79, -3),
            ("m_abilCmdIndex", 2, -2),
            ("m_state", 112, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_soundtrack", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_planetId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_key", 112, -2),
            ("m_flags", 112, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_resources", 109, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_fulfillRequestId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cancelRequestId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_researchItemId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_mercenaryId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 84, -2),
            ("m_difficultyLevel", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_battleReportId", 84, -1),
        ],
    },
    TypeInfo::Int { bounds: IntBounds { min: 0, bitlen: 19 } },
    TypeInfo::Struct {
        fields: &[
            ("m_decrementMs", 164, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_portraitId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_functionName", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_result", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_gameMenuItemIndex", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_purchaseCategoryId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_button", 79, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 84, -2),
            ("m_bookmarkName", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 84, -3),
            ("m_conversationLine", 20, -2),
            ("m_altConversationLine", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_cutsceneId", 84, -2),
            ("m_conversationLine", 20, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_leaveReason", 1, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_observe", 24, -7),
            ("m_name", 9, -6),
            ("m_toonHandle", 123, -5),
            ("m_clanTag", 41, -4),
            ("m_clanLogo", 42, -3),
            ("m_hijack", 13, -2),
            ("m_hijackCloneGameUserId", 59, -1),
        ],
    },
    TypeInfo::Optional { typeid: 96 },
    TypeInfo::Struct {
        fields: &[
            ("m_state", 24, -2),
            ("m_sequence", 178, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_target", 93, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_target", 94, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_catalog", 10, -4),
            ("m_entry", 79, -3),
            ("m_field", 9, -2),
            ("m_value", 9, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_index", 6, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_shown", 13, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_string", 30, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_recipient", 12, -2),
            ("m_point", 85, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_progress", 84, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_status", 24, -1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_scoreValueMineralsCurrent", 84, 0),
            ("m_scoreValueVespeneCurrent", 84, 1),
            ("m_scoreValueMineralsCollectionRate", 84, 2),
            ("m_scoreValueVespeneCollectionRate", 84, 3),
            ("m_scoreValueWorkersActiveCount", 84, 4),
            ("m_scoreValueMineralsUsedInProgressArmy", 84, 5),
            ("m_scoreValueMineralsUsedInProgressEconomy", 84, 6),
            ("m_scoreValueMineralsUsedInProgressTechnology", 84, 7),
            ("m_scoreValueVespeneUsedInProgressArmy", 84, 8),
            ("m_scoreValueVespeneUsedInProgressEconomy", 84, 9),
            ("m_scoreValueVespeneUsedInProgressTechnology", 84, 10),
            ("m_scoreValueMineralsUsedCurrentArmy", 84, 11),
            ("m_scoreValueMineralsUsedCurrentEconomy", 84, 12),
            ("m_scoreValueMineralsUsedCurrentTechnology", 84, 13),
            ("m_scoreValueVespeneUsedCurrentArmy", 84, 14),
            ("m_scoreValueVespeneUsedCurrentEconomy", 84, 15),
            ("m_scoreValueVespeneUsedCurrentTechnology", 84, 16),
            ("m_scoreValueMineralsLostArmy", 84, 17),
            ("m_scoreValueMineralsLostEconomy", 84, 18),
            ("m_scoreValueMineralsLostTechnology", 84, 19),
            ("m_scoreValueVespeneLostArmy", 84, 20),
            ("m_scoreValueVespeneLostEconomy", 84, 21),
            ("m_scoreValueVespeneLostTechnology", 84, 22),
            ("m_scoreValueMineralsKilledArmy", 84, 23),
            ("m_scoreValueMineralsKilledEconomy", 84, 24),
            ("m_scoreValueMineralsKilledTechnology", 84, 25),
            ("m_scoreValueVespeneKilledArmy", 84, 26),
            ("m_scoreValueVespeneKilledEconomy", 84, 27),
            ("m_scoreValueVespeneKilledTechnology", 84, 28),
            ("m_scoreValueFoodUsed", 84, 29),
            ("m_scoreValueFoodMade", 84, 30),
            ("m_scoreValueMineralsUsedActiveForces", 84, 31),
            ("m_scoreValueVespeneUsedActiveForces", 84, 32),
            ("m_scoreValueMineralsFriendlyFireArmy", 84, 33),
            ("m_scoreValueMineralsFriendlyFireEconomy", 84, 34),
            ("m_scoreValueMineralsFriendlyFireTechnology", 84, 35),
            ("m_scoreValueVespeneFriendlyFireArmy", 84, 36),
            ("m_scoreValueVespeneFriendlyFireEconomy", 84, 37),
            ("m_scoreValueVespeneFriendlyFireTechnology", 84, 38),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_playerId", 1, 0),
            ("m_stats", 189, 1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
            ("m_unitTypeName", 29, 2),
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
            ("m_killerPlayerId", 59, 2),
            ("m_x", 10, 3),
            ("m_y", 10, 4),
            ("m_killerUnitTagIndex", 43, 5),
            ("m_killerUnitTagRecycle", 43, 6),
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
            ("m_unitTypeName", 29, 2),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_playerId", 1, 0),
            ("m_upgradeTypeName", 29, 1),
            ("m_count", 84, 2),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_unitTagIndex", 6, 0),
            ("m_unitTagRecycle", 6, 1),
        ],
    },
    TypeInfo::Array { bounds: IntBounds { min: 0, bitlen: 10 }, typeid: 84 },
    TypeInfo::Struct {
        fields: &[
            ("m_firstUnitIndex", 6, 0),
            ("m_items", 197, 1),
        ],
    },
    TypeInfo::Struct {
        fields: &[
            ("m_playerId", 1, 0),
            ("m_type", 6, 1),
            ("m_userId", 43, 2),
            ("m_slotId", 43, 3),
        ],
    },
];

