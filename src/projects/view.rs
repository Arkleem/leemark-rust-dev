use maud::{html, Markup, PreEscaped};
use crate::state::AppState;

pub fn projects_section(state: &AppState) -> Markup {
    html! {
        section
            id="projects"
            class="py-20 overflow-hidden"
            style="background-color: #ffffff; font-family: 'EB Garamond', 'Garamond', 'Times New Roman', serif;"
            data-signals="{selectedProject: '', mainImage: '', expanded: false}"
        {
            div class="container mx-auto px-4" {
                div class="text-center mb-12" {
                    h2 class="text-3xl md:text-4xl font-bold mt-2 mb-4" style="color: #1a1a1a;" { "My Projects" }
                    div class="w-20 h-1 mx-auto rounded-full" style="background: linear-gradient(to right, #d4af37, #b8960f);" {}
                }

                // Project Cards
                div class="flex gap-6 overflow-x-auto pb-6 px-4 -mx-4 scroll-smooth" style="scroll-snap-type: x mandatory; -webkit-overflow-scrolling: touch;" {
                    @for project in &state.projects {
                        div class="min-w-[320px] max-w-[350px] flex-shrink-0 scroll-snap-align-start group rounded-xl border transition-all duration-300 overflow-hidden hover:-translate-y-2 hover:shadow-xl flex flex-col" style="background-color: #f8f8f8; border-color: #e8e8e8;" {
                            // Card image - use images[0] or fallback
                            @if let Some(images) = &project.images {
                                @if !images.is_empty() {
                                    @if images[0].starts_with("/static/") {
                                        img src=(images[0]) alt=(project.title) class="w-full h-48 object-cover";
                                    } @else {
                                        div class="w-full h-48 flex items-center justify-center text-6xl" style="background: #f0f0f0;" {
                                            (images[0])
                                        }
                                    }
                                } @else {
                                    div class="w-full h-48 flex items-center justify-center text-6xl" style="background: #f0f0f0;" {
                                        ""
                                    }
                                }
                            } @else {
                                div class="w-full h-48 flex items-center justify-center text-6xl" style="background: #f0f0f0;" {
                                    ""
                                }
                            }

                            div class="p-6 flex flex-col flex-1" {
                                h3 class="text-lg font-bold mb-2 transition-colors" style="color: #1a1a1a; font-family: 'EB Garamond', 'Garamond', serif;" {
                                    (project.title)
                                }
                                p class="text-sm leading-relaxed mb-4 line-clamp-3 flex-1" style="color: #4a4a4a; font-family: 'EB Garamond', 'Garamond', serif;" {
                                    (project.description)
                                }

                                div class="flex flex-wrap gap-1.5 mb-4" {
                                    @for tag in &project.tags {
                                        span class="px-2 py-0.5 rounded-full text-xs" style="background-color: #e8e8e8; color: #4a4a4a;" {
                                            (tag)
                                        }
                                    }
                                }

                                // View button
                                button
                                    data-on:click=(PreEscaped(format!("$selectedProject = '{}'; $mainImage = '{}'", project.id, project.images.as_ref().and_then(|imgs| imgs.first()).unwrap_or(&"".to_string()))))
                                    class="w-full inline-flex items-center justify-center gap-2 px-4 py-2 font-semibold rounded-lg transition-all flex-shrink-0"
                                    style="background-color: #d4af37; color: #ffffff; font-family: 'EB Garamond', 'Garamond', serif; cursor: pointer;"
                                {
                                    i class="fas fa-eye" {}
                                    "View"
                                }
                            }
                        }
                    }
                }

                // ---- MODAL / POPUP ----
                div
                    data-show="$selectedProject != ''"
                    style="
                        display: none;
                        position: fixed;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        z-index: 9999;
                        background: rgba(0, 0, 0, 0.6);
                        backdrop-filter: blur(8px);
                        justify-content: center;
                        align-items: center;
                        padding: 20px;
                    "
                {
                    div style="
                        background: #ffffff;
                        max-width: 600px;
                        width: 100%;
                        max-height: 90vh;
                        border-radius: 16px;
                        padding: 30px 35px;
                        position: relative;
                        overflow-y: auto;
                        box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                        animation: modalFadeIn 0.3s ease;
                        margin: auto;
                    " {
                        // Close button
                        button
                            data-on:click=(PreEscaped("$selectedProject = ''"))
                            style="
                                position: sticky;
                                top: 0;
                                float: right;
                                background: none;
                                border: none;
                                font-size: 24px;
                                cursor: pointer;
                                color: #94a3b8;
                                padding: 0 0 0 10px;
                                z-index: 10;
                                font-family: inherit;
                            "
                        {}

                        // Project content - only show the one with matching ID
                        @for project in &state.projects {
                            div
                                data-show=(PreEscaped(format!("$selectedProject == '{}'", project.id)))
                                style="display: none; width: 100%;"
                            {
                                // ---- MAIN IMAGE ----
                                @if let Some(images) = &project.images {
                                    @if !images.is_empty() {
                                        @let main_image_src = format!(
                                            "$mainImage != '' ? $mainImage : '{}'",
                                            images[0]
                                        );
                                        div style="margin-bottom: 12px; border-radius: 12px; overflow: hidden; background: #f0f0f0; width: 100%; position: relative;" {
                                            img
                                                data-attr:src=(PreEscaped(main_image_src))
                                                alt=(project.title)
                                                style="width: 100%; max-height: 300px; object-fit: contain; display: block; background: #f0f0f0; cursor: pointer;"
                                                data-on:click="$expanded = true"
                                            {}
                                        }
                                    }
                                }

                                // ---- THUMBNAIL GALLERY (Clickable) ----
                                @if let Some(images) = &project.images {
                                    @if images.len() > 1 {
                                        div style="display: flex; gap: 8px; margin-bottom: 20px; overflow-x: auto; padding-bottom: 8px; flex-wrap: nowrap;" {
                                            @for img in images {
                                                img
                                                    src=(img)
                                                    alt=(project.title)
                                                    style="
                                                        min-width: 70px;
                                                        max-width: 70px;
                                                        height: 50px;
                                                        object-fit: cover;
                                                        border-radius: 6px;
                                                        cursor: pointer;
                                                        border: 2px solid transparent;
                                                        transition: all 0.2s;
                                                    "
                                                    data-on:click=(PreEscaped(format!("$mainImage = '{}'", img)))
                                                    onmouseenter="this.style.borderColor = '#d4af37'"
                                                    onmouseleave="this.style.borderColor = 'transparent'"
                                                {}
                                            }
                                        }
                                    }
                                }

                                // Project Title
                                h3 style="font-size: 1.5rem; font-weight: 700; margin: 0 0 8px 0; color: #1a1a1a; font-family: 'EB Garamond', 'Garamond', serif; text-align: left;" {
                                    (project.title)
                                }

                                // Full Description
                                p style="font-size: 0.95rem; line-height: 1.8; color: #4a4a4a; margin: 0 0 16px 0; text-align: justify; font-family: 'EB Garamond', 'Garamond', serif;" {
                                    @if let Some(full_desc) = &project.full_description {
                                        (full_desc)
                                    } @else {
                                        (project.description)
                                    }
                                }

                                // Tech Stack
                                div style="margin-bottom: 16px;" {
                                    div style="font-weight: 600; font-size: 0.85rem; color: #1a1a1a; margin-bottom: 6px;" { "Tech Stack" }
                                    div style="display: flex; flex-wrap: wrap; gap: 6px;" {
                                        @for tag in &project.tags {
                                            span style="padding: 4px 14px; border-radius: 20px; font-size: 0.75rem; background: #f0f0f0; color: #4a4a4a;" {
                                                (tag)
                                            }
                                        }
                                    }
                                }

                                // Features
                                @if let Some(features) = &project.features {
                                    div style="margin-bottom: 16px;" {
                                        div style="font-weight: 600; font-size: 0.85rem; color: #1a1a1a; margin-bottom: 6px;" { "Features" }
                                        ul style="margin: 0; padding-left: 20px; color: #4a4a4a; font-size: 0.9rem; line-height: 1.8;" {
                                            @for feature in features {
                                                li { (feature) }
                                            }
                                        }
                                    }
                                }

                                // Action Buttons
                                div style="display: flex; gap: 12px; margin-top: 20px; flex-wrap: wrap;" {
                                    @if let Some(github) = &project.github_link {
                                        a
                                            href=(github)
                                            target="_blank"
                                            rel="noopener"
                                            style="
                                                flex: 1;
                                                min-width: 120px;
                                                text-align: center;
                                                padding: 10px 20px;
                                                border-radius: 8px;
                                                font-weight: 600;
                                                text-decoration: none;
                                                transition: all 0.2s;
                                                background: #1a1a1a;
                                                color: #ffffff;
                                                font-size: 0.9rem;
                                                font-family: 'EB Garamond', 'Garamond', serif;
                                            "
                                        {
                                            i class="fab fa-github" {}
                                            " GitHub"
                                        }
                                    }

                                    @if let Some(demo) = &project.demo_link {
                                        a
                                            href=(demo)
                                            target="_blank"
                                            rel="noopener"
                                            style="
                                                flex: 1;
                                                min-width: 120px;
                                                text-align: center;
                                                padding: 10px 20px;
                                                border-radius: 8px;
                                                font-weight: 600;
                                                text-decoration: none;
                                                transition: all 0.2s;
                                                background: #d4af37;
                                                color: #ffffff;
                                                font-size: 0.9rem;
                                                font-family: 'EB Garamond', 'Garamond', serif;
                                            "
                                        {
                                            i class="fas fa-external-link-alt" {}
                                            " Live Demo"
                                        }
                                    }

                                    // Close button inside modal
                                    button
                                        data-on:click=(PreEscaped("$selectedProject = ''"))
                                        style="
                                            flex: 0.5;
                                            min-width: 80px;
                                            text-align: center;
                                            padding: 10px 20px;
                                            border-radius: 8px;
                                            font-weight: 600;
                                            border: 1px solid #e0e0e0;
                                            background: transparent;
                                            color: #4a4a4a;
                                            cursor: pointer;
                                            font-size: 0.9rem;
                                            font-family: 'EB Garamond', 'Garamond', serif;
                                            transition: all 0.2s;
                                        "
                                    {
                                        "Close"
                                    }
                                }
                            }
                        }
                    }
                }

                // ---- FULL-SCREEN LIGHTBOX ----
                div
                    data-show="$expanded == true"
                    style="
                        display: none;
                        position: fixed;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        z-index: 99999;
                        background: rgba(0, 0, 0, 0.9);
                        justify-content: center;
                        align-items: center;
                        cursor: zoom-out;
                        padding: 40px;
                    "
                    data-on:click="$expanded = false"
                {
                    img
                        data-attr:src="$mainImage"
                        alt="Full screen"
                        style="
                            max-width: 95%;
                            max-height: 95%;
                            object-fit: contain;
                            box-shadow: 0 0 60px rgba(0,0,0,0.5);
                            border-radius: 8px;
                        "
                    {}
                }

                style {
                    r#"
                    #projects .scroll-container::-webkit-scrollbar {
                        height: 8px;
                    }
                    #projects .scroll-container::-webkit-scrollbar-track {
                        background: #f0f0f0;
                        border-radius: 4px;
                    }
                    #projects .scroll-container::-webkit-scrollbar-thumb {
                        background: #d4af37;
                        border-radius: 4px;
                    }
                    #projects .scroll-container::-webkit-scrollbar-thumb:hover {
                        background: #b8960f;
                    }
                    @keyframes modalFadeIn {
                        from { opacity: 0; transform: scale(0.95) translateY(10px); }
                        to { opacity: 1; transform: scale(1) translateY(0); }
                    }
                    "#
                }
            }
        }
    }
}
