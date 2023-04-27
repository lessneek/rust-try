pub mod store {
    use std::{cell::Cell, collections::HashMap};

    pub type ItemId = usize;
    pub type ComputeFn<T> = Box<dyn Fn(&[T]) -> T>;
    pub type Items<T> = HashMap<ItemId, Item<T>>;

    pub struct ValueComputer<T> {
        deps: Vec<ItemId>,
        compute_fn: ComputeFn<T>,
    }

    impl<T: Copy> ValueComputer<T> {
        pub fn new(deps: &[ItemId], compute_fn: ComputeFn<T>) -> Self {
            Self {
                deps: deps.to_vec(),
                compute_fn,
            }
        }

        pub fn compute(&self, items: &Items<T>) -> Result<T, StoreError> {
            let dep_values = self.deps.iter().try_fold(vec![], |mut acc, &item_id| {
                acc.push(
                    items
                        .get(&item_id)
                        .ok_or(StoreError::ItemNotFound(item_id))?
                        .value(),
                );
                Ok(acc)
            })?;

            Ok((self.compute_fn)(&dep_values))
        }
    }

    pub struct InputItem<T> {
        id: ItemId,
        value: Cell<T>,
    }

    pub struct ComputedItem<T> {
        id: ItemId,
        value: Cell<T>,
        value_computer: ValueComputer<T>,
    }

    impl<T: Copy> ComputedItem<T> {
        pub fn update_value(&self, items: &Items<T>) -> Result<T, StoreError> {
            self.value.set(self.value_computer.compute(items)?);
            Ok(self.value.get())
        }
    }

    pub enum Item<T> {
        Input(InputItem<T>),
        Computed(ComputedItem<T>),
    }

    impl<T: Copy> Item<T> {
        pub fn value(&self) -> T {
            match self {
                Item::Input(item) => item.value.get(),
                Item::Computed(item) => item.value.get(),
            }
        }
    }

    impl<T> Item<T> {
        pub fn new_input(id: ItemId, value: T) -> Self {
            Item::Input(InputItem {
                id,
                value: Cell::new(value),
            })
        }

        pub fn new_computed(id: ItemId, value: T, value_computer: ValueComputer<T>) -> Self {
            Item::Computed(ComputedItem {
                id,
                value: Cell::new(value),
                value_computer,
            })
        }

        pub fn id(&self) -> ItemId {
            match self {
                Item::Input(item) => item.id,
                Item::Computed(item) => item.id,
            }
        }
    }

    pub struct Store<T> {
        items: HashMap<usize, Item<T>>,
    }

    #[derive(Debug)]
    pub enum StoreError {
        ItemNotFound(usize),
        ItemIdsEnded,
        UnableToSetComputedValue,
    }

    impl<T> Store<T> {
        pub fn new() -> Self {
            Self {
                items: Default::default(),
            }
        }
    }

    impl<T> Default for Store<T> {
        fn default() -> Self {
            Self {
                items: Default::default(),
            }
        }
    }

    impl<T: Copy> Store<T> {
        pub fn new_input_item(&mut self, value: T) -> Result<ItemId, StoreError> {
            let item_id = self.get_new_item_id()?;
            let item = Item::new_input(item_id, value);

            self.items.insert(item_id, item);

            Ok(item_id)
        }

        pub fn new_computed_item(
            &mut self,
            deps: &[ItemId],
            compute_fn: ComputeFn<T>,
        ) -> Result<ItemId, StoreError> {
            for item_id in deps.iter() {
                if !self.items.contains_key(item_id) {
                    return Err(StoreError::ItemNotFound(*item_id));
                }
            }

            let item_id = self.get_new_item_id()?;
            let value_computer = ValueComputer::new(deps, compute_fn);
            let value = value_computer.compute(&self.items)?;
            let item = Item::new_computed(item_id, value, value_computer);

            self.items.insert(item_id, item);

            Ok(item_id)
        }

        pub fn get_value(&self, item_id: ItemId) -> Result<T, StoreError> {
            let item = self
                .items
                .get(&item_id)
                .ok_or(StoreError::ItemNotFound(item_id))?;

            Ok(item.value())
        }

        pub fn set_value(&self, item_id: ItemId, value: T) -> Result<(), StoreError> {
            let item = self
                .items
                .get(&item_id)
                .ok_or(StoreError::ItemNotFound(item_id))?;

            match item {
                Item::Input(input_item) => {
                    input_item.value.set(value);
                    self.update(item)?;
                }
                Item::Computed(_) => return Err(StoreError::UnableToSetComputedValue),
            }
            Ok(())
        }

        pub fn update(&self, item: &Item<T>) -> Result<(), StoreError> {
            if let Item::Computed(item) = item {
                item.update_value(&self.items)?;
            }

            for dep_item in self.items.values() {
                if let Item::Computed(computed_dep_item) = dep_item {
                    if computed_dep_item.value_computer.deps.contains(&item.id()) {
                        self.update(dep_item)?;
                    }
                }
            }

            Ok(())
        }

        fn get_new_item_id(&self) -> Result<ItemId, StoreError> {
            (0..)
                .find(|item_id| !self.items.contains_key(item_id))
                .ok_or(StoreError::ItemIdsEnded)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::store::*;

    #[test]
    fn should_get_and_set_value() {
        let mut store = Store::<u64>::new();

        let input_id1 = store.new_input_item(42).unwrap();

        assert_eq!(store.get_value(input_id1).unwrap(), 42);

        store.set_value(input_id1, 139).unwrap();

        assert_eq!(store.get_value(input_id1).unwrap(), 139);
    }

    #[test]
    fn should_update() {
        let mut store = Store::<u64>::new();

        let input_id1 = store.new_input_item(1).unwrap();
        let input_id2 = store.new_input_item(3).unwrap();
        let input_id3 = store.new_input_item(9).unwrap();

        assert_eq!(store.get_value(input_id1).unwrap(), 1);
        assert_eq!(store.get_value(input_id2).unwrap(), 3);
        assert_eq!(store.get_value(input_id3).unwrap(), 9);

        let deps = [input_id1, input_id2, input_id3];

        let computed_item_id = store
            .new_computed_item(&deps, Box::new(|args| args.iter().sum()))
            .unwrap();

        assert_eq!(store.get_value(computed_item_id).unwrap(), 13);

        store.set_value(input_id1, 7).unwrap();
        assert_eq!(store.get_value(input_id1).unwrap(), 7);
        assert_eq!(store.get_value(computed_item_id).unwrap(), 19)
    }
}
