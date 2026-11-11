// big file for dicking around with GUI
// idk what im doing im just trying to copy stuff from https://codeberg.org/RustAudio/nice-plug/src/branch/main/examples/gain_iced and get it to work Lol
use iced_audio::Gesture;
use iced_audio::param::nice_to_iced;
use nice_plug::{editor::dpi::LogicalSize, prelude::*};
use nice_plug_iced::{
    IcedEditor, IcedEditorState, IcedNiceContext, IcedNiceSettings, PersistentState,
    iced::{
        self, Center, PollSubNotifier, Subscription, Theme,
        alignment::{self, Horizontal::Left},
        program::graphics::text::cosmic_text::skrifa::bitmap::Origin::TopLeft,
        widget::{Column, ProgressBar, button, column, container, pick_list, row, slider, text},
    },
};

use std::sync::Arc;

use crate::PassoidPluginParams;

pub fn tttt_theme() -> Theme {
    Theme::custom(
        "/tttt/",
        iced::theme::Palette {
            background: iced::Color::from_rgb8(244, 231, 227), //bg
            text: iced::Color::from_rgb8(107, 52, 52),         //red
            primary: iced::Color::from_rgb8(133, 144, 58),     //green
            success: iced::Color::from_rgb(0.0, 0.0, 0.0),
            warning: iced::Color::from_rgb8(173, 167, 163), //gray
            danger: iced::Color::from_rgb(0.0, 0.0, 0.0),
        },
    )
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// Sent when the application should poll parameters/meters and redraw.
    Poll,
    WindowResized,
    SetScaleFactor(f32),
    CutoffGestured(Gesture),
    FilterCountGestured(Gesture),
}

/// State relating to the editor itself (not necessarly the GUI). Put any
/// state that should persist between editor opens here.
///

pub struct EditorState {
    pub params: Arc<PassoidPluginParams>,
}

pub struct PassoidGui {
    /// The editor state is stored inside of a wrapper which allows the
    /// state to persist across editor opens.
    persistent_state: PersistentState<EditorState>,

    ctx: IcedNiceContext,

    value: i64,
}

impl PassoidGui {
    pub fn new(persistent_state: PersistentState<EditorState>, ctx: IcedNiceContext) -> Self {
        Self {
            persistent_state,
            ctx,
            value: 0,
        }
    }

    pub fn theme(&self) -> Option<Theme> {
        Some(tttt_theme())
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            // This subscription is trigged by the `PollSubNotifier`. This can be used,
            // for example, to notify the GUI that parameters have changed or to notify
            // the GUI that it should update its decibel meter.
            iced::poll_events().map(|_| Message::Poll),
            // A subscription that triggers when the window is resized.
            iced::window_resized().map(|_| Message::WindowResized),
        ])
    }

    /// Return the user's scale (zoom) factor. This is applied on top of the system's
    /// scale factor.
    pub fn scale_factor(&self) -> f32 {
        self.ctx.user_scale_factor()
    }

    pub fn update(&mut self, message: Message) {
        let setter = self.ctx.nice_context.param_setter();
        let params = &self.persistent_state.params;

        match message {
            Message::Poll => {
                // we dont really need anything here
            }
            Message::SetScaleFactor(scale_factor) => {
                self.ctx.set_user_scale_factor(scale_factor);

                // Note, this new scale factor should generally be stored in a config
                // file somewhere so that it persists across sessions.
                //
                // Fuck that hoe
            }
            Message::WindowResized => {
                // Sync the current window size to the iced editor's state. This is
                // needed to persist window size across editor opens.
                self.ctx.sync_window_size();
            } /*
            * Keeping this here so i know what the fuck to do
            Message::GainGestured(gesture) => {
            iced_audio::param::set_nice_param(&params.gain, gesture, &setter);
            }
             */
            Message::CutoffGestured(gesture) => {
                iced_audio::param::set_nice_param(&params.cutoff, gesture, &setter);
            }
            Message::FilterCountGestured(gesture) => {
                iced_audio::param::set_nice_param(&params.filter_count, gesture, &setter);
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let params = &self.persistent_state.params;
        let palette = tttt_theme().palette();

        let cutoff_knob =
            iced_audio::Knob::new(nice_to_iced(&params.cutoff)).on_gesture(Message::CutoffGestured);
        let filters_knob = iced_audio::Knob::new(nice_to_iced(&params.filter_count))
            .on_gesture(Message::FilterCountGestured);

        let scale_opts = [
            ScaleOption(0.5),
            ScaleOption(0.75),
            ScaleOption(1.0),
            ScaleOption(1.25),
            ScaleOption(1.5),
            ScaleOption(1.75),
            ScaleOption(2.0),
        ];

        column![
            row![text("passoid").color(palette.text).size(18)],
            row![
                text(">be me, allpass filter series")
                    .color(palette.primary)
                    .size(15)
            ],
            row![text(">cutoff").color(palette.primary).size(15), cutoff_knob]
                .align_y(Center)
                .spacing(30),
            row![
                text(">filter count").color(palette.primary).size(15),
                filters_knob
            ]
            .align_y(Center)
            .spacing(30),
            row![
                text("scale:").size(15),
                pick_list(
                    scale_opts,
                    Some(ScaleOption(self.ctx.user_scale_factor())),
                    |opt| Message::SetScaleFactor(opt.0)
                )
                .text_size(13)
            ]
        ]
        .padding(5)
        .spacing(5.0)
        .align_x(Left)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScaleOption(f32);

impl std::fmt::Display for ScaleOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", (self.0 * 100.0).round())
    }
}
