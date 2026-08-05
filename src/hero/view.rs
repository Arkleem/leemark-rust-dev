use maud::{html, Markup};

pub fn hero_section() -> Markup {
    html! {
        link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap";

        section
            id="home"
            class="min-h-screen flex items-center py-6 sm:py-12 relative overflow-hidden overflow-y-auto"
            style="background: linear-gradient(to bottom, #ffffff, #f8f8f8); font-family: 'EB Garamond', 'Garamond', 'Times New Roman', serif;"
            data-signals="{showTech: ''}"
        {
            // Decorative background elements
            div class="absolute top-0 right-0 w-[200px] sm:w-[400px] md:w-[600px] h-[200px] sm:h-[400px] md:h-[600px] rounded-full blur-3xl" style="background: rgba(212, 175, 55, 0.05);" {}
            div class="absolute bottom-0 left-0 w-[150px] sm:w-[300px] md:w-[400px] h-[150px] sm:h-[300px] md:h-[400px] rounded-full blur-3xl" style="background: rgba(212, 175, 55, 0.05);" {}
            div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[300px] sm:w-[600px] md:w-[800px] h-[300px] sm:h-[600px] md:h-[800px] border rounded-full" style="border-color: rgba(212, 175, 55, 0.08);" {}
            div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[200px] sm:w-[450px] md:w-[600px] h-[200px] sm:h-[450px] md:h-[600px] border rounded-full" style="border-color: rgba(212, 175, 55, 0.05);" {}

            div class="container mx-auto px-4 md:px-100 relative z-10" {
                div class="flex flex-col lg:flex-row-reverse items-center justify-center gap-6 md:gap-12 lg:gap-20 min-h-[calc(100vh-60px)]" {

                    // Profile Image
                    div class="flex-shrink-0 order-first lg:order-last" {
                        div class="relative" {
                            div class="relative w-40 h-40 sm:w-48 sm:h-48 md:w-56 md:h-56 lg:w-64 lg:h-64" {
                                img src="/static/rustdev.png" alt="Leemark Arojo - Profile Picture" class="w-full h-full object-cover rounded-xl shadow-lg";
                            }
                        }
                    }

                    // Left content
                    div class="flex-1 text-center lg:text-left px-4 sm:px-6" {
                        // Availability badge
                        div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full mb-4" style="background: rgba(34, 197, 94, 0.1); border: 1px solid rgba(34, 197, 94, 0.2);" {
                            span class="w-2 h-2 rounded-full bg-green-500 animate-pulse" {}
                            span class="text-xs sm:text-sm font-medium" style="color: #22c55e;" { "Available for work" }
                        }

                        // Heading
                        h1 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold mb-3 sm:mb-4 leading-relaxed" style="color: #1a1a1a; font-family: 'Press Start 2P', monospace;" {
                            "Hi,I'm "
                            span style="background: linear-gradient(to right, #d4af37, #b8960f); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;" { "Leemark" }
                        }

                        // Subtitle
                        div class="flex items-center gap-3 mb-3 justify-center lg:justify-start" {
                            div class="w-8 sm:w-10 h-0.5 rounded-full" style="background: #d4af37;" {}
                            p class="text-lg sm:text-xl md:text-2x2 font-medium" style="color: #333333;" {
                                "Rust Full-Stack Developer"
                            }
                            div class="w-8 sm:w-10 h-0.5 rounded-full" style="background: #d4af37;" {}
                        }

                        // Description
                        p class="text-base sm:text-lg max-w-lg mx-auto lg:mx-0 leading-relaxed mb-5" style="color: #4a4a4a;" {
                            "I build server-rendered web applications and backend systems in "

                            span
                                data-on:click="$showTech = 'rust'"
                                style="color: #d4af37; text-decoration: underline; cursor: pointer; font-weight: 500; display: inline;"
                            { "Rust" }

                            " using "

                            span
                                data-on:click="$showTech = 'axum'"
                                style="color: #d4af37; text-decoration: underline; cursor: pointer; font-weight: 500; display: inline;"
                            { "Axum" }

                            ", "

                            span
                                data-on:click="$showTech = 'maud'"
                                style="color: #d4af37; text-decoration: underline; cursor: pointer; font-weight: 500; display: inline;"
                            { "Maud" }

                            ", "

                            span
                                data-on:click="$showTech = 'datastar'"
                                style="color: #d4af37; text-decoration: underline; cursor: pointer; font-weight: 500; display: inline;"
                            { "Datastar" }

                            ", and "

                            span
                                data-on:click="$showTech = 'surrealdb'"
                                style="color: #d4af37; text-decoration: underline; cursor: pointer; font-weight: 500; display: inline;"
                            { "SurrealDB" }

                            ", focusing on performance, maintainability, and clean architecture."
                        }

                        // Description Box
                        div style="position: relative;" {
                            div
                                data-show="$showTech != ''"
                                style="
                                    display: none;
                                    position: absolute;
                                    top: 100%;
                                    left: 0;
                                    margin-top: 15px;
                                    padding: 16px 20px;
                                    border-radius: 12px;
                                    background: rgba(26, 26, 26, 0.95);
                                    border: 1px solid rgba(212, 175, 55, 0.3);
                                    max-width: 500px;
                                    width: 100%;
                                    text-align: left;
                                    transition: all 0.3s ease;
                                    z-index: 100;
                                    backdrop-filter: blur(10px);
                                    box-shadow: 0 10px 40px rgba(0,0,0,0.2);
                                    color: #f0f0f0;
                                "
                            {
                                button
                                    data-on:click="$showTech = ''"
                                    style="
                                        position: absolute;
                                        top: 8px;
                                        right: 12px;
                                        color: #94a3b8;
                                        font-size: 16px;
                                        cursor: pointer;
                                        background: none;
                                        border: none;
                                        padding: 0;
                                        font-family: inherit;
                                    "
                                { "✕" }

                                div data-show="$showTech == 'rust'" {
                                    div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;" {
                                        strong style="color: #d4af37; font-size: 16px;" { "Rust" }
                                    }
                                    p style="color: #d4af37; font-size: 13px; margin: 0 0 8px 0;" {
                                        "A systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."
                                    }
                                    a href="https://www.rust-lang.org/" target="_blank" rel="noopener" style="color: #d4af37; font-size: 12px; text-decoration: underline;"
                                    { "https://www.rust-lang.org/" }
                                }

                                div data-show="$showTech == 'axum'" {
                                    div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;" {
                                        strong style="color: #d4af37; font-size: 16px;" { "Axum" }
                                    }
                                    p style="color: #d4af37; font-size: 13px; margin: 0 0 8px 0;" {
                                        "A web application framework for Rust that focuses on ergonomics and modularity."
                                    }
                                    a href="https://docs.rs/axum/latest/axum/" target="_blank" rel="noopener" style="color: #d4af37; font-size: 12px; text-decoration: underline;"
                                    { "https://docs.rs/axum/latest/axum/" }
                                }

                                div data-show="$showTech == 'maud'" {
                                    div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;" {
                                        strong style="color: #d4af37; font-size: 16px;" { "Maud" }
                                    }
                                    p style="color: #d4af37; font-size: 13px; margin: 0 0 8px 0;" {
                                        "A compile-time HTML template engine for Rust. Write HTML directly in your Rust code with type safety."
                                    }
                                    a href="https://docs.rs/maud/latest/maud/" target="_blank" rel="noopener" style="color: #d4af37; font-size: 12px; text-decoration: underline;"
                                    { "https://docs.rs/maud/latest/maud/" }
                                }

                                div data-show="$showTech == 'datastar'" {
                                    div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;" {
                                        strong style="color: #d4af37; font-size: 16px;" { "Datastar" }
                                    }
                                    p style="color: #d4af37; font-size: 13px; margin: 0 0 8px 0;" {
                                        "A hypermedia framework for building reactive web applications. Uses Server-Sent Events for real-time updates."
                                    }
                                    a href="https://data-star.dev/" target="_blank" rel="noopener" style="color: #d4af37; font-size: 12px; text-decoration: underline;"
                                    { "https://data-star.dev/" }
                                }

                                div data-show="$showTech == 'surrealdb'" {
                                    div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;" {
                                        strong style="color: #d4af37; font-size: 16px;" { "SurrealDB" }
                                    }
                                    p style="color: #d4af37; font-size: 13px; margin: 0 0 8px 0;" {
                                        "A scalable, distributed, and collaborative document-database for the modern web."
                                    }
                                    a href="https://surrealdb.com/" target="_blank" rel="noopener" style="color: #d4af37; font-size: 12px; text-decoration: underline;"
                                    { "https://surrealdb.com/" }
                                }
                            }
                        }

                        // CTA Buttons - SMALLER AND INLINE
                        div class="flex flex-wrap gap-2 justify-center lg:justify-start mb-5" {
                            a href="#projects" class="px-4 py-2 font-medium rounded-lg transition-all duration-300 hover:-translate-y-0.5 hover:shadow-lg flex items-center gap-1.5 text-xs sm:text-sm whitespace-nowrap" style="background: linear-gradient(to right, #d4af37, #b8960f); color: #ffffff;" {
                                i class="fas fa-code text-xs" {}
                                "View Projects"
                            }
                            a href="#contact" class="px-4 py-2 font-medium rounded-lg transition-all duration-300 hover:-translate-y-0.5 hover:shadow-lg flex items-center gap-1.5 text-xs sm:text-sm whitespace-nowrap" style="border: 2px solid #d4af37; color: #d4af37; background: transparent;" {
                                i class="fas fa-paper-plane text-xs" {}
                                "Contact Me"
                            }
                        }

                        // Social links
                        div class="flex gap-4 justify-center lg:justify-start" {
                            a href="https://github.com/Arkleem" target="_blank" rel="noopener" class="w-10 h-10 sm:w-12 sm:h-12 rounded-full flex items-center justify-center text-base sm:text-xl transition-all duration-300 hover:-translate-y-1" style="background: #f0f0f0; border: 1px solid #e0e0e0; color: #4a4a4a;" {
                                i class="fab fa-github" {}
                            }
                            a href="https://www.linkedin.com/in/leemarkarojo/" target="_blank" rel="noopener" class="w-10 h-10 sm:w-12 sm:h-12 rounded-full flex items-center justify-center text-base sm:text-xl transition-all duration-300 hover:-translate-y-1" style="background: #f0f0f0; border: 1px solid #e0e0e0; color: #4a4a4a;" {
                                i class="fab fa-linkedin" {}
                            }
                            a href="https://www.facebook.com/krameel28/" target="_blank" rel="noopener" class="w-10 h-10 sm:w-12 sm:h-12 rounded-full flex items-center justify-center text-base sm:text-xl transition-all duration-300 hover:-translate-y-1" style="background: #f0f0f0; border: 1px solid #e0e0e0; color: #4a4a4a;" {
                                i class="fab fa-facebook" {}
                            }
                        }
                    }
                }
            }

            style {
                r#"
                @keyframes spin-slow {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
                @keyframes spin-slower {
                    from { transform: rotate(360deg); }
                    to { transform: rotate(0deg); }
                }
                .animate-spin-slow {
                    animation: spin-slow 20s linear infinite;
                }
                .animate-spin-slower {
                    animation: spin-slower 25s linear infinite;
                }
                "#
            }
        }
    }
}
