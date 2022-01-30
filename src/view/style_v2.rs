

pub mod style {
    use iced::{button,container,Color,Vector};

    
    pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: Color::from_rgb(36.0/255.0, 36.0/255.0, 134.0/255.0).into(),
                    border_radius: 8.0,
                    shadow_offset: Vector::new(1.0, 1.0),
                    text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 2.0),
                    ..self.active()
                }
            }
        }

        pub struct Container;

        impl container::StyleSheet for Container {
            fn style(&self) -> container::Style {
                container::Style {
                    background: Color::from_rgb(1.0, 1.0, 1.0).into(),
                    text_color: Color::from_rgb(77.0, 77.0, 255.0).into(),
                    ..container::Style::default()
                }
            }
        }

       
}