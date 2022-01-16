use super::{Color, ColorSet, ColorSets};

pub fn hex_str_to_rgba<'a>(s: &'a str) -> [f32; 4] {
    let re = regex::Regex::new(r"#([a-fA-F0-9]{6})").unwrap();
    if !re.is_match(s) {
        panic!("{} is not in hex format", s);
    };

    let rgb: Vec<f32> = s[1..]
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|chunk| {
            hex::decode(chunk)
                .expect(format!("unable to decode chuck {} in hex {}", chunk.as_str(), s).as_str())
                [0] as f32
        })
        .collect();

    [rgb[0], rgb[1], rgb[2], 255.0]
}

pub fn hex_str_to_normalized_rgba<'a>(s: &'a str) -> [f32; 4] {
    let rgba = hex_str_to_rgba(s)
        .iter()
        .map(|v| v / 255.0)
        .collect::<Vec<f32>>();

    [rgba[0], rgba[1], rgba[2], rgba[3]]
}

pub fn colorsets_from_vec_hex_strings(vec_hex_strings: Vec<Vec<&str>>) -> ColorSets {
    ColorSets {
        n: 0,
        colorsets: vec_hex_strings
            .iter()
            .map(|hex_strings| ColorSet {
                colors: vec_hex_to_vec_color(hex_strings.to_owned()),
            })
            .collect(),
    }
}

pub fn colorset_from_hex_strings(hex_strings: Vec<&str>) -> ColorSet {
    ColorSet {
        colors: vec_hex_to_vec_color(hex_strings),
    }
}

pub fn vec_hex_to_vec_color(hex_strings: Vec<&str>) -> Vec<Color> {
    hex_strings
        .iter()
        .map(|hex_string| hex_to_color(hex_string))
        .collect()
}

pub fn hex_to_color(hex_string: &str) -> Color {
    let decoded = hex_str_to_normalized_rgba(hex_string);
    Color {
        r: decoded[0],
        g: decoded[1],
        b: decoded[2],
        a: decoded[3],
        shade: 1.0,
    }
}

// fn hex_to_rbg(hex_string: &str) -> (f32, f32, f32) {
// let decoded: Vec<f32> = hex_string[1..]
// .chars()
// .collect::<Vec<char>>()
// .chunks(2)
// .map(|c| c.iter().collect::<String>())
// .collect::<Vec<String>>()
// .iter()
// .map(|chunk| {
// hex::decode(chunk).expect(
// format!(
// "unable to decode chuck {} in hex {}",
// chunk.as_str(),
// hex_string
// )
// .as_str(),
// )[0] as f32
// })
// .collect();

// (decoded[0], decoded[1], decoded[2])
// }

#[test]
#[should_panic]
fn test_bad_hex_str_to_rgba() {
    let bad_hex_str = "af4573";
    hex_str_to_rgba(bad_hex_str);
}

#[test]
#[should_panic]
fn test_bad_hex_str_to_rgba_2() {
    let bad_hex_str = "#af457";
    hex_str_to_rgba(bad_hex_str);
}

#[test]
fn test_hex_str_to_rgba() {
    let hex_str = "#af4573";
    let rgba = hex_str_to_rgba(hex_str);
    assert_eq!(rgba, [175.0, 69.0, 115.0, 255.0]);
}

#[test]
fn test_hex_str_to_normalized_rgba() {
    let hex_str = "#af4573";
    let rgba = hex_str_to_normalized_rgba(hex_str);
    assert_eq!(rgba, [0.6862745, 0.27058825, 0.4509804, 1.0,])
}

// #[test]
// fn test_hex_to_rbg_success() {
// assert_eq!(hex_to_rbg("#ffffff"), (255.0, 255.0, 255.0));
// assert_eq!(hex_to_rbg("#000000"), (0.0, 0.0, 0.0));
// assert_eq!(hex_to_rbg("#96B780"), (150.0, 183.0, 128.0));
// }

// #[test]
// #[should_panic]
// fn test_hex_to_rbg_errors() {
// assert_eq!(hex_to_rbg("#396B780"), (150.0, 183.0, 128.0));
// }

#[test]
fn test_hex_to_color() {
    assert_eq!(
        hex_to_color("#96B780"),
        Color {
            r: 150.0 / 255.0,
            g: 183.0 / 255.0,
            b: 128.0 / 255.0,
            shade: 1.0
        }
    );
}
