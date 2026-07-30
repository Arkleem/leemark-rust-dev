use maud::{html, Markup};
use crate::state::AppState;

pub fn skills_section(state: &AppState) -> Markup {
    html! {
        section id="skills" class="py-20" style="background-color: #ffffff; font-family: 'EB Garamond', 'Garamond', 'Times New Roman', serif;" {
            div class="container mx-auto px-4 max-w-5xl" {
                div class="text-center mb-12" {
                    h2 class="text-3xl md:text-4xl font-bold" style="color: #1a1a1a;" { "Tech Stack" }
                    div class="w-20 h-1 mx-auto rounded-full mt-4" style="background: linear-gradient(to right, #d4af37, #b8960f);" {}
                    p class="text-lg mt-4" style="color: #4a4a4a; font-style: italic;" {
                        "Technologies I work with"
                    }
                }

                // All skills in one flow
                div class="flex flex-wrap justify-center gap-2" {
                    @for skill in &state.skills {
                        span class="px-4 py-2 rounded-full text-sm flex items-center gap-2 transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            i class=(skill.icon) style="color: #d4af37;" {} (skill.name)
                        }
                    }
                }

                // Tools
                div class="mt-12 pt-8 border-t" style="border-color: #e0e0e0;" {
                    h4 class="text-sm font-semibold uppercase tracking-wider text-center mb-3" style="color: #d4af37; letter-spacing: 2px;" {
                        "Tools & Platforms"
                    }
                    div class="flex flex-wrap justify-center gap-2" {
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Git"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "GitHub"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Cargo"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Linux VPS"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Zed"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Railway"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Podman"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "DigitalOcean"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Render"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Shuttle"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Cloudflare"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Tauri"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "Gemini"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "DeepSeek"
                        }
                        span class="px-4 py-2 rounded-full text-sm transition-all duration-300 hover:-translate-y-1 hover:shadow-md" style="background-color: #f5f5f5; color: #1a1a1a; border: 1px solid #e0e0e0; font-family: 'EB Garamond', 'Garamond', serif;" {
                            "AI Code Agents"
                        }
                    }
                }
            }
        }
    }
}
