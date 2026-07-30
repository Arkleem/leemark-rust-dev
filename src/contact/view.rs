use maud::{html, Markup};

pub fn contact_section() -> Markup {
    html! {
        section id="contact" class="py-20" style="background-color: #ffffff; font-family: 'EB Garamond', 'Garamond', 'Times New Roman', serif;" {
            div class="container mx-auto px-4 max-w-4xl" {
                div class="text-center mb-12" {
                    h2 class="text-3xl md:text-4xl font-bold mb-4" style="color: #1a1a1a;" { "Get In Touch" }
                    p class="text-lg max-w-2xl mx-auto" style="color: #4a4a4a; font-style: italic;" {
                        "Have a question or want to work together? Feel free to reach out!"
                    }
                    div class="w-20 h-1 mx-auto rounded-full mt-4" style="background: linear-gradient(to right, #d4af37, #b8960f);" {}
                }

                div class="grid md:grid-cols-2 gap-8" {
                    // Left side - Contact Info
                    div class="space-y-6" {
                        div class="flex items-start gap-4 p-4 rounded-lg transition-colors" style="background-color: #f8f8f8; border: 1px solid #e8e8e8;" {
                            div class="w-12 h-12 rounded-full flex items-center justify-center text-xl flex-shrink-0" style="background-color: rgba(212, 175, 55, 0.1); color: #d4af37;" {
                                i class="fas fa-envelope" {}
                            }
                            div {
                                h4 class="font-medium" style="color: #1a1a1a;" { "Email" }
                                a href="mailto:leemarkarojo7@gmail.com" class="transition-colors" style="color: #4a4a4a;" {
                                    "leemarkarojo7@gmail.com"
                                }
                            }
                        }
                        div class="flex items-start gap-4 p-4 rounded-lg transition-colors" style="background-color: #f8f8f8; border: 1px solid #e8e8e8;" {
                            div class="w-12 h-12 rounded-full flex items-center justify-center text-xl flex-shrink-0" style="background-color: rgba(212, 175, 55, 0.1); color: #d4af37;" {
                                i class="fas fa-phone" {}
                            }
                            div {
                                h4 class="font-medium" style="color: #1a1a1a;" { "Phone" }
                                span style="color: #4a4a4a;" { "+63 909 683 0094" }
                            }
                        }
                        div class="flex items-start gap-4 p-4 rounded-lg transition-colors" style="background-color: #f8f8f8; border: 1px solid #e8e8e8;" {
                            div class="w-12 h-12 rounded-full flex items-center justify-center text-xl flex-shrink-0" style="background-color: rgba(212, 175, 55, 0.1); color: #d4af37;" {
                                i class="fas fa-map-marker-alt" {}
                            }
                            div {
                                h4 class="font-medium" style="color: #1a1a1a;" { "Location" }
                                span style="color: #4a4a4a;" { "Philippines" }
                            }
                        }
                    }

                    // Right side - Form (standard HTML form - NO JavaScript)
                    div class="rounded-xl p-6 border" style="background-color: #f8f8f8; border-color: #e8e8e8;" {
                        form
                            action="/contact"
                            method="post"
                            class="space-y-4"
                        {
                            div {
                                label class="block text-sm font-medium mb-1" style="color: #1a1a1a;" for="name" {
                                    "Name"
                                }
                                input
                                    type="text"
                                    id="name"
                                    name="name"
                                    required
                                    placeholder="Your name"
                                    class="w-full px-4 py-2 rounded-lg border transition-all focus:outline-none focus:ring-2"
                                    style="background-color: #ffffff; border-color: #e0e0e0; color: #1a1a1a; font-family: 'EB Garamond', 'Garamond', serif;"
                                {}
                            }
                            div {
                                label class="block text-sm font-medium mb-1" style="color: #1a1a1a;" for="email" {
                                    "Email"
                                }
                                input
                                    type="email"
                                    id="email"
                                    name="email"
                                    required
                                    placeholder="your@email.com"
                                    class="w-full px-4 py-2 rounded-lg border transition-all focus:outline-none focus:ring-2"
                                    style="background-color: #ffffff; border-color: #e0e0e0; color: #1a1a1a; font-family: 'EB Garamond', 'Garamond', serif;"
                                {}
                            }
                            div {
                                label class="block text-sm font-medium mb-1" style="color: #1a1a1a;" for="message" {
                                    "Message"
                                }
                                textarea
                                    id="message"
                                    name="message"
                                    rows="4"
                                    required
                                    placeholder="Your message..."
                                    class="w-full px-4 py-2 rounded-lg border transition-all focus:outline-none focus:ring-2 resize-y min-h-[100px]"
                                    style="background-color: #ffffff; border-color: #e0e0e0; color: #1a1a1a; font-family: 'EB Garamond', 'Garamond', serif;"
                                {}
                            }
                            button
                                type="submit"
                                class="w-full py-3 font-semibold rounded-lg transition-all hover:-translate-y-0.5 hover:shadow-lg"
                                style="background: linear-gradient(to right, #d4af37, #b8960f); color: #ffffff; font-family: 'EB Garamond', 'Garamond', serif;"
                            {
                                i class="fas fa-paper-plane mr-2" {}
                                " Send Message"
                            }
                        }
                    }
                }
            }
        }

        // Native HTML dialog - shows when page reloads with message param
        @if let Some(message) = std::env::var("CONTACT_MESSAGE").ok() {
            div style="
                position: fixed;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                z-index: 10000;
                padding: 35px 45px;
                border-radius: 16px;
                max-width: 450px;
                width: 90%;
                text-align: center;
                font-family: 'EB Garamond', 'Garamond', serif;
                background: #f0faf0;
                border: 2px solid #b3ffb3;
                box-shadow: 0 25px 80px rgba(0,0,0,0.4);
                animation: popupFadeIn 0.3s ease;
            " {
                div style="font-size: 52px; margin-bottom: 12px;" { "✅" }
                h3 style="color: #2d7a2d; font-size: 1.5rem; margin: 0 0 8px 0; font-weight: 700;" { "Message Sent!" }
                p style="color: #2d7a2d; font-size: 1rem; margin: 0 0 25px 0; line-height: 1.6;" { (message) }
                a
                    href="/#contact"
                    style="
                        display: inline-block;
                        padding: 10px 40px;
                        background: linear-gradient(to right, #d4af37, #b8960f);
                        color: white;
                        border: none;
                        border-radius: 30px;
                        font-size: 1rem;
                        cursor: pointer;
                        font-weight: 600;
                        text-decoration: none;
                        font-family: 'EB Garamond', 'Garamond', serif;
                        letter-spacing: 0.5px;
                        transition: all 0.3s ease;
                    "
                {
                    "Close"
                }
            }
            div style="
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                z-index: 9999;
                background: rgba(0, 0, 0, 0.5);
                backdrop-filter: blur(6px);
            " {}
        }

        style {
            r#"
            @keyframes popupFadeIn {
                from {
                    opacity: 0;
                    transform: translate(-50%, -50%) scale(0.9);
                }
                to {
                    opacity: 1;
                    transform: translate(-50%, -50%) scale(1);
                }
            }
            "#
        }
    }
}
