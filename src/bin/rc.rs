use std::rc::Rc;
use std::cell::{RefCell, RefMut};

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64  // Mhz
}

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
        GroundStation {
            radio_freq: 87.65
        }
    ));
    let base_ptr  = &base as *const Rc<RefCell<GroundStation>>;

    println!("base: {:?} - {:?}", base, base_ptr);

    {
        let mut base_2 = base.borrow_mut();
        let base_ptr_2  = &base_2 as *const RefMut<GroundStation>;
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?} {:?}", base_2, base_ptr_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    let base_ptr_2  = &base as *const Rc<RefCell<GroundStation>>;
    let base_ptr_3  = &base_3 as *const RefMut<GroundStation>;

    println!("base: {:?} - {:?}", base, base_ptr_2);

    println!("base X: {:?}", base);
    println!("base_3: {:?} {:?}", base_3, base_ptr_3);

    let base_clone = base.clone();
    let base_clone_ptr = &base as * const Rc<RefCell<GroundStation>>;
    println!("Base Clone {:?} {:?}", base_clone, base_clone_ptr);

    let GS = RefCell::new(
        GroundStation {
            radio_freq: 123.0
        }
    );
    let GS_ptr = &GS as *const RefCell<GroundStation>;
    println!("GS {:?} {:?}", GS, GS_ptr);

    let mut GS_mut = GS.borrow_mut();
    GS_mut.radio_freq = 100.100;
    let GS_mut_ptr = &GS_mut as *const RefMut<GroundStation>;

    println!("GS mut {:?} {:?}", GS_mut, GS_mut_ptr);


    /*
    RefCell.borrow_mut cannot be borrow multiple times at the same scope (will caused panic)

    let mut GS_mut_2 = GS.borrow_mut();
    GS_mut_2.radio_freq = 120.200;

    let GS_mut_2_ptr = &GS_mut_2 as *const RefMut<GroundStation>;

    println!("GS mut 2 {:?} {:?}", GS_mut_2, GS_mut_2_ptr);
     */

}