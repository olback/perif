use std::sync::{
    Arc,
    Mutex
};

// Spinlocks in user-space is not recomended

pub fn safe_lock<R, F, T>(state: &Arc<Mutex<T>>, f: F) -> R
    where F: FnOnce(&mut T) -> R {

    let l_state = state.clone();

    let r: R;

    loop {

        match l_state.try_lock().as_mut() {

            Ok(lock) => {
                r = f(&mut **lock);
                drop(lock);
                break;
            },
            Err(_) => std::sync::atomic::spin_loop_hint()

        }

    }

    r

}
