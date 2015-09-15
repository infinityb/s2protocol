use std::io::{self};

use phf::Map as PhfMap;

use super::{
    Protocol,
    InitData,
    ReplayDetails,
    ReplayGameEvents,
    ReplayMessageEvents,
    ReplayTrackerEvents,
    ReplayAttributesEvents,
};

pub struct Protocol15405;

impl Protocol for Protocol15405 {
    fn protocol_num(&self) -> u32 { 15405 }

    fn decode_replay_initdata(&self, rdr: &mut io::Read) -> io::Result<InitData> {
        unimplemented!();
    }

    fn decode_replay_details(&self, rdr: &mut io::Read) -> io::Result<ReplayDetails> {
        unimplemented!();
    }

    fn decode_replay_game_events(&self, rdr: &mut io::Read) -> io::Result<ReplayGameEvents> {
        unimplemented!();
    }

    fn decode_replay_message_events(&self, rdr: &mut io::Read) -> io::Result<ReplayMessageEvents> {
        unimplemented!();
    }

    fn decode_replay_tracker_events(&self, rdr: &mut io::Read) -> io::Result<ReplayTrackerEvents> {
        unimplemented!();
    }

    fn decode_replay_attributes_events(&self, rdr: &mut io::Read) -> io::Result<ReplayAttributesEvents> {
        unimplemented!();
    }
}

// FIXME: check value type. u8? u16?
const GAME_EVENTID_TYPEID: u32 = 0;

// FIXME: check key type. u8? u16?  Value type too.
static GAME_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
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

// FIXME: check value type. u8? u16?
const MESSAGE_EVENTID_TYPEID: u32 = 1;

// FIXME: check key type. u8? u16?  Value type too.
static MESSAGE_EVENT_TYPES: PhfMap<u32, (u32, &'static str)> = phf_map! {
    0_u32 => (128, "NNet.Game.SChatMessage"),
    1_u32 => (129, "NNet.Game.SPingMessage"),
    2_u32 => (130, "NNet.Game.SLoadingProgressMessage"),
    3_u32 => (62, "NNet.Game.SServerPingMessage"),
};