use maud::{html, Markup};

pub fn summary_section() -> Markup {
    html! {
        section id="summary" style="padding: 5rem 0; background-color: #ffffff; font-family: 'EB Garamond', 'Garamond', 'Times New Roman', serif;" {
            div class="container mx-auto px-4 max-w-6xl" {
                div style="text-align: center; margin-bottom: 2.5rem;" {
                    h2 style="font-size: 2.5rem; margin-bottom: 0.5rem; color: #1a1a1a;" { "About Me" }
                    div style="width: 80px; height: 3px; background: linear-gradient(to right, #d4af37, #b8960f); border-radius: 9999px; margin: 1rem auto;" {}
                }

                article style="max-width: 1000px; margin: 0 auto; padding: 2.5rem; background: #f8f8f8; border-radius: 12px; border: 1px solid #e8e8e8;" {
                    div style="display: flex; flex-direction: column; gap: 1.25rem;" {
                        p style="font-size: 1.05rem; line-height: 1.8; color: #333333; text-align: justify; margin: 0;" {
                            "Computer Science graduate specializing in Rust development and reliable software systems. Skilled in building backend services, REST APIs, database-driven applications, and server-rendered web applications using Rust, Axum, Tokio, Maud, Datastar, and SurrealDB. Experienced in designing clean, maintainable software architectures with a focus on performance, scalability, and asynchronous programming. Proficient with Linux, and modern development workflows. Passionate about backend engineering and eager to contribute to building high-quality software solutions while continuously learning and growing as a software engineer."
                        }
                        p style="font-size: 1.05rem; line-height: 1.8; color: #333333; text-align: justify; margin: 0;" {
                            "Focused on clean architecture, performance optimization, and developing maintainable software solutions."
                        }

                        // Availability Badges + Freelance in one row
                        div style="
                            display: flex;
                            flex-wrap: wrap;
                            justify-content: center;
                            align-items: center;
                            gap: 8px;
                            margin: 5px 0 10px 0;
                        " {
                            // Full-time badge
                            span style="
                                background: rgba(212, 175, 55, 0.12);
                                border: 1px solid #d4af37;
                                padding: 4px 14px;
                                border-radius: 20px;
                                font-size: 0.75rem;
                                font-weight: 500;
                                color: #d4af37;
                            " { "Full-time" }

                            // Remote badge
                            span style="
                                background: rgba(34, 197, 94, 0.1);
                                border: 1px solid #22c55e;
                                padding: 4px 14px;
                                border-radius: 20px;
                                font-size: 0.75rem;
                                font-weight: 500;
                                color: #22c55e;
                            " { "Remote" }

                            // Part-time badge
                            span style="
                                background: rgba(59, 130, 246, 0.1);
                                border: 1px solid #3b82f6;
                                padding: 4px 14px;
                                border-radius: 20px;
                                font-size: 0.75rem;
                                font-weight: 500;
                                color: #3b82f6;
                            " { "Part-time" }

                            // Separator dot
                            span style="color: #d4af37; font-size: 0.5rem;" { "•" }

                            // Open for Freelance Work (gold)
                            span style="
                                background: rgba(212, 175, 55, 0.12);
                                border: 1px solid #d4af37;
                                padding: 4px 14px;
                                border-radius: 20px;
                                font-size: 0.75rem;
                                font-weight: 500;
                                color: #d4af37;
                            " { "Freelance" }

                            // Separator dot
                            span style="color: #d4af37; font-size: 0.5rem;" { "•" }

                            // Available for Employment (green)
                            span style="
                                background: rgba(34, 197, 94, 0.1);
                                border: 1px solid #22c55e;
                                padding: 4px 14px;
                                border-radius: 20px;
                                font-size: 0.75rem;
                                font-weight: 500;
                                color: #22c55e;
                            " { "Available for Employment" }
                        }

                        // --- SERVICES SECTION ---
                        div style="margin-top: 15px;" {
                            // Clickable "Services" heading - smaller
                            div
                                data-on:click="$showServices = !$showServices"
                                style="
                                    font-size: 1rem;
                                    color: #d4af37;
                                    text-align: center;
                                    font-weight: 500;
                                    cursor: pointer;
                                    display: inline-block;
                                    padding: 5px 18px;
                                    border: 1.5px solid #d4af37;
                                    border-radius: 20px;
                                    background: rgba(212, 175, 55, 0.05);
                                    transition: all 0.3s ease;
                                    margin: 0 auto 12px auto;
                                    width: auto;
                                "
                            {
                                "Click to View Services"
                            }

                            // Service Tags (hidden by default, shown when clicked)
                            div
                                data-show="$showServices == true"
                                style="
                                    display: none;
                                    margin-top: 12px;
                                    padding: 16px 18px;
                                    background: white;
                                    border-radius: 10px;
                                    border: 1px solid #e8e8e8;
                                "
                            {
                                // Server-Rendered Web Apps
                                div style="margin-bottom: 10px;" {
                                    div style="font-weight: 500; color: #d4af37; font-size: 0.85rem; margin-bottom: 5px;" { "Server-Rendered Web Apps" }
                                    div style="display: flex; flex-wrap: wrap; gap: 5px;" {
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Dashboards" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Admin Panels" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "CMS" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Inventory" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "HR Systems" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Booking" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "School Mgmt" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "CRM" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Analytics" }
                                    }
                                }

                                // Mobile Applications
                                div style="margin-bottom: 10px;" {
                                    div style="font-weight: 500; color: #d4af37; font-size: 0.85rem; margin-bottom: 5px;" { "Mobile Applications" }
                                    div style="display: flex; flex-wrap: wrap; gap: 5px;" {
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Android" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "iOS" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Offline-first" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Business Apps" }
                                    }
                                }

                                // Desktop Applications
                                div style="margin-bottom: 10px;" {
                                    div style="font-weight: 500; color: #d4af37; font-size: 0.85rem; margin-bottom: 5px;" { "Desktop Applications" }
                                    div style="display: flex; flex-wrap: wrap; gap: 5px;" {
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Cross-platform" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "POS Systems" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "File Tools" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Offline Apps" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "AI Clients" }
                                    }
                                }

                                // Linux & Server Deployment
                                div style="margin-bottom: 10px;" {
                                    div style="font-weight: 500; color: #d4af37; font-size: 0.85rem; margin-bottom: 5px;" { "Linux & Server Deployment" }
                                    div style="display: flex; flex-wrap: wrap; gap: 5px;" {
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Server Setup" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "VPS Deployment" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Docker" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "CI/CD" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Nginx" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Monitoring" }
                                    }
                                }

                                // Business Software
                                div style="margin-bottom: 10px;" {
                                    div style="font-weight: 500; color: #d4af37; font-size: 0.85rem; margin-bottom: 5px;" { "Business Software" }
                                    div style="display: flex; flex-wrap: wrap; gap: 5px;" {
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "ERP" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Project Mgmt" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Document Mgmt" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Asset Mgmt" }
                                        span style="background: #f0f0f0; padding: 3px 10px; border-radius: 16px; font-size: 0.72rem; color: #4a4a4a;" { "Attendance" }
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
