//! # Canvas

use crate::errors::*;
use crate::plane::{Cell, Plane};
use crate::point::{Point, Points, POINT_ZERO};
use crate::rect::{Rect, Rectangles};
use dmntk_common::Result;

/// Number of layers in canvas.
pub const LAYER_COUNT: usize = 4;

/// Names of layers in canvas.
pub const LAYER_NAMES: [&str; LAYER_COUNT] = ["TEXT", "THIN", "BODY", "GRID"];

/// Layer containing original text of the decision table.
pub const LAYER_TEXT: usize = 0;

/// Layer containing all the regions with single lines.
pub const LAYER_THIN: usize = 1;

/// Layer containing decision table body without information item name.
pub const LAYER_BODY: usize = 2;

/// Layer containing grid of single lines.
pub const LAYER_GRID: usize = 3;

const CHAR_WS: char = ' ';
const CHAR_OUTER: char = '░';

const CORNER_TOP_LEFT: char = '┌';
const CORNER_BOTTOM_RIGHT: char = '┘';

const CORNERS_TOP_LEFT: [char; 4] = ['┌', '├', '┬', '┼'];
const CORNERS_TOP_RIGHT: [char; 4] = ['┐', '┤', '┬', '┼'];
const CORNERS_BOTTOM_RIGHT: [char; 4] = ['┘', '┤', '┴', '┼'];
const CORNERS_BOTTOM_LEFT: [char; 4] = ['└', '├', '┴', '┼'];

type Layer = usize;
type Layers = [char; LAYER_COUNT];

pub struct Canvas {
  /// Content of the canvas build from layers.
  content: Vec<Vec<Layers>>,
  /// Current cursor position in the canvas.
  cursor: Point,
  /// Position of the required double line crossing between inputs and outputs.
  cross: Option<Point>,
  /// Position of the optional double line crossing between outputs and annotations in horizontal decision table.
  cross_horz: Option<Point>,
  /// Position of the optional double line crossing between outputs and annotations in vertical decision table.
  cross_vert: Option<Point>,
  /// Optional information item name.
  pub information_item_name: Option<String>,
  /// Rectangle containing the body of the decision table.
  body_rect: Option<Rect>,
}

impl Canvas {
  fn recognize_information_item_name(&mut self) -> Result<()> {
    // search for information item name in the original text
    let layer = LAYER_TEXT;
    // move to the beginning of the canvas
    self.move_to(POINT_ZERO);
    // search for the top left corner of the decision table (must be present, error otherwise)
    self.search(layer, &['┌']).and_then(|(_, top_left)| {
      // search for the crossing of the top edge with double line (must be present, error otherwise)
      self.search(layer, &['╥']).and_then(|(_, top_edge)| {
        // if the edge is below the corner than the information item region is present
        // so try to recognize the whole information item name
        if top_left.y < top_edge.y {
          // move to the top-left corner of the decision table
          self.move_to(top_left);
          // move right along the top information item name edge until the corner is encountered
          self.search_right(layer, &['┐'], &['─']).and_then(|_| {
            // move down to the nearest crossing
            self.search_down(layer, &['┴', '┤', '┼'], &['│']).and_then(|(_, bottom_right)| {
              // move left until the left body edge
              self.search_left(layer, &['├'], &['─', '┬', '╥']).and_then(|_| {
                // move up until the original top-left corner is reached
                self.search_up(layer, &['┌'], &['│']).and_then(|(_, closing)| {
                  // verify if the rectangle may be closed and retrieve the text
                  self.close_rectangle(closing, top_left, bottom_right).map(|rect| {
                    let text = self.text_from_rect(layer, &rect);
                    self.information_item_name = Some(text);
                  })
                })
              })
            })
          })
        } else {
          // TODO Report an error when the left top corner is below top edge.
          Ok(())
        }
      })
    })
  }

  fn recognize_crossings(&mut self) -> Result<()> {
    self.move_to(POINT_ZERO); // move to the top-left corner
    self.search(LAYER_TEXT, &['╬']).map(|(_, point)| {
      self.cross = Some(point);
      self.move_to(point);
      if let Ok((_, p)) = self.search_right(LAYER_TEXT, &['╬'], &['═', '╪']) {
        self.cross_horz = Some(p)
      }
      self.move_to(point);
      if let Ok((_, p)) = self.search_down(LAYER_TEXT, &['╬'], &['║', '╫']) {
        self.cross_vert = Some(p)
      }
      // TODO Add validation checking whether two crossings exist.
    })
  }

  fn recognize_body_rect(&mut self) -> Result<Rect> {
    let layer = LAYER_TEXT;
    // move to the top-left corner
    self.move_to(POINT_ZERO);
    // find the first double line crossing
    self.search(layer, &['╬']).and_then(|(_, cross_point)| {
      // move up until the top edge of the body is reached
      self.search_up(layer, &['╥'], &['║', '╫', '╟', '╢']).and_then(|(_, top_point)| {
        // move down until the bottom edge of the body is reached
        self.search_down(layer, &['╨'], &['║', '╫', '╟', '╢', '╬']).and_then(|(_, bottom_point)| {
          // move again to the first crossing
          self.move_to(cross_point);
          // move left until the left edge of the body is reached
          self.search_left(layer, &['╞'], &['═', '╪', '╧', '╤']).and_then(|(_, left_point)| {
            // move right until the right edge of the body is reached
            self
              .search_right(layer, &['╡'], &['═', '╪', '╧', '╤', '╬'])
              .map(|(_, right_point)| Rect::new(left_point.x, top_point.y, right_point.x + 1, bottom_point.y + 1))
          })
        })
      })
    })
  }

  /// Replaces all double lines and double crossings with single lines.
  /// All character other than lines are replaced with whitespace.
  fn prepare_regions(&mut self, src: Layer, dst: Layer) {
    for row in self.content.iter_mut() {
      for col in row.iter_mut() {
        match col[src] {
          '┌' | '┐' | '└' | '┘' | '┬' | '┴' | '─' | '│' | '├' | '┼' | '┤' | ' ' | CHAR_OUTER => col[dst] = col[src],
          '╥' | '╤' => col[dst] = '┬',
          '║' => col[dst] = '│',
          '╨' | '╧' => col[dst] = '┴',
          '═' => col[dst] = '─',
          '╞' | '╟' => col[dst] = '├',
          '╡' | '╢' => col[dst] = '┤',
          '╫' | '╪' | '╬' => col[1] = '┼',
          _ => col[dst] = CHAR_WS,
        }
      }
    }
  }

  /// Removes the information item name region.
  fn remove_information_item_region(&mut self, src: Layer, dst: Layer) {
    if let Some(body_rect) = self.body_rect {
      let (left, top, right, bottom) = body_rect.into_inner();
      if top > 0 {
        for y in 0..top - 1 {
          for x in left..right {
            for layer in dst..LAYER_COUNT {
              self.content[y][x][layer] = CHAR_OUTER;
            }
          }
        }
      }
      for x in left..right {
        match self.content[top][x][src] {
          '├' => self.content[top][x][dst] = '┌',
          '┴' => self.content[top][x][dst] = '─',
          '┤' => self.content[top][x][dst] = '┐',
          '┼' => self.content[top][x][dst] = '┬',
          _ => self.content[top][x][dst] = self.content[top][x][src],
        }
      }
      for y in top + 1..bottom {
        for x in left..right {
          self.content[y][x][dst] = self.content[y][x][src];
        }
      }
    }
  }

  /// Fills the lacking lines to constitute regular grid
  fn make_grid(&mut self, src: Layer, dst: Layer) {
    if let Some(body_rect) = self.body_rect {
      self.copy_layer(src, dst);
      let (left, top, right, bottom) = body_rect.into_inner();
      for y in top..bottom {
        let mut has_horz_line = false;
        for x in left..right {
          if self.content[y][x][dst] == '─' {
            has_horz_line = true;
            break;
          }
        }
        if has_horz_line {
          for x in left..right {
            match self.content[y][x][dst] {
              '│' if x == left => self.content[y][x][dst] = '├',
              '│' if x == right - 1 => self.content[y][x][dst] = '┤',
              '│' => self.content[y][x][dst] = '┼',
              '┤' if x < right - 1 => self.content[y][x][dst] = '┼',
              '├' if x > 0 => self.content[y][x][dst] = '┼',
              CHAR_WS => self.content[y][x][dst] = '─',
              _ => {}
            }
          }
        }
      }
      for x in left..right {
        let mut has_vert_line = false;
        for y in top..bottom {
          if self.content[y][x][dst] == '│' {
            has_vert_line = true;
            break;
          }
        }
        if has_vert_line {
          for y in top..bottom {
            match self.content[y][x][dst] {
              '─' if y == top => self.content[y][x][dst] = '┬',
              '─' if y == bottom - 1 => self.content[y][x][dst] = '┴',
              '─' => self.content[y][x][dst] = '┼',
              '┴' if y < bottom - 1 => self.content[y][x][dst] = '┼',
              '┬' if y > top => self.content[y][x][dst] = '┼',
              CHAR_WS => self.content[y][x][dst] = '│',
              _ => {}
            }
          }
        }
      }
    }
  }

  /// Generates matrix plane describing parts of the analyzed decision table.
  /// The matrix is a rectangular set of cells, where each cell represents
  /// a single graphical element of a decision table like region with text,
  /// double line or double line crossing.
  pub fn plane(&mut self) -> Result<Plane> {
    let regions = self.recognize_regions()?;
    let mut plane: Plane = Default::default(); // the plane to be generated
    let mut row = 0; // current row index
    let mut col; // current column index
    let mut width = 0; // width of the plane
    let mut cross_col = None; // column index of the main crossing
    let mut cross_horz_col = None; // column index of the annotation crossing in horizontal decision table
    for y in 0..self.content.len() {
      if let Some(cross_point) = self.cross {
        if y == cross_point.y {
          for i in 0..width {
            if let Some(index) = cross_col {
              if index == i {
                plane.add_cell(row, Cell::MainDoubleCrossing);
                continue;
              }
            }
            if let Some(index) = cross_horz_col {
              if index == i {
                plane.add_cell(row, Cell::HorizontalDoubleCrossing);
                continue;
              }
            }
            plane.add_cell(row, Cell::HorizontalOutputDoubleLine);
          }
          row += 1;
        }
      }
      if let Some(cross_vert_point) = self.cross_vert {
        if y == cross_vert_point.y {
          for i in 0..width {
            if let Some(index) = cross_col {
              if index == i {
                plane.add_cell(row, Cell::VerticalDoubleCrossing);
                continue;
              }
            }
            plane.add_cell(row, Cell::HorizontalAnnotationsDoubleLine);
          }
          row += 1;
        }
      }
      col = 0;
      let mut matched = false;
      for x in 0..self.content[y].len() {
        if CORNERS_TOP_LEFT.contains(&self.content[y][x][LAYER_GRID]) {
          matched = true;
          if let Some(point) = self.cross {
            if x == point.x {
              cross_col = Some(col);
              plane.add_cell(row, Cell::VerticalOutputDoubleLine);
              col += 1;
            }
          }
          if let Some(point) = self.cross_horz {
            if x == point.x {
              cross_horz_col = Some(col);
              plane.add_cell(row, Cell::VerticalAnnotationDoubleLine);
              col += 1;
            }
          }
          let rect = self.recognize_rectangle(LAYER_GRID, Point::new(x, y))?;
          let mut found = false;
          for (i, region) in regions.iter().enumerate() {
            if region.contains(&rect) {
              let text = self.text_from_rect(LAYER_TEXT, region);
              plane.add_cell(row, Cell::Region(i, *region, text));
              found = true;
              break;
            }
          }
          if !found {
            return Err(err_canvas_region_not_found(rect));
          }
          col += 1;
        }
      }
      if matched {
        width = plane.row_len(row);
        row += 1;
      }
    }
    Ok(plane)
  }

  /// Recognizes all rectangular regions in layer.
  fn recognize_regions(&mut self) -> Result<Rectangles> {
    let layer = LAYER_THIN;
    let points = self.find_top_left_corners(layer);
    let mut rectangles = vec![];
    for point in points {
      let rectangle = self.recognize_region(layer, point)?;
      rectangles.push(rectangle);
    }
    Ok(rectangles)
  }

  /// Recognizes single rectangular region in specified **layer**.
  /// Recognition starts from the **top_left** position in layer.
  fn recognize_region(&mut self, layer: Layer, top_left: Point) -> Result<Rect> {
    // start in the top left corner of the rectangle
    self.move_to(top_left);
    // move right to the nearest top-right corner
    self.search_right(layer, &CORNERS_TOP_RIGHT, &['─', '┴']).and_then(|_| {
      // move down to the nearest bottom-right corner
      self.search_down(layer, &CORNERS_BOTTOM_RIGHT, &['│', '├']).and_then(|(_, bottom_right)| {
        // move left to the nearest bottom-left corner
        self.search_left(layer, &CORNERS_BOTTOM_LEFT, &['─', '┬']).and_then(|_| {
          // move up to the nearest top-left corner
          self
            .search_up(layer, &CORNERS_TOP_LEFT, &['│', '┤'])
            .and_then(|(_, closing)| self.close_rectangle(closing, top_left, bottom_right))
        })
      })
    })
  }

  fn recognize_rectangle(&mut self, layer: Layer, top_left: Point) -> Result<Rect> {
    // start in the top left corner of the rectangle
    self.move_to(top_left);
    // move right to the nearest corner, this is the top-right corner of the recognized rectangle
    self.search_right(layer, &['┼', '┬', '┤', '┐'], &['─']).and_then(|_| {
      // move down to the nearest corner, this is the bottom-right corner of the recognized rectangle
      self.search_down(layer, &['┼', '┴', '┤', '┘'], &['│']).and_then(|(_, bottom_right)| {
        // move left to the nearest corner, this is the bottom-left corner of the recognized rectangle
        self.search_left(layer, &['┼', '└', '├', '┴'], &['─']).and_then(|_| {
          // move up to the nearest corner, this is the top-left corner of the recognized rectangle
          self
            .search_up(layer, &['┼', '┬', '├', '┌'], &['│'])
            .and_then(|(_, closing)| self.close_rectangle(closing, top_left, bottom_right))
        })
      })
    })
  }

  fn close_rectangle(&self, closing: Point, top_left: Point, bottom_right: Point) -> Result<Rect> {
    if closing == top_left {
      Ok(Rect::new(top_left.x, top_left.y, bottom_right.x + 1, bottom_right.y + 1))
    } else {
      Err(err_canvas_rectangle_not_closed(closing, top_left))
    }
  }

  /// Searches for all characters that constitute a top-left corner of a rectangle.
  /// Position of this character is the starting point for recognizing rectangles.
  /// Returns a vector of all top-left corner points found in specified **layer**.
  fn find_top_left_corners(&self, layer: Layer) -> Points {
    let mut points = vec![];
    for y in 0..self.content.len() {
      for x in 0..self.content[y].len() {
        if CORNERS_TOP_LEFT.contains(&self.content[y][x][layer]) {
          points.push(Point::new(x, y));
        }
      }
    }
    points
  }

  /// Moves the cursor to the specified position defined as a **point**.
  fn move_to(&mut self, point: Point) {
    let row_count = self.content.len();
    let y = if point.y < row_count {
      point.y
    } else if row_count > 0 {
      row_count - 1
    } else {
      0
    };
    let col_count = self.content[y].len();
    let x = if point.x < col_count {
      point.x
    } else if col_count > 0 {
      col_count - 1
    } else {
      0
    };
    self.cursor = Point::new(x, y);
  }

  /// Searches for characters specified in `searched` array.
  /// The search starts at the current cursor position and continues from left to right
  /// and from top to bottom of specified layer in canvas.
  /// When any of the specified character is found, the cursor position is updated
  /// and the function returns successfully.
  /// Otherwise the function reports an error.
  fn search(&mut self, layer: Layer, searched: &[char]) -> Result<(char, Point)> {
    let (x, y) = self.cursor.into_inner();
    for c in x..self.content[y].len() {
      let ch = self.content[y][c][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(c, y);
        return Ok((ch, self.cursor));
      }
    }
    for r in y + 1..self.content.len() {
      for c in 0..self.content[r].len() {
        let ch = self.content[r][c][layer];
        if searched.contains(&ch) {
          self.cursor = Point::new(c, r);
          return Ok((ch, self.cursor));
        }
      }
    }
    Err(err_canvas_expected_characters_not_found(searched.to_vec()))
  }

  fn search_up(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (x, mut y) = self.cursor.into_inner();
    while y > 0 {
      y -= 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed.to_vec()));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched.to_vec()))
  }

  fn search_left(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (mut x, y) = self.cursor.into_inner();
    while x > 0 {
      x -= 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed.to_vec()));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched.to_vec()))
  }

  fn search_right(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (mut x, y) = self.cursor.into_inner();
    while x < self.content[y].len() - 1 {
      x += 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed.to_vec()));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched.to_vec()))
  }

  fn search_down(&mut self, layer: usize, searched: &[char], allowed: &[char]) -> Result<(char, Point)> {
    let (x, mut y) = self.cursor.into_inner();
    while y < self.content.len() - 1 {
      y += 1;
      let ch = self.content[y][x][layer];
      if searched.contains(&ch) {
        self.cursor = Point::new(x, y);
        return Ok((ch, self.cursor));
      }
      if !allowed.contains(&ch) {
        return Err(err_canvas_character_is_not_allowed(ch, allowed.to_vec()));
      }
    }
    Err(err_canvas_expected_characters_not_found(searched.to_vec()))
  }

  /// Retrieves the text enclosed inside a rectangle `r` in selected `layer`.
  fn text_from_rect(&self, layer: usize, r: &Rect) -> String {
    let mut text = String::new();
    let mut new_line = false;
    for row in self.content[(r.top + 1)..(r.bottom - 1)].iter() {
      for column in row[(r.left + 1)..(r.right - 1)].iter() {
        if new_line {
          text.push('\n');
          new_line = false;
        }
        text.push(column[layer]);
      }
      new_line = true;
    }
    text
  }

  /// Copies characters from source layer to destination layer.
  fn copy_layer(&mut self, src: Layer, dst: Layer) {
    for y in 0..self.content.len() {
      for x in 0..self.content[y].len() {
        self.content[y][x][dst] = self.content[y][x][src];
      }
    }
  }

  /// Displays the content of the text layer.
  pub fn display_text_layer(&self) {
    self.display_layer(LAYER_TEXT);
  }

  /// Displays the content of the thin lines layer.
  pub fn display_thin_layer(&self) {
    self.display_layer(LAYER_THIN);
  }

  /// Displays the content of the body part of decision table.
  pub fn display_body_layer(&self) {
    self.display_layer(LAYER_BODY);
  }

  /// Displays the content of the grid layer.
  pub fn display_grid_layer(&self) {
    self.display_layer(LAYER_GRID);
  }

  /// Displays the content of the specified layer.
  fn display_layer(&self, l: Layer) {
    if l < LAYER_COUNT {
      println!("{}", LAYER_NAMES[l]);
      for row in &self.content {
        for layers in row {
          print!("{}", layers[l])
        }
        println!()
      }
    }
  }
}

/// Prepares a `canvas` containing the textual definition of `decision table`.
/// This function traverse the input text line-by-line and adds non empty lines of text to the canvas.
/// Adding lines to canvas begins with the line starting with the `┌` character
/// (U+250C BOX DRAWINGS LIGHT DOWN AND RIGHT), because this is the top-left corner of every decision table.
/// Adding lines to canvas ends with the line that ends with the `┘` character
/// (U+2518 BOX DRAWINGS LIGHT UP AND LEFT), because this is the bottom right corner of every decision table.
/// Shorter lines are filled up with additional characters to form a rectangular area.
pub fn scan(text: &str) -> Result<Canvas> {
  let mut width: usize = 0;
  let mut height: usize = 0;
  let mut content = vec![vec![]];
  // iterate the text line by line
  let mut start_adding = false;
  let mut end_adding = false;
  for line in text.lines() {
    // remove white characters
    let line = line.trim();
    if !line.is_empty() {
      if line.starts_with(CORNER_TOP_LEFT) && !start_adding && !end_adding {
        start_adding = true;
      }
      if start_adding && !end_adding {
        // add new, empty line
        content.push(vec![]);
        height += 1;
        let mut count = 0;
        let mut layers = [CHAR_WS; LAYER_COUNT];
        for chr in line.chars() {
          layers[0] = chr;
          content[height - 1].push(layers);
          count += 1;
        }
        if count > width {
          width = count;
        }
      }
      if line.ends_with(CORNER_BOTTOM_RIGHT) && start_adding && !end_adding {
        end_adding = true;
      }
    }
  }
  // fill shorter lines up to the width of the longest line
  if height > 0 && width > 0 {
    for row in &mut content {
      while row.len() < width {
        row.push([CHAR_OUTER; LAYER_COUNT]);
      }
    }
  }
  // create the canvas
  let mut canvas = Canvas {
    content,
    cursor: POINT_ZERO,
    cross: None,
    cross_horz: None,
    cross_vert: None,
    information_item_name: None,
    body_rect: None,
  };
  canvas.recognize_information_item_name()?;
  canvas.recognize_crossings()?;
  canvas.body_rect = Some(canvas.recognize_body_rect()?);
  canvas.prepare_regions(LAYER_TEXT, LAYER_THIN);
  canvas.remove_information_item_region(LAYER_THIN, LAYER_BODY);
  canvas.make_grid(LAYER_BODY, LAYER_GRID);
  Ok(canvas)
}
