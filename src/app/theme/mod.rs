use ratatui::style::Color;

#[derive(Debug)]
pub struct Theme {
    pub rose: Color,
    pub flamingo: Color,
    pub pink: Color,
    pub mauve: Color,
    pub red: Color,
    pub maroon: Color,
    pub peach: Color,
    pub yellow: Color,
    pub green: Color,
    pub teal: Color,
    pub sky: Color,
    pub sapphire: Color,
    pub blue: Color,
    pub lavender: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext2: Color,
    pub overlay1: Color,
    pub overlay2: Color,
    pub overlay3: Color,
    pub surface1: Color,
    pub surface2: Color,
    pub surface3: Color,
    pub base: Color,
    pub mantle: Color,
    pub crust: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            rose: Color::Rgb(242, 213, 207),
            flamingo: Color::Rgb(238, 190, 190),
            pink: Color::Rgb(244, 184, 228),
            mauve: Color::Rgb(202, 158, 230),
            red: Color::Rgb(231, 130, 132),
            maroon: Color::Rgb(234, 153, 156),
            peach: Color::Rgb(239, 159, 118),
            yellow: Color::Rgb(229, 200, 144),
            green: Color::Rgb(166, 209, 137),
            teal: Color::Rgb(129, 200, 190),
            sky: Color::Rgb(153, 209, 219),
            sapphire: Color::Rgb(133, 193, 220),
            blue: Color::Rgb(140, 170, 238),
            lavender: Color::Rgb(186, 187, 241),
            text: Color::Rgb(198, 208, 245),
            subtext1: Color::Rgb(181, 191, 226),
            subtext2: Color::Rgb(165, 173, 206),
            overlay1: Color::Rgb(148, 156, 187),
            overlay2: Color::Rgb(131, 139, 167),
            overlay3: Color::Rgb(115, 121, 148),
            surface1: Color::Rgb(98, 104, 128),
            surface2: Color::Rgb(81, 87, 109),
            surface3: Color::Rgb(65, 69, 89),
            base: Color::Rgb(48, 52, 70),
            mantle: Color::Rgb(41, 44, 60),
            crust: Color::Rgb(35, 38, 52),
        }
    }
}
