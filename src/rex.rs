pub mod entity {
    use crate::rex::component::Component;

    #[derive(Default)]
    pub struct Entity {
        id: usize,
        name: String,
        components: Vec<Box<dyn Component>>,
    }

    impl Entity {
        pub fn set_id(&mut self, id: usize) {
            self.id = id;
        }

        pub fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }

        pub fn add_component<T: Default + 'static>(&mut self) {
            self.components.push(Box::new(T::default()));
        }

        pub fn get_component<T: 'static>(&self) -> Option<&T> {
            let pos = self
                .components
                .iter()
                .position(|x| (**x).as_any().is::<T>())?;
            let value = self.components.get(pos)?;
            let cast = (**value).as_any().downcast_ref::<T>()?;
            Some(cast)
        }

        pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
            let pos = self
                .components
                .iter()
                .position(|x| (**x).as_any().is::<T>())?;
            let value = self.components.get_mut(pos)?;
            let cast = (**value).as_any_mut().downcast_mut::<T>()?;
            Some(cast)
        }
    }
}

pub mod component {
    use std::any::Any;

    pub trait Component {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    impl<T: 'static> Component for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
}
