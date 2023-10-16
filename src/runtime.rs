use crate::{
    configuration::Configuration,
    processor::Processor,
    scripts::{collection::ScriptsCollection, Script},
};

#[derive(Debug)]
pub struct Runtime<C: Configuration> {
    config: C,
    scripts: ScriptsCollection,
}

impl<C: Configuration> Runtime<C> {
    // TODO: This will need to be broken into many methods on the builder, maybe
    pub fn start(&mut self) -> Result<(), String> {
        // TODO: Should I save these? Will they need to be reprocessed?
        let processed_map = Processor::create_map_from(&self.scripts);

        // Now, set the processed values on the config object
        self.config.merge_script_map(processed_map)?;

        // TODO: THis should also start of the VM and probably other things
        // TODO: This should probably return something useful
        Ok(())
    }

    pub fn config(&self) -> &C {
        &self.config
    }

    pub fn config_mut(&mut self) -> &C {
        &mut self.config
    }

    pub fn scripts(&self) -> &ScriptsCollection {
        &self.scripts
    }

    pub fn scripts_mut(&mut self) -> &mut ScriptsCollection {
        &mut self.scripts
    }
}

pub struct RuntimeBuilder<C: Configuration> {
    config: Option<C>,
    scripts: ScriptsCollection,
}

#[derive(Debug)]
pub enum RuntimeBuilderError {
    MissingDependencies(Vec<String>),
}

impl<C: Configuration> Default for RuntimeBuilder<C> {
    fn default() -> Self {
        RuntimeBuilder {
            config: None,
            scripts: ScriptsCollection::default(),
        }
    }
}

impl<C: Configuration> RuntimeBuilder<C> {
    pub fn config<T: Into<C>>(mut self, config: T) -> Self {
        self.config = Some(config.into());
        self
    }

    pub fn add_script(&mut self, script: Script) -> &Self {
        self.scripts.add(script, None);
        self
    }

    // pub fn add_script_file(mut self, path: String) -> Self {
    //     self.add_script(Script::from_file(path));
    //     self
    // }

    pub fn build(self) -> Result<Runtime<C>, RuntimeBuilderError> {
        let mut missing_deps = Vec::new();

        if self.config.is_none() {
            missing_deps.push("config".to_string());
        }

        if self.scripts.is_empty() {
            missing_deps.push("scripts".to_string());
        }

        if missing_deps.is_empty() {
            Ok(Runtime {
                config: self.config.unwrap(),
                scripts: self.scripts,
            })
        } else {
            Err(RuntimeBuilderError::MissingDependencies(missing_deps))
        }
    }
}
