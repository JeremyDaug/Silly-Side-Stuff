use std::collections::HashMap;

/// # Orthonormal Bases
/// 
/// Stores the information for our Orthonormal bases.
/// 
/// Creates values for Basis and allows us to both set the Square of 
/// the Basis and their names when printed.
/// 
/// Includes storage space for notes on each O.N.Basis as well.
/// 
/// As a standard rule, These vectors are always perpendicular to each other
/// and when multiply produce a higher order K-vector with a unit size of 1.
#[derive(Debug)]
pub struct OrthonormalBases {
    /// The names of our Bases and how they will appear.
    names: HashMap<usize, String>,
    /// The Bases available to us and their squares.
    squares: HashMap<usize, f64>,
    /// Notes on our various Bases, for user purposes.
    notes: HashMap<usize, String>
}

impl OrthonormalBases {
    /// # New
    /// 
    /// Creates an empty basis set to be modified later.
    pub fn new() -> Self {
        OrthonormalBases {
            names: HashMap::new(),
            squares: HashMap::new(),
            notes: HashMap::new(),
        }
    }

    /// # Name Of
    /// 
    /// Gets the name of a basis with ID basis.
    pub fn name_of(&self, basis: &usize) -> &String {
        &self.names[basis]
    }

    /// # Set Name Of
    /// 
    /// Sets the name of a basis with ID basis.
    /// 
    /// If new basis, it also adds the basis to square with value of 0.0
    /// and an empty note.
    pub fn set_name_of(&mut self, basis: usize, name: String) -> Result<(), &str> {
        if !self.names.contains_key(&basis) {
            self.add_basis(basis, 0.0, name, "".to_string())?;
        } else {
            self.names.insert(basis, name);
        }
        Ok(())
    }

    /// # Square of 
    /// 
    /// Gets the square of a basis vector with ID 
    pub fn square_of(&self, basis: &usize) -> &f64 {
        &self.squares[basis]
    }

    /// # Set Square Of
    /// 
    /// Sets what value a basis vector of multiplies to. 
    /// 
    /// Must be +1, -1, or 0. If given a value other than these three
    /// it normalizes them to them. Positive values become +1, negative -1.
    /// 
    /// If new basis, it also adds in with name of e_{basisID} and an empty note.
    pub fn set_square_of(&mut self, basis: usize, square: f64) -> Result<(), &str> {
        if !self.squares.contains_key(&basis) {
            self.add_basis(basis, square, format!("e_{0}", basis), "".to_string())?;
        } else {
            let value = match square {
                val if val.is_sign_positive() => 1.0,
                val if val.is_sign_negative() => -1.0,
                _ => 0.0
            };
            self.squares.insert(basis, value);
        }
        Ok(())
    }

    /// # Note Of
    /// 
    /// Gets the notes for a basis
    pub fn note_of(&self, basis: &usize) -> Option<&String> {
        self.notes.get(basis)
    }

    /// # Set Note Of
    /// 
    /// Sets a note for a basis.
    /// 
    /// If no existing Basis, it adds that basis with a square value of 0 and a name of e_{basis ID}
    pub fn set_note_of(&mut self, basis: usize, note: String) -> Result<(), &str> {
        if !self.notes.contains_key(&basis) {
            self.add_basis(basis, 0.0, format!("e_{0}", basis), note)?;
        } else {
            self.notes.insert(basis, note);
        }
        Ok(())
    }

    /// # Add Basis
    /// 
    /// Adds a new basis to the ONB, including the ID, square, name, and any notes.
    /// 
    /// ## Returns
    /// 
    /// Returns Err if basis already exists in our data, or it's name is a duplicate.
    pub fn add_basis(&mut self, basis: usize, square: f64, name: String, notes: String) -> Result<(), &str>{
        if self.names.contains_key(&basis) {
            return Err("Basis ID already exists.");
        } else if self.names.iter().any(|(_, ex_name)| ex_name.eq(&name)) {
            return Err("Name already exists in basis Set.");
        }

        self.names.insert(basis, name);
        self.squares.insert(basis, square);
        self.notes.insert(basis, notes);

        Ok(())
    }
}
