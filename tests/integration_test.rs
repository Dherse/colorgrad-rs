use colorgrad::{
    BasisGradient, BlendMode, CatmullRomGradient, Color, Gradient, GradientBuilder, LinearGradient,
};

#[test]
fn custom_gradient() {
    // Custom gradient default
    let g = GradientBuilder::new().build::<LinearGradient>().unwrap();
    assert_eq!(g.domain(), (0.0, 1.0));
    assert_eq!(g.at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.at(1.0).to_hex_string(), "#ffffff");

    // Custom colors
    let g = GradientBuilder::new()
        .colors(&[
            Color::new(1.0, 0.0, 0.0, 1.0),
            Color::new(1.0, 1.0, 0.0, 1.0),
            Color::new(0.0, 0.0, 1.0, 1.0),
        ])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.domain(), (0.0, 1.0));
    assert_eq!(g.at(0.0).to_hex_string(), "#ff0000");
    assert_eq!(g.at(0.5).to_hex_string(), "#ffff00");
    assert_eq!(g.at(1.0).to_hex_string(), "#0000ff");

    // Custom colors #2
    let g = GradientBuilder::new()
        .html_colors(&["#00f", "#00ffff"])
        .colors(&[Color::new(1.0, 1.0, 0.0, 0.5)])
        .html_colors(&["lime"])
        .build::<LinearGradient>()
        .unwrap();
    let colors = g.colors(4);
    assert_eq!(colors[0].to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(colors[1].to_rgba8(), [0, 255, 255, 255]);
    assert_eq!(colors[2].to_rgba8(), [255, 255, 0, 128]);
    assert_eq!(colors[3].to_rgba8(), [0, 255, 0, 255]);

    // Single color
    let g = GradientBuilder::new()
        .colors(&[Color::new(1.0, 0.0, 0.0, 1.0)])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [255, 0, 0, 255]);

    // Builder pattern style 2
    let mut gb = GradientBuilder::new();
    gb.colors(&[
        Color::from_rgba8(255, 0, 0, 255),
        Color::from_rgba8(0, 0, 255, 255),
        Color::from_rgba8(0, 255, 0, 255),
    ]);
    gb.domain(&[0.0, 0.5, 1.0]);
    gb.mode(BlendMode::Rgb);

    let mut gb2 = gb.clone();

    let g = gb.build::<LinearGradient>().unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 255, 0, 255]);

    // change color position
    gb2.domain(&[0.0, 35.0, 100.0]);
    let g = gb2.build::<LinearGradient>().unwrap();

    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(35.0).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(100.0).to_rgba8(), [0, 255, 0, 255]);
}

#[test]
fn custom_gradient_blend_mode() {
    // Blend mode RGB
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Rgb)
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Blend mode Linear RGB
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::LinearRgb)
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Blend mode HSV
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Hsv)
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Blend mode Oklab
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Oklab)
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);
}

#[test]
fn custom_gradient_interpolation_mode() {
    // Interpolation linear
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Interpolation catmull-rom
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .build::<CatmullRomGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Interpolation basis
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .build::<BasisGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert!(g.at(0.5).to_rgba8() != [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);
}

#[test]
fn custom_gradient_error() {
    // Invalid HTML colors
    let g = GradientBuilder::new()
        .html_colors(&["#777", "bloodred", "#bbb", "#zzz"])
        .build::<LinearGradient>();
    assert_eq!(
        g.unwrap_err().to_string(),
        "invalid html colors: 'bloodred', '#zzz'"
    );

    // Wrong domain #1
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0.0, 0.75, 1.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err().to_string(), "wrong domain count");

    // Wrong domain #2
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0.0, 0.71, 0.7, 1.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err().to_string(), "wrong domain");

    // Wrong domain #3
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[1.0, 0.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err().to_string(), "wrong domain");
}

#[test]
fn custom_gradient_domain() {
    // Custom domain #1
    let g = GradientBuilder::new()
        .html_colors(&["yellow", "blue", "lime"])
        .domain(&[0.0, 100.0])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(50.0).to_hex_string(), "#0000ff");
    assert_eq!(g.at(100.0).to_hex_string(), "#00ff00");

    assert_eq!(g.at(-10.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(110.0).to_hex_string(), "#00ff00");
    assert_eq!(g.at(f64::NAN).to_hex_string(), "#000000");

    // Custom domain #2
    let g = GradientBuilder::new()
        .html_colors(&["yellow", "blue", "lime"])
        .domain(&[-1.0, 1.0])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(-1.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(0.0).to_hex_string(), "#0000ff");
    assert_eq!(g.at(1.0).to_hex_string(), "#00ff00");

    assert_eq!(g.at(-2.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(2.0).to_hex_string(), "#00ff00");

    // Custom color position #1
    let g = GradientBuilder::new()
        .html_colors(&["yellow", "blue", "lime"])
        .domain(&[0.0, 0.75, 1.0])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(0.75).to_hex_string(), "#0000ff");
    assert_eq!(g.at(1.0).to_hex_string(), "#00ff00");

    assert_eq!(g.at(-10.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(110.0).to_hex_string(), "#00ff00");

    // Custom color position #2
    let g = GradientBuilder::new()
        .html_colors(&["yellow", "blue", "lime", "red"])
        .domain(&[15.0, 25.0, 29.0, 63.0])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(15.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(25.0).to_hex_string(), "#0000ff");
    assert_eq!(g.at(29.0).to_hex_string(), "#00ff00");
    assert_eq!(g.at(63.0).to_hex_string(), "#ff0000");

    assert_eq!(g.at(10.0).to_hex_string(), "#ffff00");
    assert_eq!(g.at(64.0).to_hex_string(), "#ff0000");
}

#[test]
fn sharp_gradient() {
    let grad = GradientBuilder::new()
        .html_colors(&["red", "lime", "blue"])
        .build::<LinearGradient>()
        .unwrap();

    let g0 = grad.sharp(0, 0.0);
    assert_eq!(g0.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    let g1 = grad.sharp(1, 0.0);
    assert_eq!(g1.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    let g3 = grad.sharp(3, 0.0);
    assert_eq!(g3.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g3.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(g3.at(0.4).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g3.at(0.5).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g3.at(0.6).to_rgba8(), [0, 255, 0, 255]);

    assert_eq!(g3.at(0.9).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g3.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    assert_eq!(g3.at(-0.1).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g3.at(1.1).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g3.at(f64::NAN).to_rgba8(), [0, 0, 0, 255]);

    let grad = GradientBuilder::new()
        .html_colors(&["red", "lime", "blue"])
        .domain(&[-1.0, 1.0])
        .build::<LinearGradient>()
        .unwrap();

    let g2 = grad.sharp(2, 0.0);
    assert_eq!(g2.at(-1.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g2.at(-0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g2.at(-0.1).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(g2.at(0.1).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g2.at(0.5).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g2.at(1.0).to_rgba8(), [0, 0, 255, 255]);
}

#[test]
fn sharp_gradient_x() {
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();

    let g0 = g.sharp(0, 0.1);
    assert_eq!(g0.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(1.0).to_rgba8(), [255, 0, 0, 255]);

    let g1 = g.sharp(1, 0.1);
    assert_eq!(g1.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(1.0).to_rgba8(), [255, 0, 0, 255]);

    let g = g.sharp(3, 0.1);
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(g.at(1.0 / 3.0).to_rgba8(), [128, 128, 0, 255]);

    assert_eq!(g.at(0.45).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g.at(0.50).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g.at(0.55).to_rgba8(), [0, 255, 0, 255]);

    assert_eq!(g.at(1.0 / 3.0 * 2.0).to_rgba8(), [0, 128, 128, 255]);

    assert_eq!(g.at(0.9).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    assert_eq!(g.at(-0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(1.5).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(f64::NAN).to_rgba8(), [0, 0, 0, 255]);
}

#[test]
fn colors() {
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();

    let colors0 = g.colors(0);
    assert_eq!(colors0.len(), 0);

    let colors1 = g.colors(1);
    assert_eq!(colors1.len(), 1);
    assert_eq!(colors1[0].to_hex_string(), "#ff0000");

    let colors2 = g.colors(2);
    assert_eq!(colors2.len(), 2);
    assert_eq!(colors2[0].to_hex_string(), "#ff0000");
    assert_eq!(colors2[1].to_hex_string(), "#0000ff");

    let colors3 = g.colors(3);
    assert_eq!(colors3.len(), 3);
    assert_eq!(colors3[0].to_hex_string(), "#ff0000");
    assert_eq!(colors3[1].to_hex_string(), "#00ff00");
    assert_eq!(colors3[2].to_hex_string(), "#0000ff");

    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .domain(&[-1.0, 1.0])
        .build::<LinearGradient>()
        .unwrap();

    let colors5 = g.colors(5);
    assert_eq!(colors5.len(), 5);
    assert_eq!(colors5[0].to_hex_string(), "#ff0000");
    assert_eq!(colors5[1].to_hex_string(), "#808000");
    assert_eq!(colors5[2].to_hex_string(), "#00ff00");
    assert_eq!(colors5[3].to_hex_string(), "#008080");
    assert_eq!(colors5[4].to_hex_string(), "#0000ff");
}

#[cfg(feature = "preset")]
#[test]
fn preset() {
    let g = colorgrad::preset::viridis();
    assert_eq!(g.at(0.0).to_hex_string(), "#440154");
    assert_eq!(g.at(0.5).to_hex_string(), "#27838e");
    assert_eq!(g.at(1.0).to_hex_string(), "#fee825");
    assert_eq!(g.at(f64::NAN).to_hex_string(), "#000000");

    let g = colorgrad::preset::greys();
    assert_eq!(g.at(0.0).to_hex_string(), "#ffffff");
    assert_eq!(g.at(1.0).to_hex_string(), "#000000");

    let g = colorgrad::preset::turbo();
    assert_eq!(g.at(0.0).to_hex_string(), "#23171b");
    assert_eq!(g.at(1.0).to_hex_string(), "#900c00");

    let g = colorgrad::preset::cividis();
    assert_eq!(g.at(0.0).to_hex_string(), "#002051");
    assert_eq!(g.at(1.0).to_hex_string(), "#fdea45");

    let g = colorgrad::preset::cubehelix_default();
    assert_eq!(g.at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.at(1.0).to_hex_string(), "#ffffff");

    let g = colorgrad::preset::warm();
    assert_eq!(g.at(0.0).to_hex_string(), "#6e40aa");
    assert_eq!(g.at(1.0).to_hex_string(), "#aff05b");

    let g = colorgrad::preset::cool();
    assert_eq!(g.at(0.0).to_hex_string(), "#6e40aa");
    assert_eq!(g.at(1.0).to_hex_string(), "#aff05b");
}

#[cfg(feature = "preset")]
#[test]
fn cyclic() {
    let g = colorgrad::preset::rainbow();
    assert_eq!(g.at(0.0).to_rgba8(), g.at(1.0).to_rgba8());

    let g = colorgrad::preset::sinebow();
    assert_eq!(g.at(0.0).to_rgba8(), g.at(1.0).to_rgba8());
}
