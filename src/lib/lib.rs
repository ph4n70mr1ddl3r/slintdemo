//! Slint Showcase Library
//! Main entry point for the Slint UI components

pub mod components;
pub mod data;

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, GridBox, TextEdit, Slider, CheckBox } from "std-widgets.slint";

    export component App {
        in property <string> title: "Slint 1.14 Web Capabilities";
        in property <string> version: "1.14.0";

        in-out property <string> current-page: "home";
        in-out property <string> selected-demo: "";

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

                    if current-page == "home": Text {
                        text: "Welcome to Slint 1.14 Showcase!\n\nClick on a category above to explore.";
                        font-size: 18px;
                        horizontal-alignment: center;
                    }

                    if current-page == "interactive": Text {
                        text: "Interactive Demos\n\nCounter, Button States, Text Input, Slider, Checkbox - coming soon!";
                        font-size: 16px;
                        horizontal-alignment: center;
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
