use std::sync::{Arc, Mutex};

// ? TODO: Make this a spin-lock?

pub fn safe_lock<R, F, T>(state: &Arc<Mutex<T>>, f: F) -> R
    where F: FnOnce(&mut T) -> R {

    let l_state = state.clone();

    let r: R;

    match l_state.try_lock().as_mut() {

        Ok(lock) => {
            r = f(&mut **lock);
            drop(lock);
        },
        Err(_) => {
            unreachable!("Deadlock!");
        }

    }

    r

}
