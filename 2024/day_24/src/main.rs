use rand::Rng;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::read_to_string;
use std::rc::Rc;

#[derive(Debug)]
struct CircularConnectionError;

impl fmt::Display for CircularConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Logic gate network contains circular connections")
    }
}

struct LogicVariable {
    value: Option<bool>,
    user_gates: Vec<(Rc<RefCell<LogicGate>>, usize)>,
}

struct LogicGate {
    input1: Option<bool>,
    input2: Option<bool>,
    operation: Box<dyn Fn(bool, bool) -> bool>,
    output: Rc<RefCell<LogicVariable>>,
}

impl LogicVariable {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value: None,
            user_gates: Vec::new(),
        }))
    }

    pub fn new_with_value(value: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value: Some(value),
            user_gates: Vec::new(),
        }))
    }

    pub fn set_value(&mut self, value: bool) -> Result<(), CircularConnectionError> {
        self.value = Some(value);
        self.propagate()
    }

    pub fn add_successor(&mut self, successor: (Rc<RefCell<LogicGate>>, usize)) {
        self.user_gates.push(successor);
    }

    fn propagate(&mut self) -> Result<(), CircularConnectionError> {
        for (gate, input_index) in &mut self.user_gates {
            match gate.try_borrow_mut() {
                Ok(mut x) => x.set_input(*input_index, self.value.unwrap_or(false))?,
                Err(_) => return Err(CircularConnectionError {}),
            }
        }
        Ok(())
    }
}

impl LogicGate {
    pub fn new<F>(operation: F, variable: Rc<RefCell<LogicVariable>>) -> Rc<RefCell<Self>>
    where
        F: Fn(bool, bool) -> bool + 'static,
    {
        Rc::new(RefCell::new(Self {
            input1: None,
            input2: None,
            operation: Box::new(operation),
            output: variable,
        }))
    }

    pub fn new_and(variable: Rc<RefCell<LogicVariable>>) -> Rc<RefCell<Self>> {
        Self::new(|a, b| a && b, variable)
    }

    pub fn new_or(variable: Rc<RefCell<LogicVariable>>) -> Rc<RefCell<Self>> {
        Self::new(|a, b| a || b, variable)
    }

    pub fn new_xor(variable: Rc<RefCell<LogicVariable>>) -> Rc<RefCell<Self>> {
        Self::new(|a, b| a ^ b, variable)
    }

    pub fn set_input(&mut self, index: usize, value: bool) -> Result<(), CircularConnectionError> {
        match index {
            0 => self.input1 = Some(value),
            1 => self.input2 = Some(value),
            _ => panic!("Invalid input index"),
        }
        self.activate()
    }

    fn activate(&mut self) -> Result<(), CircularConnectionError> {
        if let (Some(a), Some(b)) = (self.input1, self.input2) {
            self.output.borrow_mut().set_value((self.operation)(a, b))?;
        }
        Ok(())
    }
}

fn parse_file(
    filename: &str,
) -> (
    HashMap<String, Rc<RefCell<LogicVariable>>>,
    HashMap<String, Rc<RefCell<LogicGate>>>,
    HashSet<String>,
) {
    let mut input_variables: HashSet<String> = HashSet::new();
    let mut variables: HashMap<String, Rc<RefCell<LogicVariable>>> = HashMap::new();
    let mut gates: HashMap<String, Rc<RefCell<LogicGate>>> = HashMap::new();
    let mut connections: Vec<(String, String, String)> = Vec::new();

    for line in read_to_string(filename)
        .expect("Failed to read file")
        .lines()
    {
        if line.contains(":") {
            let parts = line.split_once(":").unwrap();
            let name = parts.0.trim().to_string();
            let value = parts.1.trim().parse::<u32>().unwrap() != 0;
            let input_var = LogicVariable::new_with_value(value);
            variables.insert(name.clone(), input_var.clone());
            input_variables.insert(name);
        } else if line.contains("->") {
            let parts = line.split_once("->").unwrap();
            let name = parts.1.trim().to_string();

            let gate_parts: Vec<&str> = parts.0.split_whitespace().collect();
            let input1_name = gate_parts[0].to_string();
            let operation = gate_parts[1];
            let input2_name = gate_parts[2].to_string();

            let output_var = LogicVariable::new();
            variables.insert(name.clone(), output_var.clone());

            let gate = match operation {
                "AND" => LogicGate::new_and(output_var.clone()),
                "OR" => LogicGate::new_or(output_var.clone()),
                "XOR" => LogicGate::new_xor(output_var.clone()),
                _ => panic!("Unknown operation: {}", operation),
            };

            gates.insert(name.clone(), gate);
            connections.push((name, input1_name, input2_name));
        }
    }

    for (gate_name, input1_name, input2_name) in connections {
        let gate = gates.get(&gate_name).unwrap();

        if let Some(input1) = variables.get_mut(&input1_name) {
            input1.borrow_mut().add_successor((gate.clone(), 0));
        } else {
            panic!("Unknown input: {}", input1_name);
        }

        if let Some(input2) = variables.get_mut(&input2_name) {
            input2.borrow_mut().add_successor((gate.clone(), 1));
        } else {
            panic!("Unknown input: {}", input2_name);
        }
    }

    (variables, gates, input_variables)
}

fn forward_input(
    variables: &mut HashMap<String, Rc<RefCell<LogicVariable>>>,
    input_variables: &HashSet<String>,
) -> Result<(), CircularConnectionError> {
    for var_name in input_variables {
        variables.get(var_name).unwrap().borrow_mut().propagate()?;
    }
    Ok(())
}

fn num_outputs(variables: &HashMap<String, Rc<RefCell<LogicVariable>>>) -> usize {
    let mut idx = 0;
    while variables.get(&format!("z{:02}", idx)).is_some() {
        idx += 1;
    }
    idx
}

fn get_output_value(variables: &HashMap<String, Rc<RefCell<LogicVariable>>>) -> u64 {
    let mut value = 0;
    let mut idx = 0;
    while let Some(var) = variables.get(&format!("z{:02}", idx)) {
        value += (1 << idx) * var.borrow().value.unwrap_or(false) as u64;
        idx += 1;
    }

    value
}

fn target_sum(variables: &HashMap<String, Rc<RefCell<LogicVariable>>>) -> u64 {
    let mut idx = 0;

    let mut value_x = 0;
    let mut value_y = 0;

    while let Some(var) = variables.get(&format!("x{:02}", idx)) {
        value_x += (1 << idx) * var.borrow().value.unwrap_or(false) as u64;
        idx += 1;
    }

    idx = 0;
    while let Some(var) = variables.get(&format!("y{:02}", idx)) {
        value_y += (1 << idx) * var.borrow().value.unwrap_or(false) as u64;
        idx += 1;
    }

    value_x + value_y
}

fn find_first_mistake_output(target: u64, actual: u64) -> Option<usize> {
    let mut bit_idx = 0;
    let mut bit_value = 1;
    while bit_value <= target || bit_value <= actual {
        if target & bit_value != actual & bit_value {
            return Some(bit_idx);
        }
        bit_idx += 1;
        bit_value <<= 1;
    }

    None
}

fn check_gates(
    variables: &mut HashMap<String, Rc<RefCell<LogicVariable>>>,
    input_variables: &HashSet<String>,
    num_tests: u32,
) -> bool {
    let initial_states: HashMap<String, Option<bool>> = input_variables
        .iter()
        .map(|x| (x.clone(), variables.get(x).unwrap().borrow().value.clone()))
        .collect();
    forward_input(variables, input_variables).unwrap();
    assert_eq!(target_sum(variables), get_output_value(variables));

    let mut result = true;
    for _ in 0..num_tests {
        assign_random_inputs(variables, input_variables);
        forward_input(variables, input_variables).unwrap();
        if target_sum(variables) != get_output_value(variables) {
            result = false;
            break;
        }
    }

    for (k, v) in initial_states {
        variables.get(&k).unwrap().borrow_mut().value = v;
    }

    result
}

fn switch_gate_outputs(
    name_1: &String,
    name_2: &String,
    gates: &mut HashMap<String, Rc<RefCell<LogicGate>>>,
) {
    let gate_1 = gates.get(name_1).expect("Gate not found").clone();
    let gate_2 = gates.get(name_2).expect("Gate not found").clone();

    let output_1 = gate_1.borrow_mut().output.clone();
    let output_2 = gate_2.borrow_mut().output.clone();

    gate_1.borrow_mut().output = output_2;
    gate_2.borrow_mut().output = output_1;

    gates.insert(name_1.clone(), gate_2.clone());
    gates.insert(name_2.clone(), gate_1.clone());
}

fn _find_fixing_switches_inner(
    target: u64,
    all_keys: &Vec<String>,
    variables: &mut HashMap<String, Rc<RefCell<LogicVariable>>>,
    gates: &mut HashMap<String, Rc<RefCell<LogicGate>>>,
    input_variables: &HashSet<String>,
    blacklist: &mut HashSet<String>,
    min_mistake_position: usize,
    num_switches: u32,
) -> Option<Vec<(String, String)>> {
    if forward_input(variables, input_variables).is_err() {
        return None;
    }

    let mistake_position_opt = find_first_mistake_output(target, get_output_value(variables));

    if mistake_position_opt.is_some_and(|x| x < min_mistake_position || num_switches == 0) {
        return None;
    }

    if mistake_position_opt.is_none() && num_switches == 0 {
        if check_gates(variables, input_variables, 100) {
            let mut candidate: Vec<_> = blacklist.iter().cloned().collect();
            candidate.sort();
            println!("Found candidate! {:?}", candidate.join(","));
            return Some(Vec::new());
        }
        return None;
    }

    if mistake_position_opt.is_none() && num_switches > 0 {
        println!("Fixed everything with less than maximum switches. Fill up with no-op switches.");
        for (idx, var_1) in all_keys.iter().enumerate() {
            if blacklist.contains(var_1) {
                continue;
            }
            for var_2 in all_keys.iter().skip(idx + 1) {
                if blacklist.contains(var_2) {
                    continue;
                }
                switch_gate_outputs(var_1, var_2, gates);
                blacklist.insert(var_1.clone());
                blacklist.insert(var_2.clone());
                let partial_result = _find_fixing_switches_inner(
                    target,
                    all_keys,
                    variables,
                    gates,
                    input_variables,
                    blacklist,
                    num_outputs(variables) + 1,
                    num_switches - 1,
                );
                blacklist.remove(var_1);
                blacklist.remove(var_2);
                switch_gate_outputs(var_1, var_2, gates);
                if let Some(mut fix) = partial_result {
                    fix.push((var_1.to_string(), var_2.to_string()));
                    return Some(fix);
                }
            }
        }
    }

    let mistake_position = mistake_position_opt.unwrap();

    let mut switch_quality = HashMap::new();
    for (idx, var_1) in all_keys.iter().enumerate() {
        if blacklist.contains(var_1) {
            continue;
        }
        for var_2 in all_keys.iter().skip(idx + 1) {
            if blacklist.contains(var_2) {
                continue;
            }
            switch_gate_outputs(&var_1, &var_2, gates);
            if forward_input(variables, input_variables).is_ok() {
                let new_mistake_position =
                    match find_first_mistake_output(target, get_output_value(variables)) {
                        Some(pos) => pos,
                        None => num_outputs(variables) + 1,
                    };
                if new_mistake_position > mistake_position {
                    switch_quality
                        .insert((var_1.to_string(), var_2.to_string()), new_mistake_position);
                }
            }
            switch_gate_outputs(&var_1, &var_2, gates);
        }
    }

    while !switch_quality.is_empty() {
        let ((var_1, var_2), mistake_pos) = switch_quality.iter().max_by_key(|x| x.1).unwrap();
        if num_switches > 1 {
            println!(
                "Investigate switch {} <-> {} leading to mistake pos {} with {} switches remaining.",
                var_1,
                var_2,
                mistake_pos,
                num_switches - 1,
            );
        }
        switch_gate_outputs(&var_1, &var_2, gates);
        blacklist.insert(var_1.clone());
        blacklist.insert(var_2.clone());
        let partial_result = _find_fixing_switches_inner(
            target,
            all_keys,
            variables,
            gates,
            input_variables,
            blacklist,
            *mistake_pos,
            num_switches - 1,
        );
        blacklist.remove(var_1);
        blacklist.remove(var_2);
        switch_gate_outputs(&var_1, &var_2, gates);

        if let Some(mut fix) = partial_result {
            fix.push((var_1.to_string(), var_2.to_string()));
            return Some(fix);
        }

        switch_quality.remove(&(var_1.clone(), var_2.clone()));
    }

    None
}

fn find_fixing_switches(
    variables: &mut HashMap<String, Rc<RefCell<LogicVariable>>>,
    gates: &mut HashMap<String, Rc<RefCell<LogicGate>>>,
    input_variables: &HashSet<String>,
    num_switches: u32,
) -> Option<Vec<(String, String)>> {
    let all_keys: Vec<String> = variables
        .keys()
        .filter(|k| !input_variables.contains(*k))
        .cloned()
        .collect();
    _find_fixing_switches_inner(
        target_sum(variables),
        &all_keys,
        variables,
        gates,
        input_variables,
        &mut HashSet::new(),
        0,
        num_switches,
    )
}

fn flatten_switches(switches: &Vec<(String, String)>) -> Vec<String> {
    let mut sorted_switches = Vec::with_capacity(switches.len() * 2);
    for (a, b) in switches {
        sorted_switches.push(a.clone());
        sorted_switches.push(b.clone());
    }
    sorted_switches.sort();

    sorted_switches
}

fn assign_random_inputs(
    variables: &mut HashMap<String, Rc<RefCell<LogicVariable>>>,
    input_variables: &HashSet<String>,
) {
    let mut rng = rand::thread_rng();
    for name in input_variables {
        variables
            .get(name)
            .unwrap()
            .borrow_mut()
            .set_value(rng.gen_bool(0.5))
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decimal_value() {
        let (mut variables, _, input_variables) = parse_file("testinput.txt");
        forward_input(&mut variables, &input_variables).unwrap();
        assert_eq!(get_output_value(&variables), 4);
    }

    #[test]
    fn test_decimal_value_2() {
        let (mut variables, _, input_variables) = parse_file("testinput2.txt");
        forward_input(&mut variables, &input_variables).unwrap();
        assert_eq!(get_output_value(&variables), 2024);
    }

    #[test]
    fn test_target_value() {
        let (variables, _, _) = parse_file("testinput.txt");
        assert_eq!(target_sum(&variables), 9);
    }

    #[test]
    fn test_target_value_2() {
        let (variables, _, _) = parse_file("testinput2.txt");
        assert_eq!(target_sum(&variables), 44);
    }

    #[test]
    fn test_mistake() {
        let (mut variables, _, input_variables) = parse_file("testinput.txt");
        forward_input(&mut variables, &input_variables).unwrap();
        assert_eq!(
            find_first_mistake_output(target_sum(&variables), get_output_value(&variables)),
            Some(0)
        );
    }

    #[test]
    fn test_mistake_2() {
        let (mut variables, _, input_variables) = parse_file("testinput2.txt");
        forward_input(&mut variables, &input_variables).unwrap();
        assert_eq!(
            find_first_mistake_output(target_sum(&variables), get_output_value(&variables)),
            Some(2)
        );
    }

    #[test]
    fn test_switch() {
        let (mut variables, mut gates, input_variables) = parse_file("testinput.txt");
        switch_gate_outputs(&String::from("z01"), &String::from("z02"), &mut gates);
        forward_input(&mut variables, &input_variables).unwrap();
        assert_eq!(get_output_value(&variables), 2);
    }

    #[test]
    fn test_flatten() {
        let switches = vec![
            (String::from("b"), String::from("c")),
            (String::from("a"), String::from("d")),
        ];
        assert_eq!(
            flatten_switches(&switches),
            vec!["a", "b", "c", "d"]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );
    }
}

fn main() {
    let (mut variables, mut gates, input_variables) = parse_file("input.txt");
    forward_input(&mut variables, &input_variables).unwrap();
    println!("Challenge 1: {}", get_output_value(&variables));

    if forward_input(&mut variables, &input_variables).is_ok() {
        println!(
            "Initial mistake position: {}",
            &find_first_mistake_output(target_sum(&variables), get_output_value(&variables))
                .unwrap_or(0)
        );
    }
    let switches =
        find_fixing_switches(&mut variables, &mut gates, &input_variables, 4).unwrap_or_default();
    dbg!(&switches);

    println!("Challenge 2: {}", flatten_switches(&switches).join(","));
}
