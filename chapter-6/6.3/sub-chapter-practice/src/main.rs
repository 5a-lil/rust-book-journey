enum BoxingBelt {
    WBC(String),
    WBA(String),
    WBO(String),
    IBF(String)
}

fn main() {
    let belt: BoxingBelt = BoxingBelt::
                            WBC(String::
                                from("Khalil"));

    let BoxingBelt::WBC(champ) = belt else {
        println!("Noob champ...");
        return;
    };

    println!("And the champ iiiiiiiiiisssss {champ}");
          
}

