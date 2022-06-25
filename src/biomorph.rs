use rand::Rng;

/// Configuration passed down from the GUI
struct Config {
  columns: u8,
  rows: u8,
}

impl Config {
  /// Default settings
  fn default() -> Self {
    Config { columns: 3, rows: 3 }
  }
}

/// Definition of conditions for all the biomorphs and groups them together
/// the Matrix is the grid displayed
struct Matrix {
  biomorphs: Vec<Biomorph>,
}

impl Matrix {
  /// Runs when Matrix first initialized
  fn initial_setup(config: Config) -> Matrix {
    // initialize vector on self
    let mut biomorphs = Vec::new();
    // loop thru every cell in the matrix
    let mut i = 0;
    while i < config.columns * config.rows - 1 {
      biomorphs.push(Biomorph::initial());
      i += 1;
    }

    Matrix {
      biomorphs,
    }
  }
}

/// Definition for the data that a biomorph has
struct Biomorph {
  genes: [i32; 9],
  segments: Vec<Vec<i32>>,
}

impl Biomorph {
  fn initial() -> Biomorph {
    let mut i= 0;
    let mut genes = [0; 9];
    while i < 8 {
      genes[i] = rand::thread_rng().gen_range(-9..10);
      i += 1;
    }

    genes[9] = rand::thread_rng().gen_range(0..10);

    Biomorph {
      genes,
      segments: Vec::new(),
    }
  }
}

/// Definition of a segment, every biomorph has a set of segments and they are rendered one by one
struct Segment {
  start_x: i32,
  start_y: i32,
  end_x: i32,
  end_y: i32,
}