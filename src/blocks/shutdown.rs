use crate::widgets::button::ButtonWidget;
use std::collections::BTreeMap;
use crate::config::Config;
use crate::util::{pseudo_uuid};
use crate::blocks::{Block, ConfigBlock, Update};

use crossbeam_channel::Sender;
use serde_derive::Deserialize;

use crate::scheduler::Task;
use crate::errors::*;
use system_shutdown::shutdown;
use crate::widget::I3BarWidget;
use crate::input::{I3BarEvent, MouseButton};
use crate::State;

pub struct Shutdown {
    id: String,
    button: ButtonWidget,
    color: String,

    #[allow(dead_code)]
    config: Config,

    #[allow(dead_code)]
    update_request: Sender<Task>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShutdownConfig {
    #[serde(default = "ShutdownConfig::default_color")]
    color: String,

    #[serde(default = "ShutdownConfig::default_color_overrides")]
    pub color_overrides: Option<BTreeMap<String, String>>,
}

impl ShutdownConfig {
    fn default_color() -> String {
        "red".to_owned()
    }

    fn default_color_overrides() -> Option<BTreeMap<String, String>> {
        None
    }
}

impl ConfigBlock for Shutdown {
    type Config = ShutdownConfig;

    fn new(
        block_config: Self::Config,
        config: Config,
        update_request: Sender<Task>
    ) -> Result<Self> {
        let id = pseudo_uuid();
        let button = ButtonWidget::new(config.clone(), &id.to_owned())
            .with_icon("shutdown");

    
        Ok(Shutdown {
            id,
            button,
            color: block_config.color,
            config,
            update_request,
        })
    }
}

impl Block for Shutdown {
    fn update(&mut self) -> Result<Option<Update>> {
        if self.color == "red" {
            self.button.set_state(State::Critical);
        }
        else if self.color == "yellow" {
            self.button.set_state(State::Warning);
        }
        else if self.color == "green" {
            self.button.set_state(State::Good);
        }
        else if self.color == "blue" {
            self.button.set_state(State::Info);
        }
        else if self.color == "grey" {
            self.button.set_state(State::Idle);
        }

        Ok(Some(Update::default()))
    }

    fn click(&mut self, event: &I3BarEvent) -> Result<()> {
        match event.button {
            MouseButton::Left => {
                match shutdown() {
                    Ok(_) => (),
                    Err(err) => self.button.set_text(format!("failed to shutdown: {}", err))               
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn view(&self) -> Vec<&dyn I3BarWidget> {
        vec![&self.button]
    }

    fn id(&self) -> &str {
        &self.id
    }
}
