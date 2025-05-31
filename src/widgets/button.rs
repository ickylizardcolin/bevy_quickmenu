use super::Widget;
use crate::style::StyleEntry;
use crate::types::{ButtonComponent, MenuAssets, WidgetId, WidgetLabel};
use crate::{ActionTrait, MenuSelection, ScreenTrait};
use bevy::prelude::*;

pub struct ButtonWidget<'a, S>
where
    S: ScreenTrait + 'static,
{
    text: &'a WidgetLabel,
    style: &'a StyleEntry,
    menu_identifier: (WidgetId, usize),
    selection: &'a MenuSelection<S>,
    selected: bool,
}

impl<'a, S> ButtonWidget<'a, S>
where
    S: ScreenTrait + 'static,
{
    pub fn new(
        text: &'a WidgetLabel,
        style: &'a StyleEntry,
        menu_identifier: (WidgetId, usize),
        selection: &'a MenuSelection<S>,
        selected: bool,
    ) -> Self {
        Self {
            text,
            style,
            menu_identifier,
            selection,
            selected,
        }
    }
}

impl<'a, A, S, State> Widget for ButtonWidget<'a, S>
where
    State: 'static,
    A: ActionTrait<State = State> + 'static,
    S: ScreenTrait<Action = A> + 'static,
{
    fn build(self, parent: &mut ChildSpawnerCommands, assets: &MenuAssets) {
        let ButtonWidget {
            text,
            style,
            menu_identifier,
            selection,
            selected,
        } = self;

        let (bg, fg) = if selected {
            (style.selected.bg, style.selected.fg)
        } else {
            (style.normal.bg, style.normal.fg)
        };

        let text_font = TextFont {
            font: assets.font.clone(),
            font_size: style.size,
        };
        let text_color = TextColor::from(fg);

        parent
            .spawn((
                Node{
                    style: Style {
                        margin: style.margin,
                        padding: style.padding,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                },
                Button {
                style: style.clone(),
                selection: selection.clone(),
                menu_identifier,
                selected,
                background_color: BackgroundColor(bg),
                ..default()
            }))
            .insert(ButtonComponent {

            })
            .with_children(|parent| {
                parent.spawn(text.bundle(&text_style));
            });
    }
}
