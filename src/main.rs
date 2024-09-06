use leptos::view;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| {
        view! {
            <div>
                <h1>"Hello, World!"</h1>
                <p>"This is a simple example of a Leptos app."</p>
            </div>
        }
    })
}
