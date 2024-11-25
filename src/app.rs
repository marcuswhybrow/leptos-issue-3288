use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/my_app.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </FlatRoutes>
        </Router>
    }
}

// On my dev machine 1000 resources is enough to create the error
static VIDEO_COUNT: usize = 1000;

// Reducing the count to 100 prevents the error.
//static VIDEO_COUNT: usize = 100;

#[component]
fn Home() -> impl IntoView {
    let videos = Resource::new(
        || (),
        |_| async move {
            let mut videos: Vec<String> = vec![];
            let mut i = 0;
            while i < VIDEO_COUNT {
                videos.push(String::from("some-video.mp4"));
                i += 1;
            }
            videos
        },
    );

    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <p> VIDEO_COUNT: { move || VIDEO_COUNT }</p>
            <Suspense fallback=move|| view! { <p>Loading</p> }>
                <ul>
                    <For each=move||videos.get().unwrap() key=|v| v.clone() let:video>
                        <li>
                            <Video video />
                        </li>
                    </For>
                </ul>
            </Suspense>
        </main>
    }
}

#[component]
fn Video(video: String) -> impl IntoView {
    let thumbnail = Resource::new(
        || (),
        |_| async move { String::from("/path/to/some-thumbnail.jpg") },
    );
    view! {
        <Suspense fallback=move|| view! { <div class="w-32 h-24 bg-slate-500"></div> }>
            <img width="200" height="150" src=thumbnail.get() />
        </Suspense>
        <p>{ video }</p>
    }
}
