impl UIColors {
    /// Returns a white-based default theme
    pub fn white_default() -> Self {
        Self {
            panel_bg: UIColor { name: "Panel Background", hex: "#FFFFFF" },
            toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#F0F0F0" },
            viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#F8F8F8" },
            blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#FF9800" },
            selection: UIColor { name: "Selection Highlight", hex: "#1976D2" },
            axis_x: UIColor { name: "Transform Axis X", hex: "#D32F2F" },
            axis_y: UIColor { name: "Transform Axis Y", hex: "#388E3C" },
            axis_z: UIColor { name: "Transform Axis Z", hex: "#1976D2" },
            text_primary: UIColor { name: "Primary Text", hex: "#222222" },
            text_secondary: UIColor { name: "Secondary Text", hex: "#757575" },
            text_disabled: UIColor { name: "Disabled Text", hex: "#BDBDBD" },
        }
    }
}
//! Primary UI color palette for NeoForge Engine (Project Arkadia) editor and engine systems
//! Based on user specification for panels, toolbars, viewports, accents, and text/icons

pub struct UIColor {
    pub name: &'static str,
    pub hex: &'static str,
}

pub struct UIColors {
    // Backgrounds
    pub panel_bg: UIColor,
    pub toolbar_bg: UIColor,
    pub viewport_grid_bg: UIColor,
    // Accents
    pub blueprint_event: UIColor,
    pub selection: UIColor,
    pub axis_x: UIColor,
    pub axis_y: UIColor,
    pub axis_z: UIColor,
    // Text & Icons
    pub text_primary: UIColor,
    pub text_secondary: UIColor,
    pub text_disabled: UIColor,
}

impl Default for UIColors {
    fn default() -> Self {
        Self {
            panel_bg: UIColor { name: "Panel Background", hex: "#1E1E1E" },
            toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#3A3F44" },
            viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#2A2A2A" },
            blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#D9782D" },
            selection: UIColor { name: "Selection Highlight", hex: "#2D8CFF" },
            axis_x: UIColor { name: "Transform Axis X", hex: "#FF0000" },
            axis_y: UIColor { name: "Transform Axis Y", hex: "#00FF00" },
            axis_z: UIColor { name: "Transform Axis Z", hex: "#0000FF" },
            text_primary: UIColor { name: "Primary Text", hex: "#E6E6E6" },
            text_secondary: UIColor { name: "Secondary Text", hex: "#A0A7AD" },
            text_disabled: UIColor { name: "Disabled Text", hex: "#6B6F73" },
        }
    }
}

/// A named theme palette for the UI
pub struct ThemePalette {
    pub name: &'static str,
    pub colors: UIColors,
}

/// Returns a list of 10 UI color themes (including the default)
pub fn all_themes() -> [ThemePalette; 11] {
    [
        ThemePalette {
            name: "White Default",
            colors: UIColors::white_default(),
        },
        ThemePalette {
            name: "Slate Dark (Default)",
            colors: UIColors::default(),
        },
        ThemePalette {
            name: "Midnight Blue",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#181C24" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#232B36" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#1A2230" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#3A8DFF" },
                selection: UIColor { name: "Selection Highlight", hex: "#4FC3F7" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#FF5370" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#C3E88D" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#82AAFF" },
                text_primary: UIColor { name: "Primary Text", hex: "#D6DEEB" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#A6ACCD" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#5C6370" },
            },
        },
        ThemePalette {
            name: "Solarized Dark",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#002B36" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#073642" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#00313A" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#B58900" },
                selection: UIColor { name: "Selection Highlight", hex: "#268BD2" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#DC322F" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#859900" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#268BD2" },
                text_primary: UIColor { name: "Primary Text", hex: "#EEE8D5" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#93A1A1" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#586E75" },
            },
        },
        ThemePalette {
            name: "Classic Light",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#F5F5F5" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#E0E0E0" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#E8E8E8" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#FFB300" },
                selection: UIColor { name: "Selection Highlight", hex: "#1976D2" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#D32F2F" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#388E3C" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#1976D2" },
                text_primary: UIColor { name: "Primary Text", hex: "#222222" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#757575" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#BDBDBD" },
            },
        },
        ThemePalette {
            name: "Emerald Night",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#1B2B24" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#234036" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#1A332A" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#00C853" },
                selection: UIColor { name: "Selection Highlight", hex: "#00BFAE" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#FF5252" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#69F0AE" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#40C4FF" },
                text_primary: UIColor { name: "Primary Text", hex: "#E0F2F1" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#80CBC4" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#4E5D54" },
            },
        },
        ThemePalette {
            name: "Crimson Steel",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#2B1B1B" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#442C2C" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#3A2323" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#FF1744" },
                selection: UIColor { name: "Selection Highlight", hex: "#FF8A65" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#FF5252" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#FFD740" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#2979FF" },
                text_primary: UIColor { name: "Primary Text", hex: "#FFE6E6" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#FFB3B3" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#8D6E63" },
            },
        },
        ThemePalette {
            name: "Cyber Neon",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#181A20" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#232946" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#232946" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#FF6AC1" },
                selection: UIColor { name: "Selection Highlight", hex: "#6CFFB7" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#FF005C" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#00FFA3" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#00CFFF" },
                text_primary: UIColor { name: "Primary Text", hex: "#F4F4F9" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#B8C1EC" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#6E738D" },
            },
        },
        ThemePalette {
            name: "Desert Sand",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#F4E2D8" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#E2CFC3" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#EADBC8" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#E17055" },
                selection: UIColor { name: "Selection Highlight", hex: "#00B894" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#D63031" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#00B894" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#0984E3" },
                text_primary: UIColor { name: "Primary Text", hex: "#3E2723" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#795548" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#BCAAA4" },
            },
        },
        ThemePalette {
            name: "Forest Canopy",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#223322" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#335533" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#2A442A" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#A3CB38" },
                selection: UIColor { name: "Selection Highlight", hex: "#38ADA9" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#EA2027" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#009432" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#006266" },
                text_primary: UIColor { name: "Primary Text", hex: "#EAF6E0" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#B8E994" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#839192" },
            },
        },
        ThemePalette {
            name: "Royal Purple",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#2E1437" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#4B2067" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#3E1F47" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#A259F7" },
                selection: UIColor { name: "Selection Highlight", hex: "#6C63FF" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#F67280" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#36D1C4" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#355C7D" },
                text_primary: UIColor { name: "Primary Text", hex: "#F3E6FF" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#B39DDB" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#7E57C2" },
            },
        },
        ThemePalette {
            name: "Sunset Orange",
            colors: UIColors {
                panel_bg: UIColor { name: "Panel Background", hex: "#FFEDD5" },
                toolbar_bg: UIColor { name: "Toolbar/Window Frame", hex: "#FFD6A5" },
                viewport_grid_bg: UIColor { name: "Viewport Grid Background", hex: "#FFB385" },
                blueprint_event: UIColor { name: "Blueprint Event Node", hex: "#FF6F3C" },
                selection: UIColor { name: "Selection Highlight", hex: "#FF9A3C" },
                axis_x: UIColor { name: "Transform Axis X", hex: "#FF3C38" },
                axis_y: UIColor { name: "Transform Axis Y", hex: "#FF8C42" },
                axis_z: UIColor { name: "Transform Axis Z", hex: "#FF5E5B" },
                text_primary: UIColor { name: "Primary Text", hex: "#4B2E05" },
                text_secondary: UIColor { name: "Secondary Text", hex: "#A0522D" },
                text_disabled: UIColor { name: "Disabled Text", hex: "#D2B48C" },
            },
        },
    ]
}
