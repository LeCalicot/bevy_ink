use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bladeink::story_callbacks::{ErrorHandler, ErrorType};
use bladeink::{choice::Choice, story::Story};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Setup,
    Playing,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<UiState>()
        .init_resource::<InkStory>()
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}

#[derive(Default, Resource)]
struct UiState {
    current_text: String,
    choices: Vec<String>,
}

fn ui_example_system(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("my_panel")
        .min_height(500.)
        .show(ctx, |ui| {
            ui.label("Story");
        });
    egui::CentralPanel::default().show(ctx, |ui| {
        // ui.label("Story");

        ui.label("Choices");
    });
}

#[derive(Resource)]
struct InkStory{
    story: Story,
};

impl Default for InkStory {
    fn default() -> Self {
        InkStory {}
    }
}

pub struct InkStoryPlugin;

impl Plugin for InkStoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InkStory>()
            .add_systems(OnEnter(GameState::Setup), start_story)
            .add_systems(Update, play_story.run_if(in_state(GameState::Playing)));
    }
}

fn start_story(story: ResMut<InkStory>, mut next_state: ResMut<NextState<GameState>>) {
    let json_string = get_json_string("assets/TheIntercept.ink.json")?;

    // REMOVE BOM if exits
    let json_string_without_bom = json_string.strip_prefix('\u{feff}').unwrap_or(&json_string);

    let mut story = Story::new(json_string_without_bom)?;
    let err_handler = EHandler::new();
    story.set_error_handler(err_handler.clone());

    let mut end = false;

    next_state.set(GameState::Playing);
}

fn play_story(story: ResMut<InkStory>) {}
