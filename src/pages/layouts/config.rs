use super::{
    preview::{LayoutPreview, PanelProperties, Position},
    Message,
};
use crate::core::resources;
use cosmic::{widget, Element};
use cosmic_ext_config_templates::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
pub enum Layout {
    Cosmic,
    Mac,
    Windows,
    Ubuntu,
}

impl Layout {
    pub fn name(&self) -> &str {
        match self {
            Layout::Cosmic => "COSMIC",
            Layout::Mac => "macOS",
            Layout::Windows => "Windows",
            Layout::Ubuntu => "Ubuntu",
        }
    }

    pub fn preview(
        &self,
        spacing: &cosmic::cosmic_theme::Spacing,
        item_width: usize,
    ) -> Element<Message> {
        let layout = match self {
            Layout::Cosmic => LayoutPreview::new(
                Some(PanelProperties::new(Position::Top, true, 10.0)),
                Some(PanelProperties::new(Position::Bottom, true, 20.0)),
                6,
                true,
            ),
            Layout::Mac => LayoutPreview::new(
                Some(PanelProperties::new(Position::Top, true, 10.0)),
                Some(PanelProperties::new(Position::Bottom, false, 20.0)),
                6,
                true,
            ),
            Layout::Windows => LayoutPreview::new(
                None,
                Some(PanelProperties::new(Position::Bottom, true, 15.0)),
                6,
                true,
            ),
            Layout::Ubuntu => LayoutPreview::new(
                Some(PanelProperties::new(Position::Top, true, 10.0)),
                Some(PanelProperties::new(Position::Left, true, 20.0)),
                3,
                true,
            ),
        };

        widget::button::custom(layout.view(&spacing))
            .on_press(Message::ApplyLayout(self.clone()))
            .class(cosmic::style::Button::Image)
            .width(item_width as f32)
            .into()
    }

    pub fn schema(&self) -> Schema {
        match self {
            Layout::Cosmic => ron::from_str::<Schema>(resources::COSMIC_LAYOUT).unwrap(),
            Layout::Mac => ron::from_str::<Schema>(resources::MAC_LAYOUT).unwrap(),
            Layout::Windows => ron::from_str::<Schema>(resources::WINDOWS_LAYOUT).unwrap(),
            Layout::Ubuntu => ron::from_str::<Schema>(resources::UBUNTU_LAYOUT).unwrap(),
        }
    }

    pub(crate) fn list() -> Vec<Layout> {
        Vec::from([Layout::Cosmic, Layout::Mac, Layout::Windows, Layout::Ubuntu])
    }
}
