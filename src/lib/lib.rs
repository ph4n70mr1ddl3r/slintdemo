//! Slint Showcase Library
//! Main entry point for the Slint UI components

pub mod components;
pub mod data;

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, GridBox, TextEdit, Slider, CheckBox } from "std-widgets.slint";

    export component CategoryCard {
        in property <string> id;
        in property <string> name;
        in property <string> description;
        in property <string> icon;
        in-out property <bool> selected;

        callback clicked(string);

        Rectangle {
            background: selected ? rgb(37, 99, 235) : rgb(255, 255, 255);
            border-radius: 8px;
            padding: 16px;

            VerticalBox {
                spacing: 8px;

                Text {
                    text: icon;
                    font-size: 32px;
                    horizontal-alignment: center;
                }

                Text {
                    text: name;
                    color: selected ? #ffffff : #000000;
                    font-size: 18px;
                    font-weight: 700;
                    horizontal-alignment: center;
                }

                Text {
                    text: description;
                    color: selected ? #ffffff : #666666;
                    font-size: 14px;
                    horizontal-alignment: center;
                }
            }

            TouchArea {
                clicked => { clicked(id); }
            }
        }
    }

    export component CapabilityCard {
        in property <string> id;
        in property <string> title;
        in property <string> description;
        in property <string> difficulty;
        in property <int> estimated-minutes;

        callback clicked(string);

        Rectangle {
            background: rgb(255, 255, 255);
            border-radius: 8px;
            padding: 16px;
            border-width: 1px;
            border-color: rgb(200, 200, 200);

            VerticalBox {
                spacing: 8px;

                HorizontalBox {
                    spacing: 8px;

                    Text {
                        text: title;
                        font-size: 16px;
                        font-weight: 700;
                    }

                    Rectangle {
                        horizontal-stretch: 1;
                    }

                    Text {
                        text: difficulty == "beginner" ? "🟢" : (difficulty == "intermediate" ? "🟡" : "🔴");
                        font-size: 12px;
                    }
                }

                Text {
                    text: description;
                    font-size: 14px;
                    color: #666666;
                }

                HorizontalBox {
                    spacing: 4px;

                    Text {
                        text: "⏱️ " + estimated-minutes + " min";
                        font-size: 12px;
                        color: #888888;
                    }
                }
            }

            TouchArea {
                clicked => { clicked(id); }
            }
        }
    }

    export component App {
        in property <string> title: "Slint 1.14 Web Capabilities";
        in property <string> version: "1.14.0";

        in-out property <string> current-page: "home";
        in-out property <string> selected-demo: "";
        in-out property <string> selected-category: "";

        callback navigate-to-page(string);

        Rectangle {
            background: rgb(245, 245, 245);

            VerticalBox {
                spacing: 16px;
                padding: 24px;

                Rectangle {
                    height: 60px;
                    background: rgb(37, 99, 235);

                    Text {
                        text: title + " v" + version;
                        color: #ffffff;
                        font-size: 24px;
                        font-weight: 700;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }

                HorizontalBox {
                    height: 48px;
                    spacing: 8px;

                    for category in [
                        {id: "home", name: "Home"},
                        {id: "interactive", name: "Interactive"},
                        {id: "performance", name: "Performance"},
                        {id: "responsive", name: "Responsive"},
                        {id: "accessibility", name: "Accessibility"}
                    ]: Button {
                        text: category.name;
                        primary: current-page == category.id;
                        clicked => { navigate-to-page(category.id); }
                    }
                }

                VerticalBox {
                    padding: 16px;

                    if current-page == "home": VerticalBox {
                        spacing: 16px;

                        Text {
                            text: "Welcome to Slint 1.14 Showcase!";
                            font-size: 24px;
                            font-weight: 700;
                            horizontal-alignment: center;
                        }

                        Text {
                            text: "Explore Slint's web capabilities through interactive demos";
                            font-size: 16px;
                            horizontal-alignment: center;
                        }

                        HorizontalBox {
                            spacing: 16px;
                            padding-top: 24px;

                            for cat in [
                                {id: "interactive", name: "Interactive", icon: "🎮"},
                                {id: "performance", name: "Performance", icon: "⚡"},
                                {id: "responsive", name: "Responsive", icon: "📱"},
                                {id: "accessibility", name: "Accessibility", icon: "♿"}
                            ]: Rectangle {
                                width: 150px;
                                height: 120px;
                                background: rgb(255, 255, 255);
                                border-radius: 12px;
                                border-width: 2px;
                                border-color: rgb(37, 99, 235);

                                VerticalBox {
                                    spacing: 8px;
                                    padding: 12px;

                                    Text {
                                        text: cat.icon;
                                        font-size: 32px;
                                        horizontal-alignment: center;
                                    }

                                    Text {
                                        text: cat.name;
                                        font-size: 14px;
                                        font-weight: 700;
                                        horizontal-alignment: center;
                                    }
                                }

                                TouchArea {
                                    clicked => { navigate-to-page(cat.id); }
                                }
                            }
                        }
                    }

                    if current-page == "interactive": VerticalBox {
                        spacing: 16px;

                        Text {
                            text: "Interactive Demos";
                            font-size: 20px;
                            font-weight: 700;
                        }

                        Text {
                            text: "Hands-on demonstrations of Slint's reactive UI capabilities";
                            font-size: 14px;
                            color: #666666;
                        }

                        VerticalBox {
                            spacing: 12px;

                            for demo in [
                                {id: "counter", title: "Counter", description: "Simple state management"},
                                {id: "button-states", title: "Button States", description: "Different button states"},
                                {id: "text-input", title: "Text Input", description: "Form input with validation"},
                                {id: "slider", title: "Interactive Slider", description: "Two-way data binding"},
                                {id: "checkbox", title: "Checkbox Group", description: "Multiple selection"}
                            ]: Rectangle {
                                background: rgb(255, 255, 255);
                                border-radius: 8px;
                                padding: 16px;

                                VerticalBox {
                                    spacing: 8px;

                                    HorizontalBox {
                                        Text {
                                            text: demo.title;
                                            font-size: 16px;
                                            font-weight: 700;
                                        }

                                        Rectangle {
                                            horizontal-stretch: 1;
                                        }

                                        Button {
                                            text: "Launch";
                                            clicked => { navigate-to-page("demo:" + demo.id); }
                                        }
                                    }

                                    Text {
                                        text: demo.description;
                                        font-size: 14px;
                                        color: #666666;
                                    }
                                }
                            }
                        }
                    }

                    if current-page == "performance": Text {
                        text: "Performance Metrics\n\nShowing Slint's speed advantages - coming soon!";
                        font-size: 16px;
                        horizontal-alignment: center;
                    }

                    if current-page == "responsive": Text {
                        text: "Responsive Design\n\nLayouts that adapt to any screen - coming soon!";
                        font-size: 16px;
                        horizontal-alignment: center;
                    }

                    if current-page == "accessibility": Text {
                        text: "Accessibility Features\n\nKeyboard navigation, screen reader support - coming soon!";
                        font-size: 16px;
                        horizontal-alignment: center;
                    }
                }
            }
        }
    }
}

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Slint version being showcased  
pub const SLINT_VERSION: &str = "1.14";
