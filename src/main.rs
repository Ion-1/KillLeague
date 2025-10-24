#![windows_subsystem = "windows"]

use qmetaobject::prelude::*;
use std::ffi::OsStr;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

enum ProcNames {
    LoLGame,
    LoLClient,
    RiotClient,
    RiotService,
}
impl ProcNames {
    #[cfg(target_os = "windows")]
    const fn value(&self) -> &'static str {
        match self {
            ProcNames::LoLGame => "League of Legends.exe",
            ProcNames::LoLClient => "LeagueClient.exe",
            ProcNames::RiotClient => "Riot Client.exe",
            ProcNames::RiotService => "RiotClientServices.exe",
        }
    }
    #[cfg(target_os = "macos")]
    const fn value(&self) -> &'static str {
        todo!("Don't have MacOS strings.")
    }
}

#[derive(QObject, Default)]
struct Backend {
    base: qt_base_class!(trait QObject),

    kill_league: qt_method!(
        fn kill_league(&mut self) {
            self.kill_proc(ProcNames::LoLGame.value());
        }
    ),

    kill_client: qt_method!(
        fn kill_client(&mut self) {
            self.kill_proc(ProcNames::LoLClient.value());
        }
    ),

    kill_riot: qt_method!(
        fn kill_riot(&mut self) {
            self.kill_proc(ProcNames::RiotClient.value());
        }
    ),

    kill_service: qt_method!(
        fn kill_service(&mut self) {
            self.kill_proc(ProcNames::RiotService.value());
        }
    ),

    pub system: System,
}
impl Backend {
    fn kill_proc(&mut self, name: impl AsRef<OsStr>) {
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );
        self.system
            .processes_by_exact_name(name.as_ref())
            .for_each(|proc| {proc.kill();});
    }
}

fn main() {
    let mut engine = QmlEngine::new();

    let backend = QObjectBox::new(Backend::default());
    engine.set_object_property(QString::from("Backend"), backend.pinned());

    engine.load_data(include_str!("ui.qml").into());

    engine.exec()
}
