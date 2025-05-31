use super::Widget;
use crate::style::StyleEntry;
use crate::types::{MenuAssets, WidgetLabel};
use bevy::prelude::*;
use bevy::text::{FontSmoothing, LineHeight};

pub struct LabelWidget<'a> {
    text: &'a WidgetLabel,
    style: &'a StyleEntry,
}

impl<'a> LabelWidget<'a> {
    pub fn new(text: &'a WidgetLabel, style: &'a StyleEntry) -> Self {
        Self { text, style }
    }
}

impl<'a> Widget for LabelWidget<'a> {
    fn build(self, parent: &mut ChildSpawnerCommands, assets: &MenuAssets) {
        let LabelWidget { text, style, .. } = self;

        let (bg, fg) = (style.normal.bg, style.selected.fg);

        let text_style = TextFont {
            font: assets.font.clone(),
            font_size: style.size,
            line_height: LineHeight::RelativeToFont(style.size),
            font_smoothing: style.font_smoothing,
        };

        let text_color = TextColor::from(fg);

        parent
            .spawn(ButtonBundle {
                style: Node {
                    margin: style.margin,
                    padding: style.padding,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(bg),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(text.bundle(&text_style));
            });
    }
}
