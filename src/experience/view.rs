use maud::{html, Markup};
use crate::state::AppState;

pub fn experience_section(state: &AppState) -> Markup {
    html! {
        section id="experience" class="py-20" style="background-color: #ffffff; font-family: 'EB Garamond', 'Garamond', 'Times New Roman', serif;" {
            div class="container mx-auto px-4 max-w-4xl" {
                div class="text-center mb-12" {
                    h2 class="text-3xl md:text-4xl font-bold mb-4" style="color: #1a1a1a;" { "Experience & Education" }
                    p class="text-lg max-w-2xl mx-auto" style="color: #4a4a4a; font-style: italic;" {
                        "My professional journey and educational background"
                    }
                    div class="w-20 h-1 mx-auto rounded-full mt-4" style="background: linear-gradient(to right, #d4af37, #b8960f);" {}
                }

                // Work Experience
                div class="mb-12" {
                    div class="flex items-center gap-3 mb-6" {
                        div class="w-1 h-8 rounded-full" style="background: linear-gradient(to bottom, #d4af37, #b8960f);" {}
                        h3 class="text-2xl font-bold flex items-center gap-2" style="color: #1a1a1a;" {
                            i class="fas fa-briefcase" style="color: #d4af37;" {}
                            "Work Experience"
                        }
                    }

                    div class="space-y-4" {
                        @for exp in &state.experiences {
                            div class="rounded-xl border transition-all duration-300 hover:shadow-lg" style="background-color: #f8f8f8; border-color: #e8e8e8;" {
                                div class="p-6" {
                                    div class="flex flex-wrap justify-between items-start gap-4" {
                                        div {
                                            h4 class="text-xl font-bold" style="color: #1a1a1a;" { (exp.company) }
                                            p class="font-medium" style="color: #d4af37;" { (exp.role) }
                                        }
                                        span class="text-sm whitespace-nowrap px-3 py-1 rounded-full" style="color: #4a4a4a; background-color: #e8e8e8;" {
                                            i class="fas fa-calendar-alt mr-1" style="color: #d4af37;" {} (exp.period)
                                        }
                                    }

                                    div class="mt-3" {
                                        p class="leading-relaxed whitespace-pre-line" style="color: #333333; font-family: 'EB Garamond', 'Garamond', serif;" {
                                            (exp.description)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Education
                div {
                    div class="flex items-center gap-3 mb-6" {
                        div class="w-1 h-8 rounded-full" style="background: linear-gradient(to bottom, #d4af37, #b8960f);" {}
                        h3 class="text-2xl font-bold flex items-center gap-2" style="color: #1a1a1a;" {
                            i class="fas fa-graduation-cap" style="color: #d4af37;" {}
                            "Education"
                        }
                    }

                    div class="space-y-4" {
                        @for edu in &state.education {
                            div class="rounded-xl border transition-all duration-300 hover:shadow-lg" style="background-color: #f8f8f8; border-color: #e8e8e8;" {
                                div class="p-6" {
                                    div class="flex flex-wrap justify-between items-start gap-4" {
                                        div {
                                            h4 class="text-xl font-bold" style="color: #1a1a1a;" { (edu.school) }
                                            p class="font-medium" style="color: #d4af37;" { (edu.degree) }
                                            @if let Some(major) = &edu.major {
                                                p class="text-sm" style="color: #4a4a4a;" {
                                                    "Major in " (major)
                                                }
                                            }
                                            @if let Some(achievement) = &edu.achievement {
                                                p class="text-sm font-semibold" style="color: #d4af37;" {
                                                    (achievement)
                                                }
                                            }
                                        }
                                        span class="text-sm whitespace-nowrap px-3 py-1 rounded-full" style="color: #4a4a4a; background-color: #e8e8e8;" {
                                            i class="fas fa-calendar-alt mr-1" style="color: #d4af37;" {} (edu.period)
                                        }
                                    }

                                    div class="mt-3" {
                                        p class="text-sm flex items-center gap-2" style="color: #4a4a4a;" {
                                            i class="fas fa-map-marker-alt" style="color: #d4af37;" {}
                                            (edu.location)
                                        }
                                    }

                                    // Coursework
                                    @if let Some(coursework) = &edu.coursework {
                                        div class="mt-3" {
                                            p class="text-sm font-medium" style="color: #1a1a1a;" { "Relevant Coursework:" }
                                            div class="flex flex-wrap gap-2 mt-1" {
                                                @for course in coursework {
                                                    span class="text-xs px-2 py-1 rounded-full" style="background-color: #e8e8e8; color: #4a4a4a;" {
                                                        (course)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
