// Recursive version, goes through all possibilities
fn calc(shares: i64, index: usize, v: &[i64], cache: &mut HashMap<(i64, usize), i64>) -> i64 {
    if index == v.len() {
        return 0;
    }

    if let Some(&n) = cache.get(&(shares, index)) {
        return n;
    }

    let mut res = -v[index] + calc(shares + 1, index + 1, v, cache);
    res = max(res, calc(shares, index + 1, v, cache));
    res = max(res, (v[index] * shares) + calc(0, index + 1, v, cache));

    cache.insert((shares, index), res);
    res
}

// Same algorithm, iterative
fn calc_iter(v: &[i64], cache: &mut HashMap<(i64, usize), i64>) -> i64 {
    let mut stack = Vec::new();
    stack.push((0, 0, 0, Cell::new(0), Cell::new(0)));

    while !stack.is_empty() {
        let &(shares, index, incr, _, _) = stack.last().unwrap();
        let phase = stack.last().unwrap().4.get();
        if let Some(n) = cache.get(&(shares, index)) {
            let parent_phase = stack[stack.len()-2].4.get();
            stack[stack.len()-2].4.set(parent_phase + 1);
            let parent_best = stack[stack.len()-2].3.get();
            if parent_best < incr + n {
                stack[stack.len()-2].3.set(incr + n);
            }
            stack.pop();
            continue;
        }
        if index == v.len() {
            let parent_phase = stack[stack.len()-2].4.get();
            stack[stack.len()-2].4.set(parent_phase + 1);
            let parent_best = stack[stack.len()-2].3.get();
            if parent_best < incr {
                stack[stack.len()-2].3.set(incr);
            }
            cache.insert((shares, index), incr);
            stack.pop();
            continue;
        }
        match phase {
            0 => stack.push((shares, index + 1, 0, Cell::new(0), Cell::new(0))),
            1 => stack.push((shares + 1, index + 1, -v[index], Cell::new(0), Cell::new(0))),
            2 => stack.push((0, index + 1, v[index]*shares, Cell::new(0), Cell::new(0))),
            _ => {
                if stack.len() == 1 {
                    break;
                }
                let parent_phase = stack[stack.len()-2].4.get();
                stack[stack.len()-2].4.set(parent_phase + 1);
                let my_best = stack[stack.len()-1].3.get();
                let parent_best = stack[stack.len()-2].3.get();
                if parent_best < incr + my_best {
                    stack[stack.len()-2].3.set(incr + my_best);
                }
                cache.insert((shares, index), incr + my_best);
                stack.pop();
            }
        }
    }

    match stack.len() {
        0 => 0,
        1 => stack[0].3.get(),
        _ => panic!("This shouldn't happen"),
    }
}
