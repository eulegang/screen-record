use std::{path::Path, process::Command};

pub struct ScreenGrab {
    pub size: Option<(u16, u16)>,
    pub offset: Option<(u16, u16)>,
    pub framerate: Option<u8>,
    pub screen: String,
}

pub struct FileOutput<'a> {
    pub path: &'a Path,
}

pub struct AudioOutput<'a> {
    pub id: &'a str,
}

pub struct Record {
    args: Vec<String>,
}

pub trait ToArgs {
    fn args(&self) -> Vec<String>;
}

impl Default for ScreenGrab {
    fn default() -> Self {
        let screen = std::env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
        let framerate = Some(25);

        ScreenGrab {
            size: None,
            offset: None,
            framerate,
            screen,
        }
    }
}

impl Default for Record {
    fn default() -> Self {
        Record {
            args: vec!["-y".to_string()],
        }
    }
}

impl Record {
    pub fn push<T>(&mut self, options: T)
    where
        T: ToArgs,
    {
        self.args.extend(options.args());
    }

    pub fn run(self) -> std::io::Result<bool> {
        let status = Command::new("ffmpeg").args(self.args).status()?;

        Ok(status.success())
    }
}

impl<'a> ToArgs for FileOutput<'a> {
    fn args(&self) -> Vec<String> {
        vec![self.path.display().to_string()]
    }
}

impl ToArgs for ScreenGrab {
    fn args(&self) -> Vec<String> {
        let mut args = vec!["-f".to_string(), "x11grab".to_string()];
        let screen = &self.screen;

        if let Some((width, height)) = self.size {
            args.push("-video_size".to_string());
            args.push(format!("{width}x{height}"));
        }

        if let Some(rate) = self.framerate {
            args.push("-framerate".to_string());
            args.push(format!("{rate}"));
        }

        let offset = self
            .offset
            .map(|(x, y)| format!("+{x}+{y}"))
            .unwrap_or_default();

        args.push("-i".to_string());
        args.push(format!("{screen}{offset}"));

        args
    }
}

impl<'a> ToArgs for AudioOutput<'a> {
    fn args(&self) -> Vec<String> {
        vec![
            "-f".to_string(),
            "pulse".to_string(),
            "-i".to_string(),
            self.id.to_string(),
        ]
    }
}
