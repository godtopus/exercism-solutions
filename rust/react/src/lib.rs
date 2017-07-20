use std::collections::HashMap;

pub type CellID = usize;
pub type CallbackID = usize;

struct Cell<'a, T> {
    value: T,
    compute_func: Option<Box<Fn(&[T]) -> T + 'a>>,
    callbacks: HashMap<CallbackID, Box<FnMut(T) -> () + 'a>>,
    dependencies: Vec<CellID>
}

pub struct Reactor<'a, T>
    where T: 'a
{
    cells: HashMap<CellID, Cell<'a, T>>
}

impl <'a, T: Copy + PartialEq + 'a> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: HashMap::new()
        }
    }

    pub fn create_input(&mut self, initial: T) -> CellID {
        let cell_id = self.cells.len();

        self.cells.insert(cell_id, Cell {
            value: initial,
            compute_func: None,
            callbacks: HashMap::new(),
            dependencies: Vec::new()
        });

        cell_id
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(&'a mut self, dependencies: &'a [CellID], compute_func: F) -> Result<CellID, ()> {
        if !dependencies.iter().all(|d| self.cells.contains_key(d)) {
            return Err(())
        }

        let cell_id = self.cells.len();
        let values = &dependencies.into_iter().map(|d| self.cells.get(&d).unwrap().value).collect::<Vec<T>>()[..];
        let cell = Cell {
            value: compute_func(values),
            compute_func: Some(Box::new(compute_func)),
            callbacks: HashMap::<CallbackID, Box<FnMut(T) -> ()>>::new(),
            dependencies: dependencies.iter().map(|a| *a).collect::<Vec<CellID>>()
        };

        self.cells.insert(cell_id, cell);

        //let v = |_| { cell.compute_func.unwrap()(&cell.dependencies.into_iter().map(|d| self.cells.get(&d).unwrap().value).collect::<Vec<T>>()[..]); };

        for d in dependencies.iter().map(|a| *a).collect::<Vec<CellID>>() {
            self.cells.get_mut(&d).unwrap().callbacks.insert(1, Box::new(move |_| {
                let values = &dependencies.into_iter().map(|d| self.cells.get(&d).unwrap().value).collect::<Vec<T>>()[..];
                compute_func(values);
            }));
        }

        Ok(cell_id)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        match self.cells.get(&id) {
            Some(cell) => Some(cell.value),
            None => None
        }
    }

    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        match self.cells.get_mut(&id) {
            Some(cell) => {
                if !cell.compute_func.is_none() {
                    return Err(())
                }

                if cell.value != new_value {
                    for (_, callback) in cell.callbacks.iter_mut() {
                        callback(new_value);
                    }

                    cell.value = new_value;
                } else {
                    return Ok(())
                }
            },
            _ => return Err(())
        }

        /*for dep in self.cells.get(&id).unwrap().dependencies.iter() {
            let c = self.cells.get(&dep).unwrap();
            let values = &c.dependencies.into_iter().map(|d| self.cells.get(&d).unwrap().value).collect::<Vec<T>>()[..];
            c.compute_func.unwrap()(values);
        }*/

        Ok(())
    }

    pub fn add_callback<F: FnMut(T) -> () + 'a>(&'a mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        match self.cells.get_mut(&id) {
            Some(cell) => {
                let callback_id = cell.callbacks.len();
                cell.callbacks.insert(callback_id, Box::new(callback));

                Ok(callback_id)
            },
            _ => Err(())
        }
    }

    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        match self.cells.get_mut(&cell) {
            Some(cell) => {
                if !cell.callbacks.contains_key(&callback) {
                    return Err(())
                }

                cell.callbacks.remove(&callback);
                Ok(())
            },
            _ => Err(())
        }
    }
}
