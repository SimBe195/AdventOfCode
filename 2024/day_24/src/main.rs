use rand::Rng;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::{read_to_string, write};
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
    operation_name: String,
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
    pub fn new<F>(
        operation_name: String,
        operation: F,
        variable: Rc<RefCell<LogicVariable>>,
    ) -> Rc<RefCell<Self>>
    where
        F: Fn(bool, bool) -> bool + 'static,
    {
        Rc::new(RefCell::new(Self {
            input1: None,
            input2: None,
            operation_name,
            operation: Box::new(operation),
            output: variable,
        }))
    }

    pub fn new_and(variable: Rc<RefCell<LogicVariable>>) -> Rc<RefCell<Self>> {
        Self::new("AND".to_string(), |a, b| a && b, variable)
    }

    pub fn new_or(variable: Rc<RefCell<LogicVariable>>) -> Rc<RefCell<Self>> {
        Self::new("OR".to_string(), |a, b| a || b, variable)
    }

    pub fn new_xor(variable: Rc<RefCell<LogicVariable>>) -> Rc<RefCell<Self>> {
        Self::new("XOR".to_string(), |a, b| a ^ b, variable)
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

fn dump_dot(
    variables: &HashMap<String, Rc<RefCell<LogicVariable>>>,
    gates: &HashMap<String, Rc<RefCell<LogicGate>>>,
    input_variables: &HashSet<String>,
) {
    let mut out_str = "digraph G {\nlayout=neato;\n".to_string();

    let mut x_positions: HashMap<String, u32> = HashMap::new();
    let mut y_positions: HashMap<String, u32> = HashMap::new();
    let mut shapes: HashMap<String, String> = HashMap::new();

    let mut var_to_name_map: HashMap<*mut LogicVariable, String> = HashMap::new();

    for (name, var) in variables {
        var_to_name_map.insert(var.as_ptr(), name.clone());
    }

    for (name, var) in variables {
        for (gate, _) in &var.borrow().user_gates {
            out_str.push_str(
                format!(
                    "{} -> {};\n",
                    name,
                    var_to_name_map.get(&gate.borrow().output.as_ptr()).unwrap()
                )
                .as_str(),
            );
        }
    }

    for (name, gate) in gates {
        shapes.insert(
            name.clone(),
            match gate.borrow().operation_name.as_str() {
                "AND" => "triangle".to_string(),
                "OR" => "square".to_string(),
                "XOR" => "diamond".to_string(),
                _ => panic!(),
            },
        );
    }

    for name in input_variables {
        let pos = name[1..].parse::<u32>().unwrap();
        shapes.insert(name.clone(), "circle".to_string());
        if name.starts_with("x") {
            x_positions.insert(name.to_string(), 2 * pos);
            y_positions.insert(name.to_string(), 0);
        } else {
            x_positions.insert(name.to_string(), 2 * pos + 1);
            y_positions.insert(name.to_string(), 1);
        }
    }

    for (name, gate) in gates {
        if name.starts_with("z") {
            let pos = name[1..].parse::<u32>().unwrap();
            x_positions.insert(name.to_string(), 2 * pos);
            y_positions.insert(name.to_string(), 6);
        } else if gate.borrow().operation_name == "XOR" {
            y_positions.insert(name.to_string(), 3);
        } else if gate.borrow().operation_name == "OR" {
            y_positions.insert(name.to_string(), 5);
        } else {
            y_positions.insert(name.to_string(), 2);
        }
    }

    let mut missing_gates: HashSet<&String> = gates.keys().collect();
    for key in x_positions.keys() {
        missing_gates.remove(key);
    }

    while !missing_gates.is_empty() {
        let mut new_x_positions = HashMap::new();
        for (key, pos) in &x_positions {
            if key.starts_with("y") {
                continue;
            }
            for (out_gate, _) in &variables.get(key).unwrap().borrow().user_gates {
                let out_name = var_to_name_map
                    .get(&out_gate.borrow().output.as_ptr())
                    .unwrap();
                if !x_positions.contains_key(out_name) {
                    new_x_positions.insert(out_name.clone(), *pos);
                }
            }
        }
        for key in new_x_positions.keys() {
            missing_gates.remove(key);
        }
        x_positions.extend(new_x_positions);
    }

    for name in input_variables {
        out_str.push_str(
            format!(
                "{} [shape={}, pos=\"{},{}!\"];\n",
                name,
                shapes.get(name).unwrap(),
                x_positions.get(name).unwrap(),
                y_positions.get(name).unwrap()
            )
            .as_str(),
        );
    }

    for name in gates.keys() {
        out_str.push_str(
            format!(
                "{} [shape={}, pos=\"{},{}!\"];\n",
                name,
                shapes.get(name).unwrap(),
                x_positions.get(name).unwrap(),
                y_positions.get(name).unwrap()
            )
            .as_str(),
        );
    }

    out_str.push_str("}\n");
    write("graph.dot", out_str).unwrap();
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

    let switches = vec![
        ("vcf".to_string(), "z10".to_string()),
        ("z17".to_string(), "fhg".to_string()),
        ("z39".to_string(), "tnc".to_string()),
        ("fsq".to_string(), "dvb".to_string()),
    ];
    for (left, right) in &switches {
        switch_gate_outputs(left, right, &mut gates);
    }
    if check_gates(&mut variables, &input_variables, 100) {
        dump_dot(&variables, &gates, &input_variables);
        println!("Challenge 2: {}", (flatten_switches(&switches)).join(","));
    }
}
