use std::process;
use terminal_menu::{
  back_button, button, has_exited, label, menu, mut_menu, numeric, run, submenu,
};

fn rgbtohsl(r: f64, g: f64, b: f64) -> String {
  let r = r / 255.0;
  let g = g / 255.0;
  let b = b / 255.0;

  let max = vec![r, g, b].iter().copied().fold(f64::NAN, f64::max);
  let min = vec![r, g, b].iter().fold(f64::INFINITY, |a, &b| a.min(b));

  let mut h;
  let s;
  let l = (max + min) / 2.0;

  if max == min {
    h = 0.0;
    s = 0.0;
  } else {
    let d = max - min;
    if l > 0.5 {
      s = d / (2.0 - max - min);
    } else {
      s = d / (max + min);
    }

    if max == r {
      if g < b {
        h = (g - b) / d + 6.0;
      } else {
        h = (g - b) / d + 0.0;
      }
    } else if max == g {
      h = (b - r) / d + 2.0;
    } else {
      h = (r - g) / d + 4.0;
    }

    h = (h / 6.0) * 360.0;
  }

  let result_string = format!(
    "RGB [{:?}, {:?}, {:?}] para HSL: [{:.2}, {:.2}, {:.2}]",
    r * 255.0,
    g * 255.0,
    b * 255.0,
    h,
    s,
    l
  );
  return result_string;
}

fn hsltorgb(h: f64, s: f64, l: f64) -> String {
  let mut r = 0.0;
  let mut g = 0.0;
  let mut b = 0.0;

  let c = (1.0 - (2.0 * l - 1.0).abs()) * s;

  let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
  let m = l - (c / 2.0);

  if 0.0 <= h && h < 60.0 {
    r = c;
    g = x;
    b = 0.0;
  } else if 60.0 <= h && h < 120.0 {
    r = x;
    g = c;
    b = 0.0;
  } else if 120.0 <= h && h < 180.0 {
    r = 0.0;
    g = c;
    b = x;
  } else if 180.0 <= h && h < 240.0 {
    r = 0.0;
    g = x;
    b = c;
  } else if 240.0 <= h && h < 300.0 {
    r = x;
    g = 0.0;
    b = c;
  } else if 300.0 <= h && h < 360.0 {
    r = c;
    g = 0.0;
    b = x;
  }

  let result_string = format!(
    "HSL [{:?}, {:?}, {:?}] para RGB: [{:.2}, {:.2}, {:.2}]",
    h,
    s,
    l,
    (r + m) * 255.0,
    (g + m) * 255.0,
    (b + m) * 255.0
  );

  return result_string;
}
fn main() {
  let menu = menu(vec![
    label("-----------------------------"),
    label("Trabalho 1 - Conversões"),
    label("Selecione a opção desejada com as setas ↑↓"),
    label("'enter' para selecionar"),
    label("'q' ou 'esc' para sair"),
    label("-----------------------------"),
    submenu(
      "RGB para HSL",
      vec![
        label("-----------------------------"),
        label("RGB para HSL"),
        label(
          "utilize 'enter' para alterar o valor ou as setas ←→ para aumentar/decrementar o valor",
        ),
        label("-----------------------------"),
        numeric("R", 255.0, Some(1.0), Some(0.0), Some(255.0)),
        numeric("G", 255.0, Some(1.0), Some(0.0), Some(255.0)),
        numeric("B", 255.0, Some(1.0), Some(0.0), Some(255.0)),
        label(" "),
        button("Converter para HSL"),
        label(" "),
        label("-----------------------------"),
        back_button("[< Voltar]"),
      ],
    ),
    submenu(
      "HSL para RGB",
      vec![
        label("-----------------------------"),
        label("HSL para RGB"),
        label(
          "utilize 'enter' para alterar o valor ou as setas ←→ para aumentar/decrementar o valor",
        ),
        label("-----------------------------"),
        numeric("H", 359.0, Some(1.0), Some(0.0), Some(359.0)),
        numeric("S", 100.0, Some(0.5), Some(0.0), Some(100.0)),
        numeric("L", 50.0, Some(0.5), Some(0.0), Some(100.0)),
        label(" "),
        button("Converter"),
        label(" "),
        back_button("[< Voltar]"),
      ],
    ),
    button("Sair"),
    label("-----------------------------")
  ]);

  if has_exited(&menu) {
    println!("{}", mut_menu(&menu).selected_item_name());
  }

  run(&menu);
  {
    if mut_menu(&menu).selected_item_name() == "RGB para HSL" {
      let r = mut_menu(&menu)
        .get_submenu("RGB para HSL")
        .numeric_value("R");
      let g = mut_menu(&menu)
        .get_submenu("RGB para HSL")
        .numeric_value("G");
      let b = mut_menu(&menu)
        .get_submenu("RGB para HSL")
        .numeric_value("B");
      println!("{}", rgbtohsl(r, g, b));
    } else if mut_menu(&menu).selected_item_name() == "HSL para RGB" {
      let h = mut_menu(&menu)
        .get_submenu("HSL para RGB")
        .numeric_value("H");
      let s = mut_menu(&menu)
        .get_submenu("HSL para RGB")
        .numeric_value("S")
        / 100.0;
      let l = mut_menu(&menu)
        .get_submenu("HSL para RGB")
        .numeric_value("L")
        / 100.0;
      println!("{}", hsltorgb(h, s, l));
    } else if mut_menu(&menu).selected_item_name() == "Sair" {
      process::exit(0);
    }
  }
}
