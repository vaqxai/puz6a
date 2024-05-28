use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Man {
    time: i32,
    name: char
}

impl Man {
    pub fn new(name: char, time: i32) -> Self {
        Man { name, time }
    }

    pub fn from_slice(men: &[(char, i32)]) -> Vec::<Man> {
        
        let mut res = Vec::<Man>::new();

        for man in men.iter() {
            res.push(Man::new(man.0, man.1));
        }

        res
    }

    pub fn get_by_name_from_vec(men: &Vec::<Man>, name: char) -> (isize, i32) {

        let mut man_i = -1;
        let mut man_time = -1;

        for (i, man) in men.iter().enumerate() {
            if man.name == name {
                man_i = i as isize;
                man_time = man.time;
            }
        }

        (man_i, man_time)
    }
}

struct Bridge {
    left_side: Vec<Man>,
    right_side: Vec<Man>
}

impl Bridge {
    pub fn new(left_side: Vec::<Man>) -> Self {
        Bridge { left_side, right_side: Vec::<Man>::new() }
    }

    pub fn cross_sel_ret(&mut self, m1_name: char, m2_name: char, ret_name: char) -> i32 {
        let (man1_i, man1_time) = Man::get_by_name_from_vec(&self.left_side, m1_name);
        let (man2_i, man2_time) = Man::get_by_name_from_vec(&self.left_side, m2_name);

        if man1_i == -1 || man2_i == -2 { panic!("m1_name or m2_name invalid"); }

        let journey_time = std::cmp::max(man1_time, man2_time);

        let bigger_i = std::cmp::max(man1_i, man2_i);
        let smaller_i = std::cmp::min(man1_i, man2_i);

        self.right_side.push(self.left_side.remove(bigger_i as usize));
        self.right_side.push(self.left_side.remove(smaller_i as usize));

        // return journey

        if ret_name == ' ' { return 0 }

        let (retman_i, retman_time) = Man::get_by_name_from_vec(&self.right_side, ret_name);

        if retman_i == -1 { panic!("ret_name invalid"); }

        self.left_side.push(self.right_side.remove(retman_i as usize));

        journey_time + retman_time
    }

    pub fn cross(&mut self, m1_name: char, m2_name: char) -> (i32, char) {

        let mut man1_i = -1;
        let mut man1_time = -1;
        let mut man2_i = -1;
        let mut man2_time = -1;

        for (i, man) in self.left_side.iter().enumerate() {
            if man.name == m1_name {
                man1_i = i as isize;
                man1_time = man.time;
            }
            else if man.name == m2_name {
                man2_i = i as isize;
                man2_time = man.time;
            }
        }

        if man1_i == -1 || man2_i == -2 { panic!("m1_name or m2_name invalid"); }

        let mut total_time = man1_time + man2_time; // cost of first journey is the greater one, cost
                                                // of return is the lesser one
        let mut lesser_man = man1_i;
        let mut greater_man = man2_i;
        if man2_time < man1_time { lesser_man = man2_i; greater_man = man1_i; }

        let mut returned_name = self.left_side.get(lesser_man as usize).unwrap().name;
        let transferred_man = self.left_side.remove(greater_man as usize);

        if self.left_side.len() == 1 {
            total_time = std::cmp::max(man1_time, man2_time);
            returned_name = ' ';
            println!("{} and {} crossed the bridge. nobody returned. took {} time.", m1_name, m2_name, total_time);
            self.right_side.push(self.left_side.remove(0));
        } else {
            println!("{} crossed the bridge, {} returned. took {} time", transferred_man.name, returned_name, total_time);
        }
        self.right_side.push(transferred_man);

        (total_time, returned_name)

    }

    pub fn print_sides(&self) {
        print!("Left: ");
        for man in self.left_side.iter() {
            print!("{} ", man.name);
        }
        println!();

        print!("Right: ");
        for man in self.right_side.iter(){
            print!("{} ", man.name);
        }
        println!();
    }

    pub fn everyone_went_through(&self) -> bool {
        self.left_side.is_empty()
    }
    
    pub fn get_first_two_names(&self) -> (char, char) {
        if self.left_side.len() == 1 {
            (self.left_side.first().unwrap().name, ' ')
        }
        else if self.left_side.is_empty() {
            (' ', ' ')
        } else {
            (self.left_side.first().unwrap().name, self.left_side.get(1).unwrap().name)
        }
    }
}

fn main() {

    let mut men = Man::from_slice(&[('A', 1), ('B', 3), ('C', 4), ('D', 6), ('E', 8), ('F', 9)]);
    let mut bridge = Bridge::new(men);

    bridge.print_sides();
    let mut total_cost = 0;

    /*
    total_cost += bridge.cross_sel_ret('A', 'B', 'A');
    total_cost += bridge.cross_sel_ret('C', 'D', 'C');
    total_cost += bridge.cross_sel_ret('E', 'F', 'B');
    total_cost += bridge.cross_sel_ret('A', 'C', 'A');
    total_cost += bridge.cross_sel_ret('A', 'B', ' ');
    
    // cost=32

    */

    /*

    // najpierw idą najszybsi, żeby mieć szybkiego po prawej stronie
    // potem idą najwolniejsi, wraca szybki
    // potem idą znów najszybsi, wraca najszybszy
    // najszybszy przeprowadza najwolniejszych pozostałych, wraca znów najszybszych
    // dwóch ostatnich to najszybsi, i wracają razem
    // kolejność A+B -> A, E+F -> B, A+B -> A, C+D -> B, A+B

    total_cost += bridge.cross_sel_ret('A','B','A');
    total_cost += bridge.cross_sel_ret('E','F','B');
    total_cost += bridge.cross_sel_ret('A','B','A');
    total_cost += bridge.cross_sel_ret('C','D','B');
    total_cost += bridge.cross_sel_ret('A','B',' ');

    // cost = 29
    */

    /*
    total_cost += bridge.cross_sel_ret('A','B','A');
    total_cost += bridge.cross_sel_ret('C','D','B');
    total_cost += bridge.cross_sel_ret('E','F','C');
    total_cost += bridge.cross_sel_ret('A','C','A');
    total_cost += bridge.cross_sel_ret('A','B',' ');

    // cost = 31
    */

    bridge.print_sides();

    println!("Total cost: {}", total_cost);
}
