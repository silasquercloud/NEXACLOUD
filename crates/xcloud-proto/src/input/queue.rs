//! Port of qt/src/streaming/InputQueue.{h,cpp}
//!
//! Coalesces polled controller frames between flush intervals (125 Hz in the
//! input crate) into one input packet, mirroring the browser client's input
//! queue. Polled frames are coalesced per gamepad index (latest wins);
//! injected one-shot frames are kept in order and emitted ahead of the polled
//! ones; timing metadata is appended as-is.

use super::models::{FrameTimingMetadata, GamepadInputFrame, KeyboardFrame, MouseFrame};
use super::packet;

/// The two `InputQueue` calls the input channel makes: `makeInitialMetadata()`
/// on open and `flush()` on every tick. Stands in for the `InputQueue*` the C++
/// `InputChannel` held, so the channel can be driven by a fake in tests.
pub trait InputPacketSource {
    /// `InputQueue::makeInitialMetadata`: the ClientMetadata packet sent when the
    /// input channel opens.
    fn make_initial_metadata(&mut self, now_ms: u64) -> Vec<u8>;
    /// `InputQueue::flush`: a packet with every queued frame, or `None` when empty.
    fn flush(&mut self, now_ms: u64) -> Option<Vec<u8>>;
}

/// Accumulates input frames between flush intervals.
#[derive(Debug, Default, Clone)]
pub struct InputQueue {
    sequence: u32,
    injected: Vec<GamepadInputFrame>,
    pending: Vec<GamepadInputFrame>,
    timing: Vec<FrameTimingMetadata>,
    keyboard: Vec<KeyboardFrame>,
    mouse: Vec<MouseFrame>,
}

impl InputQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stores the latest polled frame per controller until the next flush.
    pub fn enqueue_gamepad_frame(&mut self, frame: GamepadInputFrame) {
        if let Some(slot) = self
            .pending
            .iter_mut()
            .rev()
            .find(|pending| pending.gamepad_index == frame.gamepad_index)
        {
            *slot = frame;
            return;
        }
        self.pending.push(frame);
    }

    /// Appends a synthetic frame without coalescing (one-shot button taps).
    pub fn enqueue_injected_gamepad_frame(&mut self, frame: GamepadInputFrame) {
        self.injected.push(frame);
    }

    /// Queues renderer timing metadata to piggyback on the next packet.
    pub fn enqueue_timing_frame(&mut self, frame: FrameTimingMetadata) {
        self.timing.push(frame);
    }

    /// Queues a native keyboard event (Windows virtual-key report).
    pub fn enqueue_keyboard_frame(&mut self, frame: KeyboardFrame) {
        self.keyboard.push(frame);
    }

    /// Queues a native mouse movement / button / wheel event.
    pub fn enqueue_mouse_frame(&mut self, frame: MouseFrame) {
        self.mouse.push(frame);
    }

    /// Builds a packet with every queued frame and clears the queue.
    /// `None` when nothing was queued (the sequence number still advances, as in C++).
    pub fn flush(&mut self, now_ms: u64) -> Option<Vec<u8>> {
        let sequence = self.next_sequence();
        let mut gamepad_frames = std::mem::take(&mut self.injected);
        gamepad_frames.append(&mut self.pending);
        let timing_frames = std::mem::take(&mut self.timing);
        let keyboard_frames = std::mem::take(&mut self.keyboard);
        let mouse_frames = std::mem::take(&mut self.mouse);

        if gamepad_frames.is_empty()
            && timing_frames.is_empty()
            && keyboard_frames.is_empty()
            && mouse_frames.is_empty()
        {
            return None;
        }

        Some(packet::input_packet(
            sequence,
            &gamepad_frames,
            &timing_frames,
            &keyboard_frames,
            &mouse_frames,
            now_ms,
        ))
    }

    /// The ClientMetadata packet sent when the input channel opens.
    pub fn make_initial_metadata(&mut self, now_ms: u64) -> Vec<u8> {
        packet::client_metadata(self.next_sequence(), 1, now_ms)
    }

    /// Returns the current sequence number and advances it (wrapping at `u32::MAX`).
    pub fn next_sequence(&mut self) -> u32 {
        let sequence = self.sequence;
        self.sequence = self.sequence.wrapping_add(1);
        sequence
    }

    /// True when a flush would produce a packet.
    pub fn has_pending(&self) -> bool {
        !(self.injected.is_empty()
            && self.pending.is_empty()
            && self.timing.is_empty()
            && self.keyboard.is_empty()
            && self.mouse.is_empty())
    }
}

impl InputPacketSource for InputQueue {
    fn make_initial_metadata(&mut self, now_ms: u64) -> Vec<u8> {
        InputQueue::make_initial_metadata(self, now_ms)
    }

    fn flush(&mut self, now_ms: u64) -> Option<Vec<u8>> {
        InputQueue::flush(self, now_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::models::GamepadButtons;

    // --- tst_inputqueue.cpp --------------------------------------------------

    #[test]
    fn empty_flush_returns_nothing() {
        let mut queue = InputQueue::new();
        assert!(queue.flush(0).is_none());
    }

    #[test]
    fn coalesces_same_gamepad() {
        let mut queue = InputQueue::new();
        let first = GamepadInputFrame {
            gamepad_index: 0,
            buttons: GamepadButtons::A,
            ..GamepadInputFrame::default()
        };
        let second = GamepadInputFrame {
            gamepad_index: 0,
            buttons: GamepadButtons::B,
            ..GamepadInputFrame::default()
        };
        queue.enqueue_gamepad_frame(first);
        queue.enqueue_gamepad_frame(second);

        let packet = queue.flush(0).expect("packet");
        // One gamepad frame only, carrying the latest state (B).
        assert_eq!(packet[14], 1);
        assert_eq!(packet[15], 0);
        assert_eq!(packet[16], (GamepadButtons::B.bits() & 0xFF) as u8);
    }

    #[test]
    fn sequence_advances() {
        let mut queue = InputQueue::new();
        let metadata = queue.make_initial_metadata(0);
        assert_eq!(packet::sequence_of(&metadata), 0);

        queue.enqueue_gamepad_frame(GamepadInputFrame::default());
        let packet = queue.flush(0).expect("packet");
        assert_eq!(packet::sequence_of(&packet), 1);
    }

    #[test]
    fn injected_frames_are_prepend() {
        let mut queue = InputQueue::new();
        let injected = GamepadInputFrame {
            gamepad_index: 0,
            buttons: GamepadButtons::MENU,
            ..GamepadInputFrame::default()
        };
        let polled = GamepadInputFrame {
            gamepad_index: 1,
            buttons: GamepadButtons::A,
            ..GamepadInputFrame::default()
        };

        queue.enqueue_injected_gamepad_frame(injected);
        queue.enqueue_gamepad_frame(polled);
        let packet = queue.flush(0).expect("packet");
        // Two gamepad frames; the injected frame comes first.
        assert_eq!(packet[14], 2);
        assert_eq!(packet[15], 0);
        assert_eq!(packet[38], 1);
    }

    // --- additional coverage (no Qt counterpart) ----------------------------

    #[test]
    fn empty_flush_still_consumes_a_sequence_number() {
        let mut queue = InputQueue::new();
        assert!(queue.flush(0).is_none());
        queue.enqueue_gamepad_frame(GamepadInputFrame::default());
        let packet = queue.flush(0).expect("packet");
        assert_eq!(packet::sequence_of(&packet), 1);
    }

    #[test]
    fn injected_frames_are_not_coalesced() {
        let mut queue = InputQueue::new();
        let tap = GamepadInputFrame {
            gamepad_index: 0,
            buttons: GamepadButtons::NEXUS,
            ..GamepadInputFrame::default()
        };
        queue.enqueue_injected_gamepad_frame(tap);
        queue.enqueue_injected_gamepad_frame(GamepadInputFrame::default());
        let packet = queue.flush(0).expect("packet");
        assert_eq!(packet[14], 2);
    }

    #[test]
    fn timing_frames_alone_produce_a_metadata_packet() {
        let mut queue = InputQueue::new();
        assert!(!queue.has_pending());
        queue.enqueue_timing_frame(FrameTimingMetadata::default());
        assert!(queue.has_pending());
        let packet = queue.flush(42).expect("packet");
        assert_eq!(packet::report_type_of(&packet), 1);
        assert_eq!(packet.len(), packet::HEADER_SIZE + 1 + 28);
        assert!(!queue.has_pending());
        assert!(queue.flush(43).is_none());
    }

    #[test]
    fn flush_clears_every_bucket() {
        let mut queue = InputQueue::new();
        queue.enqueue_injected_gamepad_frame(GamepadInputFrame::default());
        queue.enqueue_gamepad_frame(GamepadInputFrame::default());
        queue.enqueue_timing_frame(FrameTimingMetadata::default());
        let packet = queue.flush(0).expect("packet");
        assert_eq!(packet::report_type_of(&packet), 3);
        assert!(queue.flush(0).is_none());
    }

    #[test]
    fn keyboard_and_mouse_reports_use_browser_wire_format() {
        use super::{KeyboardFrame, MouseFrame};
        use crate::input::packet::ReportType;

        let mut queue = InputQueue::new();
        queue.enqueue_keyboard_frame(KeyboardFrame::vkey(0x57, true)); // W down
        queue.enqueue_keyboard_frame(KeyboardFrame::vkey(0x57, false)); // W up
        queue.enqueue_mouse_frame(MouseFrame::relative(-4, 9, 0, 0, 1));

        let packet = queue.flush(7).expect("packet");
        let report = ReportType(packet::report_type_of(&packet));
        assert!(report.contains(ReportType::MOUSE));
        assert!(report.contains(ReportType::KEYBOARD));
        assert!(!report.contains(ReportType::GAMEPAD));

        // Header (14) + mouse count/frame (1 + 18) + keyboard count/frames (1 + 6).
        assert_eq!(packet.len(), 14 + 19 + 7);
        assert_eq!(packet[14], 1); // one mouse frame
        assert_eq!(
            u32::from_le_bytes(packet[15..19].try_into().unwrap()),
            (-4i32) as u32
        );
        assert_eq!(packet[31], 1); // left button
        assert_eq!(packet[32], 0); // relative
        assert_eq!(packet[33], 2); // two keyboard frames
        assert_eq!(&packet[34..37], &[2, 1, 0x57]); // VKey, pressed, W
        assert_eq!(&packet[37..40], &[2, 0, 0x57]); // VKey, released, W
        assert!(!queue.has_pending());
    }
}
