pub mod geometry {
    pub enum ResizeType {
        Percentage,
        Pixel,
    }
    pub struct Resize {
        pub resize_type: ResizeType,
        pub width: i32,
        pub height: i32,
    }

    pub struct Position {
        pub x: i32,
        pub y: i32,
    }
    pub struct Geometry {
        pub resize: Option<Resize>,
        pub position: Option<Position>,
    }

    impl Geometry {
        pub fn to_string(&self) -> String {
            let mut s = String::from("");

            if let Some(resize) = &self.resize {
                let width = resize.width.to_string();
                let height = resize.height.to_string();

                let meta = match resize.resize_type {
                    ResizeType::Percentage => "%",
                    _ => "",
                };

                s.push_str(width.as_str());
                s.push_str(meta);
                s.push('x');
                s.push_str(height.as_str());
                s.push_str(meta);
            }

            if let Some(position) = &self.position {
                let x = position.x.to_string();
                let y = position.y.to_string();

                if position.x >= 0 {
                    s.push('+');
                }
                s.push_str(x.as_str());
                if position.x >= 0 {
                    s.push('+');
                }
                s.push_str(y.as_str());
            }

            s
        }
    }
}
