use gpui::{Hsla, Rgba, WindowControlArea, rgb, rgba, prelude::*};

use ui::prelude::*;

#[derive(IntoElement)]
pub struct WindowsWindowControls {
    button_height: Pixels,
}

impl WindowsWindowControls {
    pub fn new(button_height: Pixels) -> Self {
        Self { button_height }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_font() -> &'static str {
        "Segoe Fluent Icons"
    }

    #[cfg(target_os = "windows")]
    fn get_font() -> &'static str {
        use windows::Wdk::System::SystemServices::RtlGetVersion;

        let mut version = unsafe { std::mem::zeroed() };
        let status = unsafe { RtlGetVersion(&mut version) };

        if status.is_ok() && version.dwBuildNumber >= 22000 {
            "Segoe Fluent Icons"
        } else {
            "Segoe MDL2 Assets"
        }
    }
}

impl RenderOnce for WindowsWindowControls {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id("windows-window-controls")
            .font_family(Self::get_font())
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(self.button_height)
            .min_h(self.button_height)
            .child(WindowsCaptionButton::Close)
            .child(WindowsCaptionButton::Minimize)
            .map(|this| {
                this.child(if window.is_maximized() {
                    WindowsCaptionButton::Restore
                } else {
                    WindowsCaptionButton::Maximize
                })
            })
    }
}

#[derive(IntoElement)]
enum WindowsCaptionButton {
    Minimize,
    Restore,
    Maximize,
    Close,
}

impl WindowsCaptionButton {
    #[inline]
    fn id(&self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Restore => "restore",
            Self::Maximize => "maximize",
            Self::Close => "close",
        }
    }

    #[inline]
    fn icon(&self) -> &'static str {
        match self {
            Self::Minimize => "\u{e949}",
            Self::Restore => "\u{e948}",
            Self::Maximize => "\u{e948}",
            Self::Close => "\u{e947}",
        }
    }

    #[inline]
    fn control_area(&self) -> WindowControlArea {
        match self {
            Self::Close => WindowControlArea::Close,
            Self::Maximize | Self::Restore => WindowControlArea::Max,
            Self::Minimize => WindowControlArea::Min,
        }
    }
}

impl RenderOnce for WindowsCaptionButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let (_hover_bg, _hover_fg, _active_bg, _active_fg) = match self {
            Self::Close => {
                let color: Hsla = Rgba {
                    r: 232.0 / 255.0,
                    g: 17.0 / 255.0,
                    b: 32.0 / 255.0,
                    a: 1.0,
                }
                .into();

                (
                    color,
                    gpui::white(),
                    color.opacity(0.8),
                    gpui::white().opacity(0.8),
                )
            },
            _ => (
                cx.theme().colors().ghost_element_hover,
                cx.theme().colors().text,
                cx.theme().colors().ghost_element_active,
                cx.theme().colors().text,
            ),
        };

        let text_color = cx.theme().colors().text;

        h_flex()
            .justify_center()
            .content_center()
            .occlude()
            .w(px(36.))
            .h_full()
            .child(
                div()
                    .id(self.id())
                    .flex()
                    .justify_center()
                    .items_center()
                    .rounded_full()
                    .occlude()
                    .w(px(18.))
                    .h(px(18.))
                    .map(|this| match self.id() {
                        "close" => this
                            .bg(rgb(0xed6a5f))
                            .text_color(rgb(0xed6a5f))
                            .hover(|style| style.text_color(text_color))
                            .active(|style| style.text_color(text_color)),
                        "minimize" => this
                            .bg(rgb(0xf6be50))
                            .text_color(rgb(0xf6be50))
                            .hover(|style| style.text_color(text_color))
                            .active(|style| style.text_color(text_color)),
                        _ => this
                            .bg(rgb(0x61c555))
                            .text_color(rgb(0x61c555))
                            .hover(|style| style.text_color(text_color))
                            .active(|style| style.text_color(text_color))
                    })
                    .text_size(px(10.0))
                    .window_control_area(self.control_area())
                    .child(self.icon())
            )
    }
}
