use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const GAMESTART_MENU_STYLE: Style = Style{
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(40.0), Val::Percent(15.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    margin: UiRect {
        left: Val::Percent(40.),
        right: Val::Percent(20.),
        top: Val::Percent(0.),
        bottom: Val::Percent(15.)
    },
    ..Style::DEFAULT
};

pub const GAMEOVER_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style{
    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
    margin: UiRect::new(
        Val::Px(8.0), 
        Val::Px(8.0), 
        Val::Px(8.0), 
        Val::Px(8.0),
    ),
    ..Style::DEFAULT
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle { 
        font: asset_server.load("FiraSans-Bold.ttf"), 
        font_size: 32.0, 
        color: Color::WHITE,
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle { 
        font: asset_server.load("NotoSansKR-Bold.otf"),
        font_size: 64.0, 
        color: Color::WHITE, 
    }
}

pub fn get_user_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle { 
        font: asset_server.load("NotoSansKR-Bold.otf"),
        font_size: 45.0, 
        color: Color::WHITE, 
    }
}

pub fn get_result_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle { 
        font: asset_server.load("NotoSansKR-Bold.otf"),
        font_size: 35.0, 
        color: Color::WHITE, 
    }
}

pub const TITLE_STYLE: Style = Style{
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.0), Val::Px(120.0)),
    ..Style::DEFAULT
};

pub const USER_STYLE: Style = Style{
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(50.0)),
    ..Style::DEFAULT
};