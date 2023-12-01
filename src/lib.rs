pub mod ecs {
    use std::any::Any;

    pub struct Entity {
        id: usize,
        name: String,
        components: Vec<Box<dyn Component>>
    }

    impl Entity {
        pub fn add_component<T: Component + 'static>(&mut self, component: T) {
            self.components.push(Box::new(component));
        }

        pub fn get_component<T: Component + 'static>(&self) -> Option<&T> {
            let position = self.components.iter().position(|x| (**x).as_any().is::<T>())?;
            let component = self.components.get(position)?;
            let downcast = (**component).as_any().downcast_ref::<T>()?;
            Some(downcast)
        }
    }

    #[derive(Default)]
    pub struct EntityBuilder {
        id: Option<usize>,
        name: Option<String>,
        components: Vec<Box<dyn Component>>,
    }

    impl EntityBuilder {
        pub fn set_id(mut self, id: usize) -> Self {
            self.id = Some(id);
            self
        }

        pub fn set_name(mut self, name: &str) -> Self {
            self.name = Some(name.to_string());
            self
        }

        pub fn add_component<T: Component + 'static>(mut self, component: T) -> Self {
            self.components.push(Box::new(component));
            self
        }

        pub fn build(self) -> Result<Entity, &'static str> {
            let id = self.id.ok_or("Please set an Entity ID")?;
            let name = self.name.unwrap_or(format!("Entity - {}", id));

            let entity = Entity {
                id,
                name,
                components: self.components,
            };

            Ok(entity)
        }
    }

    pub trait Component {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    #[derive(Default)]
    pub struct System {
        entities: Vec<Entity>,
    }

    impl System {
        pub fn create_entity<F>(&mut self, builder: F) -> &Entity where F: FnOnce() -> Entity {
            let entity = builder();
            self.entities.push(entity);
            let pos = self.entities.len();
            let entity = self.entities.get(pos);
            entity.expect("Entity failed to create")
        }

        pub fn get_entity(&self, id: usize) -> Option<&Entity> {
            let pos = self.entities.iter().position(|x| x.id == id)?;
            self.entities.get(pos)
        }
    }

}