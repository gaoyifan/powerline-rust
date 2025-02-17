use std::marker::PhantomData;
use std::env;

use super::Module;
use crate::{utils, Color, Powerline, Style};

pub struct Host<S: HostScheme> {
    show_on_local: bool,
    scheme: PhantomData<S>,
}

pub trait HostScheme {
    const HOSTNAME_FG: Color;
    const HOSTNAME_BG: Color;
    const HOSTNAME_SSH_FG: Color;
    const HOSTNAME_SSH_BG: Color;
}

impl<S: HostScheme> Host<S> {
    pub fn new() -> Host<S> {
        Host { show_on_local: true, scheme: PhantomData }
    }

    pub fn show_on_remote_shell() -> Host<S> {
        Host { show_on_local: false, scheme: PhantomData }
    }
}

impl<S: HostScheme> Module for Host<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if self.show_on_local || utils::is_remote_shell() {
            if let Ok(host) = hostname::get() {
                if env::var_os("SSH_CLIENT").is_some() {
                    powerline.add_segment(host.to_str().unwrap(), Style::simple(S::HOSTNAME_SSH_FG, S::HOSTNAME_SSH_BG));
                } else {
                    powerline.add_segment(host.to_str().unwrap(), Style::simple(S::HOSTNAME_FG, S::HOSTNAME_BG));
                }
            }
        }
    }
}
