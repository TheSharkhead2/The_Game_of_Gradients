use leptos::*;

/// Main app/webpage for game
#[component]
pub fn MainApp() -> impl IntoView {
    view! {
        // we need canvas element for bevy to grab to
        <canvas id="main-game-canvas"></canvas>
    }
}
