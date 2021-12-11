pub fn adjacent<T: Into<usize>+From<usize>>(x: T, y: T, max_x: T, max_y: T) -> Vec<(T, T)> {
    let mut adj = Vec::new();
    let x: usize = x.into();
    let y: usize = y.into();
    let max_y: usize = max_y.into();
    let max_x: usize = max_x.into();
    if x > 0 {
        adj.push(((x - 1).into(), y.into()));
    }
    if x < max_x - 1 {
        adj.push(((x + 1).into(), y.into()));
    }
    if y > 0 {
        adj.push((x.into(), (y - 1).into()));
    }
    if y < max_y - 1 {
        adj.push((x.into(), (y + 1).into()));
    }
    adj
}

pub fn adjacent_with_diags<T: From<usize>+Into<usize>>(x: T, y: T, max_x: T, max_y: T) -> Vec<(T, T)> {
    let mut adj: Vec<(T, T)> = Vec::new();
    let x: usize = x.into();
    let y: usize = y.into();
    let max_y: usize = max_y.into();
    let max_x: usize = max_x.into();
    if x > 0 {
        adj.push(((x - 1).into(), y.into()));
        if y > 0 {
            adj.push(((x - 1).into(), (y - 1).into()));
        }
        if y < max_y - 1 {
            adj.push(((x - 1).into(), (y + 1).into()));
        }
    }
    if x < max_x - 1 {
        adj.push(((x + 1).into(), y.into()));
        if y > 0 {
            adj.push(((x + 1).into(), (y - 1).into()));
        }
        if y < max_y - 1 {
            adj.push(((x + 1).into(), (y + 1).into()));
        }
    }
    if y < max_y - 1 {
        adj.push((x.into(), (y + 1).into()));
    }
    if y > 0 {
        adj.push((x.into(), (y - 1).into()));
    }
    adj
}