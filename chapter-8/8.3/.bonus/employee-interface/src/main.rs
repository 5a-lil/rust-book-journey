use std::collections::HashMap;

fn main() {
    let mut comps: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    println!("{comps:?}");
    add_employee_to_company(& mut comps, String::from("Comp1"), String::from("Dep1"), String::from("Empl1"));
    add_employee_to_company(& mut comps, String::from("Comp2"), String::from("Dep1"), String::from("Empl1"));
    add_employee_to_company(& mut comps, String::from("Comp1"), String::from("Dep2"), String::from("Empl2"));
    add_employee_to_company(& mut comps, String::from("Comp2"), String::from("Dep2"), String::from("Empl2"));
    println!("{comps:#?}");

    print_company(& mut comps, "Comp1".to_string());
    print_company(& mut comps, "Comp2".to_string());
    print_company(& mut comps, "Comp3".to_string());

    //print_company();


    // {
    //     let mut deps: HashMap<String, Vec<String>> = HashMap::new();

    //     {

    //         println!("{deps:?}");
    //         let test = deps.entry("Sales".to_string()).or_insert(Vec::new());
    //         test.push("new employee".to_string());
    //         println!("{deps:?}");
    //         let test = deps.entry("Sales".to_string()).or_insert(Vec::new());
    //         test.push("new employee".to_string());
    //         println!("{deps:?}");
    //         let test = deps.entry("Salos".to_string()).or_insert(Vec::new());
    //         test.push("new employee".to_string());
    //         println!("{deps:?}");
    //         let test = deps.entry("Sales".to_string()).or_insert(Vec::new());
    //         test.push("new employee 2".to_string());
    //         println!("{deps:?}");
    //     }

    //     {
    //         let ok_opt = deps.get("Sales");
    //         if let Some(empl_vec) = ok_opt {
    //             println!("Sales department:");
    //             for empl in empl_vec {
    //                 println!("{empl}");
    //             }
    //         } else {
    //             println!("This department does not exist");
    //         }

    //         // -----

    //         print_dep(& mut deps, "Sals".to_string());
    //         print_dep(& mut deps, "Sals".to_string());
    //     }
    // }
}

fn print_dep(deps: & HashMap<String, Vec<String>>, 
                dep_name: String) {
    let ok_opt = deps.get(&dep_name);
    if let Some(empl_vec) = ok_opt {
        println!("{dep_name} department employees:");
        for empl in empl_vec {
            println!("{empl}");
        }
    } else {
        println!("{dep_name} department does not exist");
    }
}

fn print_company(comps: & mut HashMap<String, HashMap<String, Vec<String>>>,
                    comp_name: String) {
    let ok_opt = comps.get_mut(&comp_name);
    if let Some(dep_map) = ok_opt {
        println!("{comp_name} company employees:");
        let temp = dep_map.clone();
        for (key, _) in dep_map {
            print_dep( &temp, key.to_string());
        }
    } else {
        println!("{comp_name} company does not exist");
    }
}

fn add_employee_to_dep(deps: & mut HashMap<String, Vec<String>>,
                dep_name: String, 
                empl_name: String) {
    let vec_value = deps.entry(dep_name).or_insert(Vec::new());
    vec_value.push(empl_name);
}

fn add_employee_to_company(comps: & mut HashMap<String, HashMap<String, Vec<String>>>,
                            comp_name: String,
                            dep_name: String,
                            empl_name: String) {
    let map_value = comps.entry(comp_name).or_insert(HashMap::new());
    add_employee_to_dep(map_value, dep_name, empl_name);

}
