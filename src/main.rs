use yew::Renderer;

mod components;

fn main() {
    Renderer::<components::app::App>::new().render();
}
