use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::console;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/visualtimer.css"/>

        <Link rel="manifest" href="/manifest.json" />
        <Script >
            "
            if ('serviceWorker' in navigator) {
              window.addEventListener('load', function() {
                navigator.serviceWorker.register('/service-worker.js').then(function(registration) {
                  // Registrierung erfolgreich
                  console.log('ServiceWorker registration successful with scope: ', registration.scope);
                }, function(err) {
                  // Registrierung fehlgeschlagen
                  console.log('ServiceWorker registration failed: ', err);
                });
              });
            }
            "
        </Script>

        // sets the document title
        <Title text="Visual Timer"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=Timer/>
                </Routes>
            </main>
        </Router>
    }
}

/// Main timer component
#[component]
fn Timer() -> impl IntoView {
    use crate::app::leptos_dom::helpers::IntervalHandle;

    // out timer in seconds u32
    let (timer, set_timer) = create_signal::<u32>(0);
    let (initial, set_initial) = create_signal::<u32>(0);
    let (handle, set_handle) = create_signal(None::<IntervalHandle>);
    let (status, set_status) = create_signal("...".to_string());
    let (is_running, set_is_running) = create_signal(false);

    view! {
        <h1>"Visual Timer"</h1>
        <input type="number"
            on:input=move |ev| {
                let seconds = event_target_value(&ev).parse();
                if let Ok(seconds) = seconds {
                    set_initial(seconds)
                }
            }
            prop:value=initial
        />
        <button on:click=move |_| {
            if is_running.get() {
                console::log_1(&"Already running...".into());
                return
            }
            set_status("Timer is running ...".to_string());
            set_timer(initial.get());
            let new_handle = set_interval_with_handle(
                move || {
                    set_is_running(true);
                    let current_time = timer();
                    if current_time > 0 {
                        set_timer(current_time - 1);
                        if timer() == 0 {
                            set_status("Finished!".to_string());
                        }
                        return
                    }
                    if let Some(handle) = handle.get() {
                        handle.clear();
                        set_is_running(false);
                    }
                }, std::time::Duration::from_secs(1));
            set_handle(Some(new_handle.expect("to be assignable")))
        }>
        Start
        </button>
        <button on:click=move |_| {
            if let Some(handle) = handle.get() {
                handle.clear();
                set_is_running(false);
            }

            set_status("...".to_string());
            set_timer(0);
        }>
        Reset
        </button>
        <br/>
        <h2>{timer}</h2>
        <div>{status}</div>
    }
}
