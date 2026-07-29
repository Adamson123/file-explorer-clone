use std::any::Any;

pub struct StatesManager {
    pub states: Vec<Box<dyn Any + Send + Sync>>,
}

impl StatesManager {
    pub fn add_state<T>(&mut self, state: T)
    where
        T: Any + Send + Sync,
    {
        self.states.push(Box::new(state));
    }

    pub fn get_state<T: Any>(&self) -> Option<&T> {
        for state in &self.states {
            let res = state.downcast_ref::<T>();
            if let Some(_) = res {
                return res;
            }
        }

        None
    }
}
