use maud::{html, Markup, DOCTYPE, PreEscaped};
use crate::components::starfield::starfield;

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                link rel="icon" type="image/png" href="/static/rustdev.png" {}
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" {}

                link rel="stylesheet" href="/static/encre.css";
                script type="module" src="/static/datastar.js" {}

                style {
                    r#"
                    #progress-container {
                        position: fixed;
                        bottom: 30px;
                        left: 30px;
                        z-index: 999;
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                        align-items: center;
                        background: transparent;
                        padding: 10px;
                        border-radius: 16px;
                    }
                    #progress-container svg {
                        width: 70px;
                        height: 70px;
                        display: block;
                    }
                    main {
                        position: relative;
                        z-index: 1;
                    }
                    particle-network {
                        display: block;
                        position: fixed;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        z-index: 0;
                        pointer-events: none;
                    }
                    #mouse-tracker {
                        position: fixed;
                        top: 20px;
                        right: 20px;
                        z-index: 1000;
                        font-family: 'Courier New', monospace;
                        font-size: 13px;
                        color: #d4af37;
                        pointer-events: none;
                        text-shadow: 0 0 20px rgba(212, 175, 55, 0.3);
                    }
                    #mouse-tracker .value {
                        color: #d4af37;
                        font-weight: bold;
                    }
                    #mouse-tracker .label {
                        color: #d4af37;
                        font-size: 12px;
                        font-weight: 500;
                    }
                    #mouse-tracker .row {
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        gap: 12px;
                    }
                    #mouse-tracker .row + .row {
                        margin-top: 4px;
                    }
                    "#
                }
            }
            body
                data-signals="{scrollPct: 0, starX: 85, starY: 34, starSpeed: 94, particles: 150, particleSpeed: 0.3, particleColor: '#d4af37', mouseX: 50, mouseY: 50}"
                data-on:scroll__window="
                    const maxScroll = document.documentElement.scrollHeight - window.innerHeight;
                    scrollPct = maxScroll > 0 ? Math.round((window.scrollY / maxScroll) * 100) : 0;
                    if (scrollPct < 0) scrollPct = 0;
                    if (scrollPct > 100) scrollPct = 100;
                    @get('/scroll-progress?pct=' + scrollPct)
                "
                // Mouse events for desktop
                data-on:mousemove__window=(PreEscaped(r#"
                    $mouseX = Math.round((event.clientX / window.innerWidth) * 100);
                    $mouseY = Math.round((event.clientY / window.innerHeight) * 100);
                "#))
                // Touch events for mobile
                data-on:touchmove__window=(PreEscaped(r#"
                    const touch = event.touches[0];
                    if (touch) {
                        $mouseX = Math.round((touch.clientX / window.innerWidth) * 100);
                        $mouseY = Math.round((touch.clientY / window.innerHeight) * 100);
                    }
                "#))
                data-on:touchstart__window=(PreEscaped(r#"
                    const touch = event.touches[0];
                    if (touch) {
                        $mouseX = Math.round((touch.clientX / window.innerWidth) * 100);
                        $mouseY = Math.round((touch.clientY / window.innerHeight) * 100);
                    }
                "#))
            {
                // ⭐ Mouse Tracker - Gold Theme (Transparent)
                div id="mouse-tracker" {
                    div class="row" {
                        span class="label" { "X:" }
                        span class="value" data-text="$mouseX" { "50" }
                    }
                    div class="row" {
                        span class="label" { "Y:" }
                        span class="value" data-text="$mouseY" { "50" }
                    }
                }

                // ⭐ Particle Network Background
                div data-signals="{particles: 150, color: '#d4af37', speed: 0.3}" {
                    particle-network
                        data-attr:count="$particles"
                        data-attr:color="$color"
                        data-attr:speed="$speed"
                        data-attr:connect-distance="150"
                        style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; z-index: 0; pointer-events: none;"
                    {}
                }

                // ⭐ Starfield background
                (starfield())

                // Progress bar
                div id="progress-container" {
                    div
                        id="progress-bar"
                        data-init="@get('/scroll-progress')"
                    {
                        svg
                            width="70"
                            height="70"
                            viewBox="-25 -25 250 250"
                            style="transform: rotate(-90deg)"
                        {
                            circle
                                r="90"
                                cx="100"
                                cy="100"
                                fill="transparent"
                                stroke="rgba(255,255,255,0.15)"
                                stroke-width="16px"
                                stroke-dasharray="565.48px"
                                stroke-dashoffset="0px"
                            {}
                            circle
                                id="progress-circle"
                                r="90"
                                cx="100"
                                cy="100"
                                fill="transparent"
                                stroke="#d4af37"
                                stroke-width="16px"
                                stroke-linecap="round"
                                stroke-dasharray="565.48px"
                                stroke-dashoffset="565.48px"
                            {}
                            text
                                id="progress-text"
                                x="44px"
                                y="115px"
                                fill="#d4af37"
                                font-size="52px"
                                font-weight="bold"
                                style="transform: rotate(90deg) translate(0px, -196px); font-family: 'EB Garamond', 'Garamond', serif;"
                            { "0%" }
                        }
                    }
                }

                main {
                    (content)
                }
            }
        }
    }
}
