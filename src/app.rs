use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::global::{BACKGROUND_CSS, FLOWER_GRADIENT, PURPLE_GRADIENT};
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-csr-tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/:thing" view=ErrorPage/>
                </Routes>
            </main>
        </Router>
    }
}

macro_rules! s {
     ($e:expr) => {
        $e.to_string()
    };
}

#[component]
fn Button(
    href: String,
    title: String,
    css: Option<String>
)-> impl IntoView {

    let (underline_css, bg_css) = if let Some(css) = &css {
        (
            format!("{} hover:underline decoration-[0.3rem]", css),
            format!("")
        )
    } else {
        (
            format!(""),
            format!("font-semibold px-8 py-2 rounded-full border border-white text-white hover:text-indigo-800 transition ease-in duration-300 hover:bg-white")
        )
    };

    view! {
        <a class=format!("inline-block {}", bg_css) target="_blank" href=href rel="noreferrer">
            <p class=format!("{}", underline_css)>{title}</p>
        </a>
    }
}

#[component]
fn HomePage() -> impl IntoView {
     view! {
         <main class=format!("{} {} font-opensans py-20", BACKGROUND_CSS, PURPLE_GRADIENT)>
             <div class=format!("w-full flex flex-col items-center justify-center space-y-[10rem]")>
                 <div class="w-2/3 text-center">
                     <p class="text-white italic text-3xl md:text-6xl font-extrabold">
                         {"A blazingly fast way to start a "}
                         <Button
                             href=s!("https://leptos-rs.github.io/leptos/")
                             title=s!("Leptos")
                             css=Some(s!("text-orange-500"))
                         /> {" "}
                         <Button
                             href=s!("https://leptos.dev/")
                             title=s!("client-side rendered")
                             css=Some(s!("text-pink-400"))
                         /> {" app with "}
                         <Button
                             href=s!("https://github.com/leptos-rs/leptos/discussions/125")
                             title=s!("tailwind.")
                             css=Some(s!("text-blue-600"))
                         />
                     </p>
                 </div>
                 <div class="w-2/3 text-center text-xl md:text-2xl space-x-[5rem]">
                     <Button
                         href=s!("https://github.com/friendlymatthew/leptos-csr-tailwind")
                         title=s!("See boilerplate")
                         css=None
                     />
                     <Button
                         href=s!("https://github.com/friendlymatthew/create-leptos-csr-tw")
                         title=s!("Usage")
                         css=None
                     />
                 </div>
             </div>
         </main>
     }
}

#[component]
fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown = move || {
        params.with(|p| p.get("thing").cloned().unwrap_or_default())
    };

    let unknown = p_unknown();

    view! {
        <main class=format!("{} {} h-screen", BACKGROUND_CSS, FLOWER_GRADIENT)>
            <p class="">Unknown command: {unknown}</p>
        </main>
    }
}